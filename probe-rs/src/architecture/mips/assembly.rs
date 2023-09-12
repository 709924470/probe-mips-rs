use paste::paste;

// 5 bit register specifier
pub const ZERO: u32 = 0x0;
pub const AT: u32 = 0x01;
pub const V0: u32 = 0x02;
pub const V1: u32 = 0x03;
pub const A0: u32 = 0x04;
pub const A1: u32 = 0x05;
pub const A2: u32 = 0x06;
pub const A3: u32 = 0x07;
pub const T0: u32 = 0x08;
pub const T1: u32 = 0x09;
pub const T2: u32 = 0x0A;
pub const T3: u32 = 0x0B;
pub const T4: u32 = 0x0C;
pub const T5: u32 = 0x0D;
pub const T6: u32 = 0x0E;
pub const T7: u32 = 0x0F;
pub const T8: u32 = 0x18;
pub const T9: u32 = 0x19;
pub const S0: u32 = 0x10;
pub const S1: u32 = 0x11;
pub const S2: u32 = 0x12;
pub const S3: u32 = 0x13;
pub const S4: u32 = 0x14;
pub const S5: u32 = 0x15;
pub const S6: u32 = 0x16;
pub const S7: u32 = 0x17;
pub const S8: u32 = 0x1E;
pub const K0: u32 = 0x1A;
pub const K1: u32 = 0x1B;
pub const GP: u32 = 0x1C;
pub const SP: u32 = 0x1D;
pub const RA: u32 = 0x1F;

/// This struct(class) records a sequence of MIPS instructions
/// and can be converted to a byte(u8, le) sequence for transferring
/// into the target acting as debug sequences and other
/// purposes. Chaining instructions is available.
///
/// For now, there are no MIPS processors that only implemented
/// MicroMIPS or MIPS16e(2), and MIPS64 is a super set of MIPS32,
/// which means MIPS32 is always compatible.
///
/// NanoMIPS and MIPSr6 are still in its early stage and don't
/// have their own processors on the market for the public use,
/// therefore ignore them until some one complains about them.
#[derive(Debug, Clone)]
pub struct Mips32Instruction {
    insts: Vec<u32>,
}

/// This macro is used for checking if the operand is fit for its masking,
/// the default is checking for 5 bit register operands, which can be used
/// in instructions that need manual encoding or have quirks that need some
/// manual adjustments.
///
/// The macro requires the reg variable and its mask for checking.
macro_rules! MIPS_INST_CHECK {
    (($head:ident:$cmp:literal)) => {
        assert!($head < $cmp);
    };
    ($gpr:ident) => {
        assert!($gpr < 0b11111u32, "Invalid register number");
    };
    ($gpr:ident, $($rest:tt),*) => {
        assert!($gpr < 0b11111u32, "Invalid register number");
        MIPS_INST_CHECK! ($($rest),*);
    };
    (($head:ident:$cmp:literal), $($rest:tt),*) => {
        assert!($head < $cmp);
        MIPS_INST_CHECK! ($($rest),*);
    };
}

/// This macro is used to define MIPS instruction generating
/// functions that have
///
/// 1) no operands,
/// 2) 1 or more operands.
///
/// The macro generates instruction with its name and encoding,
/// also the operand's bit mask, with customizable names to help
/// developers to mark how to use them.
macro_rules! MIPS_INST_DEF {
    ($name: ident, $enc:literal) => {
        pub fn $name(mut self) -> Self {
            self.insts.push($enc);
            self
        }
        paste!{
            pub const [<MIPS_ $name:upper>] : u32 = $enc;
        }
    };
    ($name:ident, $enc:literal, $($reg:ident:$start:literal:$mask:literal),+) => {
        pub fn $name(mut self, $($reg: u32),+) -> Self {
            MIPS_INST_CHECK! ($(($reg:$mask)),+);
            let result = $enc | $(($reg & $mask) << $start)|+;
            self.insts.push(result);
            self
        }
        paste!{
            pub fn [<MIPS_ $name:upper>]($($reg: u32),+) -> u32 {
                MIPS_INST_CHECK! ($(($reg:$mask)),+);
                $enc | $(($reg & $mask) << $start)|+
            }
        }
    };
}

impl Default for Mips32Instruction {
    fn default() -> Self {
        Self::new()
    }
}

/// Here lies some of the essential MIPS instructions
/// for basic debugging functionality.
impl Mips32Instruction {
    pub fn new() -> Self {
        Self { insts: vec![0u32] }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.insts
            .into_iter()
            .flat_map(|inst| inst.to_le_bytes())
            .collect()
    }

    pub fn get_raw(&self) -> Vec<u32> {
        self.insts.clone()
    }

    pub fn get_last(mut self) -> u32 {
        let Some(result) = self.insts.pop() else {
            panic! ("No instruction in queue! Please report this bug!");
        };
        self.insts.push(result);
        result
    }

    pub fn get_count(&self) -> u32 {
        self.insts.len() as u32
    }

    // MIPS special insts
    MIPS_INST_DEF!(nop, 0u32);
    MIPS_INST_DEF!(ehb, 0x000000c0);
    MIPS_INST_DEF!(sync, 0x0000000f);
    MIPS_INST_DEF!(sdbbp, 0x7000003f);

    MIPS_INST_DEF!(wrpgpr,  0x41c00000, hwr:11:0x1f, reg:16:0x1f);
    MIPS_INST_DEF!(rdhwr,   0x7c00003b, hwr:11:0x1f, tgt:16:0x1f);
    MIPS_INST_DEF!(cache,   0xbc000000, base:21:0x1f, op:16:0x1f, off:0:0xffff);

    // Arithmatics, load constants
    MIPS_INST_DEF!(srl,     0x00000002, dst:11:0x1f, src:16:0x1f, shift:6:0x1f);
    MIPS_INST_DEF!(sllv,    0x00000004, dst:11:0x1f, reg:21:0x1f, src:16:0x1f);
    MIPS_INST_DEF!(mfhi,    0x00000010, reg:11:0x1f);
    MIPS_INST_DEF!(mflo,    0x00000012, reg:11:0x1f);
    MIPS_INST_DEF!(addiu,   0x24000000, src:21:0x1f, tgt:16:0x1f, imm:0:0xffff);
    MIPS_INST_DEF!(ori,     0x34000000, src:21:0x1f, tgt:16:0x1f, imm:0:0xffff);
    MIPS_INST_DEF!(xori,    0x38000000, src:21:0x1f, tgt:16:0x1f, imm:0:0xffff);
    MIPS_INST_DEF!(lui,     0x3c000000, tgt:16:0x1f, imm:0:0xffff);

    // Memory Load/Store
    MIPS_INST_DEF!(lw,      0x8c000000, base:21:0x1f, reg:16:0x1f, off:0:0xffff);
    MIPS_INST_DEF!(lbu,     0x90000000, base:21:0x1f, reg:16:0x1f, off:0:0xffff);

    MIPS_INST_DEF!(sw,      0xac000000, base:21:0x1f, reg:16:0x1f, off:0:0xffff);
    MIPS_INST_DEF!(sb,      0xa0000000, base:21:0x1f, reg:16:0x1f, off:0:0xffff);
    MIPS_INST_DEF!(sh,      0xa4000000, base:21:0x1f, reg:16:0x1f, off:0:0xffff);

    // Branch ops
    MIPS_INST_DEF!(beq,     0x10000000, src:21:0x1f, tgt:16:0x1f, off:0:0xffff);
    MIPS_INST_DEF!(bne,     0x14000000, src:21:0x1f, tgt:16:0x1f, off:0:0xffff);
    pub fn b(mut self, offset: u32) -> Self {
        self.beq(0, 0, offset)
    }
    MIPS_INST_DEF!(jr,      0x00000008, reg:21:0x1f);
    MIPS_INST_DEF!(jr_hb,   0x00000408, reg:21:0x1f);

    // MIPS CP0 insts
    MIPS_INST_DEF!(mfc0,    0x40000000, dst:16:0x1f, cpr:11:0x1f, sel:0:0x1f);
    MIPS_INST_DEF!(mtc0,    0x40800000, src:16:0x1f, cpr:11:0x1f, sel:0:0x1f);

    // MIPS CP1/FPU insts
    MIPS_INST_DEF!(mfc1,    0x44000000, fpr:11:0x1f, dst:16:0x1f);
    MIPS_INST_DEF!(cfc1,    0x44400000, fpr:11:0x1f, dst:16:0x1f);
    MIPS_INST_DEF!(mfhc1,   0x44600000, fpr:11:0x1f, dst:16:0x1f);
    MIPS_INST_DEF!(mtc1,    0x44800000, fpr:11:0x1f, src:16:0x1f);
    MIPS_INST_DEF!(mthc1,   0x44e00000, fpr:11:0x1f, src:16:0x1f);
    MIPS_INST_DEF!(swc1,    0xe4000000, base:21:0x1f, ft:16:0x1f, off:0:0xffff);
}
