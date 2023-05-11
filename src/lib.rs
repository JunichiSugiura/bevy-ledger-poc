mod apdu;
mod constant;
mod device;
mod error;
pub mod event;
mod hid;
mod transport;
pub mod ui;

use apdu::APDUCommand;
use bevy::{log, prelude::*};
use constant::{CLA_DEVICE_INFO, INS_DEVICE_INFO};
use device::Device;
use event::{GetDeviceInfo, ScanDevices};
use hid::HidManager;
use hidapi::HidApi;

pub struct DevicePlugin;

impl Plugin for DevicePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScanDevices>()
            .add_event::<GetDeviceInfo>()
            .add_startup_system(setup_hid_manager)
            .add_systems((scan_devices, log_added_devices, get_device_info));
    }
}

fn setup_hid_manager(mut commands: Commands) {
    log::info!("Setting up HID manager");

    match HidApi::new() {
        Ok(api) => {
            commands.insert_resource(HidManager::new(api));
        }
        Err(e) => {
            log::error!("Error: {}", e);
        }
    }
}

fn scan_devices(
    mut events: EventReader<ScanDevices>,
    mut commands: Commands,
    hid_manager: Res<HidManager>,
) {
    events.iter().for_each(|_e| {
        log::info!("Scanning devices");

        let mut is_empty = true;
        hid_manager.list().for_each(|d| {
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

fn get_device_info(
    mut events: EventReader<GetDeviceInfo>,
    hid_manager: Res<HidManager>,
    query: Query<(Entity, &Device)>,
) {
    events.iter().for_each(|e| {
        query.iter().for_each(|(entity, device)| {
            if e.entity == entity {
                let cmd = APDUCommand {
                    cla: CLA_DEVICE_INFO,
                    ins: INS_DEVICE_INFO,
                    p1: 0x00,
                    p2: 0x00,
                    data: Vec::<u8>::new(),
                };

                let t = hid_manager.open(&device).expect("Failed to open transport");

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

fn log_added_devices(query: Query<&Device, Added<Device>>) {
    query.iter().for_each(|d| {
        log::info!("New device added: {d}");
    });
}
