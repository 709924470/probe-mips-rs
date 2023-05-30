use super::ejtag::Ejtag;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EjtagVersion {
    NoModule,
    EJTAG_V20,
    EJTAG_V25,
    EJTAG_V26,
    EJTAG_V31,
    EJTAG_V41,
    EJTAG_V51,
    Unknown(u8),
}

#[derive(Debug)]
pub enum EjtagError {
    Ok = 0,
    Error,
}

impl From<u8> for EjtagVersion {
    fn from(raw: u8) -> Self {
        match raw {
            0 => EjtagVersion::EJTAG_V20,
            1 => EjtagVersion::EJTAG_V25,
            2 => EjtagVersion::EJTAG_V26,
            3 => EjtagVersion::EJTAG_V31,
            4 => EjtagVersion::EJTAG_V41,
            5 => EjtagVersion::EJTAG_V51,
            255 => EjtagVersion::NoModule,
            other => EjtagVersion::Unknown(other),
        }
    }
}

pub struct MIPSCommunicationInterfaceState {
    ejtag_version: EjtagVersion,
}

pub struct MIPSCommunicationInterface {
    ejtag: Ejtag,
    state: MIPSCommunicationInterfaceState,
}
