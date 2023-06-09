#![allow(unused, missing_docs)]
use std::time::Duration;

use crate::{
    core::CoreRegisters, error::Error, memory_mapped_bitfield_register, probe::JTAGAccess,
    CoreInformation, CoreInterface, CoreRegister, MemoryInterface, RegisterId, RegisterValue,
};
use anyhow::Result;
use bitfield::bitfield;
use probe_rs_target::{Architecture, CoreType, InstructionSet};

use crate::CoreStatus;

use self::{
    communication_interface::MipsCommunicationInterface,
    registers::{MIPS32_CORE_REGSISTERS, MIPS32_WITH_FPU_CORE_REGSISTERS},
};

pub mod communication_interface;
pub mod sequences;

pub mod assembly;
pub mod ejtag;
pub mod registers;

bitfield! {
    pub struct Config1(u32);
    impl Debug;

    M, _: 			31;
    MMUSize, _:		30, 25;
    IS, _: 			24, 22;
    IL, _: 			21, 19;
    IA, _: 			18, 16;
    DS, _: 			15, 13;
    DL, _: 			12, 10;
    DA, _: 			9, 7;
    C2, _: 			6;
    MD, _: 			5;
    PC, _: 			4;
    WR, _: 			3;
    CA, _: 			2;
    EP, _: 			1;
    FP, _: 			0;
}

memory_mapped_bitfield_register! {
    /// The Debug Control register (DCR)
    /// A register located at DRSEG+0x0000
    pub struct DebugCtrl(u32);
    0x00, "dcr",
    impl From;

    enm, _:                29;
    pcim, _:               26;
    pc_noasid, _:          25;
    dasq, _:               24;
    dasen, _:              23;
    das, _:                22;
    fdc_impl, _:           18;
    data_brk, _:           17;
    inst_brk, _:           16;
    ivm, _:                15;
    dvm, _:                14;
    rdvec, set_rdvec:   11;
    cbt, _:                10;
    pcs, _:                9;
    pcr, set_pcr:       8, 6;
    pcse, set_pcs:      5;
    int_en, set_int:    4;
    nmi_en, set_nmi:    3;
    nmi_pend, _:           2;
    srst_en, set_srst:  1;
    prob_en, _:            0;
}

bitfield! {
    struct ImpCode(u32);
    impl Debug;

    ejtag_ver, _:           31, 29;
    dint_impl, _:           24;
    asid_size, _:           23, 21;
    mips16, _:              16;
    no_dma, _:              14;
    typ, _:                 13, 11;
    typ_info, _:            10, 1;
}

bitfield! {
    struct IDCode(u32);
    impl Debug;

    version, _:             31, 28;
    part_no, _:             27, 12;
    manuf_id, _:            11, 1;
}

bitfield! {
    struct EjtagCtrl(u32);
    impl Debug;

    rocc, set_rocc:         31;
    psz, _:                 30, 29;
    vpe_de, _:              23;
    doze, _:                22;
    halt, _:                21;
    per_rst, set_perrst:    20;
    prnw, _:                19;
    pracc, set_pracc:       18;
    prrst, set_prrst:       16;
    prob_en, set_prob_en:   15;
    prob_trap, set_trap:    14;
    ejtag_brk, set_brk:     12;
    dbg_mode, _:            3;
}

// fastdata register is omitted, since it only contains 1 single bit spracc r/w able

#[repr(C)]
pub struct EjtagData {
    ctrl: [u8; 4],
    data: [u8; 4],
    addr: [u8; 4],
}

#[derive(Debug)]
pub struct MipsState {
    cfg: Config1,
}

impl MipsState {
    pub(crate) fn new() -> Self {
        Self { cfg: Config1(0) }
    }
}

pub struct Mips32<'probe> {
    interface: &'probe mut MipsCommunicationInterface,
    state: &'probe mut MipsState,
    probe: Box<dyn JTAGAccess + 'probe>,
}

impl<'probe> Mips32<'probe> {
    pub fn new(
        mut probe: Box<dyn JTAGAccess + 'probe>,
        interface: &'probe mut MipsCommunicationInterface,
        state: &'probe mut MipsState,
    ) -> Self {
        Self {
            interface,
            state,
            probe,
        }
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

    fn registers(&self) -> &'static CoreRegisters {
        if !self.state.cfg.FP() {
            &MIPS32_CORE_REGSISTERS
        } else {
            &MIPS32_WITH_FPU_CORE_REGSISTERS
        }
    }

    fn hw_breakpoints_enabled(&self) -> bool {
        self.interface.ejtag.debug_ctrl.inst_brk()
    }

    fn debug_on_sw_breakpoint(&mut self, _enabled: bool) -> Result<(), Error> {
        // This default will have override methods for architectures that require special behavior, e.g. RISV-V.
        Ok(())
    }

    fn architecture(&self) -> Architecture {
        Architecture::Mips
    }

    fn core_type(&self) -> CoreType {
        CoreType::Mips
    }

    fn instruction_set(&mut self) -> Result<InstructionSet, Error> {
        todo!("Get ISA on PC")
    }

    fn fpu_support(&mut self) -> Result<bool, Error> {
        Ok(self.state.cfg.FP())
    }

    fn on_session_stop(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn program_counter(&self) -> &'static CoreRegister {
        &super::mips::registers::PC
    }

    fn frame_pointer(&self) -> &'static CoreRegister {
        &super::mips::registers::FP
    }

    fn stack_pointer(&self) -> &'static CoreRegister {
        &super::mips::registers::SP
    }

    fn return_address(&self) -> &'static CoreRegister {
        &super::mips::registers::RA
    }
}

impl<'probe> MemoryInterface for Mips32<'probe> {
    fn supports_native_64bit_access(&mut self) -> bool {
        false
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
        Ok(true)
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
