use crate::{
    device::Device,
    event::{GetVersion, OpenApp, ScanDevices},
};
use bevy::{ecs::entity::Entity, log, prelude::*};

pub struct Ui2DPlugin;

impl Plugin for Ui2DPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_buttons).add_systems((
            button_hover_style,
            on_click_scan_devices,
            on_click_get_version,
            on_click_open_app,
        ));
    }
}

#[derive(Component)]
struct ScanButton;

#[derive(Component)]
struct GetVersionButton;

#[derive(Component)]
struct OpenAppButton;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);

/// Spawn buttons
fn spawn_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Scan devices button
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

    // Get version button
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
                    GetVersionButton,
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

    // Open app button
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
                    OpenAppButton,
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

/// Change button color on mouse hover
fn button_hover_style(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    query.for_each_mut(|(interaction, mut color)| {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        };
    });
}

/// Emit `ScanDevices` event when user clicks button
fn on_click_scan_devices(
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

/// Emit `GetDeviceInfo` event on click
fn on_click_get_version(
    interactions: Query<&Interaction, (Changed<Interaction>, With<Button>, With<GetVersionButton>)>,
    devices: Query<(Entity, &Device)>,
    mut get_version: EventWriter<GetVersion>,
) {
    interactions.for_each(|interaction| {
        match *interaction {
            Interaction::Clicked => {
                if devices.is_empty() {
                    log::error!(
                        "No device is detected by device manager. Make sure to scan devices first."
                    );
                } else {
                    get_version.send(GetVersion {
                        // Todo: Let user choose a device
                        device_id: devices.single().0,
                    });
                }
            }
            _ => {}
        };
    });
}

/// Emit `OpenDeviceApp` event on click
fn on_click_open_app(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<OpenAppButton>)>,
    devices: Query<(Entity, &Device)>,
    mut open_app: EventWriter<OpenApp>,
) {
    query.for_each(|interaction| {
        match *interaction {
            Interaction::Clicked => {
                open_app.send(OpenApp {
                    // Todo: Let user choose a device
                    device_id: devices.single().0,
                    name: "Ethereum",
                });
            }
            _ => {}
        };
    });
}
