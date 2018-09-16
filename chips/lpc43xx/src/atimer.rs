
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// Alarm timer
#[repr(C)]
struct AtimerRegisters {
/// Downcounter register
downcounter: ReadWrite<u32>,
/// Preset value register
preset: ReadWrite<u32>,
_reserved0: [u8; 4048],
/// Interrupt clear enable register
clr_en: WriteOnly<u32>,
/// Interrupt set enable register
set_en: WriteOnly<u32>,
/// Status register
status: ReadOnly<u32>,
/// Enable register
enable: ReadOnly<u32>,
/// Clear register
clr_stat: WriteOnly<u32>,
/// Set register
set_stat: WriteOnly<u32>,
}

const ATIMER_BASE: StaticRef<AtimerRegisters> =
    unsafe { StaticRef::new(0x40040000 as *const AtimerRegisters) };
