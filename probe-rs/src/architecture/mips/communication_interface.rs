#![allow(missing_docs)]
use super::ejtag::{Ejtag, EjtagVersion};
use crate::{
    memory_mapped_bitfield_register, DebugProbeError, Error as ProbeRsError, MemoryMappedRegister,
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

pub trait MultipleRegisterOffsets<T>: MemoryMappedRegister<T> {
    const N_OFFSET: u32;

    fn get_mmio_address_by_id(base_address: u64, index: u32) -> Result<u64, anyhow::Error> {
        if let Some(actual_offset) =
            Self::ADDRESS_OFFSET.checked_add((Self::N_OFFSET * index).into())
        {
            if let Some(actual_address) = base_address.checked_add(actual_offset) {
                return Ok(actual_address);
            }
        }
        Err(
            anyhow::anyhow!(
                "Overflow while calculating the MMIO address for {}{} at offset {:#x} from base address {:#x}", 
                Self::NAME, 
                index, 
                Self::ADDRESS_OFFSET, 
                base_address
            )
        )
    }
}

memory_mapped_bitfield_register! {
    struct IBS(u32);
    0x1000, "ibs",
    impl From;

    asid_up, _: 30;
    bcn, _: 27, 24;
    bp3, set_bp3: 3;
    bp2, set_bp2: 2;
    bp1, set_bp1: 1;
    bp0, set_bp0: 0;
}

memory_mapped_bitfield_register! {
    struct IBA(u32);
    0x1100, "iba",
    impl From;

    iba, set_iba: 31, 1;
    isa, set_isa: 0;
}

memory_mapped_bitfield_register! {
    struct IBM(u32);
    0x1108, "ibm",
    impl From;

    ibm, set_ibm: 31, 1;
    isam, set_isam: 0;
}

memory_mapped_bitfield_register! {
    struct IBASID(u32);
    0x1110, "ibasid",
    impl From;

    asid, set_asid: 7, 0;
}

memory_mapped_bitfield_register! {
    struct IBC(u32);
    0x1118, "ibc",
    impl From;

    tc, set_tc: 31, 24;
    asid_use, set_asid_use: 23;
    tc_use, _: 22;
    te, set_te: 2;
    be, set_be: 0;
}

impl MultipleRegisterOffsets<u32> for IBA {
const N_OFFSET: u32 = 0x100;
}
impl MultipleRegisterOffsets<u32> for IBM {
const N_OFFSET: u32 = 0x100;
}
impl MultipleRegisterOffsets<u32> for IBASID {
const N_OFFSET: u32 = 0x100;
}
impl MultipleRegisterOffsets<u32> for IBC {
const N_OFFSET: u32 = 0x100;
}
