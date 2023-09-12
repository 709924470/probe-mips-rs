#![allow(unused, missing_docs, non_snake_case)]
use std::time::Duration;

use crate::{
    core::{CoreRegisters, ExceptionInterface},
    error::Error,
    memory_mapped_bitfield_register,
    probe::JTAGAccess,
    CoreInformation, CoreInterface, CoreRegister, MemoryInterface, MemoryMappedRegister,
    RegisterId, RegisterValue,
};
use anyhow::Result;
use bitfield::bitfield;
use probe_rs_target::{Architecture, CoreType, InstructionSet};

use crate::CoreStatus;

use self::{
    communication_interface::MipsCommunicationInterface,
    ejtag::EJTAG_DRSEG,
    registers::{MIPS32_CORE_REGISTERS, MIPS32_WITH_FPU_CORE_REGISTERS},
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
    pub struct ImpCode(u32);
    impl Debug;

    ejtag_ver, _:           31, 29;
    r3k, _:                 28;
    dint_impl, _:           24;
    asid_size, _:           23, 21;
    mips16, _:              16;
    no_dma, _:              14;
    typ, _:                 13, 11;
    typ_info, _:            10, 1;
    isa, _:                 0;
}

bitfield! {
    pub struct IDCode(u32);
    impl Debug;

    version, _:             31, 28;
    part_no, _:             27, 12;
    manuf_id, _:            11, 1;
}

bitfield! {
    pub struct EjtagCtrl(u32);
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
pub(crate) struct EjtagData {
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
    id: usize,
    interface: &'probe mut MipsCommunicationInterface,
    state: &'probe mut MipsState,

    breakpoints: u32,
}

impl<'probe> Mips32<'probe> {
    pub fn new(
        interface: &'probe mut MipsCommunicationInterface,
        state: &'probe mut MipsState,
        id: usize,
    ) -> Self {
        Self {
            id,
            interface,
            state,
            breakpoints: 0,
        }
    }
}

impl<'probe> ExceptionInterface for Mips32<'probe> {
    fn exception_details(
        &mut self,
        _stackframe_registers: &crate::debug::DebugRegisters,
    ) -> Result<Option<crate::core::ExceptionInfo>, Error> {
        // For architectures where the exception handling has not been implemented in probe-rs,
        // this will result in maintaining the current `unwind` behavior, i.e. unwinding will stop
        // when the first frame is reached that was called from an exception handler.
        Err(Error::NotImplemented(
            "Unwinding of exception frames has not yet been implemented for this architecture.",
        ))
    }

    fn calling_frame_registers(
        &mut self,
        _stackframe_registers: &crate::debug::DebugRegisters,
    ) -> Result<crate::debug::DebugRegisters, crate::Error> {
        Err(Error::NotImplemented(
            "Not implemented for this architecture.",
        ))
    }

    fn exception_description(
        &mut self,
        _stackframe_registers: &crate::debug::DebugRegisters,
    ) -> Result<String, crate::Error> {
        Err(Error::NotImplemented(
            "Not implemented for this architecture.",
        ))
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
        if self.breakpoints == 0 {
            let ibs_off = self.interface.ejtag.ejtag_ibs_offs;
            let ibs_addr = IBS::get_mmio_address_from_base(ibs_off.into())?;
            let ibs = IBS(self.read_word_32(ibs_addr)?);
            self.breakpoints = ibs.bcn();
        }
        let mut bps: Vec<Option<u64>> = Vec::new();
        let offset = self.interface.ejtag.ejtag_iba0_offs;
        let step = self.interface.ejtag.ejtag_iba_step_size;
        for n in 0..self.breakpoints {
            let iban_addr = IBA::get_mmio_address_from_base((n * step + offset).into())?;
            let iban = IBA(self.read_word_32(iban_addr)?);
            bps.push(match iban.0 {
                0 => None,
                val => Some(val.into()),
            });
        }
        Ok(bps)
    }

    fn enable_breakpoints(&mut self, state: bool) -> Result<(), Error> {
        let ibs_off = self.interface.ejtag.ejtag_ibs_offs;
        let ibs_addr = IBS::get_mmio_address_from_base(ibs_off.into())?;
        let mut ibs = IBS(self.read_word_32(ibs_addr)?);
        if self.breakpoints == 0 {
            self.breakpoints = ibs.bcn();
        }
        let mask = if state { 0b1111u32 } else { 0u32 };
        match ibs.bcn() {
            4 => ibs.0 |= mask & 0b1111,
            2 => ibs.0 |= mask & 0b0111,
            3 => ibs.0 |= mask & 0b0011,
            1 => ibs.0 |= mask & 0b0001,
            _ => (),
        }
        self.write_word_32(ibs_addr, ibs.0);
        Ok(())
    }

    fn set_hw_breakpoint(&mut self, unit_index: usize, addr: u64) -> Result<(), Error> {
        if addr & 0xFFFFFFFF_00000000u64 != 0 {
            return Err(Error::Mips(
                communication_interface::MipsError::BreakpointError(addr as u32),
            ));
        }
        if self.breakpoints == 0 {
            let ibs_off = self.interface.ejtag.ejtag_ibs_offs;
            let ibs_addr = IBS::get_mmio_address_from_base(ibs_off.into())?;
            let ibs = IBS(self.read_word_32(ibs_addr)?);
            self.breakpoints = ibs.bcn();
        }
        let offset = self.interface.ejtag.ejtag_iba0_offs;
        let step = self.interface.ejtag.ejtag_iba_step_size;
        let index = unit_index as u32;
        let iban_addr = IBA::get_mmio_address_from_base((index * step + offset).into())?;
        self.write_word_32(iban_addr, addr as u32);
        Ok(())
    }

    fn clear_hw_breakpoint(&mut self, unit_index: usize) -> Result<(), Error> {
        if self.breakpoints == 0 {
            let ibs_off = self.interface.ejtag.ejtag_ibs_offs;
            let ibs_addr = IBS::get_mmio_address_from_base(ibs_off.into())?;
            let ibs = IBS(self.read_word_32(ibs_addr)?);
            self.breakpoints = ibs.bcn();
        }
        let offset = self.interface.ejtag.ejtag_iba0_offs;
        let step = self.interface.ejtag.ejtag_iba_step_size;
        for n in 0..self.breakpoints {
            let index = unit_index as u32;
            let iban_addr = IBA::get_mmio_address_from_base((index * step + offset).into())?;
            self.write_word_32(iban_addr, 0);
        }
        Ok(())
    }

    fn registers(&self) -> &'static CoreRegisters {
        if !self.state.cfg.FP() {
            &MIPS32_CORE_REGISTERS
        } else {
            &MIPS32_WITH_FPU_CORE_REGISTERS
        }
    }

    fn hw_breakpoints_enabled(&self) -> bool {
        /*let Ok(dcr_addr) = DebugCtrl::get_mmio_address_from_base(EJTAG_DRSEG) else {
            return false;
        };
        let Ok(dcr) = self.read_word_32(dcr_addr) else {
            return false;
        };*/
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

    fn id(&self) -> usize {
        self.id
    }

    fn reset_catch_set(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn reset_catch_clear(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn debug_core_stop(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn on_session_stop(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn enable_vector_catch(
        &mut self,
        _condition: crate::VectorCatchCondition,
    ) -> Result<(), Error> {
        Err(Error::NotImplemented("vector catch"))
    }

    fn disable_vector_catch(
        &mut self,
        _condition: crate::VectorCatchCondition,
    ) -> Result<(), Error> {
        Err(Error::NotImplemented("vector catch"))
    }
}

impl<'probe> MemoryInterface for Mips32<'probe> {
    fn supports_native_64bit_access(&mut self) -> bool {
        // TODO: implement mips64
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

memory_mapped_bitfield_register! {
    struct IBS(u32);
    EJTAG_DRSEG, "ibs",
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
    EJTAG_DRSEG, "iba",
    impl From;

    iba, set_iba: 31, 1;
    isa, set_isa: 0;
}

memory_mapped_bitfield_register! {
    struct IBM(u32);
    EJTAG_DRSEG, "ibm",
    impl From;

    ibm, set_ibm: 31, 1;
    isam, set_isam: 0;
}

memory_mapped_bitfield_register! {
    struct IBASID(u32);
    EJTAG_DRSEG, "ibasid",
    impl From;

    asid, set_asid: 7, 0;
}

memory_mapped_bitfield_register! {
    struct IBC(u32);
    EJTAG_DRSEG, "ibc",
    impl From;

    tc, set_tc: 31, 24;
    asid_use, set_asid_use: 23;
    tc_use, _: 22;
    te, set_te: 2;
    be, set_be: 0;
}

memory_mapped_bitfield_register! {
    struct DBS(u32);
    EJTAG_DRSEG, "dbs",
    impl From;

    asid, _: 30;
    bcn, _: 27, 24;
    bp1, set_bp1: 1;
    bp0, set_bp0: 0;
}

memory_mapped_bitfield_register! {
    struct DBA(u32);
    EJTAG_DRSEG, "dba",
    impl From;

    iba, set_iba: 31, 0;
}

memory_mapped_bitfield_register! {
    struct DBM(u32);
    EJTAG_DRSEG, "dbm",
    impl From;

    ibm, set_ibm: 31, 0;
}

memory_mapped_bitfield_register! {
    struct DBASID(u32);
    EJTAG_DRSEG, "dbasid",
    impl From;

    asid, set_asid: 7, 0;
}

memory_mapped_bitfield_register! {
    struct DBC(u32);
    EJTAG_DRSEG, "dbc",
    impl From;

    tc, set_tc: 31, 24;
    asid_use, set_asid_use: 23;
    tc_use, _: 22;
    bai, set_bai: 21, 14;
    nsb, set_nsb: 13;
    nlb, set_nlb: 12;
    blm, set_blm: 11, 4;
    te, set_te: 2;
    be, set_be: 0;
}

memory_mapped_bitfield_register! {
    struct DBV(u32);
    EJTAG_DRSEG, "dbv",
    impl From;

    dbv, set_dbv: 31, 0;
}

memory_mapped_bitfield_register! {
    struct DBVH(u32);
    EJTAG_DRSEG, "dbvh",
    impl From;

    dbvh, set_dbvh: 31, 0;
}
