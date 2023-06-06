#![allow(unused, missing_docs)]
use std::time::Duration;

use crate::{
    error::Error, memory_mapped_bitfield_register, CoreInformation, CoreInterface, MemoryInterface,
    RegisterFile, RegisterId, RegisterValue,
};
use anyhow::Result;
use probe_rs_target::{Architecture, CoreType, InstructionSet};

use crate::CoreStatus;

use self::communication_interface::MipsCommunicationInterface;

pub mod communication_interface;
pub mod sequences;

pub mod assembly;
pub mod ejtag;
pub mod registers;

#[derive(Debug)]
pub struct MipsState {}

impl MipsState {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

pub struct Mips32<'probe> {
    interface: &'probe mut MipsCommunicationInterface,
    state: &'probe mut MipsState,
}

impl<'probe> Mips32<'probe> {
    pub fn new(
        interface: &'probe mut MipsCommunicationInterface,
        state: &'probe mut MipsState,
    ) -> Self {
        Self { interface, state }
    }
}

impl<'probe> CoreInterface for Mips32<'probe> {
    fn wait_for_core_halted(&mut self, timeout: Duration) -> Result<(), Error> {
        todo!()
    }

    fn core_halted(&mut self) -> Result<bool, Error> {
        todo!()
    }

    fn status(&mut self) -> Result<CoreStatus, Error> {
        todo!()
    }

    fn halt(&mut self, timeout: Duration) -> Result<CoreInformation, Error> {
        todo!()
    }

    fn run(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn reset(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn reset_and_halt(&mut self, timeout: Duration) -> Result<CoreInformation, Error> {
        todo!()
    }

    fn step(&mut self) -> Result<CoreInformation, Error> {
        todo!()
    }

    fn read_core_reg(&mut self, address: RegisterId) -> Result<RegisterValue, Error> {
        todo!()
    }

    fn write_core_reg(&mut self, address: RegisterId, value: RegisterValue) -> Result<(), Error> {
        todo!()
    }

    fn available_breakpoint_units(&mut self) -> Result<u32, Error> {
        todo!()
    }

    fn hw_breakpoints(&mut self) -> Result<Vec<Option<u64>>, Error> {
        todo!()
    }

    fn enable_breakpoints(&mut self, state: bool) -> Result<(), Error> {
        todo!()
    }

    fn set_hw_breakpoint(&mut self, unit_index: usize, addr: u64) -> Result<(), Error> {
        todo!()
    }

    fn clear_hw_breakpoint(&mut self, unit_index: usize) -> Result<(), Error> {
        todo!()
    }

    fn registers(&self) -> &'static RegisterFile {
        todo!()
    }

    fn hw_breakpoints_enabled(&self) -> bool {
        todo!()
    }

    fn architecture(&self) -> Architecture {
        todo!()
    }

    fn core_type(&self) -> CoreType {
        todo!()
    }

    fn instruction_set(&mut self) -> Result<InstructionSet, Error> {
        todo!()
    }

    fn fpu_support(&mut self) -> Result<bool, Error> {
        todo!()
    }

    fn debug_on_sw_breakpoint(&mut self, _enabled: bool) -> Result<(), Error> {
        // This default will have override methods for architectures that require special behavior, e.g. RISV-V.
        Ok(())
    }

    fn on_session_stop(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl<'probe> MemoryInterface for Mips32<'probe> {
    fn supports_native_64bit_access(&mut self) -> bool {
        todo!()
    }

    fn read_word_64(&mut self, address: u64) -> Result<u64, Error> {
        todo!()
    }

    fn read_word_32(&mut self, address: u64) -> Result<u32, Error> {
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
        todo!()
    }

    fn flush(&mut self) -> Result<(), Error> {
        todo!()
    }
}
