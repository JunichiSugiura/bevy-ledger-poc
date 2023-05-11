use bevy::ecs::entity::Entity;

pub struct ScanDevices;

pub struct GetDeviceInfo {
    pub device_id: Entity,
}

pub struct OpenDeviceApp {
    pub device_id: Entity,
}
