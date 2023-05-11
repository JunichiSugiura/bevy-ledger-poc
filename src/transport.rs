use crate::{
    apdu::{APDUAnswer, APDUCommand},
    constant::{LEDGER_CHANNEL, LEDGER_PACKET_READ_SIZE, LEDGER_PACKET_WRITE_SIZE, LEDGER_TIMEOUT},
    device::Device,
    error::DeviceHIDError,
};
use bevy::{ecs::system::Resource, log};
use byteorder::{BigEndian, ReadBytesExt};
use hidapi::{HidApi, HidDevice};
use std::{io::Cursor, ops::Deref, sync::Mutex};

#[derive(Resource)]
pub struct Transport {
    device: Mutex<HidDevice>,
}

impl Transport {
    pub fn open(device: &Device) -> Result<Transport, DeviceHIDError> {
        let api = Self::api();
        let device = device.open(&api)?;
        device.set_blocking_mode(true)?;
        let transport = Transport::new(device);

        Ok(transport)
    }

    pub fn new(device: HidDevice) -> Self {
        Self {
            device: Mutex::new(device),
        }
    }

    pub fn exchange<I: Deref<Target = [u8]>>(
        &self,
        command: APDUCommand<I>,
    ) -> eyre::Result<APDUAnswer<Vec<u8>>> {
        let device = &self.device.lock().unwrap();
        Self::write_apdu(&device, LEDGER_CHANNEL, &command.serialize())?;

        let mut answer: Vec<u8> = Vec::with_capacity(256);
        Self::read_apdu(&device, LEDGER_CHANNEL, &mut answer)?;

        let answer = APDUAnswer::from_answer(answer)
            .map_err(|_| DeviceHIDError::Comm("response was too short"))?;

        Ok(answer)
    }

    fn api() -> HidApi {
        HidApi::new().unwrap()
    }

    fn write_apdu(device: &HidDevice, channel: u16, apdu_command: &[u8]) -> eyre::Result<i32> {
        let command_length = apdu_command.len() as usize;
        let mut in_data = Vec::with_capacity(command_length + 2);
        in_data.push(((command_length >> 8) & 0xFF) as u8);
        in_data.push((command_length & 0xFF) as u8);
        in_data.extend_from_slice(apdu_command);

        let mut buffer = vec![0u8; LEDGER_PACKET_WRITE_SIZE as usize];
        // Windows platform requires 0x00 prefix and Linux/Mac tolerate this as well
        buffer[0] = 0x00;
        buffer[1] = ((channel >> 8) & 0xFF) as u8; // channel big endian
        buffer[2] = (channel & 0xFF) as u8; // channel big endian
        buffer[3] = 0x05u8;

        for (sequence_idx, chunk) in in_data
            .chunks((LEDGER_PACKET_WRITE_SIZE - 6) as usize)
            .enumerate()
        {
            buffer[4] = ((sequence_idx >> 8) & 0xFF) as u8; // sequence_idx big endian
            buffer[5] = (sequence_idx & 0xFF) as u8; // sequence_idx big endian
            buffer[6..6 + chunk.len()].copy_from_slice(chunk);

            log::info!("[{:3}] << {:}", buffer.len(), hex::encode(&buffer));

            let result = device.write(&buffer);

            match result {
                Ok(size) => {
                    if size < buffer.len() {
                        return Err(DeviceHIDError::Comm(
                            "USB write error. Could not send whole message",
                        )
                        .into());
                    }
                }
                Err(x) => return Err(DeviceHIDError::Hid(x).into()),
            }
        }
        Ok(1)
    }

    fn read_apdu(
        device: &HidDevice,
        channel: u16,
        apdu_answer: &mut Vec<u8>,
    ) -> eyre::Result<usize> {
        let mut buffer = vec![0u8; LEDGER_PACKET_READ_SIZE as usize];
        let mut sequence_idx = 0u16;
        let mut expected_apdu_len = 0usize;

        loop {
            let res = device.read_timeout(&mut buffer, LEDGER_TIMEOUT)?;

            if (sequence_idx == 0 && res < 7) || res < 5 {
                return Err(DeviceHIDError::Comm("Read error. Incomplete header").into());
            }

            let mut rdr = Cursor::new(&buffer);

            let rcv_channel = rdr.read_u16::<BigEndian>()?;
            let rcv_tag = rdr.read_u8()?;
            let rcv_seq_idx = rdr.read_u16::<BigEndian>()?;

            if rcv_channel != channel {
                return Err(DeviceHIDError::Comm("Invalid channel").into());
            }
            if rcv_tag != 0x05u8 {
                return Err(DeviceHIDError::Comm("Invalid tag").into());
            }

            if rcv_seq_idx != sequence_idx {
                return Err(DeviceHIDError::Comm("Invalid sequence idx").into());
            }

            if rcv_seq_idx == 0 {
                expected_apdu_len = rdr.read_u16::<BigEndian>()? as usize;
            }

            let available: usize = buffer.len() - rdr.position() as usize;
            let missing: usize = expected_apdu_len - apdu_answer.len();
            let end_p = rdr.position() as usize + std::cmp::min(available, missing);

            let new_chunk = &buffer[rdr.position() as usize..end_p];

            log::info!("[{:3}] << {:}", new_chunk.len(), hex::encode(&new_chunk));

            apdu_answer.extend_from_slice(new_chunk);

            if apdu_answer.len() >= expected_apdu_len {
                return Ok(apdu_answer.len());
            }

            sequence_idx += 1;
        }
    }
}
