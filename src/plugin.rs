mod general;

use bevy::app::{PluginGroup, PluginGroupBuilder};
use general::GeneralPlugin;

pub struct LedgerPlugins;

impl PluginGroup for LedgerPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(GeneralPlugin)
    }
}
