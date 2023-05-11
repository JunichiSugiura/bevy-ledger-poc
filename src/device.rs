use crate::constant::{LEDGER_USAGE_PAGE, LEDGER_VID};
use bevy::ecs::component::Component;
use hidapi::{DeviceInfo, HidApi, HidDevice, HidResult};
use std::fmt;

#[derive(Component)]
pub struct Device {
    inner: DeviceInfo,
}

impl Device {
    pub fn is_ledger(&self) -> bool {
        self.inner.vendor_id() == LEDGER_VID && self.inner.usage_page() == LEDGER_USAGE_PAGE
    }

    pub fn open(&self, hidapi: &HidApi) -> HidResult<HidDevice> {
        self.inner.open_device(&hidapi)
    }
}

impl From<DeviceInfo> for Device {
    fn from(inner: DeviceInfo) -> Self {
        Self { inner }
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:04x}:{:04x}",
            self.inner.vendor_id(),
            self.inner.product_id()
        )
    }
}
