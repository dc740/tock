
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly, register_bitfields};
    /// Windowed Watchdog timer (WWDT)
#[repr(C)]
struct WwdtRegisters {
/// Watchdog mode register. This register contains the basic mode and status of the
mod_: ReadWrite<u32, MOD::Register>,
/// Watchdog timer constant register. This register determines the time-out value.
tc: ReadWrite<u32>,
/// Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register
feed: WriteOnly<u32>,
/// Watchdog timer value register. This register reads out the current value of the
tv: ReadOnly<u32>,
_reserved0: [u8; 4],
/// Watchdog warning interrupt register. This register contains the Watchdog warning
warnint: ReadWrite<u32>,
/// Watchdog timer window register. This register contains the Watchdog window value
window: ReadWrite<u32>,
}
register_bitfields![u32,
MOD [
    /// Watchdog enable bit. This bit is Set Only.
    WDEN OFFSET(0) NUMBITS(1) [
        /// The watchdog timer is stopped.
        TheWatchdogTimerIsStopped = 0,
        /// The watchdog timer is running.
        TheWatchdogTimerIsRunning = 1
    ],
    /// Watchdog reset enable bit. This bit is Set Only.
    WDRESET OFFSET(1) NUMBITS(1) [
        /// A watchdog time-out will not cause a chip reset.
        AWatchdogTimeOutWillNotCauseAChipReset = 0,
        /// A watchdog time-out will cause a chip reset.
        AWatchdogTimeOutWillCauseAChipReset = 1
    ],
    /// Watchdog time-out flag. Set when the watchdog timer times out, by a feed error,
    WDTOF OFFSET(2) NUMBITS(1) [],
    /// Watchdog interrupt flag. Set when the timer reaches the value in the WARNINT reg
    WDINT OFFSET(3) NUMBITS(1) [],
    /// Watchdog update mode. This bit is Set Only.
    WDPROTECT OFFSET(4) NUMBITS(1) [
        /// The watchdog time-out value (WDTC) can be changed at any time.
        TheWatchdogTimeOutValueWDTCCanBeChangedAtAnyTime = 0,
        /// The watchdog time-out value (WDTC) can be changed only after the counter is belo
        LOCK = 1
    ]
]
];
const WWDT_BASE: StaticRef<WwdtRegisters> =
    unsafe { StaticRef::new(0x40080000 as *const WwdtRegisters) };
