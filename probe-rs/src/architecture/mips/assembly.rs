pub const MIPS_NOP: u32 = 0x0;

// 6 bit operation code
pub const POOL32A: u32 = 0u32;
pub const POOL32B: u32 = 0b00_1000;
pub const POOL32I: u32 = 0b01_0000;
pub const POOL32AXF: u32 = 0x3C;

// Mips32
pub const MIPS32_OP_ADDU: u32 = 0x21;
pub const MIPS32_OP_ADDIU: u32 = 0x09;
pub const MIPS32_OP_ANDI: u32 = 0x0C;
pub const MIPS32_OP_BEQ: u32 = 0x04;
pub const MIPS32_OP_BGTZ: u32 = 0x07;
pub const MIPS32_OP_BNE: u32 = 0x05;
pub const MIPS32_OP_ADDI: u32 = 0x08;
pub const MIPS32_OP_AND: u32 = 0x24;
pub const MIPS32_OP_CACHE: u32 = 0x2F;
pub const MIPS32_OP_COP0: u32 = 0x10;
pub const MIPS32_OP_J: u32 = 0x02;
pub const MIPS32_OP_JR: u32 = 0x08;
pub const MIPS32_OP_LUI: u32 = 0x0F;
pub const MIPS32_OP_LW: u32 = 0x23;
pub const MIPS32_OP_LB: u32 = 0x20;
pub const MIPS32_OP_LBU: u32 = 0x24;
pub const MIPS32_OP_LHU: u32 = 0x25;
pub const MIPS32_OP_MFHI: u32 = 0x10;
pub const MIPS32_OP_MTHI: u32 = 0x11;
pub const MIPS32_OP_MFLO: u32 = 0x12;
pub const MIPS32_OP_MTLO: u32 = 0x13;
pub const MIPS32_OP_RDHWR: u32 = 0x3B;
pub const MIPS32_OP_SB: u32 = 0x28;
pub const MIPS32_OP_SH: u32 = 0x29;
pub const MIPS32_OP_SW: u32 = 0x2B;
pub const MIPS32_OP_ORI: u32 = 0x0D;
pub const MIPS32_OP_XORI: u32 = 0x0E;
pub const MIPS32_OP_XOR: u32 = 0x26;
pub const MIPS32_OP_SLTU: u32 = 0x2B;
pub const MIPS32_OP_SRL: u32 = 0x03;
pub const MIPS32_OP_SYNCI: u32 = 0x1F;
pub const MIPS32_OP_SLL: u32 = 0x00;
pub const MIPS32_OP_SLTI: u32 = 0x0A;
pub const MIPS32_OP_MOVN: u32 = 0x0B;

pub const MIPS32_OP_REGIMM: u32 = 0x01;
pub const MIPS32_OP_SDBBP: u32 = 0x3F;
pub const MIPS32_OP_SPECIAL: u32 = 0x00;
pub const MIPS32_OP_SPECIAL2: u32 = 0x07;
pub const MIPS32_OP_SPECIAL3: u32 = 0x1F;

pub const MIPS32_COP0_MF: u32 = 0x00;
pub const MIPS32_COP0_MT: u32 = 0x04;

pub const MIPS32_ISA_DRET: u32 = 0x4200001F;
/* MIPS32_ISA_J_INST(MIPS32_ISA_OP_SPECIAL2, MIPS32_ISA_OP_SDBBP) */
pub const MIPS32_ISA_SDBBP: u32 = 0x7000003F;
pub const MIPS16_ISA_SDBBP: u32 = 0xE801;

// MicroMips
pub const MMIPS32_OP_ADDI: u32 = 0x04;
pub const MMIPS32_OP_ADDIU: u32 = 0x0C;
pub const MMIPS32_OP_ADDU: u32 = 0x150;
pub const MMIPS32_OP_AND: u32 = 0x250;
pub const MMIPS32_OP_ANDI: u32 = 0x34;
pub const MMIPS32_OP_BEQ: u32 = 0x25;
pub const MMIPS32_OP_BGTZ: u32 = 0x06;
pub const MMIPS32_OP_BNE: u32 = 0x2D;
pub const MMIPS32_OP_CACHE: u32 = 0x06;
pub const MMIPS32_OP_J: u32 = 0x35;
pub const MMIPS32_OP_JALR: u32 = 0x03C;
pub const MMIPS32_OP_LB: u32 = 0x07;
pub const MMIPS32_OP_LBU: u32 = 0x05;
pub const MMIPS32_OP_LHU: u32 = 0x0D;
pub const MMIPS32_OP_LUI: u32 = 0x0D;
pub const MMIPS32_OP_LW: u32 = 0x3F;
pub const MMIPS32_OP_MFC0: u32 = 0x03;
pub const MMIPS32_OP_MTC0: u32 = 0x0B;
pub const MMIPS32_OP_MFLO: u32 = 0x075;
pub const MMIPS32_OP_MFHI: u32 = 0x035;
pub const MMIPS32_OP_MTLO: u32 = 0x0F5;
pub const MMIPS32_OP_MTHI: u32 = 0x0B5;
pub const MMIPS32_OP_MOVN: u32 = 0x018;
pub const MMIPS32_OP_ORI: u32 = 0x14;
pub const MMIPS32_OP_RDHWR: u32 = 0x1AC;
pub const MMIPS32_OP_SB: u32 = 0x06;
pub const MMIPS32_OP_SH: u32 = 0x0E;
pub const MMIPS32_OP_SW: u32 = 0x3E;
pub const MMIPS32_OP_SLTU: u32 = 0x390;
pub const MMIPS32_OP_SLL: u32 = 0x000;
pub const MMIPS32_OP_SLTI: u32 = 0x24;
pub const MMIPS32_OP_SRL: u32 = 0x040;
pub const MMIPS32_OP_SYNCI: u32 = 0x10;
pub const MMIPS32_OP_XOR: u32 = 0x310;
pub const MMIPS32_OP_XORI: u32 = 0x1C;

pub const MMIPS32_DRET: u32 = 0x0000E37C; /* MIPS32_R_INST(POOL32A, 0, 0, 0, 0x38D, POOL32AXF) */
pub const MMIPS32_SDBBP: u32 = 0x0000DB7C; /* MIPS32_R_INST(POOL32A, 0, 0, 0, 0x1BD, POOL32AXF) */
pub const MMIPS16_SDBBP: u32 = 0x46C0; /* POOL16C instr */

// 5 bit register specifier

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

// The assembly code generated here are only for mipsel

pub fn mips32_i_inst(opcode: u32, reg_src: u32, reg_tgt: u32, imm: u32) -> u32 {
    opcode << 26 | reg_src << 21 | reg_tgt << 16 | imm
}

pub fn mips32_j_inst(opcode: u32, addr: u32) -> u32 {
    opcode << 26 | addr
}

pub fn mips32_r_inst(
    opcode: u32,
    reg_src: u32,
    reg_tgt: u32,
    reg_dst: u32,
    shft: u32,
    funct: u32,
) -> u32 {
    opcode << 26 | reg_src << 21 | reg_tgt << 16 | reg_dst << 11 | shft << 6 | funct
}

pub fn mips32_isa_addi(tar: u32, src: u32, val: u32) -> u32 {
    mips32_i_inst(MIPS32_OP_ADDI, src, tar, val)
}
pub fn mips32_isa_addiu(tar: u32, src: u32, val: u32) -> u32 {
    mips32_i_inst(MIPS32_OP_ADDIU, src, tar, val)
}
pub fn mips32_isa_addu(dst: u32, src: u32, tar: u32) -> u32 {
    mips32_r_inst(MIPS32_OP_SPECIAL, src, tar, dst, 0, MIPS32_OP_ADDU)
}
pub fn mips32_isa_and(dst: u32, src: u32, tar: u32) -> u32 {
    mips32_r_inst(0, src, tar, dst, 0, MIPS32_OP_AND)
}
pub fn mips32_isa_andi(tar: u32, src: u32, val: u32) -> u32 {
    mips32_i_inst(MIPS32_OP_ANDI, src, tar, val)
}

pub fn mips32_isa_b(off: u32) -> u32 {
    mips32_isa_beq(0, 0, off)
}
pub fn mips32_isa_beq(src: u32, tar: u32, off: u32) -> u32 {
    mips32_i_inst(MIPS32_OP_BEQ, src, tar, off)
}
pub fn mips32_isa_bgtz(reg: u32, off: u32) -> u32 {
    mips32_i_inst(MIPS32_OP_BGTZ, reg, 0, off)
}
pub fn mips32_isa_bne(src: u32, tar: u32, off: u32) -> u32 {
    mips32_i_inst(MIPS32_OP_BNE, src, tar, off)
}
pub fn mips32_isa_cache(op: u32, off: u32, base: u32) -> u32 {
    mips32_i_inst(MIPS32_OP_CACHE, base, op, off)
}
pub fn mips32_isa_j(tar: u32) -> u32 {
    mips32_j_inst(MIPS32_OP_J, (0x0FFFFFFFu32 & (tar)) >> 2)
}
pub fn mips32_isa_jr(reg: u32) -> u32 {
    mips32_r_inst(0, reg, 0, 0, 0, MIPS32_OP_JR)
}

pub fn mips32_isa_lb(reg: u32, off: u32, base: u32) -> u32 {
    mips32_i_inst(MIPS32_OP_LB, base, reg, off)
}
pub fn mips32_isa_lbu(reg: u32, off: u32, base: u32) -> u32 {
    mips32_i_inst(MIPS32_OP_LBU, base, reg, off)
}
pub fn mips32_isa_lhu(reg: u32, off: u32, base: u32) -> u32 {
    mips32_i_inst(MIPS32_OP_LHU, base, reg, off)
}
pub fn mips32_isa_lui(reg: u32, val: u32) -> u32 {
    mips32_i_inst(MIPS32_OP_LUI, 0, reg, val)
}
pub fn mips32_isa_lw(reg: u32, off: u32, base: u32) -> u32 {
    mips32_i_inst(MIPS32_OP_LW, base, reg, off)
}

pub fn mips32_isa_mfc0(gpr: u32, cpr: u32, sel: u32) -> u32 {
    mips32_r_inst(MIPS32_OP_COP0, MIPS32_COP0_MF, gpr, cpr, 0, sel)
}
pub fn mips32_isa_mtc0(gpr: u32, cpr: u32, sel: u32) -> u32 {
    mips32_r_inst(MIPS32_OP_COP0, MIPS32_COP0_MT, gpr, cpr, 0, sel)
}
pub fn mips32_isa_mflo(reg: u32) -> u32 {
    mips32_r_inst(0, 0, 0, reg, 0, MIPS32_OP_MFLO)
}
pub fn mips32_isa_mfhi(reg: u32) -> u32 {
    mips32_r_inst(0, 0, 0, reg, 0, MIPS32_OP_MFHI)
}
pub fn mips32_isa_mtlo(reg: u32) -> u32 {
    mips32_r_inst(0, reg, 0, 0, 0, MIPS32_OP_MTLO)
}
pub fn mips32_isa_mthi(reg: u32) -> u32 {
    mips32_r_inst(0, reg, 0, 0, 0, MIPS32_OP_MTHI)
}

pub fn mips32_isa_movn(dst: u32, src: u32, tar: u32) -> u32 {
    mips32_r_inst(MIPS32_OP_SPECIAL, src, tar, dst, 0, MIPS32_OP_MOVN)
}
pub fn mips32_isa_ori(tar: u32, src: u32, val: u32) -> u32 {
    mips32_i_inst(MIPS32_OP_ORI, src, tar, val)
}
pub fn mips32_isa_rdhwr(tar: u32, dst: u32) -> u32 {
    mips32_r_inst(MIPS32_OP_SPECIAL3, 0, tar, dst, 0, MIPS32_OP_RDHWR)
}
pub fn mips32_isa_sb(reg: u32, off: u32, base: u32) -> u32 {
    mips32_i_inst(MIPS32_OP_SB, base, reg, off)
}
pub fn mips32_isa_sh(reg: u32, off: u32, base: u32) -> u32 {
    mips32_i_inst(MIPS32_OP_SH, base, reg, off)
}
pub fn mips32_isa_sw(reg: u32, off: u32, base: u32) -> u32 {
    mips32_i_inst(MIPS32_OP_SW, base, reg, off)
}

pub fn mips32_isa_sll(dst: u32, src: u32, sa: u32) -> u32 {
    mips32_r_inst(MIPS32_OP_SPECIAL, 0, src, dst, sa, MIPS32_OP_SLL)
}
pub fn mips32_isa_slti(tar: u32, src: u32, val: u32) -> u32 {
    mips32_i_inst(MIPS32_OP_SLTI, src, tar, val)
}
pub fn mips32_isa_sltu(dst: u32, src: u32, tar: u32) -> u32 {
    mips32_r_inst(MIPS32_OP_SPECIAL, src, tar, dst, 0, MIPS32_OP_SLTU)
}
pub fn mips32_isa_srl(reg: u32, src: u32, off: u32) -> u32 {
    mips32_r_inst(0, 0, src, reg, off, MIPS32_OP_SRL)
}
pub fn mips32_isa_sync() -> u32 {
    0xFu32
}
pub fn mips32_isa_synci(off: u32, base: u32) -> u32 {
    mips32_i_inst(MIPS32_OP_REGIMM, base, MIPS32_OP_SYNCI, off)
}

pub fn mips32_isa_xor(reg: u32, val1: u32, val2: u32) -> u32 {
    mips32_r_inst(0, val1, val2, reg, 0, MIPS32_OP_XOR)
}
pub fn mips32_isa_xori(tar: u32, src: u32, val: u32) -> u32 {
    mips32_i_inst(MIPS32_OP_XORI, src, tar, val)
}

pub fn mips32_isa_synci_step() -> u32 {
    0x1u32
}

pub fn mmips32_addi(tar: u32, src: u32, val: u32) -> u32 {
    mips32_i_inst(MMIPS32_OP_ADDI, tar, src, val)
}
pub fn mmips32_addiu(tar: u32, src: u32, val: u32) -> u32 {
    mips32_i_inst(MMIPS32_OP_ADDIU, tar, src, val)
}
pub fn mmips32_addu(dst: u32, src: u32, tar: u32) -> u32 {
    mips32_r_inst(POOL32A, tar, src, dst, 0, MMIPS32_OP_ADDU)
}
pub fn mmips32_and(dst: u32, src: u32, tar: u32) -> u32 {
    mips32_r_inst(POOL32A, tar, src, dst, 0, MMIPS32_OP_AND)
}
pub fn mmips32_andi(tar: u32, src: u32, val: u32) -> u32 {
    mips32_i_inst(MMIPS32_OP_ANDI, tar, src, val)
}

pub fn mmips32_b(off: u32) -> u32 {
    mmips32_beq(0, 0, off)
}
pub fn mmips32_beq(src: u32, tar: u32, off: u32) -> u32 {
    mips32_i_inst(MMIPS32_OP_BEQ, tar, src, off)
}
pub fn mmips32_bgtz(reg: u32, off: u32) -> u32 {
    mips32_i_inst(POOL32I, MMIPS32_OP_BGTZ, reg, off)
}
pub fn mmips32_bne(src: u32, tar: u32, off: u32) -> u32 {
    mips32_i_inst(MMIPS32_OP_BNE, tar, src, off)
}
pub fn mmips32_cache(op: u32, off: u32, base: u32) -> u32 {
    mips32_r_inst(POOL32B, op, base, MMIPS32_OP_CACHE << 1, 0, off)
}

pub fn mmips32_j(tar: u32) -> u32 {
    mips32_j_inst(MMIPS32_OP_J, 0x07FFFFFFu32 & ((tar) >> 1))
}
pub fn mmips32_jr(reg: u32) -> u32 {
    mips32_r_inst(POOL32A, 0, reg, 0, MMIPS32_OP_JALR, POOL32AXF)
}
pub fn mmips32_lb(reg: u32, off: u32, base: u32) -> u32 {
    mips32_i_inst(MMIPS32_OP_LB, reg, base, off)
}
pub fn mmips32_lbu(reg: u32, off: u32, base: u32) -> u32 {
    mips32_i_inst(MMIPS32_OP_LBU, reg, base, off)
}
pub fn mmips32_lhu(reg: u32, off: u32, base: u32) -> u32 {
    mips32_i_inst(MMIPS32_OP_LHU, reg, base, off)
}
pub fn mmips32_lui(reg: u32, val: u32) -> u32 {
    mips32_i_inst(POOL32I, MMIPS32_OP_LUI, reg, val)
}
pub fn mmips32_lw(reg: u32, off: u32, base: u32) -> u32 {
    mips32_i_inst(MMIPS32_OP_LW, reg, base, off)
}

pub fn mmips32_mfc0(gpr: u32, cpr: u32, sel: u32) -> u32 {
    mips32_r_inst(POOL32A, gpr, cpr, sel, MMIPS32_OP_MFC0, POOL32AXF)
}
pub fn mmips32_mflo(reg: u32) -> u32 {
    mips32_r_inst(POOL32A, 0, reg, 0, MMIPS32_OP_MFLO, POOL32AXF)
}
pub fn mmips32_mfhi(reg: u32) -> u32 {
    mips32_r_inst(POOL32A, 0, reg, 0, MMIPS32_OP_MFHI, POOL32AXF)
}
pub fn mmips32_mtc0(gpr: u32, cpr: u32, sel: u32) -> u32 {
    mips32_r_inst(POOL32A, gpr, cpr, sel, MMIPS32_OP_MTC0, POOL32AXF)
}
pub fn mmips32_mtlo(reg: u32) -> u32 {
    mips32_r_inst(POOL32A, 0, reg, 0, MMIPS32_OP_MTLO, POOL32AXF)
}
pub fn mmips32_mthi(reg: u32) -> u32 {
    mips32_r_inst(POOL32A, 0, reg, 0, MMIPS32_OP_MTHI, POOL32AXF)
}

pub fn mmips32_movn(dst: u32, src: u32, tar: u32) -> u32 {
    mips32_r_inst(POOL32A, tar, src, dst, 0, MMIPS32_OP_MOVN)
}
pub fn mmips32_ori(tar: u32, src: u32, val: u32) -> u32 {
    mips32_i_inst(MMIPS32_OP_ORI, tar, src, val)
}
pub fn mmips32_rdhwr(tar: u32, dst: u32) -> u32 {
    mips32_r_inst(POOL32A, dst, tar, 0, MMIPS32_OP_RDHWR, POOL32AXF)
}
pub fn mmips32_sb(reg: u32, off: u32, base: u32) -> u32 {
    mips32_i_inst(MMIPS32_OP_SB, reg, base, off)
}
pub fn mmips32_sh(reg: u32, off: u32, base: u32) -> u32 {
    mips32_i_inst(MMIPS32_OP_SH, reg, base, off)
}
pub fn mmips32_sw(reg: u32, off: u32, base: u32) -> u32 {
    mips32_i_inst(MMIPS32_OP_SW, reg, base, off)
}

pub fn mmips32_srl(reg: u32, src: u32, off: u32) -> u32 {
    mips32_r_inst(POOL32A, reg, src, off, 0, MMIPS32_OP_SRL)
}
pub fn mmips32_sltu(dst: u32, src: u32, tar: u32) -> u32 {
    mips32_r_inst(POOL32A, tar, src, dst, 0, MMIPS32_OP_SLTU)
}
pub fn mmips32_synci(off: u32, base: u32) -> u32 {
    mips32_i_inst(POOL32I, MMIPS32_OP_SYNCI, base, off)
}
pub fn mmips32_sll(dst: u32, src: u32, sa: u32) -> u32 {
    mips32_r_inst(POOL32A, dst, src, sa, 0, MMIPS32_OP_SLL)
}
pub fn mmips32_slti(tar: u32, src: u32, val: u32) -> u32 {
    mips32_i_inst(MMIPS32_OP_SLTI, tar, src, val)
}
pub fn mmips32_sync() -> u32 {
    0x00001A7Cu32 /* MIPS32_R_INST(POOL32A, 0, 0, 0, 0x1ADu, POOL32AXF) */
}

pub fn mmips32_xor(reg: u32, val1: u32, val2: u32) -> u32 {
    mips32_r_inst(POOL32A, val1, val2, reg, 0, MMIPS32_OP_XOR)
}
pub fn mmips32_xori(tar: u32, src: u32, val: u32) -> u32 {
    mips32_i_inst(MMIPS32_OP_XORI, tar, src, val)
}

pub fn mmips32_synci_step() -> u32 {
    0x1u32 /* reg num od address step size to be used with synci instruction */
}

/* instruction code with isa selection */
pub fn mips32_nop() -> u32 {
    0 /* same for both isa's */
}
pub fn mips32_addi(micro: bool, tar: u32, src: u32, val: u32) -> u32 {
    if micro {
        mmips32_addi(tar, src, val)
    } else {
        mips32_isa_addi(tar, src, val)
    }
}
pub fn mips32_addiu(micro: bool, tar: u32, src: u32, val: u32) -> u32 {
    if micro {
        mmips32_addiu(tar, src, val)
    } else {
        mips32_isa_addiu(tar, src, val)
    }
}
pub fn mips32_addu(micro: bool, dst: u32, src: u32, tar: u32) -> u32 {
    if micro {
        mmips32_addu(dst, src, tar)
    } else {
        mips32_isa_addu(dst, src, tar)
    }
}
pub fn mips32_and(micro: bool, dst: u32, src: u32, tar: u32) -> u32 {
    if micro {
        mmips32_and(dst, src, tar)
    } else {
        mips32_isa_and(dst, src, tar)
    }
}
pub fn mips32_andi(micro: bool, tar: u32, src: u32, val: u32) -> u32 {
    if micro {
        mmips32_andi(tar, src, val)
    } else {
        mips32_isa_andi(tar, src, val)
    }
}

pub fn mips32_b(micro: bool, off: u32) -> u32 {
    if micro {
        mmips32_b(off)
    } else {
        mips32_isa_b(off)
    }
}
pub fn mips32_beq(micro: bool, src: u32, tar: u32, off: u32) -> u32 {
    if micro {
        mmips32_beq(src, tar, off)
    } else {
        mips32_isa_beq(src, tar, off)
    }
}
pub fn mips32_bgtz(micro: bool, reg: u32, off: u32) -> u32 {
    if micro {
        mmips32_bgtz(reg, off)
    } else {
        mips32_isa_bgtz(reg, off)
    }
}
pub fn mips32_bne(micro: bool, src: u32, tar: u32, off: u32) -> u32 {
    if micro {
        mmips32_bne(src, tar, off)
    } else {
        mips32_isa_bne(src, tar, off)
    }
}
pub fn mips32_cache(micro: bool, op: u32, off: u32, base: u32) -> u32 {
    if micro {
        mmips32_cache(op, off, base)
    } else {
        mips32_isa_cache(op, off, base)
    }
}

pub fn mips32_j(micro: bool, tar: u32) -> u32 {
    if micro {
        mmips32_j(tar)
    } else {
        mips32_isa_j(tar)
    }
}
pub fn mips32_jr(micro: bool, reg: u32) -> u32 {
    if micro {
        mmips32_jr(reg)
    } else {
        mips32_isa_jr(reg)
    }
}
pub fn mips32_lb(micro: bool, reg: u32, off: u32, base: u32) -> u32 {
    if micro {
        mmips32_lb(reg, off, base)
    } else {
        mips32_isa_lb(reg, off, base)
    }
}
pub fn mips32_lbu(micro: bool, reg: u32, off: u32, base: u32) -> u32 {
    if micro {
        mmips32_lbu(reg, off, base)
    } else {
        mips32_isa_lbu(reg, off, base)
    }
}
pub fn mips32_lhu(micro: bool, reg: u32, off: u32, base: u32) -> u32 {
    if micro {
        mmips32_lhu(reg, off, base)
    } else {
        mips32_isa_lhu(reg, off, base)
    }
}
pub fn mips32_lw(micro: bool, reg: u32, off: u32, base: u32) -> u32 {
    if micro {
        mmips32_lw(reg, off, base)
    } else {
        mips32_isa_lw(reg, off, base)
    }
}
pub fn mips32_lui(micro: bool, reg: u32, val: u32) -> u32 {
    if micro {
        mmips32_lui(reg, val)
    } else {
        mips32_isa_lui(reg, val)
    }
}

pub fn mips32_mfc0(micro: bool, gpr: u32, cpr: u32, sel: u32) -> u32 {
    if micro {
        mmips32_mfc0(gpr, cpr, sel)
    } else {
        mips32_isa_mfc0(gpr, cpr, sel)
    }
}
pub fn mips32_mtc0(micro: bool, gpr: u32, cpr: u32, sel: u32) -> u32 {
    if micro {
        mmips32_mtc0(gpr, cpr, sel)
    } else {
        mips32_isa_mtc0(gpr, cpr, sel)
    }
}
pub fn mips32_mflo(micro: bool, reg: u32) -> u32 {
    if micro {
        mmips32_mflo(reg)
    } else {
        mips32_isa_mflo(reg)
    }
}
pub fn mips32_mfhi(micro: bool, reg: u32) -> u32 {
    if micro {
        mmips32_mfhi(reg)
    } else {
        mips32_isa_mfhi(reg)
    }
}
pub fn mips32_mtlo(micro: bool, reg: u32) -> u32 {
    if micro {
        mmips32_mtlo(reg)
    } else {
        mips32_isa_mtlo(reg)
    }
}
pub fn mips32_mthi(micro: bool, reg: u32) -> u32 {
    if micro {
        mmips32_mthi(reg)
    } else {
        mips32_isa_mthi(reg)
    }
}

pub fn mips32_movn(micro: bool, dst: u32, src: u32, tar: u32) -> u32 {
    if micro {
        mmips32_movn(dst, src, tar)
    } else {
        mips32_isa_movn(dst, src, tar)
    }
}
pub fn mips32_ori(micro: bool, tar: u32, src: u32, val: u32) -> u32 {
    if micro {
        mmips32_ori(tar, src, val)
    } else {
        mips32_isa_ori(tar, src, val)
    }
}
pub fn mips32_rdhwr(micro: bool, tar: u32, dst: u32) -> u32 {
    if micro {
        mmips32_rdhwr(tar, dst)
    } else {
        mips32_isa_rdhwr(tar, dst)
    }
}
pub fn mips32_sb(micro: bool, reg: u32, off: u32, base: u32) -> u32 {
    if micro {
        mmips32_sb(reg, off, base)
    } else {
        mips32_isa_sb(reg, off, base)
    }
}
pub fn mips32_sh(micro: bool, reg: u32, off: u32, base: u32) -> u32 {
    if micro {
        mmips32_sh(reg, off, base)
    } else {
        mips32_isa_sh(reg, off, base)
    }
}
pub fn mips32_sw(micro: bool, reg: u32, off: u32, base: u32) -> u32 {
    if micro {
        mmips32_sw(reg, off, base)
    } else {
        mips32_isa_sw(reg, off, base)
    }
}

pub fn mips32_sll(micro: bool, dst: u32, src: u32, sa: u32) -> u32 {
    if micro {
        mmips32_sll(dst, src, sa)
    } else {
        mips32_isa_sll(dst, src, sa)
    }
}
pub fn mips32_slti(micro: bool, tar: u32, src: u32, val: u32) -> u32 {
    if micro {
        mmips32_slti(tar, src, val)
    } else {
        mips32_isa_slti(tar, src, val)
    }
}
pub fn mips32_sltu(micro: bool, dst: u32, src: u32, tar: u32) -> u32 {
    if micro {
        mmips32_sltu(dst, src, tar)
    } else {
        mips32_isa_sltu(dst, src, tar)
    }
}
pub fn mips32_srl(micro: bool, reg: u32, src: u32, off: u32) -> u32 {
    if micro {
        mmips32_srl(reg, src, off)
    } else {
        mips32_isa_srl(reg, src, off)
    }
}

pub fn mips32_synci(micro: bool, off: u32, base: u32) -> u32 {
    if micro {
        mmips32_synci(off, base)
    } else {
        mips32_isa_synci(off, base)
    }
}
pub fn mips32_sync(micro: bool) -> u32 {
    if micro {
        mmips32_sync()
    } else {
        mips32_isa_sync()
    }
}
pub fn mips32_xor(micro: bool, reg: u32, val1: u32, val2: u32) -> u32 {
    if micro {
        mmips32_xor(reg, val1, val2)
    } else {
        mips32_isa_xor(reg, val1, val2)
    }
}
pub fn mips32_xori(micro: bool, tar: u32, src: u32, val: u32) -> u32 {
    if micro {
        mmips32_xori(tar, src, val)
    } else {
        mips32_isa_xori(tar, src, val)
    }
}

pub fn mips32_synci_step() -> u32 {
    0x1
}

/* ejtag specific instructions */
pub fn mips32_dret(micro: bool) -> u32 {
    if micro {
        MMIPS32_DRET
    } else {
        MIPS32_ISA_DRET
    }
}
pub fn mips32_sdbbp(micro: bool) -> u32 {
    if micro {
        MMIPS32_SDBBP
    } else {
        MIPS32_ISA_SDBBP
    }
}

pub fn mips16_sdbbp(micro: bool) -> u32 {
    if micro {
        MMIPS16_SDBBP
    } else {
        MIPS16_ISA_SDBBP
    }
}
