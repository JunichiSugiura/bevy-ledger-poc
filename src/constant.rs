pub const LEDGER_VID: u16 = 0x2c97;
pub const LEDGER_USAGE_PAGE: u16 = 0xFFA0;
pub const LEDGER_CHANNEL: u16 = 0x0101;
// for Windows compatability, we prepend the buffer with a 0x00
// so the actual buffer is 64 bytes
pub const LEDGER_PACKET_WRITE_SIZE: u8 = 65;
pub const LEDGER_PACKET_READ_SIZE: u8 = 64;
pub const LEDGER_TIMEOUT: i32 = 10_000_000;

pub const CLA_DEVICE_INFO: u8 = 0xe0;
pub const INS_DEVICE_INFO: u8 = 0x01;

pub const CLA_OPEN_APP: u8 = 0xe0;
pub const INS_OPEN_APP: u8 = 0xd8;
