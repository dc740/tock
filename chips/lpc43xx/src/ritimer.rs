
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// Repetitive Interrupt Timer (RIT)
#[repr(C)]
struct RitimerRegisters {
/// Compare register
compval: ReadWrite<u32>,
/// Mask register. This register holds the 32-bit mask value. A 1 written to any bit
mask: ReadWrite<u32>,
/// Control register.
ctrl: ReadWrite<u32, CTRL::Register>,
/// 32-bit counter
counter: ReadWrite<u32>,
}
register_bitfields![u32,
CTRL [
    /// Interrupt flag
    RITINT OFFSET(0) NUMBITS(1) [
        /// This bit is set to 1 by hardware whenever the counter value equals the masked co
        THIS_BIT_IS_SET_TO_1 = 1,
        /// The counter value does not equal the masked compare value.
        TheCounterValueDoesNotEqualTheMaskedCompareValue = 0
    ],
    /// Timer enable clear
    RITENCLR OFFSET(1) NUMBITS(1) [
        /// The timer will be cleared to 0 whenever the counter value equals the masked comp
        THE_TIMER_WILL_BE_CL = 1,
        /// The timer will not be cleared to 0.
        TheTimerWillNotBeClearedTo0 = 0
    ],
    /// Timer enable for debug
    RITENBR OFFSET(2) NUMBITS(1) [
        /// The timer is halted when the processor is halted for debugging.
        TheTimerIsHaltedWhenTheProcessorIsHaltedForDebugging = 1,
        /// Debug has no effect on the timer operation.
        DebugHasNoEffectOnTheTimerOperation = 0
    ],
    /// Timer enable.
    RITEN OFFSET(3) NUMBITS(1) [
        /// Timer enabled. This can be overruled by a debug halt if enabled in bit 2.
        TimerEnabledThisCanBeOverruledByADebugHaltIfEnabledInBit2 = 1,
        /// Timer disabled.
        TimerDisabled = 0
    ]
]
];
const RITIMER_BASE: StaticRef<RitimerRegisters> =
    unsafe { StaticRef::new(0x400C0000 as *const RitimerRegisters) };
