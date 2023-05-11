use std::ops::Deref;

#[derive(Debug, Clone)]
/// An APDU command
pub struct APDUCommand<B> {
    /// APDU Class
    ///
    /// An incorrect APDU Class will prevent you from communicating with the device
    pub cla: u8,
    /// APDU Instruction
    pub ins: u8,
    /// First parameter of instruction
    pub p1: u8,
    /// Second parameter of instruction
    pub p2: u8,
    /// Payload of the instruction, can be empty
    pub data: B,
}

// #[cfg(feature = "std")]
impl<B> APDUCommand<B>
where
    B: Deref<Target = [u8]>,
{
    /// Serialize this [APDUCommand] to be sent to the device
    pub fn serialize(&self) -> std::vec::Vec<u8> {
        let mut v = std::vec![self.cla, self.ins, self.p1, self.p2, self.data.len() as u8];
        v.extend(self.data.iter());
        v
    }
}

#[derive(Debug)]
/// An APDU answer, whole last 2 bytes are interpreted as `retcode`
pub struct APDUAnswer<B> {
    data: B,
    retcode: u16,
}

impl<B> APDUAnswer<B>
where
    B: std::ops::Deref<Target = [u8]>,
{
    /// Attempt to interpret the given slice as an APDU answer
    pub fn from_answer(answer: B) -> eyre::Result<Self> {
        // Todo: Investigate whay it exists
        // if answer.len() >= 2 {
        //     return Err(APDUAnswerError::TooShort.into());
        // }
        let retcode = arrayref::array_ref!(answer, answer.len() - 2, 2);
        let retcode = u16::from_be_bytes(*retcode);

        Ok(APDUAnswer {
            data: answer,
            retcode,
        })
    }

    // /// Will return the answer's payload
    // #[inline(always)]
    // pub fn apdu_data(&self) -> &[u8] {
    //     &self.data[..self.data.len() - 2]
    // }

    // /// Will return the answer's payload
    // #[inline(always)]
    // pub fn data(&self) -> &[u8] {
    //     self.apdu_data()
    // }

    // /// Will attempt to interpret the error code as an [APDUErrorCode],
    // /// returning the code as is otherwise
    // pub fn error_code(&self) -> eyre::Result<APDUErrorCode, u16> {
    //     todo!("Parse error code into APDUErrorCode");
    //     // self.retcode.try_into().map_err(|_| self.retcode)
    // }

    // /// Returns the raw return code
    // #[inline(always)]
    // pub fn retcode(&self) -> u16 {
    //     self.retcode
    // }
}
