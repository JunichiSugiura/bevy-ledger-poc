use bevy::ecs::entity::Entity;

/// Event for scanning Ledger devices connected via HID
pub struct ScanDevices;

/// Get device information including the versions of its components, the onboarding status and its current state.
pub struct GetVersion {
    pub device_id: Entity,
}

/// Get information on the application currently running on the device. When no application is running on the device we will get BOLOS (or OLOS for very old firmware versions) meaning the device is currently on the dashboard.
pub struct GetAppAndVersion;

/// List all the applications installed on a device alongside their versions.
pub struct ListApps;

/// Prompt to open an application by name as seen on the manager api endpoint. Note that the successful execution of this command will also trigger a disconnect meaning we will get the response following by a connection loss and a reconnection (on USB at least) when the device lands on target app.
pub struct OpenApp {
    pub device_id: Entity,
    pub name: &'static str,
}

/// Quit the application currently running on the connected device. Note that the successful execution of this command will also trigger a disconnect meaning we will get the response following by a connection loss and a reconnection (on USB at least) when the device lands on the dashboard (BOLOS app)
pub struct QuitApp;

/// Get the device name, this is a user blocking APDU where the user can refuse the operation.
pub struct GetDeviceName;

///
pub struct EditDeviceName;

pub struct UninstallLanguage;

pub struct StaxFetchImageSize;

pub struct GetBatteryState;
