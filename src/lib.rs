mod apdu;
mod constant;
mod device;
mod error;
pub mod event;
mod transport;
pub mod ui;

use apdu::APDUCommand;
use bevy::{log, prelude::*};
use constant::{CLA_DEVICE_INFO, INS_DEVICE_INFO};
use device::Device;
use event::{GetDeviceInfo, ScanDevices};
use hidapi::HidApi;
use transport::Transport;

pub struct DevicePlugin;

impl Plugin for DevicePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScanDevices>()
            .add_event::<GetDeviceInfo>()
            .add_systems((scan_devices, log_added_devices, get_device_info));
    }
}

fn scan_devices(mut events: EventReader<ScanDevices>, mut commands: Commands) {
    events.iter().for_each(|_e| {
        log::info!("Scanning devices");

        let api = HidApi::new().unwrap();
        let mut is_empty = true;

        api.device_list().for_each(|d| {
            let device = Device::from(d.clone());
            if device.is_ledger() {
                is_empty = false;
                commands.spawn(device);
            }
        });

        if is_empty {
            log::info!("Cannot find any Ledger devices. Make sure your device is connected.");
        }
    });
}

fn get_device_info(mut events: EventReader<GetDeviceInfo>, query: Query<(Entity, &Device)>) {
    events.iter().for_each(|e| {
        query.iter().for_each(|(device_id, device)| {
            if e.device_id == device_id {
                let t = Transport::open(&device).expect("Failed to open transport");
                let cmd = APDUCommand {
                    cla: CLA_DEVICE_INFO,
                    ins: INS_DEVICE_INFO,
                    p1: 0x00,
                    p2: 0x00,
                    data: Vec::<u8>::new(),
                };

                match t.exchange(cmd) {
                    Ok(res) => {
                        log::info!("{res:#?}");
                    }
                    Err(e) => {
                        log::error!("{e}");
                    }
                }
            }
        });
    });
}

// fn open_device_app(
//     mut events: EventReader<GetDeviceInfo>,
//     hid_manager: Res<HidManager>,
//     query: Query<(Entity, &Device)>,
// ) {
// }

fn log_added_devices(query: Query<&Device, Added<Device>>) {
    query.iter().for_each(|d| {
        log::info!("New device added: {d}");
    });
}
