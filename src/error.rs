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
/// Common known APDU error codes
pub enum APDUErrorCode {
    ///success
    NoError = 0x9000,
    ///error during apdu execution
    ExecutionError = 0x6400,
    ///apdu command wrong length
    WrongLength = 0x6700,
    ///empty apdu buffer
    EmptyBuffer = 0x6982,
    ///apdu buffer too small
    OutputBufferTooSmall = 0x6983,
    ///apdu parameters invalid
    DataInvalid = 0x6984,
    ///apdu preconditions not satisfied
    ConditionsNotSatisfied = 0x6985,
    ///apdu command not allowed
    CommandNotAllowed = 0x6986,
    ///apdu data field incorrect (bad key)
    BadKeyHandle = 0x6A80,
    ///apdu p1 or p2 incorrect
    InvalidP1P2 = 0x6B00,
    ///apdu instruction not supported or invalid
    InsNotSupported = 0x6D00,
    ///apdu class not supported or invalid
    ClaNotSupported = 0x6E00,
    ///unknown apdu error
    Unknown = 0x6F00,
    ///apdu sign verify error
    SignVerifyError = 0x6F01,
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
/// Error interpreting bytes as an APDU answer
pub enum APDUAnswerError {
    #[error("answer too short (< 2 bytes)")]
    /// Passed APDU answer was less than the minimum 2 bytes required for the return code
    TooShort,
}
