use core::time;
use std::time::Instant;

use bitfield::bitfield;

use crate::{memory_mapped_bitfield_register, probe::JTAGAccess, DebugProbeError};

use super::{
    assembly::{Mips32Instruction, T0},
    communication_interface::MipsError,
    DebugCtrl, EjtagCtrl, IDCode, ImpCode,
};

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum EjtagVersion {
    NoModule,
    EJTAG_V20,
    EJTAG_V25,
    EJTAG_V26,
    EJTAG_V31,
    EJTAG_V41,
    EJTAG_V51,
    Unknown(u8),
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

/// This is a direction translation from OpenOCD
#[derive(Debug)]
pub struct Ejtag {
    pub probe: Box<dyn JTAGAccess>,
    pub(crate) idcode: IDCode,
    pub(crate) impcode: ImpCode,
    pub(crate) debug_ctrl: DebugCtrl,
    pub(crate) ejtag_ctrl: EjtagCtrl,
    pub(crate) ejtag_version: EjtagVersion,
    pub fast_access_save: i32,

    pub config_regs: u32,
    pub config: [u32; 4],

    /// Saves temp register contents
    pub(crate) reg_t0: u32,
    pub(crate) reg_t1: u32,

    // Usually under 200_0000 ns(200us, 0.2ms)
    pub scan_delay: u32,
    pub mode: i32,

    pub pa_ctrl: u32,
    pub pa_addr: u32,

    pub isa: u32,
    pub endianness: u8,

    pub debug_caps: u32,
    // Access to these fields should be kept in the crate
    pub(crate) ejtag_ibs_offs: u32,
    pub(crate) ejtag_iba0_offs: u32,
    pub(crate) ejtag_ibc_offs: u32,
    pub(crate) ejtag_ibm_offs: u32,
    pub(crate) ejtag_ibasid_offs: u32,

    pub(crate) ejtag_dbs_addr: u32,
    pub(crate) ejtag_dba0_addr: u32,
    pub(crate) ejtag_dbc_offs: u32,
    pub(crate) ejtag_dbm_offs: u32,
    pub(crate) ejtag_dbv_offs: u32,
    pub(crate) ejtag_dbasid_offs: u32,

    pub(crate) ejtag_iba_step_size: u32,
    pub(crate) ejtag_dba_step_size: u32,
}

pub const EJTAG_DRSEG: u64 = 0xFF300000;

const EJTAG_INST_IDCODE: u32 = 0x01;
const EJTAG_INST_IMPCODE: u32 = 0x03;
const EJTAG_INST_ADDRESS: u32 = 0x08;
const EJTAG_INST_DATA: u32 = 0x09;
const EJTAG_INST_CONTROL: u32 = 0x0A;
const EJTAG_INST_ALL: u32 = 0x0B;
const EJTAG_INST_EJTAGBOOT: u32 = 0x0C;
const EJTAG_INST_NORMALBOOT: u32 = 0x0D;
const EJTAG_INST_FASTDATA: u32 = 0x0E;
const EJTAG_INST_TCBCONTROLA: u32 = 0x10;
const EJTAG_INST_TCBCONTROLB: u32 = 0x11;
const EJTAG_INST_TCBDATA: u32 = 0x12;
const EJTAG_INST_TCBCONTROLC: u32 = 0x13;
const EJTAG_INST_PCSAMPLE: u32 = 0x14;
const EJTAG_INST_TCBCONTROLD: u32 = 0x15;
const EJTAG_INST_TCBCONTROLE: u32 = 0x16;
const EJTAG_INST_FDC: u32 = 0x17;
const EJTAG_INST_BYPASS: u32 = 0xFF;

pub const EJTAG_V20_IBS: u64 = 0x0004;
pub const EJTAG_V20_IBA0: u64 = 0x0100;
pub const EJTAG_V20_IBC_OFFS: u64 = 0x0104; /* IBC Offset */
pub const EJTAG_V20_IBM_OFFS: u64 = 0x0108;
pub const EJTAG_V20_IBAN_STEP: u32 = 0x10; /* Offset for next channel */
pub const EJTAG_V20_DBS: u64 = 0x0008;
pub const EJTAG_V20_DBA0: u64 = 0x0200;
pub const EJTAG_V20_DBC_OFFS: u64 = 0x0204;
pub const EJTAG_V20_DBM_OFFS: u64 = 0x0208;
pub const EJTAG_V20_DBV_OFFS: u64 = 0x020C;
pub const EJTAG_V20_DBAN_STEP: u32 = 0x10;

pub const EJTAG_V25_IBS: u64 = 0x1000;
pub const EJTAG_V25_IBA0: u64 = 0x1100;
pub const EJTAG_V25_IBM_OFFS: u64 = 0x0008;
pub const EJTAG_V25_IBASID_OFFS: u64 = 0x0010;
pub const EJTAG_V25_IBC_OFFS: u64 = 0x0018;
pub const EJTAG_V25_IBAN_STEP: u32 = 0x0100;
pub const EJTAG_V25_DBS: u64 = 0x2000;
pub const EJTAG_V25_DBA0: u64 = 0x2100;
pub const EJTAG_V25_DBM_OFFS: u64 = 0x2108;
pub const EJTAG_V25_DBASID_OFFS: u64 = 0x2110;
pub const EJTAG_V25_DBC_OFFS: u64 = 0x2118;
pub const EJTAG_V25_DBV_OFFS: u64 = 0x2120;
pub const EJTAG_V25_DBAN_STEP: u32 = 0x0100;

/* v2.0(Lexra) 29 - 1’b1 - Lexra Internal Trace Buffer implemented. This bit
 * overlaps with version bit of MIPS EJTAG specification. */
const EJTAG_V26_IMP_R3K: u32 = 1 << 28;
/* v2.0 - 24:25 - 2’b00- No profiling support */
const EJTAG_V26_IMP_DINT: u32 = 1 << 24;
const EJTAG_V20_IMP_SDBBP: u32 = 1 << 23 /* 1’b1 - sdbbp is Special2 Opcode */;
const EJTAG_IMP_ASID8: u32 = 1 << 22;
const EJTAG_IMP_ASID6: u32 = 1 << 21;
const EJTAG_V20_IMP_COMPLEX_BREAK: u32 = 1 << 20 /* Complex Breaks supported*/;
const EJTAG_V20_IMP_EADDR_NO32BIT: u32 = 1 << 19 /* EJTAG_ADDR > 32 bits wide */;
const EJTAG_V20_IMP_DCACHE_COH: u32 = 1 << 18 /* DCache does keep DMA coherent */;
const EJTAG_V20_IMP_ICACHE_COH: u32 = 1 << 17 /* DCache does keep DMA coherent */;
const EJTAG_IMP_MIPS16: u32 = 1 << 16;
const EJTAG_IMP_NODMA: u32 = 1 << 14;
/* v2.0 - 11:13 external PC trace. Trace PC Width. */
/* v2.0 - 8:10 external PC trace. PCST Width and DCLK Division Factor */
const EJTAG_V20_IMP_NOPB: u32 = 1 << 7 /* no processor breaks */;
const EJTAG_V20_IMP_NODB: u32 = 1 << 6 /* no data breaks */;
const EJTAG_V20_IMP_NOIB: u32 = 1 << 5 /* no instruction breaks implemented */;
/* v2.0 - 1:4 Number of Break Channels. */
const EJTAG_V20_IMP_BCHANNELS_MASK: u32 = 0xf;
const EJTAG_V20_IMP_BCHANNELS_SHIFT: u32 = 1;
const EJTAG_IMP_MIPS64: u32 = 1 << 0;

impl Ejtag {
    pub fn new(mut probe: Box<dyn JTAGAccess>) -> Result<Self, (Box<dyn JTAGAccess>, MipsError)> {
        // EJTAG instructions are 5 bits
        probe.set_ir_len(5);
        let idcode = u32::from_le_bytes(match probe.read_register(EJTAG_INST_IDCODE, 32) {
            Ok(val) => val[..4].try_into().unwrap(),
            Err(e) => {
                return Err((probe, e.into()));
            }
        });
        let impcode = u32::from_le_bytes(match probe.read_register(EJTAG_INST_IMPCODE, 32) {
            Ok(val) => val[..4].try_into().unwrap(),
            Err(e) => {
                return Err((probe, e.into()));
            }
        });
        let ejtag_version = EjtagVersion::from(((impcode >> 29) & 0x07) as u8);

        let ejtag_ctrl = u32::from_le_bytes(match probe.read_register(EJTAG_INST_CONTROL, 32) {
            Ok(val) => val[..4].try_into().unwrap(),
            Err(e) => {
                return Err((probe, e.into()));
            }
        });

        let mut this = Self {
            probe,
            idcode: IDCode(idcode),
            impcode: ImpCode(impcode),
            debug_ctrl: DebugCtrl(0),
            ejtag_ctrl: EjtagCtrl(ejtag_ctrl),
            ejtag_version,
            fast_access_save: 0,
            config_regs: 0,
            /// cp0 config0~3
            config: [0, 0, 0, 0],
            /// temporary save
            reg_t0: 0,
            /// temporary save
            reg_t1: 0,
            scan_delay: 0,
            mode: 0,
            pa_ctrl: 0,
            pa_addr: 0,
            isa: (impcode & EJTAG_IMP_MIPS16),
            endianness: 0,
            debug_caps: 0,
            ejtag_ibs_offs: EJTAG_V25_IBS as u32,
            ejtag_iba0_offs: EJTAG_V25_IBA0 as u32,
            ejtag_ibc_offs: EJTAG_V25_IBC_OFFS as u32,
            ejtag_ibm_offs: EJTAG_V25_IBM_OFFS as u32,
            ejtag_ibasid_offs: EJTAG_V25_IBASID_OFFS as u32,
            ejtag_dbs_addr: EJTAG_V25_DBS as u32,
            ejtag_dba0_addr: EJTAG_V25_DBA0 as u32,
            ejtag_dbc_offs: EJTAG_V25_DBC_OFFS as u32,
            ejtag_dbm_offs: EJTAG_V25_DBM_OFFS as u32,
            ejtag_dbv_offs: EJTAG_V25_DBV_OFFS as u32,
            ejtag_dbasid_offs: EJTAG_V25_DBASID_OFFS as u32,
            ejtag_iba_step_size: EJTAG_V25_IBAN_STEP,
            ejtag_dba_step_size: EJTAG_V25_DBAN_STEP,
        };

        if ejtag_version == EjtagVersion::EJTAG_V20 {
            this.ejtag_ibs_offs = EJTAG_V20_IBS as u32;
            this.ejtag_iba0_offs = EJTAG_V20_IBA0 as u32;
            this.ejtag_ibc_offs = EJTAG_V20_IBC_OFFS as u32;
            this.ejtag_ibm_offs = EJTAG_V20_IBM_OFFS as u32;

            this.ejtag_dbs_addr = EJTAG_V20_DBS as u32;
            this.ejtag_dba0_addr = EJTAG_V20_DBA0 as u32;
            this.ejtag_dbc_offs = EJTAG_V20_DBC_OFFS as u32;
            this.ejtag_dbm_offs = EJTAG_V20_DBM_OFFS as u32;
            this.ejtag_dbv_offs = EJTAG_V20_DBV_OFFS as u32;

            this.ejtag_iba_step_size = EJTAG_V20_IBAN_STEP;
            this.ejtag_dba_step_size = EJTAG_V20_DBAN_STEP;
        }

        Ok(this)
    }
    fn target_reset_assert(&mut self, halt: bool) -> Result<(), DebugProbeError> {
        self.probe.target_reset_assert();
        let boot = match halt {
            false => EJTAG_INST_NORMALBOOT,
            true => EJTAG_INST_EJTAGBOOT,
        };
        let bypass: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
        self.probe.write_register(boot, &bypass, 32)?;
        Ok(())
    }
    fn exec_queue(&mut self, queue: Mips32Instruction) -> Result<(), DebugProbeError> {
        const ZERO: [u8; 4] = [0, 0, 0, 0];
        let mut pracc_text: u32 = 0xff200200;
        let ejtag_ctrl = self.get_ejtag_ctrl()?.0.to_le_bytes();
        queue.get_raw().iter().for_each(|inst| {
            let mut data = ejtag_ctrl.to_vec();
            data.extend(inst.to_le_bytes().iter());
            data.extend(ZERO.iter());
            self.probe
                .write_register(EJTAG_INST_ALL, data.as_slice(), 96)?;
        });
        Ok(())
    }
    fn wait_for_pracc(&mut self) -> Result<(), DebugProbeError> {
        let time = Instant::now();
        let timeout = time::Duration::from_secs(1);
        loop {
            let ctrl = self.get_ejtag_ctrl()?;
            if ctrl.pracc() {
                break;
            }
            if time.elapsed() > timeout {
                return Err(DebugProbeError::Timeout);
            }
        }
        Ok(())
    }
    fn clean_pracc(&mut self) -> Result<(), DebugProbeError> {
        let time = Instant::now();
        let timeout = time::Duration::from_secs(1);
        loop {
            let ctrl = self.get_ejtag_ctrl()?;
            if ctrl.pracc() {
                break;
            }
            if time.elapsed() > timeout {
                return Err(DebugProbeError::Timeout);
            }
        }
        Ok(())
    }
    pub fn get_ejtag_ctrl(&mut self) -> Result<EjtagCtrl, DebugProbeError> {
        let ctrl = self.probe.read_register(EJTAG_INST_CONTROL, 32)?;
        if ctrl.len() < 4 {
            return Err(DebugProbeError::DebugSequenceNotSupported(
                "Invalid EJTAG Control sequence.",
            ));
        }
        let mut _ctrl: [u8; 4] = [0, 0, 0, 0];
        _ctrl.copy_from_slice(ctrl.as_slice());
        Ok(EjtagCtrl(u32::from_le_bytes(_ctrl)))
    }
    pub fn set_ejtag_ctrl(&mut self, ejtag_ctrl: EjtagCtrl) -> Result<(), DebugProbeError> {
        let ctrl =
            self.probe
                .write_register(EJTAG_INST_CONTROL, &ejtag_ctrl.0.to_le_bytes(), 32)?;
        if ctrl.len() < 4 {
            return Err(DebugProbeError::DebugSequenceNotSupported(
                "Invalid EJTAG Control sequence.",
            ));
        }
        let mut _ctrl: [u8; 4] = [0, 0, 0, 0];
        _ctrl.copy_from_slice(ctrl.as_slice());
        self.ejtag_ctrl = EjtagCtrl(u32::from_le_bytes(_ctrl));
        Ok(())
    }
    pub fn supports_hardware_breakpoint(self) -> bool {
        self.debug_ctrl.inst_brk()
    }
    pub fn enable_step(&mut self, enabled: bool) -> Result<(), DebugProbeError> {
        let mut queue = Mips32Instruction::new();
        queue = queue.mfc0(T0, 23, 0).ori(T0, T0, 0x100);

        if !enabled {
            queue = queue.xori(T0, T0, 0x100);
        }

        queue = queue
            .mtc0(T0, 23, 0) // Apply the bit
            .lui(T0, self.reg_t0 >> 16); // Restore t8

        queue = queue
            .clone()
            .b((0u32 - (queue.get_count() * 4)) & 0xFFFFu32) // Jump back to start, with forbidden slot
            .ori(T0, T0, self.reg_t0 & 0xFFFF);
        self.exec_queue(queue)
    }
}
