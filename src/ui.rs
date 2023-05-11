use crate::{
    device::Device,
    event::{GetDeviceInfo, OpenDeviceApp, ScanDevices},
};
use bevy::{ecs::entity::Entity, log, prelude::*};

pub struct Ui2DPlugin;

impl Plugin for Ui2DPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui).add_systems((
            button_color,
            scan_devices_button,
            device_info_button,
            open_device_app_button,
        ));
    }
}

#[derive(Component)]
struct ScanButton;

#[derive(Component)]
struct DeviceInfoButton;

#[derive(Component)]
struct OpenDeviceAppButton;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Scan button
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                padding: UiRect::vertical(Val::Px(10.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ScanButton,
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(120.0), Val::Px(40.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Scan devices",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });

    // Device info button
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                padding: UiRect::vertical(Val::Px(10.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(120.0), Val::Px(40.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    DeviceInfoButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Device info",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });

    // Open device app button
    // Todo: Text field
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                padding: UiRect::vertical(Val::Px(10.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(200.0), Val::Px(40.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    OpenDeviceAppButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Open Ethereum app",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn button_color(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    query.for_each_mut(|(interaction, mut color)| match *interaction {
        Interaction::Clicked => {
            *color = PRESSED_BUTTON.into();
        }
        Interaction::Hovered => {
            *color = HOVERED_BUTTON.into();
        }
        Interaction::None => {
            *color = NORMAL_BUTTON.into();
        }
    });
}

fn scan_devices_button(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<ScanButton>)>,
    mut scan_devices: EventWriter<ScanDevices>,
) {
    query.for_each(|interaction| {
        match *interaction {
            Interaction::Clicked => {
                scan_devices.send(ScanDevices);
            }
            _ => {}
        };
    });
}

fn device_info_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<Button>, With<DeviceInfoButton>)>,
    devices: Query<(Entity, &Device)>,
    mut get_device_info: EventWriter<GetDeviceInfo>,
) {
    interactions.for_each(|interaction| {
        match *interaction {
            Interaction::Clicked => {
                if devices.is_empty() {
                    log::error!(
                        "No device is detected by device manager. Make sure to scan devices first."
                    );
                } else {
                    get_device_info.send(GetDeviceInfo {
                        // Todo: Let user choose a device
                        device_id: devices.single().0,
                    });
                }
            }
            _ => {}
        };
    });
}

fn open_device_app_button(
    query: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<Button>,
            With<OpenDeviceAppButton>,
        ),
    >,
    devices: Query<(Entity, &Device)>,
    mut open_device_app: EventWriter<OpenDeviceApp>,
) {
    query.for_each(|interaction| {
        match *interaction {
            Interaction::Clicked => {
                open_device_app.send(OpenDeviceApp {
                    // Todo: Let user choose a device
                    device_id: devices.single().0,
                    name: "ethereum",
                });
            }
            _ => {}
        };
    });
}
