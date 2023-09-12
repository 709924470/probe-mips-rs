use crate::{
    core::{CoreRegisters, RegisterDataType, RegisterId, RegisterRole, UnwindRule},
    CoreRegister,
};

pub static MIPS32_DSP_REGISTERS: &[CoreRegister] = &[
    CoreRegister {
        roles: &[RegisterRole::Other("hi1")],
        id: RegisterId(72),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Other("lo1")],
        id: RegisterId(73),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Other("hi2")],
        id: RegisterId(74),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Other("lo2")],
        id: RegisterId(75),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Other("hi3")],
        id: RegisterId(76),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Other("lo3")],
        id: RegisterId(77),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Other("dspctl")],
        id: RegisterId(78),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
];
