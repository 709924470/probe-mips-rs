use crate::{probe::JTAGAccess, DebugProbeError};

use super::communication_interface::EjtagVersion;

/// This is a direction translation from OpenOCD
#[derive(Debug)]
pub struct Ejtag {
    pub probe: Box<dyn JTAGAccess>,
    idcode: u32,
    impcode: u32,
    ejtag_ctrl: u32,
    ejtag_version: EjtagVersion,
    fast_access_save: i32,

    config_regs: u32,
    config: [u32; 4],

    /// Saves temp register contents
    reg_t0: u32,
    reg_t1: u32,

    scan_delay: u32,
    mode: i32,

    pa_ctrl: u32,
    pa_addr: u32,

    isa: u32,
    endianness: u8,

    debug_caps: u32,
    ejtag_ibs_addr: u32,
    ejtag_iba0_addr: u32,
    ejtag_ibc_offs: u32,
    ejtag_ibm_offs: u32,
    ejtag_ibasid_offs: u32,

    ejtag_dbs_addr: u32,
    ejtag_dba0_addr: u32,
    ejtag_dbc_offs: u32,
    ejtag_dbm_offs: u32,
    ejtag_dbv_offs: u32,
    ejtag_dbasid_offs: u32,

    ejtag_iba_step_size: u32,
    ejtag_dba_step_size: u32,
}

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

const EJTAG_V20_IBS: u32 = 0xFF300004;
const EJTAG_V20_IBA0: u32 = 0xFF300100;
const EJTAG_V20_IBC_OFFS:u32 = 0x4	/* IBC Offset */;
const EJTAG_V20_IBM_OFFS: u32 = 0x8;
const EJTAG_V20_IBAN_STEP:u32 = 0x10	/* Offset for next channel */;
const EJTAG_V20_DBS: u32 = 0xFF300008;
const EJTAG_V20_DBA0: u32 = 0xFF300200;
const EJTAG_V20_DBC_OFFS: u32 = 0x4;
const EJTAG_V20_DBM_OFFS: u32 = 0x8;
const EJTAG_V20_DBV_OFFS: u32 = 0xc;
const EJTAG_V20_DBAN_STEP: u32 = 0x10;

const EJTAG_V25_IBS: u32 = 0xFF301000;
const EJTAG_V25_IBA0: u32 = 0xFF301100;
const EJTAG_V25_IBM_OFFS: u32 = 0x8;
const EJTAG_V25_IBASID_OFFS: u32 = 0x10;
const EJTAG_V25_IBC_OFFS: u32 = 0x18;
const EJTAG_V25_IBAN_STEP: u32 = 0x100;
const EJTAG_V25_DBS: u32 = 0xFF302000;
const EJTAG_V25_DBA0: u32 = 0xFF302100;
const EJTAG_V25_DBM_OFFS: u32 = 0x8;
const EJTAG_V25_DBASID_OFFS: u32 = 0x10;
const EJTAG_V25_DBC_OFFS: u32 = 0x18;
const EJTAG_V25_DBV_OFFS: u32 = 0x20;
const EJTAG_V25_DBAN_STEP: u32 = 0x100;

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
    fn ejtag_enable(&mut self) -> Result<(), DebugProbeError> {
        self.probe.set_ir_len(5);
        self.probe
            .write_register(EJTAG_INST_EJTAGBOOT, &[0u8; 32], 32)?;
        Ok(())
    }

    fn ejtag_get_impcode(&mut self) -> Result<(), DebugProbeError> {
        self.impcode = u32::from_le_bytes(
            self.probe.read_register(EJTAG_INST_IMPCODE, 32)?[..4]
                .try_into()
                .unwrap(),
        );
        self.ejtag_version = EjtagVersion::from(((self.impcode >> 29) & 0x07) as u8);
        if self.ejtag_version == EjtagVersion::EJTAG_V20 {
            self.ejtag_ibs_addr = EJTAG_V20_IBS;
            self.ejtag_iba0_addr = EJTAG_V20_IBA0;
            self.ejtag_ibc_offs = EJTAG_V20_IBC_OFFS;
            self.ejtag_ibm_offs = EJTAG_V20_IBM_OFFS;

            self.ejtag_dbs_addr = EJTAG_V20_DBS;
            self.ejtag_dba0_addr = EJTAG_V20_DBA0;
            self.ejtag_dbc_offs = EJTAG_V20_DBC_OFFS;
            self.ejtag_dbm_offs = EJTAG_V20_DBM_OFFS;
            self.ejtag_dbv_offs = EJTAG_V20_DBV_OFFS;

            self.ejtag_iba_step_size = EJTAG_V20_IBAN_STEP;
            self.ejtag_dba_step_size = EJTAG_V20_DBAN_STEP;
        } else {
            self.ejtag_ibs_addr = EJTAG_V25_IBS;
            self.ejtag_iba0_addr = EJTAG_V25_IBA0;
            self.ejtag_ibm_offs = EJTAG_V25_IBM_OFFS;
            self.ejtag_ibasid_offs = EJTAG_V25_IBASID_OFFS;
            self.ejtag_ibc_offs = EJTAG_V25_IBC_OFFS;

            self.ejtag_dbs_addr = EJTAG_V25_DBS;
            self.ejtag_dba0_addr = EJTAG_V25_DBA0;
            self.ejtag_dbm_offs = EJTAG_V25_DBM_OFFS;
            self.ejtag_dbasid_offs = EJTAG_V25_DBASID_OFFS;
            self.ejtag_dbc_offs = EJTAG_V25_DBC_OFFS;
            self.ejtag_dbv_offs = EJTAG_V25_DBV_OFFS;

            self.ejtag_iba_step_size = EJTAG_V25_IBAN_STEP;
            self.ejtag_dba_step_size = EJTAG_V25_DBAN_STEP;
        }

        Ok(())
    }
}
