#![allow(clippy::no_effect, clippy::identity_op)]
use once_cell::sync::Lazy;

use crate::CoreRegisters;

use self::{base::MIPS32_REGISTERS_SET, cp0::MIPS32_CP0_REGISTERS, fpu::MIPS32_FPU_REGISTERS};

pub mod base;
pub mod cp0;
pub mod dsp;
pub mod fpu;

pub use base::{FP, PC, RA, SP, STATUS};

pub(crate) static MIPS32_CORE_REGISTERS: Lazy<CoreRegisters> = Lazy::new(|| {
    CoreRegisters::new(
        MIPS32_REGISTERS_SET
            .iter()
            .chain(MIPS32_CP0_REGISTERS)
            .collect(),
    )
});

pub(crate) static MIPS32_WITH_FPU_CORE_REGISTERS: Lazy<CoreRegisters> = Lazy::new(|| {
    CoreRegisters::new(
        MIPS32_REGISTERS_SET
            .iter()
            .chain(MIPS32_FPU_REGISTERS)
            .chain(MIPS32_CP0_REGISTERS)
            .collect(),
    )
});
