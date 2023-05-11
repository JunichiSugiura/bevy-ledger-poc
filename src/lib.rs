mod apdu;
mod constant;
mod device;
mod error;
pub mod event;
mod transport;
pub mod ui;

use apdu::APDUCommand;
use bevy::{log, prelude::*};
use constant::{CLA_DEVICE_INFO, CLA_OPEN_APP, INS_DEVICE_INFO, INS_OPEN_APP};
use device::Device;
use event::{GetDeviceInfo, OpenDeviceApp, ScanDevices};
use hidapi::HidApi;
use transport::Transport;

pub struct DevicePlugin;

impl Plugin for DevicePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScanDevices>()
            .add_event::<GetDeviceInfo>()
            .add_event::<OpenDeviceApp>()
            .add_systems((
                scan_devices,
                log_added_devices,
                get_device_info,
                open_device_app,
            ));
    }
}

/// Scan Ledger devices connected via USB
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

/// Request device info (getVersion)
// Todo: Ledger device: communication error `response was too short`
fn get_device_info(mut events: EventReader<GetDeviceInfo>, query: Query<(Entity, &Device)>) {
    events.iter().for_each(|e| {
        query.iter().for_each(|(device_id, device)| {
            if e.device_id == device_id {
                match Transport::open(&device) {
                    Ok(t) => {
                        let cmd = APDUCommand {
                            cla: CLA_DEVICE_INFO,
                            ins: INS_DEVICE_INFO,
                            p1: 0x00,
                            p2: 0x00,
                            data: Vec::<u8>::new(),
                        };

                        match t.exchange(cmd) {
                            Ok(res) => {
                                log::info!("{res:?}");
                            }
                            Err(e) => {
                                log::error!("{e}");
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("{e}");
                    }
                }
            }
        });
    });
}

/// Open specific firmware app
/// https://ledgerhq.atlassian.net/wiki/spaces/WALLETCO/pages/3753377984/An+attempt+at+APDU+specs#openApp-e0d80000xx
// Todo: Ledger device: communication error `response was too short`
fn open_device_app(mut events: EventReader<OpenDeviceApp>, query: Query<(Entity, &Device)>) {
    events.iter().for_each(|e| {
        query.iter().for_each(|(device_id, device)| {
            if e.device_id == device_id {
                match Transport::open(&device) {
                    Ok(t) => {
                        let cmd = APDUCommand {
                            cla: CLA_OPEN_APP,
                            ins: INS_OPEN_APP,
                            p1: 0x00,
                            p2: 0x00,
                            data: Vec::from(e.name.as_bytes()),
                        };

                        match t.exchange(cmd) {
                            Ok(res) => {
                                log::info!("{res:?}");
                            }
                            Err(e) => {
                                log::error!("{e}");
                            }
                        }
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
