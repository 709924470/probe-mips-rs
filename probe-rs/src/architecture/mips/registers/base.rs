use crate::{
    core::{CoreRegisters, RegisterDataType, RegisterId, RegisterRole, UnwindRule},
    CoreRegister,
};

pub const PC: CoreRegister = CoreRegister {
    roles: &[RegisterRole::ProgramCounter],
    id: RegisterId(37),
    data_type: RegisterDataType::UnsignedInteger(32),
    unwind_rule: UnwindRule::Clear,
};

pub const SP: CoreRegister = CoreRegister {
    roles: &[RegisterRole::Core("r29"), RegisterRole::StackPointer],
    id: RegisterId(29),
    data_type: RegisterDataType::UnsignedInteger(32),
    unwind_rule: UnwindRule::Clear,
};

pub const FP: CoreRegister = CoreRegister {
    roles: &[RegisterRole::Core("r30"), RegisterRole::FramePointer],
    id: RegisterId(30),
    data_type: RegisterDataType::UnsignedInteger(32),
    unwind_rule: UnwindRule::Clear,
};

pub const RA: CoreRegister = CoreRegister {
    roles: &[RegisterRole::Core("r31"), RegisterRole::ReturnAddress],
    id: RegisterId(31),
    data_type: RegisterDataType::UnsignedInteger(32),
    unwind_rule: UnwindRule::Clear,
};

pub const STATUS: CoreRegister = CoreRegister {
    roles: &[RegisterRole::ProcessorStatus],
    id: RegisterId(32),
    data_type: RegisterDataType::UnsignedInteger(32),
    unwind_rule: UnwindRule::Clear,
};

pub static MIPS32_REGISTERS_SET: &[CoreRegister] = &[
    CoreRegister {
        roles: &[RegisterRole::Core("r0"), RegisterRole::Other("zero")],
        id: RegisterId(0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r1"), RegisterRole::Other("at")],
        id: RegisterId(1),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r2"), RegisterRole::Return("v0")],
        id: RegisterId(2),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r3"), RegisterRole::Return("v0")],
        id: RegisterId(3),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r4"), RegisterRole::Argument("a0")],
        id: RegisterId(4),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r5"), RegisterRole::Argument("a1")],
        id: RegisterId(5),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r6"), RegisterRole::Argument("a2")],
        id: RegisterId(6),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r7"), RegisterRole::Argument("a3")],
        id: RegisterId(7),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r8"), RegisterRole::Other("t0")],
        id: RegisterId(8),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r9"), RegisterRole::Other("t1")],
        id: RegisterId(9),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r10"), RegisterRole::Other("t2")],
        id: RegisterId(10),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r11"), RegisterRole::Other("t3")],
        id: RegisterId(11),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r12"), RegisterRole::Other("t4")],
        id: RegisterId(12),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r13"), RegisterRole::Other("t5")],
        id: RegisterId(13),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r14"), RegisterRole::Other("t6")],
        id: RegisterId(14),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r15"), RegisterRole::Other("t7")],
        id: RegisterId(15),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r16"), RegisterRole::Other("s0")],
        id: RegisterId(16),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r17"), RegisterRole::Other("s1")],
        id: RegisterId(17),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r18"), RegisterRole::Other("s2")],
        id: RegisterId(18),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r19"), RegisterRole::Other("s3")],
        id: RegisterId(19),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r20"), RegisterRole::Other("s4")],
        id: RegisterId(20),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r21"), RegisterRole::Other("s5")],
        id: RegisterId(21),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r22"), RegisterRole::Other("s6")],
        id: RegisterId(22),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r23"), RegisterRole::Other("s7")],
        id: RegisterId(23),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r24"), RegisterRole::Other("t8")],
        id: RegisterId(24),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r25"), RegisterRole::Other("t9")],
        id: RegisterId(25),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r26"), RegisterRole::Other("k0")],
        id: RegisterId(26),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r27"), RegisterRole::Other("k1")],
        id: RegisterId(27),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Core("r28"), RegisterRole::Other("gp")],
        id: RegisterId(28),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    SP,
    FP,
    RA,
    CoreRegister {
        roles: &[RegisterRole::Other("lo")],
        id: RegisterId(33),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Other("hi")],
        id: RegisterId(34),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    PC,
];
