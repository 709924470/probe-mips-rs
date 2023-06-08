#![allow(missing_docs)]
use super::ejtag::{Ejtag, EjtagVersion};
use crate::{DebugProbeError, Error as ProbeRsError};

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
    ejtag: Ejtag,
    state: MipsCommunicationInterfaceState,
}
