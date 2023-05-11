use crate::{error::DeviceHIDError, transport::Transport, Device};
use bevy::ecs::system::Resource;
use hidapi::{DeviceInfo, HidApi};

#[derive(Resource)]
pub struct HidManager {
    inner: HidApi,
}

impl HidManager {
    pub fn new(inner: HidApi) -> Self {
        Self { inner }
    }

    pub fn list(&self) -> impl Iterator<Item = &DeviceInfo> {
        self.inner.device_list()
    }

    pub fn open(&self, device: &Device) -> Result<Transport, DeviceHIDError> {
        let device = device.open(&self.inner)?;
        device.set_blocking_mode(true)?;
        let transport = Transport::new(device);

        Ok(transport)
    }
}
