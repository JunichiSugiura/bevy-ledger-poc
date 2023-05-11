use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceHIDError {
    /// Device not found error
    #[error("Ledger device not found")]
    DeviceNotFound,
    /// Communication error
    #[error("Ledger device: communication error `{0}`")]
    Comm(&'static str),
    /// i/o error
    #[error("Ledger device: i/o error")]
    Io(#[from] std::io::Error),
    /// HID error
    #[error("Ledger device: Io error")]
    Hid(#[from] hidapi::HidError),
    /// UT8F error
    #[error("Ledger device: UTF8 error")]
    UTF8(#[from] std::str::Utf8Error),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
/// A list of known error values that the device can send back:
/// `6f00` ~ `6fff`: Invalid parameter received
pub enum APDUErrorCode {
    // From Ledger Live src
    PinRemainingAttempts = 0x63c0,
    /// The APDU has an incorrect length
    IncorrectLength = 0x6700,
    /// The APDU is missing a critical parameter
    MissingCriticalParameter = 0x6800,
    CommandIncompatibleFileStructure = 0x6981,
    /// The connection with the device has somme not satisfied security (dongle locked or have invalid access rights)
    SecurityStatusNotSatisfied = 0x6982,
    /// Condition of use not satisfied (denied by the user ?)
    ConditionsOfUseNotSatisfied = 0x6985,
    /// Invalid data received
    IncorrectData = 0x6a80,
    NotEnoughMoneySpace = 0x6a84,
    ReferencedDataNotFound = 0x6a88,
    FileAlreadyExists = 0x6a89,
    /// Invalid parameter received
    IncorrectP1P2 = 0x6b00,
    InsNotSupported = 0x6d00,
    ClaNotSupported = 0x6e00,
    TechnicalProblem = 0x6f00,
    MemoryProblem = 0x9240,
    NoEFSelected = 0x9400,
    InvalidOffset = 0x9402,
    FileNotFound = 0x9404,
    InconsistentFile = 0x9408,
    AlgorithmNotSupported = 0x9484,
    InvalidKCV = 0x9485,
    CodeNotInitialized = 0x9802,
    AccessConditionNotFulfilled = 0x9804,
    ContradictionSecretCodeStatus = 0x9808,
    ContradictionInvalidation = 0x9810,
    CodeBlocked = 0x9840,
    MaxValueReached = 0x9850,
    GPAuthFailed = 0x6300,
    Licensing = 0x6f42,
    Halted = 0x6faa,
    /// Starting from FTS firmware version 0.4.4 this error means that the device is currently locked. Only happens on seeded device.
    LockedDevice = 0x5515,
    /// No more space in the memory to add something. It can occur when adding an app, or a language pack, or an locked screen image.
    NotEnoughSpace = 0x5102,

    // From ledger-rs
    /// Success
    NoError = 0x9000,
    /// Error during apdu execution
    ExecutionError = 0x6400,
    /// Apdu buffer too small
    OutputBufferTooSmall = 0x6983,
    /// Apdu parameters invalid
    DataInvalid = 0x6984,
    /// Apdu command not allowed
    CommandNotAllowed = 0x6986,
    /// Apdu sign verify error
    SignVerifyError = 0x6F01,
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
/// Error interpreting bytes as an APDU answer
pub enum APDUAnswerError {
    #[error("answer too short (< 2 bytes)")]
    /// Passed APDU answer was less than the minimum 2 bytes required for the return code
    TooShort,
}
