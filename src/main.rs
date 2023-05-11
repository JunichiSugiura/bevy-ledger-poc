use bevy::{prelude::*, winit::WinitSettings};
use bevy_ledger::{ui::Ui2DPlugin, LedgerPlugins};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Ledger".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LedgerPlugins) // <- I made these this time
        .add_plugin(Ui2DPlugin) // <-
        .insert_resource(WinitSettings::desktop_app())
        .run();
}
