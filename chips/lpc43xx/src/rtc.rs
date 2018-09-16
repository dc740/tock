
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// Real-Time Clock (RTC) and event recorder
#[repr(C)]
struct RtcRegisters {
/// Interrupt Location Register
ilr: WriteOnly<u32, ILR::Register>,
_reserved0: [u8; 4],
/// Clock Control Register
ccr: ReadWrite<u32, CCR::Register>,
/// Counter Increment Interrupt Register
ciir: ReadWrite<u32, CIIR::Register>,
/// Alarm Mask Register
amr: ReadWrite<u32, AMR::Register>,
/// Consolidated Time Register 0
ctime0: ReadOnly<u32, CTIME0::Register>,
/// Consolidated Time Register 1
ctime1: ReadOnly<u32, CTIME1::Register>,
/// Consolidated Time Register 2
ctime2: ReadOnly<u32>,
/// Seconds Register
sec: ReadWrite<u32>,
/// Minutes Register
min: ReadWrite<u32>,
/// Hours Register
hrs: ReadWrite<u32>,
/// Day of Month Register
dom: ReadWrite<u32>,
/// Day of Week Register
dow: ReadWrite<u32>,
/// Day of Year Register
doy: ReadWrite<u32>,
/// Months Register
month: ReadWrite<u32>,
/// Years Register
year: ReadWrite<u32>,
/// Calibration Value Register
calibration: ReadWrite<u32, CALIBRATION::Register>,
_reserved1: [u8; 28],
/// Alarm value for Seconds
asec: ReadWrite<u32>,
/// Alarm value for Minutes
amin: ReadWrite<u32>,
/// Alarm value for Hours
ahrs: ReadWrite<u32>,
/// Alarm value for Day of Month
adom: ReadWrite<u32>,
/// Alarm value for Day of Week
adow: ReadWrite<u32>,
/// Alarm value for Day of Year
adoy: ReadWrite<u32>,
/// Alarm value for Months
amon: ReadWrite<u32>,
/// Alarm value for Year
ayrs: ReadWrite<u32>,
/// Event Monitor/Recorder Status register. Contains status flags for event channels
erstatus: ReadWrite<u32, ERSTATUS::Register>,
/// Event Monitor/Recorder Control register. Contains bits that control actions for
ercontro: ReadWrite<u32, ERCONTRO::Register>,
/// Event Monitor/Recorder Counters register. Allows reading the counters associated
ercounters: ReadOnly<u32, ERCOUNTERS::Register>,
_reserved2: [u8; 4],
/// Event Monitor/Recorder First Stamp register for channel 0. Retains the time stam
erfirststamp0: ReadOnly<u32, ERFIRSTSTAMP0::Register>,
/// Event Monitor/Recorder First Stamp register for channel 0. Retains the time stam
erfirststamp1: ReadOnly<u32, ERFIRSTSTAMP1::Register>,
/// Event Monitor/Recorder First Stamp register for channel 0. Retains the time stam
erfirststamp2: ReadOnly<u32, ERFIRSTSTAMP2::Register>,
_reserved3: [u8; 4],
/// Event Monitor/Recorder Last Stamp register for channel 0. Retains the time stamp
erlaststamp0: ReadOnly<u32, ERLASTSTAMP0::Register>,
/// Event Monitor/Recorder Last Stamp register for channel 0. Retains the time stamp
erlaststamp1: ReadOnly<u32, ERLASTSTAMP1::Register>,
/// Event Monitor/Recorder Last Stamp register for channel 0. Retains the time stamp
erlaststamp2: ReadOnly<u32, ERLASTSTAMP2::Register>,
}
register_bitfields![u32,
ILR [
    /// When one, the Counter Increment Interrupt block generated an interrupt. Writing
    RTCCIF OFFSET(0) NUMBITS(1) [],
    /// When one, the alarm registers generated an interrupt. Writing a one to this bit
    RTCALF OFFSET(1) NUMBITS(1) []
],
CCR [
    /// Clock Enable.
    CLKEN OFFSET(0) NUMBITS(1) [
        /// The time counters are disabled so that they may be initialized.
        TheTimeCountersAreDisabledSoThatTheyMayBeInitialized = 0,
        /// The time counters are enabled.
        TheTimeCountersAreEnabled = 1
    ],
    /// CTC Reset.
    CTCRST OFFSET(1) NUMBITS(1) [
        /// No effect.
        NoEffect = 0,
        /// When one, the elements in the internal oscillator divider are reset, and remain
        RESET = 1
    ],
    /// Calibration counter enable.
    CCALEN OFFSET(4) NUMBITS(1) [
        /// The calibration counter is enabled and counting, using the 1 Hz clock. When the
        ENABLED = 0,
        /// The calibration counter is disabled and reset to zero.
        TheCalibrationCounterIsDisabledAndResetToZero = 1
    ]
],
CIIR [
    /// When 1, an increment of the Second value generates an interrupt.
    IMSEC OFFSET(0) NUMBITS(1) [],
    /// When 1, an increment of the Minute value generates an interrupt.
    IMMIN OFFSET(1) NUMBITS(1) [],
    /// When 1, an increment of the Hour value generates an interrupt.
    IMHOUR OFFSET(2) NUMBITS(1) [],
    /// When 1, an increment of the Day of Month value generates an interrupt.
    IMDOM OFFSET(3) NUMBITS(1) [],
    /// When 1, an increment of the Day of Week value generates an interrupt.
    IMDOW OFFSET(4) NUMBITS(1) [],
    /// When 1, an increment of the Day of Year value generates an interrupt.
    IMDOY OFFSET(5) NUMBITS(1) [],
    /// When 1, an increment of the Month value generates an interrupt.
    IMMON OFFSET(6) NUMBITS(1) [],
    /// When 1, an increment of the Year value generates an interrupt.
    IMYEAR OFFSET(7) NUMBITS(1) []
],
AMR [
    /// When 1, the Second value is not compared for the alarm.
    AMRSEC OFFSET(0) NUMBITS(1) [],
    /// When 1, the Minutes value is not compared for the alarm.
    AMRMIN OFFSET(1) NUMBITS(1) [],
    /// When 1, the Hour value is not compared for the alarm.
    AMRHOUR OFFSET(2) NUMBITS(1) [],
    /// When 1, the Day of Month value is not compared for the alarm.
    AMRDOM OFFSET(3) NUMBITS(1) [],
    /// When 1, the Day of Week value is not compared for the alarm.
    AMRDOW OFFSET(4) NUMBITS(1) [],
    /// When 1, the Day of Year value is not compared for the alarm.
    AMRDOY OFFSET(5) NUMBITS(1) [],
    /// When 1, the Month value is not compared for the alarm.
    AMRMON OFFSET(6) NUMBITS(1) [],
    /// When 1, the Year value is not compared for the alarm.
    AMRYEAR OFFSET(7) NUMBITS(1) []
],
CTIME0 [
    /// Seconds value in the range of 0 to 59
    SECONDS OFFSET(0) NUMBITS(6) [],
    /// Minutes value in the range of 0 to 59
    MINUTES OFFSET(8) NUMBITS(6) [],
    /// Hours value in the range of 0 to 23
    HOURS OFFSET(16) NUMBITS(5) [],
    /// Day of week value in the range of 0 to 6
    DOW OFFSET(24) NUMBITS(3) []
],
CTIME1 [
    /// Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the mont
    DOM OFFSET(0) NUMBITS(5) [],
    /// Month value in the range of 1 to 12.
    MONTH OFFSET(8) NUMBITS(4) [],
    /// Year value in the range of 0 to 4095.
    YEAR OFFSET(16) NUMBITS(12) []
],
CALIBRATION [
    /// If enabled, the calibration counter counts up to this value. The maximum value i
    CALVAL OFFSET(0) NUMBITS(17) [],
    /// Calibration direction
    CALDIR OFFSET(17) NUMBITS(1) [
        /// Forward calibration. When CALVAL is equal to the calibration counter, the RTC ti
        FORWARD_CALIBRATION_ = 0,
        /// Backward calibration. When CALVAL is equal to the calibration counter, the RTC t
        BACKWARD_CALIBRATION = 1
    ]
],
ERCONTRO [
    /// Interrupt and wake-up enable for channel 0.
    INTWAKE_EN0 OFFSET(0) NUMBITS(1) [
        /// No interrupt or wake-up will be generated by event channel 0.
        NoInterruptOrWakeUpWillBeGeneratedByEventChannel0 = 0,
        /// An event in channel 0 will trigger an (RTC) interrupt and a wake-up request.
        AnEventInChannel0WillTriggerAnRTCInterruptAndAWakeUpRequest = 1
    ],
    /// Enables automatically clearing the RTC general purpose registers when an event o
    GPCLEAR_EN0 OFFSET(1) NUMBITS(1) [
        /// Channel 0 has no influence on the general purpose registers.
        Channel0HasNoInfluenceOnTheGeneralPurposeRegisters = 0,
        /// An event in channel 0 will clear the general purpose registers asynchronously.
        AnEventInChannel0WillClearTheGeneralPurposeRegistersAsynchronously = 1
    ],
    /// Selects the polarity of an event on input pin WAKEUP0.
    POL0 OFFSET(2) NUMBITS(1) [
        /// A channel 0 event is defined as a negative edge on WAKEUP0.
        AChannel0EventIsDefinedAsANegativeEdgeOnWAKEUP0 = 0,
        /// A channel 0 event is defined as a positive edge on WAKEUP0.
        AChannel0EventIsDefinedAsAPositiveEdgeOnWAKEUP0 = 1
    ],
    /// Event enable control for channel 0. Event Inputs should remain DISABLED when not
    EV0_INPUT_EN OFFSET(3) NUMBITS(1) [
        /// Event 0 input is disabled and forced high internally.
        Event0InputIsDisabledAndForcedHighInternally = 0,
        /// Event 0 input is enabled.
        Event0InputIsEnabled = 1
    ],
    /// Interrupt and wake-up enable for channel 1.
    INTWAKE_EN1 OFFSET(10) NUMBITS(1) [
        /// No interrupt or wake-up will be generated by event channel 1.
        NoInterruptOrWakeUpWillBeGeneratedByEventChannel1 = 0,
        /// An event in channel 1 will trigger an (RTC) interrupt and a wake-up request.
        AnEventInChannel1WillTriggerAnRTCInterruptAndAWakeUpRequest = 1
    ],
    /// Enables automatically clearing the RTC general purpose registers when an event o
    GPCLEAR_EN1 OFFSET(11) NUMBITS(1) [
        /// Channel 1 has no influence on the general purpose registers.
        Channel1HasNoInfluenceOnTheGeneralPurposeRegisters = 0,
        /// A n event in channel 1 will clear the general purpose registers asynchronously.
        ANEventInChannel1WillClearTheGeneralPurposeRegistersAsynchronously = 1
    ],
    /// Selects the polarity of an event on input pin WAKEUP1.
    POL1 OFFSET(12) NUMBITS(1) [
        /// A channel 1 event is defined as a negative edge on WAKEUP1.
        AChannel1EventIsDefinedAsANegativeEdgeOnWAKEUP1 = 0,
        /// A channel 1 event is defined as a positive edge on WAKEUP1.
        AChannel1EventIsDefinedAsAPositiveEdgeOnWAKEUP1 = 1
    ],
    /// Event enable control for channel 1. Event Inputs should remain DISABLED when not
    EV1_INPUT_EN OFFSET(13) NUMBITS(1) [
        /// Event 1 input is disabled and forced high internally.
        Event1InputIsDisabledAndForcedHighInternally = 0,
        /// Event 1 input is enabled.
        Event1InputIsEnabled = 1
    ],
    /// Interrupt and wake-up enable for channel 2.
    INTWAKE_EN2 OFFSET(20) NUMBITS(1) [
        /// No interrupt or wake-up will be generated by event channel 2.
        NoInterruptOrWakeUpWillBeGeneratedByEventChannel2 = 0,
        /// An event in channel 2 will trigger an (RTC) interrupt and a wake-up request.
        AnEventInChannel2WillTriggerAnRTCInterruptAndAWakeUpRequest = 1
    ],
    /// Enables automatically clearing the RTC general purpose registers when an event o
    GPCLEAR_EN2 OFFSET(21) NUMBITS(1) [
        /// Channel 2 has no influence on the general purpose registers.
        Channel2HasNoInfluenceOnTheGeneralPurposeRegisters = 0,
        /// An event in channel 2 will clear the general purpose registers asynchronously.
        AnEventInChannel2WillClearTheGeneralPurposeRegistersAsynchronously = 1
    ],
    /// Selects the polarity of an event on input pin WAKEUP2.
    POL2 OFFSET(22) NUMBITS(1) [
        /// A channel 2 event is defined as a negative edge on WAKEUP2.
        AChannel2EventIsDefinedAsANegativeEdgeOnWAKEUP2 = 0,
        /// A channel 2 event is defined as a positive edge on WAKEUP2.
        AChannel2EventIsDefinedAsAPositiveEdgeOnWAKEUP2 = 1
    ],
    /// Event enable control for channel 2. Event Inputs should remain DISABLED when not
    EV2_INPUT_EN OFFSET(23) NUMBITS(1) [
        /// Event 2 input is disabled and forced high internally.
        Event2InputIsDisabledAndForcedHighInternally = 0,
        /// Event 2 input is enabled.
        Event2InputIsEnabled = 1
    ],
    /// Controls enabling the Event Monitor/Recorder and selecting its operating frequen
    ERMODE OFFSET(30) NUMBITS(2) [
        /// Disable Event Monitor/Recorder clocks. Operation of the Event Monitor/Recorder i
        DISABLE_EVENT_MONITO = 0,
        /// 16 Hz sample clock. Enable Event Monitor/Recorder and select a 16 Hz sample cloc
        _16_HZ_SAMPLE_CLOCK = 1,
        /// 64 Hz sample clock. Enable Event Monitor/Recorder and select a 64 Hz sample cloc
        _64_HZ_SAMPLE_CLOCK = 2,
        /// 1 kHz sample clock. Enable Event Monitor/Recorder and select a 1 kHz sample cloc
        _1_KHZ_SAMPLE_CLOCK = 3
    ]
],
ERSTATUS [
    /// Channel0 event flag (WAKEUP0 pin). Set at the end of any second if there has bee
    EV0 OFFSET(0) NUMBITS(1) [
        /// No event change on channel 0.
        NoEventChangeOnChannel0 = 0,
        /// At least one event has occurred on channel 0.
        AtLeastOneEventHasOccurredOnChannel0 = 1
    ],
    /// Channel1 Event flag (WAKEUP1 pin). Set at the end of any second if there has bee
    EV1 OFFSET(1) NUMBITS(1) [
        /// No event change on channel 1.
        NoEventChangeOnChannel1 = 0,
        /// At least one event has occurred on channel 1.
        AtLeastOneEventHasOccurredOnChannel1 = 1
    ],
    /// Channel2 Event flag (WAKEUP2 pin). Set at the end of any second if there has bee
    EV2 OFFSET(2) NUMBITS(1) [
        /// No event change on channel 2.
        NoEventChangeOnChannel2 = 0,
        /// At least one event has occurred on channel 2.
        AtLeastOneEventHasOccurredOnChannel2 = 1
    ],
    /// General purpose register asynchronous clear flag. This bit is cleared by writing
    GP_CLEARED OFFSET(3) NUMBITS(1) [
        /// General purpose registers have not been asynchronous cleared.
        GeneralPurposeRegistersHaveNotBeenAsynchronousCleared = 0,
        /// General purpose registers have been asynchronous cleared.
        GeneralPurposeRegistersHaveBeenAsynchronousCleared = 1
    ],
    /// Interrupt/wake-up request flag (Read-only). This bit is cleared by writing a 1 t
    WAKEUP OFFSET(31) NUMBITS(1) [
        /// No interrupt/wake-up request is pending
        NoInterruptWakeUpRequestIsPending = 0,
        /// An interrupt/wake-up request is pending.
        AnInterruptWakeUpRequestIsPending = 1
    ]
],
ERCOUNTERS [
    /// Value of the counter for Event 0. If the counter reaches full count (the value 7
    COUNTER0 OFFSET(0) NUMBITS(3) [],
    /// Value of the counter for event 1. See description for COUNTER0.
    COUNTER1 OFFSET(8) NUMBITS(3) [],
    /// Value of the counter for event 2. See description for COUNTER0.
    COUNTER2 OFFSET(16) NUMBITS(3) []
],
ERFIRSTSTAMP0 [
    /// Seconds value in the range of 0 to 59.
    SEC OFFSET(0) NUMBITS(6) [],
    /// Minutes value in the range of 0 to 59.
    MIN OFFSET(6) NUMBITS(6) [],
    /// Hours value in the range of 0 to 23.
    HOUR OFFSET(12) NUMBITS(5) [],
    /// Day of Year value in the range of 1 to 366.
    DOY OFFSET(17) NUMBITS(9) []
],
ERFIRSTSTAMP1 [
    /// Seconds value in the range of 0 to 59.
    SEC OFFSET(0) NUMBITS(6) [],
    /// Minutes value in the range of 0 to 59.
    MIN OFFSET(6) NUMBITS(6) [],
    /// Hours value in the range of 0 to 23.
    HOUR OFFSET(12) NUMBITS(5) [],
    /// Day of Year value in the range of 1 to 366.
    DOY OFFSET(17) NUMBITS(9) []
],
ERFIRSTSTAMP2 [
    /// Seconds value in the range of 0 to 59.
    SEC OFFSET(0) NUMBITS(6) [],
    /// Minutes value in the range of 0 to 59.
    MIN OFFSET(6) NUMBITS(6) [],
    /// Hours value in the range of 0 to 23.
    HOUR OFFSET(12) NUMBITS(5) [],
    /// Day of Year value in the range of 1 to 366.
    DOY OFFSET(17) NUMBITS(9) []
],
ERLASTSTAMP0 [
    /// Seconds value in the range of 0 to 59.
    SEC OFFSET(0) NUMBITS(6) [],
    /// Minutes value in the range of 0 to 59.
    MIN OFFSET(6) NUMBITS(6) [],
    /// Hours value in the range of 0 to 23.
    HOUR OFFSET(12) NUMBITS(5) [],
    /// Day of Year value in the range of 1 to 366.
    DOY OFFSET(17) NUMBITS(9) []
],
ERLASTSTAMP1 [
    /// Seconds value in the range of 0 to 59.
    SEC OFFSET(0) NUMBITS(6) [],
    /// Minutes value in the range of 0 to 59.
    MIN OFFSET(6) NUMBITS(6) [],
    /// Hours value in the range of 0 to 23.
    HOUR OFFSET(12) NUMBITS(5) [],
    /// Day of Year value in the range of 1 to 366.
    DOY OFFSET(17) NUMBITS(9) []
],
ERLASTSTAMP2 [
    /// Seconds value in the range of 0 to 59.
    SEC OFFSET(0) NUMBITS(6) [],
    /// Minutes value in the range of 0 to 59.
    MIN OFFSET(6) NUMBITS(6) [],
    /// Hours value in the range of 0 to 23.
    HOUR OFFSET(12) NUMBITS(5) [],
    /// Day of Year value in the range of 1 to 366.
    DOY OFFSET(17) NUMBITS(9) []
]
];
const RTC_BASE: StaticRef<RtcRegisters> =
    unsafe { StaticRef::new(0x40046000 as *const RtcRegisters) };
