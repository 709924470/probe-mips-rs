use crate::{
    core::{CoreRegisters, RegisterDataType, RegisterId, RegisterRole, UnwindRule},
    CoreRegister,
};

/// CP0 registers are marked by setting msb
pub static MIPS32_CP0_REGISTERS: &[CoreRegister] = &[
    CoreRegister {
        roles: &[RegisterRole::Other("cp0.0.0"), RegisterRole::Other("index")],
        id: RegisterId(0x8000),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.1.0"),
            RegisterRole::Other("random"),
        ],
        id: RegisterId(0x8000 | 1 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.2.0"),
            RegisterRole::Other("entrylo0"),
        ],
        id: RegisterId(0x8000 | 2 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.3.0"),
            RegisterRole::Other("entrylo1"),
        ],
        id: RegisterId(0x8000 | 3 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.4.0"),
            RegisterRole::Other("context"),
        ],
        id: RegisterId(0x8000 | 4 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.5.0"),
            RegisterRole::Other("pagemask"),
        ],
        id: RegisterId(0x8000 | 5 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Other("cp0.6.0"), RegisterRole::Other("wired")],
        id: RegisterId(0x8000 | 6 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.7.0"),
            RegisterRole::Other("hwrena"),
        ],
        id: RegisterId(0x8000 | 7 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.8.0"),
            RegisterRole::Other("badvaddr"),
        ],
        id: RegisterId(0x8000 | 8 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Other("cp0.9.0"), RegisterRole::Other("count")],
        id: RegisterId(0x8000 | 9 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.10.0"),
            RegisterRole::Other("entryhi"),
        ],
        id: RegisterId(0x8000 | 10 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.11.0"),
            RegisterRole::Other("compare"),
        ],
        id: RegisterId(0x8000 | 11 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.12.0"),
            RegisterRole::Other("status"),
        ],
        id: RegisterId(0x8000 | 12 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.13.0"),
            RegisterRole::Other("cause"),
        ],
        id: RegisterId(0x8000 | 13 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Other("cp0.14.0"), RegisterRole::Other("epc")],
        id: RegisterId(0x8000 | 14 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Other("cp0.15.0"), RegisterRole::Other("prid")],
        id: RegisterId(0x8000 | 15 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.16.0"),
            RegisterRole::Other("config"),
        ],
        id: RegisterId(0x8000 | 16 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.17.0"),
            RegisterRole::Other("lladdr"),
        ],
        id: RegisterId(0x8000 | 17 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.18.0"),
            RegisterRole::Other("watchlo0"),
        ],
        id: RegisterId(0x8000 | 18 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.19.0"),
            RegisterRole::Other("watchhi0"),
        ],
        id: RegisterId(0x8000 | 19 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.20.0"),
            RegisterRole::Other("xcontext"),
        ],
        id: RegisterId(0x8000 | 20 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.23.0"),
            RegisterRole::Other("debug"),
        ],
        id: RegisterId(0x8000 | 23 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Other("cp0.24.0"), RegisterRole::Other("depc")],
        id: RegisterId(0x8000 | 24 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.25.0"),
            RegisterRole::Other("perfctl0"),
        ],
        id: RegisterId(0x8000 | 25 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.26.0"),
            RegisterRole::Other("errctl"),
        ],
        id: RegisterId(0x8000 | 26 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.27.0"),
            RegisterRole::Other("cacheerr"),
        ],
        id: RegisterId(0x8000 | 27 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.28.0"),
            RegisterRole::Other("itaglo"),
        ],
        id: RegisterId(0x8000 | 28 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.29.0"),
            RegisterRole::Other("itaghi"),
        ],
        id: RegisterId(0x8000 | 29 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.30.0"),
            RegisterRole::Other("errorepc"),
        ],
        id: RegisterId(0x8000 | 30 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.31.0"),
            RegisterRole::Other("desave"),
        ],
        id: RegisterId(0x8000 | 31 << 3 | 0),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.0.1"),
            RegisterRole::Other("mvpcontrol"),
        ],
        id: RegisterId(0x8000 | 0 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.1.1"),
            RegisterRole::Other("vpecontrol"),
        ],
        id: RegisterId(0x8000 | 1 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.2.1"),
            RegisterRole::Other("tcstatus"),
        ],
        id: RegisterId(0x8000 | 2 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.3.1"),
            RegisterRole::Other("globalnumber"),
        ],
        id: RegisterId(0x8000 | 3 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.4.1"),
            RegisterRole::Other("contextconfig"),
        ],
        id: RegisterId(0x8000 | 4 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.5.1"),
            RegisterRole::Other("pagegrain"),
        ],
        id: RegisterId(0x8000 | 5 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.6.1"),
            RegisterRole::Other("srsconf0"),
        ],
        id: RegisterId(0x8000 | 6 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.8.1"),
            RegisterRole::Other("badinstr"),
        ],
        id: RegisterId(0x8000 | 8 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.12.1"),
            RegisterRole::Other("intctl"),
        ],
        id: RegisterId(0x8000 | 12 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.15.1"),
            RegisterRole::Other("ebase"),
        ],
        id: RegisterId(0x8000 | 15 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.16.1"),
            RegisterRole::Other("config1"),
        ],
        id: RegisterId(0x8000 | 16 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Other("cp0.17.1"), RegisterRole::Other("maar")],
        id: RegisterId(0x8000 | 17 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.18.1"),
            RegisterRole::Other("watchlo1"),
        ],
        id: RegisterId(0x8000 | 18 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.19.1"),
            RegisterRole::Other("watchhi1"),
        ],
        id: RegisterId(0x8000 | 19 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.23.1"),
            RegisterRole::Other("tracecontrol"),
        ],
        id: RegisterId(0x8000 | 23 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.25.1"),
            RegisterRole::Other("perfcnt0"),
        ],
        id: RegisterId(0x8000 | 25 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.26.1"),
            RegisterRole::Other("ierrctl"),
        ],
        id: RegisterId(0x8000 | 26 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.28.1"),
            RegisterRole::Other("idatalo"),
        ],
        id: RegisterId(0x8000 | 28 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.29.1"),
            RegisterRole::Other("idatahi"),
        ],
        id: RegisterId(0x8000 | 29 << 3 | 1),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.0.2"),
            RegisterRole::Other("mvpconf0"),
        ],
        id: RegisterId(0x8000 | 0 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.1.2"),
            RegisterRole::Other("vpeconf0"),
        ],
        id: RegisterId(0x8000 | 1 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.2.2"),
            RegisterRole::Other("tcbind"),
        ],
        id: RegisterId(0x8000 | 2 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.4.2"),
            RegisterRole::Other("userlocal"),
        ],
        id: RegisterId(0x8000 | 4 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.5.2"),
            RegisterRole::Other("segctl0"),
        ],
        id: RegisterId(0x8000 | 5 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.6.2"),
            RegisterRole::Other("srsconf1"),
        ],
        id: RegisterId(0x8000 | 6 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.8.2"),
            RegisterRole::Other("badinstrp"),
        ],
        id: RegisterId(0x8000 | 8 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.12.2"),
            RegisterRole::Other("srsctl"),
        ],
        id: RegisterId(0x8000 | 12 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.15.2"),
            RegisterRole::Other("cdmmbase"),
        ],
        id: RegisterId(0x8000 | 15 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.16.2"),
            RegisterRole::Other("config2"),
        ],
        id: RegisterId(0x8000 | 16 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.17.2"),
            RegisterRole::Other("maari"),
        ],
        id: RegisterId(0x8000 | 17 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.18.2"),
            RegisterRole::Other("watchlo2"),
        ],
        id: RegisterId(0x8000 | 18 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.19.2"),
            RegisterRole::Other("watchhi2"),
        ],
        id: RegisterId(0x8000 | 19 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.23.2"),
            RegisterRole::Other("tracecontrol2"),
        ],
        id: RegisterId(0x8000 | 23 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.24.2"),
            RegisterRole::Other("tracecontrol3"),
        ],
        id: RegisterId(0x8000 | 24 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.25.2"),
            RegisterRole::Other("perfctl1"),
        ],
        id: RegisterId(0x8000 | 25 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.28.2"),
            RegisterRole::Other("dtaglo"),
        ],
        id: RegisterId(0x8000 | 28 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.29.2"),
            RegisterRole::Other("dtaghi"),
        ],
        id: RegisterId(0x8000 | 29 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.31.2"),
            RegisterRole::Other("kscratch1"),
        ],
        id: RegisterId(0x8000 | 31 << 3 | 2),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.0.3"),
            RegisterRole::Other("mvpconf1"),
        ],
        id: RegisterId(0x8000 | 0 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.1.3"),
            RegisterRole::Other("vpeconf1"),
        ],
        id: RegisterId(0x8000 | 1 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.2.3"),
            RegisterRole::Other("tcrestart"),
        ],
        id: RegisterId(0x8000 | 2 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.4.3"),
            RegisterRole::Other("xcontextconfig"),
        ],
        id: RegisterId(0x8000 | 4 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.5.3"),
            RegisterRole::Other("segctl1"),
        ],
        id: RegisterId(0x8000 | 5 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.6.3"),
            RegisterRole::Other("srsconf2"),
        ],
        id: RegisterId(0x8000 | 6 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.12.3"),
            RegisterRole::Other("srsmap"),
        ],
        id: RegisterId(0x8000 | 12 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.15.3"),
            RegisterRole::Other("cmgcrbase"),
        ],
        id: RegisterId(0x8000 | 15 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.16.3"),
            RegisterRole::Other("config3"),
        ],
        id: RegisterId(0x8000 | 16 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.18.3"),
            RegisterRole::Other("watchlo3"),
        ],
        id: RegisterId(0x8000 | 18 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.19.3"),
            RegisterRole::Other("watchhi3"),
        ],
        id: RegisterId(0x8000 | 19 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.23.3"),
            RegisterRole::Other("usertracedata1"),
        ],
        id: RegisterId(0x8000 | 23 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.24.3"),
            RegisterRole::Other("usertracedata2"),
        ],
        id: RegisterId(0x8000 | 24 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.25.3"),
            RegisterRole::Other("perfcnt1"),
        ],
        id: RegisterId(0x8000 | 25 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.28.3"),
            RegisterRole::Other("ddatalo"),
        ],
        id: RegisterId(0x8000 | 28 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.29.3"),
            RegisterRole::Other("ddatahi"),
        ],
        id: RegisterId(0x8000 | 29 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.31.3"),
            RegisterRole::Other("kscratch2"),
        ],
        id: RegisterId(0x8000 | 31 << 3 | 3),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.1.4"),
            RegisterRole::Other("yqmask"),
        ],
        id: RegisterId(0x8000 | 1 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.2.4"),
            RegisterRole::Other("tchalt"),
        ],
        id: RegisterId(0x8000 | 2 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.4.4"),
            RegisterRole::Other("debugcontextid"),
        ],
        id: RegisterId(0x8000 | 4 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.5.4"),
            RegisterRole::Other("segctl2"),
        ],
        id: RegisterId(0x8000 | 5 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.6.4"),
            RegisterRole::Other("srsconf3"),
        ],
        id: RegisterId(0x8000 | 6 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.10.4"),
            RegisterRole::Other("guestctl1"),
        ],
        id: RegisterId(0x8000 | 10 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.12.4"),
            RegisterRole::Other("view_ipl"),
        ],
        id: RegisterId(0x8000 | 12 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.13.4"),
            RegisterRole::Other("view_ripl"),
        ],
        id: RegisterId(0x8000 | 13 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.15.4"),
            RegisterRole::Other("bevva"),
        ],
        id: RegisterId(0x8000 | 15 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.16.4"),
            RegisterRole::Other("config4"),
        ],
        id: RegisterId(0x8000 | 16 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.18.4"),
            RegisterRole::Other("watchlo4"),
        ],
        id: RegisterId(0x8000 | 18 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.19.4"),
            RegisterRole::Other("watchhi4"),
        ],
        id: RegisterId(0x8000 | 19 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.23.4"),
            RegisterRole::Other("traceibpc"),
        ],
        id: RegisterId(0x8000 | 23 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.25.4"),
            RegisterRole::Other("perfctl2"),
        ],
        id: RegisterId(0x8000 | 25 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.28.4"),
            RegisterRole::Other("l23taglo"),
        ],
        id: RegisterId(0x8000 | 28 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.31.4"),
            RegisterRole::Other("kscratch3"),
        ],
        id: RegisterId(0x8000 | 31 << 3 | 4),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.1.5"),
            RegisterRole::Other("vpeschedule"),
        ],
        id: RegisterId(0x8000 | 1 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.2.5"),
            RegisterRole::Other("tccontext"),
        ],
        id: RegisterId(0x8000 | 2 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Other("cp0.4.5"), RegisterRole::Other("mmid")],
        id: RegisterId(0x8000 | 4 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.6.5"),
            RegisterRole::Other("srsconf4"),
        ],
        id: RegisterId(0x8000 | 6 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.12.5"),
            RegisterRole::Other("srsmap2"),
        ],
        id: RegisterId(0x8000 | 12 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.16.5"),
            RegisterRole::Other("config5"),
        ],
        id: RegisterId(0x8000 | 16 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.18.5"),
            RegisterRole::Other("watchlo5"),
        ],
        id: RegisterId(0x8000 | 18 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.19.5"),
            RegisterRole::Other("watchhi5"),
        ],
        id: RegisterId(0x8000 | 19 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.23.5"),
            RegisterRole::Other("tracedbpc"),
        ],
        id: RegisterId(0x8000 | 23 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.25.5"),
            RegisterRole::Other("perfcnt2"),
        ],
        id: RegisterId(0x8000 | 25 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.28.5"),
            RegisterRole::Other("l23datalo"),
        ],
        id: RegisterId(0x8000 | 28 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.29.5"),
            RegisterRole::Other("l23datahi"),
        ],
        id: RegisterId(0x8000 | 29 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.31.5"),
            RegisterRole::Other("kscratch4"),
        ],
        id: RegisterId(0x8000 | 31 << 3 | 5),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.1.6"),
            RegisterRole::Other("vpeschefback"),
        ],
        id: RegisterId(0x8000 | 1 << 3 | 6),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.2.6"),
            RegisterRole::Other("tcschedule"),
        ],
        id: RegisterId(0x8000 | 2 << 3 | 6),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.12.6"),
            RegisterRole::Other("guestctl0"),
        ],
        id: RegisterId(0x8000 | 12 << 3 | 6),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.16.6"),
            RegisterRole::Other("config6"),
        ],
        id: RegisterId(0x8000 | 16 << 3 | 6),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.18.6"),
            RegisterRole::Other("watchlo6"),
        ],
        id: RegisterId(0x8000 | 18 << 3 | 6),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.19.6"),
            RegisterRole::Other("watchhi6"),
        ],
        id: RegisterId(0x8000 | 19 << 3 | 6),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.23.6"),
            RegisterRole::Other("debug2"),
        ],
        id: RegisterId(0x8000 | 23 << 3 | 6),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.25.6"),
            RegisterRole::Other("perfctl3"),
        ],
        id: RegisterId(0x8000 | 25 << 3 | 6),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.31.6"),
            RegisterRole::Other("kscratch5"),
        ],
        id: RegisterId(0x8000 | 31 << 3 | 6),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.1.7"),
            RegisterRole::Other("vpeopt"),
        ],
        id: RegisterId(0x8000 | 1 << 3 | 7),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.2.7"),
            RegisterRole::Other("tcschefback"),
        ],
        id: RegisterId(0x8000 | 2 << 3 | 7),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[RegisterRole::Other("cp0.3.7"), RegisterRole::Other("tcopt")],
        id: RegisterId(0x8000 | 3 << 3 | 7),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.16.7"),
            RegisterRole::Other("config7"),
        ],
        id: RegisterId(0x8000 | 16 << 3 | 7),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.18.7"),
            RegisterRole::Other("watchlo7"),
        ],
        id: RegisterId(0x8000 | 18 << 3 | 7),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.19.7"),
            RegisterRole::Other("watchhi7"),
        ],
        id: RegisterId(0x8000 | 19 << 3 | 7),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.25.7"),
            RegisterRole::Other("perfcnt3"),
        ],
        id: RegisterId(0x8000 | 25 << 3 | 7),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
    CoreRegister {
        roles: &[
            RegisterRole::Other("cp0.31.7"),
            RegisterRole::Other("kscratch6"),
        ],
        id: RegisterId(0x8000 | 31 << 3 | 7),
        data_type: RegisterDataType::UnsignedInteger(32),
        unwind_rule: UnwindRule::Clear,
    },
];
// MSA registers are marked by setting 2 msbs
// TODO: Add all msa registers and test them
// static MIPS_MSA_REGISTERS: &[CoreRegister] = &[CoreRegister {
//     roles: &[RegisterRole::Other("w0"), RegisterRole::Other("w0")],
//     id: RegisterId(0xc000),
//     data_type: RegisterDataType::UnsignedInteger(64),
//      unwind_rule: UnwindRule::Clear,
// }];
