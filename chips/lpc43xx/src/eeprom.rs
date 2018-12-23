
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly, register_bitfields};
    /// EEPROM
#[repr(C)]
struct EepromRegisters {
/// EEPROM command register
cmd: ReadWrite<u32>,
_reserved0: [u8; 4],
/// EEPROM read wait state register
rwstate: ReadWrite<u32, RWSTATE::Register>,
/// EEPROM auto programming register
autoprog: ReadWrite<u32>,
/// EEPROM wait state register
wstate: ReadWrite<u32, WSTATE::Register>,
/// EEPROM clock divider register
clkdiv: ReadWrite<u32>,
/// EEPROM power-down register
pwrdwn: ReadWrite<u32>,
_reserved1: [u8; 4028],
/// EEPROM interrupt enable clear
intenclr: WriteOnly<u32>,
/// EEPROM interrupt enable set
intenset: WriteOnly<u32>,
/// EEPROM interrupt status
intstat: ReadOnly<u32>,
/// EEPROM interrupt enable
inten: ReadOnly<u32>,
/// EEPROM interrupt status clear
intstatclr: WriteOnly<u32>,
}
register_bitfields![u32,
RWSTATE [
    /// Wait states 2 (minus 1 encoded). The number of system clock periods to meet the
    RPHASE2 OFFSET(0) NUMBITS(8) [],
    /// Wait states 1 (minus 1 encoded). The number of system clock periods to meet a du
    RPHASE1 OFFSET(8) NUMBITS(8) []
],
WSTATE [
    /// Wait states for phase 3 (minus 1 encoded). The number of system clock periods to
    PHASE3 OFFSET(0) NUMBITS(8) [],
    /// Wait states for phase 2 (minus 1 encoded). The number of system clock periods to
    PHASE2 OFFSET(8) NUMBITS(8) [],
    /// Wait states for phase 1 (minus 1 encoded). The number of system clock periods to
    PHASE1 OFFSET(16) NUMBITS(8) [],
    /// Lock timing parameters for write, erase and program operation  0 = WSTATE and CL
    LCK_PARWEP OFFSET(31) NUMBITS(1) []
]
];
const EEPROM_BASE: StaticRef<EepromRegisters> =
    unsafe { StaticRef::new(0x4000E000 as *const EepromRegisters) };
