#![allow(clippy::no_effect, clippy::identity_op)]
use once_cell::sync::Lazy;

use crate::{
    core::{CoreRegisters, RegisterDataType, RegisterId, RegisterRole},
    CoreRegister,
};

pub const PC: CoreRegister = CoreRegister {
    name: "pc",
    roles: &[RegisterRole::ProgramCounter],
    id: RegisterId(37),
    data_type: RegisterDataType::UnsignedInteger(32),
};

pub const SP: CoreRegister = CoreRegister {
    name: "sp",
    roles: &[RegisterRole::StackPointer],
    id: RegisterId(29),
    data_type: RegisterDataType::UnsignedInteger(32),
};

pub const FP: CoreRegister = CoreRegister {
    name: "fp",
    roles: &[RegisterRole::FramePointer],
    id: RegisterId(30),
    data_type: RegisterDataType::UnsignedInteger(32),
};

pub const RA: CoreRegister = CoreRegister {
    name: "ra",
    roles: &[RegisterRole::ReturnAddress],
    id: RegisterId(31),
    data_type: RegisterDataType::UnsignedInteger(32),
};

const STATUS: CoreRegister = CoreRegister {
    name: "status",
    roles: &[RegisterRole::ProcessorStatus],
    id: RegisterId(32),
    data_type: RegisterDataType::UnsignedInteger(32),
};

pub(crate) static MIPS32_CORE_REGSISTERS: Lazy<CoreRegisters> = Lazy::new(|| {
    CoreRegisters::new(
        MIPS32_REGISTERS_SET
            .iter()
            .chain(MIPS32_CP0_REGISTERS)
            .collect(),
    )
});

pub(crate) static MIPS32_WITH_FPU_CORE_REGSISTERS: Lazy<CoreRegisters> = Lazy::new(|| {
    CoreRegisters::new(
        MIPS32_REGISTERS_SET
            .iter()
            .chain(MIPS32_FPU_REGISTERS)
            .chain(MIPS32_CP0_REGISTERS)
            .collect(),
    )
});

static MIPS32_REGISTERS_SET: &[CoreRegister] = &[
    CoreRegister {
        name: "r0",
        roles: &[RegisterRole::Other("r0"), RegisterRole::Other("zero")],
        id: RegisterId(0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r1",
        roles: &[RegisterRole::Other("r1"), RegisterRole::Other("at")],
        id: RegisterId(1),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r2",
        roles: &[RegisterRole::Other("r2"), RegisterRole::Return("v0")],
        id: RegisterId(2),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r3",
        roles: &[RegisterRole::Other("r3"), RegisterRole::Return("v0")],
        id: RegisterId(3),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r4",
        roles: &[RegisterRole::Other("r4"), RegisterRole::Argument("a0")],
        id: RegisterId(4),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r5",
        roles: &[RegisterRole::Other("r5"), RegisterRole::Argument("a1")],
        id: RegisterId(5),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r6",
        roles: &[RegisterRole::Other("r6"), RegisterRole::Argument("a2")],
        id: RegisterId(6),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r7",
        roles: &[RegisterRole::Other("r7"), RegisterRole::Argument("a3")],
        id: RegisterId(7),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r8",
        roles: &[RegisterRole::Other("r8"), RegisterRole::Other("t0")],
        id: RegisterId(8),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r9",
        roles: &[RegisterRole::Other("r9"), RegisterRole::Other("t1")],
        id: RegisterId(9),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r10",
        roles: &[RegisterRole::Other("r10"), RegisterRole::Other("t2")],
        id: RegisterId(10),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r11",
        roles: &[RegisterRole::Other("r11"), RegisterRole::Other("t3")],
        id: RegisterId(11),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r12",
        roles: &[RegisterRole::Other("r12"), RegisterRole::Other("t4")],
        id: RegisterId(12),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r13",
        roles: &[RegisterRole::Other("r13"), RegisterRole::Other("t5")],
        id: RegisterId(13),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r14",
        roles: &[RegisterRole::Other("r14"), RegisterRole::Other("t6")],
        id: RegisterId(14),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r15",
        roles: &[RegisterRole::Other("r15"), RegisterRole::Other("t7")],
        id: RegisterId(15),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r16",
        roles: &[RegisterRole::Other("r16"), RegisterRole::Other("s0")],
        id: RegisterId(16),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r17",
        roles: &[RegisterRole::Other("r17"), RegisterRole::Other("s1")],
        id: RegisterId(17),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r18",
        roles: &[RegisterRole::Other("r18"), RegisterRole::Other("s2")],
        id: RegisterId(18),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r19",
        roles: &[RegisterRole::Other("r19"), RegisterRole::Other("s3")],
        id: RegisterId(19),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r20",
        roles: &[RegisterRole::Other("r20"), RegisterRole::Other("s4")],
        id: RegisterId(20),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r21",
        roles: &[RegisterRole::Other("r21"), RegisterRole::Other("s5")],
        id: RegisterId(21),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r22",
        roles: &[RegisterRole::Other("r22"), RegisterRole::Other("s6")],
        id: RegisterId(22),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r23",
        roles: &[RegisterRole::Other("r23"), RegisterRole::Other("s7")],
        id: RegisterId(23),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r24",
        roles: &[RegisterRole::Other("r24"), RegisterRole::Other("t8")],
        id: RegisterId(24),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r25",
        roles: &[RegisterRole::Other("r25"), RegisterRole::Other("t9")],
        id: RegisterId(25),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r26",
        roles: &[RegisterRole::Other("r26"), RegisterRole::Other("k0")],
        id: RegisterId(26),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r27",
        roles: &[RegisterRole::Other("r27"), RegisterRole::Other("k1")],
        id: RegisterId(27),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "r28",
        roles: &[RegisterRole::Other("r28"), RegisterRole::Other("gp")],
        id: RegisterId(28),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    SP,
    FP,
    RA,
    CoreRegister {
        name: "lo",
        roles: &[RegisterRole::Other("lo")],
        id: RegisterId(33),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "hi",
        roles: &[RegisterRole::Other("hi")],
        id: RegisterId(34),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    PC,
];

static MIPS32_FPU_REGISTERS: &[CoreRegister] = &[
    CoreRegister {
        name: "f0",
        roles: &[RegisterRole::Other("f0"), RegisterRole::FloatingPoint],
        id: RegisterId(38),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f1",
        roles: &[RegisterRole::Other("f1"), RegisterRole::FloatingPoint],
        id: RegisterId(39),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f2",
        roles: &[RegisterRole::Other("f2"), RegisterRole::FloatingPoint],
        id: RegisterId(40),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f3",
        roles: &[RegisterRole::Other("f3"), RegisterRole::FloatingPoint],
        id: RegisterId(41),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f4",
        roles: &[RegisterRole::Other("f4"), RegisterRole::FloatingPoint],
        id: RegisterId(42),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f5",
        roles: &[RegisterRole::Other("f5"), RegisterRole::FloatingPoint],
        id: RegisterId(43),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f6",
        roles: &[RegisterRole::Other("f6"), RegisterRole::FloatingPoint],
        id: RegisterId(44),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f7",
        roles: &[RegisterRole::Other("f7"), RegisterRole::FloatingPoint],
        id: RegisterId(45),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f8",
        roles: &[RegisterRole::Other("f8"), RegisterRole::FloatingPoint],
        id: RegisterId(46),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f9",
        roles: &[RegisterRole::Other("f9"), RegisterRole::FloatingPoint],
        id: RegisterId(47),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f10",
        roles: &[RegisterRole::Other("f10"), RegisterRole::FloatingPoint],
        id: RegisterId(48),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f11",
        roles: &[RegisterRole::Other("f11"), RegisterRole::FloatingPoint],
        id: RegisterId(49),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f12",
        roles: &[RegisterRole::Other("f12"), RegisterRole::FloatingPoint],
        id: RegisterId(50),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f13",
        roles: &[RegisterRole::Other("f13"), RegisterRole::FloatingPoint],
        id: RegisterId(51),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f14",
        roles: &[RegisterRole::Other("f14"), RegisterRole::FloatingPoint],
        id: RegisterId(52),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f15",
        roles: &[RegisterRole::Other("f15"), RegisterRole::FloatingPoint],
        id: RegisterId(53),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f16",
        roles: &[RegisterRole::Other("f16"), RegisterRole::FloatingPoint],
        id: RegisterId(54),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f17",
        roles: &[RegisterRole::Other("f17"), RegisterRole::FloatingPoint],
        id: RegisterId(55),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f18",
        roles: &[RegisterRole::Other("f18"), RegisterRole::FloatingPoint],
        id: RegisterId(56),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f19",
        roles: &[RegisterRole::Other("f19"), RegisterRole::FloatingPoint],
        id: RegisterId(57),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f20",
        roles: &[RegisterRole::Other("f20"), RegisterRole::FloatingPoint],
        id: RegisterId(58),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f21",
        roles: &[RegisterRole::Other("f21"), RegisterRole::FloatingPoint],
        id: RegisterId(59),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f22",
        roles: &[RegisterRole::Other("f22"), RegisterRole::FloatingPoint],
        id: RegisterId(60),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f23",
        roles: &[RegisterRole::Other("f23"), RegisterRole::FloatingPoint],
        id: RegisterId(61),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f24",
        roles: &[RegisterRole::Other("f24"), RegisterRole::FloatingPoint],
        id: RegisterId(62),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f25",
        roles: &[RegisterRole::Other("f25"), RegisterRole::FloatingPoint],
        id: RegisterId(63),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f26",
        roles: &[RegisterRole::Other("f26"), RegisterRole::FloatingPoint],
        id: RegisterId(64),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f27",
        roles: &[RegisterRole::Other("f27"), RegisterRole::FloatingPoint],
        id: RegisterId(65),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f28",
        roles: &[RegisterRole::Other("f28"), RegisterRole::FloatingPoint],
        id: RegisterId(66),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f29",
        roles: &[RegisterRole::Other("f29"), RegisterRole::FloatingPoint],
        id: RegisterId(67),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f30",
        roles: &[RegisterRole::Other("f30"), RegisterRole::FloatingPoint],
        id: RegisterId(68),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "f31",
        roles: &[RegisterRole::Other("f31"), RegisterRole::FloatingPoint],
        id: RegisterId(69),
        data_type: RegisterDataType::FloatingPoint(32),
    },
    CoreRegister {
        name: "fcsr",
        roles: &[
            RegisterRole::Other("fcsr"),
            RegisterRole::FloatingPointStatus,
        ],
        id: RegisterId(70),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "fir",
        roles: &[
            RegisterRole::Other("fir"),
            RegisterRole::FloatingPointStatus,
        ],
        id: RegisterId(71),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
];

static MIPS32_DSP_REGISTERS: &[CoreRegister] = &[
    CoreRegister {
        name: "hi1",
        roles: &[RegisterRole::Other("hi1"), RegisterRole::Other("hi1")],
        id: RegisterId(72),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "lo1",
        roles: &[RegisterRole::Other("lo1"), RegisterRole::Other("lo1")],
        id: RegisterId(73),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "hi2",
        roles: &[RegisterRole::Other("hi2"), RegisterRole::Other("hi2")],
        id: RegisterId(74),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "lo2",
        roles: &[RegisterRole::Other("lo2"), RegisterRole::Other("lo2")],
        id: RegisterId(75),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "hi3",
        roles: &[RegisterRole::Other("hi3"), RegisterRole::Other("hi3")],
        id: RegisterId(76),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "lo3",
        roles: &[RegisterRole::Other("lo3"), RegisterRole::Other("lo3")],
        id: RegisterId(77),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "dspctl",
        roles: &[RegisterRole::Other("dspctl"), RegisterRole::Other("dspctl")],
        id: RegisterId(78),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
];

/// CP0 registers are marked by setting msb
static MIPS32_CP0_REGISTERS: &[CoreRegister] = &[
    CoreRegister {
        name: "cp0.0.0",
        roles: &[RegisterRole::Other("cp0.0.0"), RegisterRole::Other("index")],
        id: RegisterId(0x8000),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.1.0",
        roles: &[
            RegisterRole::Other("cp0.1.0"),
            RegisterRole::Other("random"),
        ],
        id: RegisterId(0x8000 | 1 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.2.0",
        roles: &[
            RegisterRole::Other("cp0.2.0"),
            RegisterRole::Other("entrylo0"),
        ],
        id: RegisterId(0x8000 | 2 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.3.0",
        roles: &[
            RegisterRole::Other("cp0.3.0"),
            RegisterRole::Other("entrylo1"),
        ],
        id: RegisterId(0x8000 | 3 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.4.0",
        roles: &[
            RegisterRole::Other("cp0.4.0"),
            RegisterRole::Other("context"),
        ],
        id: RegisterId(0x8000 | 4 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.5.0",
        roles: &[
            RegisterRole::Other("cp0.5.0"),
            RegisterRole::Other("pagemask"),
        ],
        id: RegisterId(0x8000 | 5 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.6.0",
        roles: &[RegisterRole::Other("cp0.6.0"), RegisterRole::Other("wired")],
        id: RegisterId(0x8000 | 6 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.7.0",
        roles: &[
            RegisterRole::Other("cp0.7.0"),
            RegisterRole::Other("hwrena"),
        ],
        id: RegisterId(0x8000 | 7 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.8.0",
        roles: &[
            RegisterRole::Other("cp0.8.0"),
            RegisterRole::Other("badvaddr"),
        ],
        id: RegisterId(0x8000 | 8 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.9.0",
        roles: &[RegisterRole::Other("cp0.9.0"), RegisterRole::Other("count")],
        id: RegisterId(0x8000 | 9 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.10.0",
        roles: &[
            RegisterRole::Other("cp0.10.0"),
            RegisterRole::Other("entryhi"),
        ],
        id: RegisterId(0x8000 | 10 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.11.0",
        roles: &[
            RegisterRole::Other("cp0.11.0"),
            RegisterRole::Other("compare"),
        ],
        id: RegisterId(0x8000 | 11 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.12.0",
        roles: &[
            RegisterRole::Other("cp0.12.0"),
            RegisterRole::Other("status"),
        ],
        id: RegisterId(0x8000 | 12 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.13.0",
        roles: &[
            RegisterRole::Other("cp0.13.0"),
            RegisterRole::Other("cause"),
        ],
        id: RegisterId(0x8000 | 13 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.14.0",
        roles: &[RegisterRole::Other("cp0.14.0"), RegisterRole::Other("epc")],
        id: RegisterId(0x8000 | 14 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.15.0",
        roles: &[RegisterRole::Other("cp0.15.0"), RegisterRole::Other("prid")],
        id: RegisterId(0x8000 | 15 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.16.0",
        roles: &[
            RegisterRole::Other("cp0.16.0"),
            RegisterRole::Other("config"),
        ],
        id: RegisterId(0x8000 | 16 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.17.0",
        roles: &[
            RegisterRole::Other("cp0.17.0"),
            RegisterRole::Other("lladdr"),
        ],
        id: RegisterId(0x8000 | 17 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.18.0",
        roles: &[
            RegisterRole::Other("cp0.18.0"),
            RegisterRole::Other("watchlo0"),
        ],
        id: RegisterId(0x8000 | 18 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.19.0",
        roles: &[
            RegisterRole::Other("cp0.19.0"),
            RegisterRole::Other("watchhi0"),
        ],
        id: RegisterId(0x8000 | 19 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.20.0",
        roles: &[
            RegisterRole::Other("cp0.20.0"),
            RegisterRole::Other("xcontext"),
        ],
        id: RegisterId(0x8000 | 20 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.23.0",
        roles: &[
            RegisterRole::Other("cp0.23.0"),
            RegisterRole::Other("debug"),
        ],
        id: RegisterId(0x8000 | 23 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.24.0",
        roles: &[RegisterRole::Other("cp0.24.0"), RegisterRole::Other("depc")],
        id: RegisterId(0x8000 | 24 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.25.0",
        roles: &[
            RegisterRole::Other("cp0.25.0"),
            RegisterRole::Other("perfctl0"),
        ],
        id: RegisterId(0x8000 | 25 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.26.0",
        roles: &[
            RegisterRole::Other("cp0.26.0"),
            RegisterRole::Other("errctl"),
        ],
        id: RegisterId(0x8000 | 26 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.27.0",
        roles: &[
            RegisterRole::Other("cp0.27.0"),
            RegisterRole::Other("cacheerr"),
        ],
        id: RegisterId(0x8000 | 27 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.28.0",
        roles: &[
            RegisterRole::Other("cp0.28.0"),
            RegisterRole::Other("itaglo"),
        ],
        id: RegisterId(0x8000 | 28 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.29.0",
        roles: &[
            RegisterRole::Other("cp0.29.0"),
            RegisterRole::Other("itaghi"),
        ],
        id: RegisterId(0x8000 | 29 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.30.0",
        roles: &[
            RegisterRole::Other("cp0.30.0"),
            RegisterRole::Other("errorepc"),
        ],
        id: RegisterId(0x8000 | 30 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.31.0",
        roles: &[
            RegisterRole::Other("cp0.31.0"),
            RegisterRole::Other("desave"),
        ],
        id: RegisterId(0x8000 | 31 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.0.1",
        roles: &[
            RegisterRole::Other("cp0.0.1"),
            RegisterRole::Other("mvpcontrol"),
        ],
        id: RegisterId(0x8000 | 0 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.1.1",
        roles: &[
            RegisterRole::Other("cp0.1.1"),
            RegisterRole::Other("vpecontrol"),
        ],
        id: RegisterId(0x8000 | 1 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.2.1",
        roles: &[
            RegisterRole::Other("cp0.2.1"),
            RegisterRole::Other("tcstatus"),
        ],
        id: RegisterId(0x8000 | 2 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.3.1",
        roles: &[
            RegisterRole::Other("cp0.3.1"),
            RegisterRole::Other("globalnumber"),
        ],
        id: RegisterId(0x8000 | 3 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.4.1",
        roles: &[
            RegisterRole::Other("cp0.4.1"),
            RegisterRole::Other("contextconfig"),
        ],
        id: RegisterId(0x8000 | 4 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.5.1",
        roles: &[
            RegisterRole::Other("cp0.5.1"),
            RegisterRole::Other("pagegrain"),
        ],
        id: RegisterId(0x8000 | 5 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.6.1",
        roles: &[
            RegisterRole::Other("cp0.6.1"),
            RegisterRole::Other("srsconf0"),
        ],
        id: RegisterId(0x8000 | 6 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.8.1",
        roles: &[
            RegisterRole::Other("cp0.8.1"),
            RegisterRole::Other("badinstr"),
        ],
        id: RegisterId(0x8000 | 8 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.12.1",
        roles: &[
            RegisterRole::Other("cp0.12.1"),
            RegisterRole::Other("intctl"),
        ],
        id: RegisterId(0x8000 | 12 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.15.1",
        roles: &[
            RegisterRole::Other("cp0.15.1"),
            RegisterRole::Other("ebase"),
        ],
        id: RegisterId(0x8000 | 15 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.16.1",
        roles: &[
            RegisterRole::Other("cp0.16.1"),
            RegisterRole::Other("config1"),
        ],
        id: RegisterId(0x8000 | 16 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.17.1",
        roles: &[RegisterRole::Other("cp0.17.1"), RegisterRole::Other("maar")],
        id: RegisterId(0x8000 | 17 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.18.1",
        roles: &[
            RegisterRole::Other("cp0.18.1"),
            RegisterRole::Other("watchlo1"),
        ],
        id: RegisterId(0x8000 | 18 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.19.1",
        roles: &[
            RegisterRole::Other("cp0.19.1"),
            RegisterRole::Other("watchhi1"),
        ],
        id: RegisterId(0x8000 | 19 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.23.1",
        roles: &[
            RegisterRole::Other("cp0.23.1"),
            RegisterRole::Other("tracecontrol"),
        ],
        id: RegisterId(0x8000 | 23 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.25.1",
        roles: &[
            RegisterRole::Other("cp0.25.1"),
            RegisterRole::Other("perfcnt0"),
        ],
        id: RegisterId(0x8000 | 25 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.26.1",
        roles: &[
            RegisterRole::Other("cp0.26.1"),
            RegisterRole::Other("ierrctl"),
        ],
        id: RegisterId(0x8000 | 26 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.28.1",
        roles: &[
            RegisterRole::Other("cp0.28.1"),
            RegisterRole::Other("idatalo"),
        ],
        id: RegisterId(0x8000 | 28 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.29.1",
        roles: &[
            RegisterRole::Other("cp0.29.1"),
            RegisterRole::Other("idatahi"),
        ],
        id: RegisterId(0x8000 | 29 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.0.2",
        roles: &[
            RegisterRole::Other("cp0.0.2"),
            RegisterRole::Other("mvpconf0"),
        ],
        id: RegisterId(0x8000 | 0 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.1.2",
        roles: &[
            RegisterRole::Other("cp0.1.2"),
            RegisterRole::Other("vpeconf0"),
        ],
        id: RegisterId(0x8000 | 1 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.2.2",
        roles: &[
            RegisterRole::Other("cp0.2.2"),
            RegisterRole::Other("tcbind"),
        ],
        id: RegisterId(0x8000 | 2 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.4.2",
        roles: &[
            RegisterRole::Other("cp0.4.2"),
            RegisterRole::Other("userlocal"),
        ],
        id: RegisterId(0x8000 | 4 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.5.2",
        roles: &[
            RegisterRole::Other("cp0.5.2"),
            RegisterRole::Other("segctl0"),
        ],
        id: RegisterId(0x8000 | 5 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.6.2",
        roles: &[
            RegisterRole::Other("cp0.6.2"),
            RegisterRole::Other("srsconf1"),
        ],
        id: RegisterId(0x8000 | 6 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.8.2",
        roles: &[
            RegisterRole::Other("cp0.8.2"),
            RegisterRole::Other("badinstrp"),
        ],
        id: RegisterId(0x8000 | 8 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.12.2",
        roles: &[
            RegisterRole::Other("cp0.12.2"),
            RegisterRole::Other("srsctl"),
        ],
        id: RegisterId(0x8000 | 12 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.15.2",
        roles: &[
            RegisterRole::Other("cp0.15.2"),
            RegisterRole::Other("cdmmbase"),
        ],
        id: RegisterId(0x8000 | 15 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.16.2",
        roles: &[
            RegisterRole::Other("cp0.16.2"),
            RegisterRole::Other("config2"),
        ],
        id: RegisterId(0x8000 | 16 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.17.2",
        roles: &[
            RegisterRole::Other("cp0.17.2"),
            RegisterRole::Other("maari"),
        ],
        id: RegisterId(0x8000 | 17 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.18.2",
        roles: &[
            RegisterRole::Other("cp0.18.2"),
            RegisterRole::Other("watchlo2"),
        ],
        id: RegisterId(0x8000 | 18 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.19.2",
        roles: &[
            RegisterRole::Other("cp0.19.2"),
            RegisterRole::Other("watchhi2"),
        ],
        id: RegisterId(0x8000 | 19 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.23.2",
        roles: &[
            RegisterRole::Other("cp0.23.2"),
            RegisterRole::Other("tracecontrol2"),
        ],
        id: RegisterId(0x8000 | 23 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.24.2",
        roles: &[
            RegisterRole::Other("cp0.24.2"),
            RegisterRole::Other("tracecontrol3"),
        ],
        id: RegisterId(0x8000 | 24 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.25.2",
        roles: &[
            RegisterRole::Other("cp0.25.2"),
            RegisterRole::Other("perfctl1"),
        ],
        id: RegisterId(0x8000 | 25 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.28.2",
        roles: &[
            RegisterRole::Other("cp0.28.2"),
            RegisterRole::Other("dtaglo"),
        ],
        id: RegisterId(0x8000 | 28 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.29.2",
        roles: &[
            RegisterRole::Other("cp0.29.2"),
            RegisterRole::Other("dtaghi"),
        ],
        id: RegisterId(0x8000 | 29 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.31.2",
        roles: &[
            RegisterRole::Other("cp0.31.2"),
            RegisterRole::Other("kscratch1"),
        ],
        id: RegisterId(0x8000 | 31 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.0.3",
        roles: &[
            RegisterRole::Other("cp0.0.3"),
            RegisterRole::Other("mvpconf1"),
        ],
        id: RegisterId(0x8000 | 0 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.1.3",
        roles: &[
            RegisterRole::Other("cp0.1.3"),
            RegisterRole::Other("vpeconf1"),
        ],
        id: RegisterId(0x8000 | 1 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.2.3",
        roles: &[
            RegisterRole::Other("cp0.2.3"),
            RegisterRole::Other("tcrestart"),
        ],
        id: RegisterId(0x8000 | 2 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.4.3",
        roles: &[
            RegisterRole::Other("cp0.4.3"),
            RegisterRole::Other("xcontextconfig"),
        ],
        id: RegisterId(0x8000 | 4 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.5.3",
        roles: &[
            RegisterRole::Other("cp0.5.3"),
            RegisterRole::Other("segctl1"),
        ],
        id: RegisterId(0x8000 | 5 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.6.3",
        roles: &[
            RegisterRole::Other("cp0.6.3"),
            RegisterRole::Other("srsconf2"),
        ],
        id: RegisterId(0x8000 | 6 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.12.3",
        roles: &[
            RegisterRole::Other("cp0.12.3"),
            RegisterRole::Other("srsmap"),
        ],
        id: RegisterId(0x8000 | 12 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.15.3",
        roles: &[
            RegisterRole::Other("cp0.15.3"),
            RegisterRole::Other("cmgcrbase"),
        ],
        id: RegisterId(0x8000 | 15 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.16.3",
        roles: &[
            RegisterRole::Other("cp0.16.3"),
            RegisterRole::Other("config3"),
        ],
        id: RegisterId(0x8000 | 16 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.18.3",
        roles: &[
            RegisterRole::Other("cp0.18.3"),
            RegisterRole::Other("watchlo3"),
        ],
        id: RegisterId(0x8000 | 18 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.19.3",
        roles: &[
            RegisterRole::Other("cp0.19.3"),
            RegisterRole::Other("watchhi3"),
        ],
        id: RegisterId(0x8000 | 19 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.23.3",
        roles: &[
            RegisterRole::Other("cp0.23.3"),
            RegisterRole::Other("usertracedata1"),
        ],
        id: RegisterId(0x8000 | 23 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.24.3",
        roles: &[
            RegisterRole::Other("cp0.24.3"),
            RegisterRole::Other("usertracedata2"),
        ],
        id: RegisterId(0x8000 | 24 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.25.3",
        roles: &[
            RegisterRole::Other("cp0.25.3"),
            RegisterRole::Other("perfcnt1"),
        ],
        id: RegisterId(0x8000 | 25 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.28.3",
        roles: &[
            RegisterRole::Other("cp0.28.3"),
            RegisterRole::Other("ddatalo"),
        ],
        id: RegisterId(0x8000 | 28 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.29.3",
        roles: &[
            RegisterRole::Other("cp0.29.3"),
            RegisterRole::Other("ddatahi"),
        ],
        id: RegisterId(0x8000 | 29 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.31.3",
        roles: &[
            RegisterRole::Other("cp0.31.3"),
            RegisterRole::Other("kscratch2"),
        ],
        id: RegisterId(0x8000 | 31 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.1.4",
        roles: &[
            RegisterRole::Other("cp0.1.4"),
            RegisterRole::Other("yqmask"),
        ],
        id: RegisterId(0x8000 | 1 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.2.4",
        roles: &[
            RegisterRole::Other("cp0.2.4"),
            RegisterRole::Other("tchalt"),
        ],
        id: RegisterId(0x8000 | 2 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.4.4",
        roles: &[
            RegisterRole::Other("cp0.4.4"),
            RegisterRole::Other("debugcontextid"),
        ],
        id: RegisterId(0x8000 | 4 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.5.4",
        roles: &[
            RegisterRole::Other("cp0.5.4"),
            RegisterRole::Other("segctl2"),
        ],
        id: RegisterId(0x8000 | 5 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.6.4",
        roles: &[
            RegisterRole::Other("cp0.6.4"),
            RegisterRole::Other("srsconf3"),
        ],
        id: RegisterId(0x8000 | 6 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.10.4",
        roles: &[
            RegisterRole::Other("cp0.10.4"),
            RegisterRole::Other("guestctl1"),
        ],
        id: RegisterId(0x8000 | 10 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.12.4",
        roles: &[
            RegisterRole::Other("cp0.12.4"),
            RegisterRole::Other("view_ipl"),
        ],
        id: RegisterId(0x8000 | 12 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.13.4",
        roles: &[
            RegisterRole::Other("cp0.13.4"),
            RegisterRole::Other("view_ripl"),
        ],
        id: RegisterId(0x8000 | 13 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.15.4",
        roles: &[
            RegisterRole::Other("cp0.15.4"),
            RegisterRole::Other("bevva"),
        ],
        id: RegisterId(0x8000 | 15 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.16.4",
        roles: &[
            RegisterRole::Other("cp0.16.4"),
            RegisterRole::Other("config4"),
        ],
        id: RegisterId(0x8000 | 16 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.18.4",
        roles: &[
            RegisterRole::Other("cp0.18.4"),
            RegisterRole::Other("watchlo4"),
        ],
        id: RegisterId(0x8000 | 18 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.19.4",
        roles: &[
            RegisterRole::Other("cp0.19.4"),
            RegisterRole::Other("watchhi4"),
        ],
        id: RegisterId(0x8000 | 19 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.23.4",
        roles: &[
            RegisterRole::Other("cp0.23.4"),
            RegisterRole::Other("traceibpc"),
        ],
        id: RegisterId(0x8000 | 23 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.25.4",
        roles: &[
            RegisterRole::Other("cp0.25.4"),
            RegisterRole::Other("perfctl2"),
        ],
        id: RegisterId(0x8000 | 25 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.28.4",
        roles: &[
            RegisterRole::Other("cp0.28.4"),
            RegisterRole::Other("l23taglo"),
        ],
        id: RegisterId(0x8000 | 28 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.31.4",
        roles: &[
            RegisterRole::Other("cp0.31.4"),
            RegisterRole::Other("kscratch3"),
        ],
        id: RegisterId(0x8000 | 31 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.1.5",
        roles: &[
            RegisterRole::Other("cp0.1.5"),
            RegisterRole::Other("vpeschedule"),
        ],
        id: RegisterId(0x8000 | 1 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.2.5",
        roles: &[
            RegisterRole::Other("cp0.2.5"),
            RegisterRole::Other("tccontext"),
        ],
        id: RegisterId(0x8000 | 2 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.4.5",
        roles: &[RegisterRole::Other("cp0.4.5"), RegisterRole::Other("mmid")],
        id: RegisterId(0x8000 | 4 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.6.5",
        roles: &[
            RegisterRole::Other("cp0.6.5"),
            RegisterRole::Other("srsconf4"),
        ],
        id: RegisterId(0x8000 | 6 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.12.5",
        roles: &[
            RegisterRole::Other("cp0.12.5"),
            RegisterRole::Other("srsmap2"),
        ],
        id: RegisterId(0x8000 | 12 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.16.5",
        roles: &[
            RegisterRole::Other("cp0.16.5"),
            RegisterRole::Other("config5"),
        ],
        id: RegisterId(0x8000 | 16 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.18.5",
        roles: &[
            RegisterRole::Other("cp0.18.5"),
            RegisterRole::Other("watchlo5"),
        ],
        id: RegisterId(0x8000 | 18 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.19.5",
        roles: &[
            RegisterRole::Other("cp0.19.5"),
            RegisterRole::Other("watchhi5"),
        ],
        id: RegisterId(0x8000 | 19 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.23.5",
        roles: &[
            RegisterRole::Other("cp0.23.5"),
            RegisterRole::Other("tracedbpc"),
        ],
        id: RegisterId(0x8000 | 23 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.25.5",
        roles: &[
            RegisterRole::Other("cp0.25.5"),
            RegisterRole::Other("perfcnt2"),
        ],
        id: RegisterId(0x8000 | 25 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.28.5",
        roles: &[
            RegisterRole::Other("cp0.28.5"),
            RegisterRole::Other("l23datalo"),
        ],
        id: RegisterId(0x8000 | 28 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.29.5",
        roles: &[
            RegisterRole::Other("cp0.29.5"),
            RegisterRole::Other("l23datahi"),
        ],
        id: RegisterId(0x8000 | 29 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.31.5",
        roles: &[
            RegisterRole::Other("cp0.31.5"),
            RegisterRole::Other("kscratch4"),
        ],
        id: RegisterId(0x8000 | 31 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.1.6",
        roles: &[
            RegisterRole::Other("cp0.1.6"),
            RegisterRole::Other("vpeschefback"),
        ],
        id: RegisterId(0x8000 | 1 << 3 | 6),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.2.6",
        roles: &[
            RegisterRole::Other("cp0.2.6"),
            RegisterRole::Other("tcschedule"),
        ],
        id: RegisterId(0x8000 | 2 << 3 | 6),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.12.6",
        roles: &[
            RegisterRole::Other("cp0.12.6"),
            RegisterRole::Other("guestctl0"),
        ],
        id: RegisterId(0x8000 | 12 << 3 | 6),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.16.6",
        roles: &[
            RegisterRole::Other("cp0.16.6"),
            RegisterRole::Other("config6"),
        ],
        id: RegisterId(0x8000 | 16 << 3 | 6),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.18.6",
        roles: &[
            RegisterRole::Other("cp0.18.6"),
            RegisterRole::Other("watchlo6"),
        ],
        id: RegisterId(0x8000 | 18 << 3 | 6),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.19.6",
        roles: &[
            RegisterRole::Other("cp0.19.6"),
            RegisterRole::Other("watchhi6"),
        ],
        id: RegisterId(0x8000 | 19 << 3 | 6),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.23.6",
        roles: &[
            RegisterRole::Other("cp0.23.6"),
            RegisterRole::Other("debug2"),
        ],
        id: RegisterId(0x8000 | 23 << 3 | 6),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.25.6",
        roles: &[
            RegisterRole::Other("cp0.25.6"),
            RegisterRole::Other("perfctl3"),
        ],
        id: RegisterId(0x8000 | 25 << 3 | 6),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.31.6",
        roles: &[
            RegisterRole::Other("cp0.31.6"),
            RegisterRole::Other("kscratch5"),
        ],
        id: RegisterId(0x8000 | 31 << 3 | 6),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.1.7",
        roles: &[
            RegisterRole::Other("cp0.1.7"),
            RegisterRole::Other("vpeopt"),
        ],
        id: RegisterId(0x8000 | 1 << 3 | 7),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.2.7",
        roles: &[
            RegisterRole::Other("cp0.2.7"),
            RegisterRole::Other("tcschefback"),
        ],
        id: RegisterId(0x8000 | 2 << 3 | 7),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.3.7",
        roles: &[RegisterRole::Other("cp0.3.7"), RegisterRole::Other("tcopt")],
        id: RegisterId(0x8000 | 3 << 3 | 7),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.16.7",
        roles: &[
            RegisterRole::Other("cp0.16.7"),
            RegisterRole::Other("config7"),
        ],
        id: RegisterId(0x8000 | 16 << 3 | 7),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.18.7",
        roles: &[
            RegisterRole::Other("cp0.18.7"),
            RegisterRole::Other("watchlo7"),
        ],
        id: RegisterId(0x8000 | 18 << 3 | 7),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.19.7",
        roles: &[
            RegisterRole::Other("cp0.19.7"),
            RegisterRole::Other("watchhi7"),
        ],
        id: RegisterId(0x8000 | 19 << 3 | 7),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.25.7",
        roles: &[
            RegisterRole::Other("cp0.25.7"),
            RegisterRole::Other("perfcnt3"),
        ],
        id: RegisterId(0x8000 | 25 << 3 | 7),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
    CoreRegister {
        name: "cp0.31.7",
        roles: &[
            RegisterRole::Other("cp0.31.7"),
            RegisterRole::Other("kscratch6"),
        ],
        id: RegisterId(0x8000 | 31 << 3 | 7),
        data_type: RegisterDataType::UnsignedInteger(32),
    },
];
// MSA registers are marked by setting 2 msbs
// TODO: Add all msa registers and test them
// static MIPS_MSA_REGISTERS: &[CoreRegister] = &[CoreRegister {
//     name: "w0",
//     roles: &[RegisterRole::Other("w0"), RegisterRole::Other("w0")],
//     id: RegisterId(0xc000),
//     data_type: RegisterDataType::UnsignedInteger(64),
// }];
