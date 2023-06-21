#![allow(missing_docs)]

use super::ejtag::{Ejtag, EjtagVersion};
use crate::{
    memory::valid_32bit_address, memory_mapped_bitfield_register, probe::JTAGAccess,
    DebugProbeError, Error as ProbeRsError, Error, MemoryInterface, MemoryMappedRegister,
};

/// Some error occurered when working with the MIPS core.
/// Comes from Codescape python api
#[derive(thiserror::Error, Debug)]
pub enum MipsError {
    #[error("Error during halting the target")]
    HaltFailed,
    #[error("Unknown error")]
    Error(#[from] DebugProbeError),
    #[error("Error during setting up breakpoint '{0}'")]
    BreakpointError(u32),
    #[error("Invalid execution state")]
    ExecutionStateError,
    #[error("Invalid argument")]
    InvalidArgError,
    #[error("Operation cancelled")]
    OperationCancelled,
    #[error("Error during connecting to the target")]
    TargetError,
    #[error("Invalid thread id '{0}'")]
    ThreadInvalidError(u16),
    #[error("No argument supplied")]
    ArgumentNotSuppliedError,
    #[error("Assert error")]
    AssertionFailed,
    #[error("Error in callback function")]
    CallBackError,
    #[error("Core is disabled")]
    CoreDisabled,
    #[error("Core is offline")]
    CoreOffline,
    #[error("Error during evaluating the expression")]
    ExpressionEvaluationError,
    #[error("Invalid conversion")]
    InvalidConversion,
    #[error("Error during loading program file")]
    LoadProgramFileError,
    #[error("An reset occurred during transaction")]
    ResetOccurred,
    #[error("Already connected to a target")]
    TargetAlreadyConnected,
    #[error("Target is disconnected")]
    TargetDisconnected,
    #[error("Target is invalid")]
    TargetInvalid,
    #[error("Target not found")]
    TargetNotFound,
    #[error("Fatal memory fault, please reset the target")]
    UnrecoverableMemoryFault,
    #[error("Could not parse program file")]
    ProgramFileError,
    #[error("Invalid program file")]
    NotAnELF,
}

impl From<MipsError> for ProbeRsError {
    fn from(err: MipsError) -> Self {
        match err {
            MipsError::Error(e) => e.into(),
            MipsError::TargetError => ProbeRsError::Timeout,
            other => ProbeRsError::Mips(other),
        }
    }
}

#[derive(Debug)]
pub struct MipsCommunicationInterfaceState {
    ejtag_version: EjtagVersion,
}

#[derive(Debug)]
pub struct MipsCommunicationInterface {
    pub ejtag: Ejtag,
    pub state: MipsCommunicationInterfaceState,
}

impl MipsCommunicationInterface {
    pub fn new(probe: Box<dyn JTAGAccess>) -> Result<Self, (Box<dyn JTAGAccess>, MipsError)> {
        let ejtag = Ejtag::new(probe)?;
        let state = MipsCommunicationInterfaceState {
            ejtag_version: ejtag.ejtag_version,
        };

        Ok(Self { ejtag, state })
    }
}

impl MemoryInterface for MipsCommunicationInterface {
    fn supports_native_64bit_access(&mut self) -> bool {
        // TODO: implement mips64
        false
    }

    fn read_word_64(&mut self, address: u64) -> Result<u64, Error> {
        let lo: u64 = self.read_word_32(address)?.into();
        let hi: u64 = self.read_word_32(address + 4)?.into();

        let result: u64 = hi << 32 | lo;
        Ok(result)
    }

    fn read_word_32(&mut self, address: u64) -> Result<u32, Error> {
        let address = valid_32bit_address(address)?;
        todo!()
    }

    fn read_word_8(&mut self, address: u64) -> Result<u8, Error> {
        todo!()
    }

    fn read_64(&mut self, address: u64, data: &mut [u64]) -> Result<(), Error> {
        todo!()
    }

    fn read_32(&mut self, address: u64, data: &mut [u32]) -> Result<(), Error> {
        todo!()
    }

    fn read_8(&mut self, address: u64, data: &mut [u8]) -> Result<(), Error> {
        todo!()
    }

    fn write_word_64(&mut self, address: u64, data: u64) -> Result<(), Error> {
        todo!()
    }

    fn write_word_32(&mut self, address: u64, data: u32) -> Result<(), Error> {
        todo!()
    }

    fn write_word_8(&mut self, address: u64, data: u8) -> Result<(), Error> {
        todo!()
    }

    fn write_64(&mut self, address: u64, data: &[u64]) -> Result<(), Error> {
        todo!()
    }

    fn write_32(&mut self, address: u64, data: &[u32]) -> Result<(), Error> {
        todo!()
    }

    fn write_8(&mut self, address: u64, data: &[u8]) -> Result<(), Error> {
        todo!()
    }

    fn supports_8bit_transfers(&self) -> Result<bool, Error> {
        Ok(true)
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
