use bevy::{prelude::*, winit::WinitSettings};
use bevy_ledger::{ui::Ui2DPlugin, LedgerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Ledger".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(LedgerPlugin) // <- I made these this time
        .add_plugin(Ui2DPlugin) // <-
        .insert_resource(WinitSettings::desktop_app())
        .run();
}
