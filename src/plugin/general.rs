use crate::{
    apdu::APDUCommand, constant::*, device::Device, event::general::*, transport::Transport,
};
use bevy::{log, prelude::*};
use hidapi::HidApi;

pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScanDevices>()
            .add_event::<GetVersion>()
            .add_event::<GetAppAndVersion>()
            .add_event::<ListApps>()
            .add_event::<OpenApp>()
            .add_event::<QuitApp>()
            .add_event::<GetDeviceName>()
            .add_event::<EditDeviceName>()
            .add_event::<UninstallLanguage>()
            .add_event::<StaxFetchImageSize>()
            .add_event::<GetBatteryState>()
            .add_systems((
                scan_devices,
                log_added_devices,
                get_version,
                open_app,
                get_app_and_version,
                list_apps,
                quit_app,
                get_device_name,
                edit_device_name,
                uninstall_language,
                stax_fetch_image_size,
                get_battery_state,
            ));
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

fn log_added_devices(query: Query<&Device, Added<Device>>) {
    query.iter().for_each(|d| {
        log::info!("New device added: {d}");
    });
}

// Todo: Ledger device: communication error `response was too short`
fn get_version(mut events: EventReader<GetVersion>, query: Query<(Entity, &Device)>) {
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

fn get_app_and_version(mut events: EventReader<GetAppAndVersion>) {
    events.iter().for_each(|_e| {
        todo!();
    })
}

fn list_apps(mut events: EventReader<ListApps>) {
    events.iter().for_each(|_e| {
        todo!();
    })
}

// Todo: Ledger device: communication error `response was too short`
fn open_app(mut events: EventReader<OpenApp>, query: Query<(Entity, &Device)>) {
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
                                // Todo: Parse APDUAnswer
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

fn quit_app(mut events: EventReader<QuitApp>) {
    events.iter().for_each(|_e| {
        todo!();
    })
}

fn get_device_name(mut events: EventReader<GetDeviceName>) {
    events.iter().for_each(|_e| {
        todo!();
    })
}

fn edit_device_name(mut events: EventReader<EditDeviceName>) {
    events.iter().for_each(|_e| {
        todo!();
    })
}

fn uninstall_language(mut events: EventReader<UninstallLanguage>) {
    events.iter().for_each(|_e| {
        todo!();
    })
}

fn stax_fetch_image_size(mut events: EventReader<StaxFetchImageSize>) {
    events.iter().for_each(|_e| {
        todo!();
    })
}

fn get_battery_state(mut events: EventReader<GetBatteryState>) {
    events.iter().for_each(|_e| {
        todo!();
    })
}
