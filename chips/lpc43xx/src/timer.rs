
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly, register_bitfields};
    /// Timer0/1/2/3
#[repr(C)]
struct TimerRegisters {
/// Interrupt Register. The IR can be written to clear interrupts. The IR can be rea
ir: ReadWrite<u32, IR::Register>,
/// Timer Control Register. The TCR is used to control the Timer Counter functions.
tcr: ReadWrite<u32, TCR::Register>,
/// Timer Counter. The 32 bit TC is incremented every PR+1 cycles of PCLK. The TC is
tc: ReadWrite<u32>,
/// Prescale Register. When the Prescale Counter (PC) is equal to this value, the ne
pr: ReadWrite<u32>,
/// Prescale Counter. The 32 bit PC is a counter which is incremented to the value s
pc: ReadWrite<u32>,
/// Match Control Register. The MCR is used to control if an interrupt is generated
mcr: ReadWrite<u32, MCR::Register>,
/// Match Register 0. MR0 can be enabled through the MCR to reset the TC, stop both
mr0: ReadWrite<u32>,
/// Match Register 0. MR0 can be enabled through the MCR to reset the TC, stop both
mr1: ReadWrite<u32>,
/// Match Register 0. MR0 can be enabled through the MCR to reset the TC, stop both
mr2: ReadWrite<u32>,
/// Match Register 0. MR0 can be enabled through the MCR to reset the TC, stop both
mr3: ReadWrite<u32>,
/// Capture Control Register. The CCR controls which edges of the capture inputs are
ccr: ReadWrite<u32, CCR::Register>,
/// Capture Register 0. CR0 is loaded with the value of TC when there is an event on
cr0: ReadOnly<u32>,
/// Capture Register 0. CR0 is loaded with the value of TC when there is an event on
cr1: ReadOnly<u32>,
/// Capture Register 0. CR0 is loaded with the value of TC when there is an event on
cr2: ReadOnly<u32>,
/// Capture Register 0. CR0 is loaded with the value of TC when there is an event on
cr3: ReadOnly<u32>,
/// External Match Register. The EMR controls the external match pins MATn.0-3 (MAT0
emr: ReadWrite<u32, EMR::Register>,
_reserved0: [u8; 48],
/// Count Control Register. The CTCR selects between Timer and Counter mode, and in
ctcr: ReadWrite<u32, CTCR::Register>,
}
register_bitfields![u32,
IR [
    /// Interrupt flag for match channel 0.
    MR0INT OFFSET(0) NUMBITS(1) [],
    /// Interrupt flag for match channel 1.
    MR1INT OFFSET(1) NUMBITS(1) [],
    /// Interrupt flag for match channel 2.
    MR2INT OFFSET(2) NUMBITS(1) [],
    /// Interrupt flag for match channel 3.
    MR3INT OFFSET(3) NUMBITS(1) [],
    /// Interrupt flag for capture channel 0 event.
    CR0INT OFFSET(4) NUMBITS(1) [],
    /// Interrupt flag for capture channel 1 event.
    CR1INT OFFSET(5) NUMBITS(1) [],
    /// Interrupt flag for capture channel 2 event.
    CR2INT OFFSET(6) NUMBITS(1) [],
    /// Interrupt flag for capture channel 3 event.
    CR3INT OFFSET(7) NUMBITS(1) []
],
TCR [
    /// When one, the Timer Counter and Prescale Counter are enabled for counting. When
    CEN OFFSET(0) NUMBITS(1) [],
    /// When one, the Timer Counter and the Prescale Counter are synchronously reset on
    CRST OFFSET(1) NUMBITS(1) []
],
MCR [
    /// Interrupt on MR0
    MR0I OFFSET(0) NUMBITS(1) [
        /// Disabled. Interrupt is disabled
        DisabledInterruptIsDisabled = 0,
        /// Enabled. Interrupt is generated when MR0 matches the value in the TC.
        EnabledInterruptIsGeneratedWhenMR0MatchesTheValueInTheTC = 1
    ],
    /// Reset on MR0
    MR0R OFFSET(1) NUMBITS(1) [
        /// Disabled. Feature disabled.
        DisabledFeatureDisabled = 0,
        /// Reset. TC will be reset if MR0 matches it.
        ResetTCWillBeResetIfMR0MatchesIt = 1
    ],
    /// Stop on MR0
    MR0S OFFSET(2) NUMBITS(1) [
        /// Disabled. Feature disabled.
        DisabledFeatureDisabled = 0,
        /// Match. TC and PC will be stopped and TCR[0] will be set to 0 if MR0 matches the
        MatchTCAndPCWillBeStoppedAndTCR0WillBeSetTo0IfMR0MatchesTheTC = 1
    ],
    /// Interrupt on MR1
    MR1I OFFSET(3) NUMBITS(1) [
        /// Disabled. Interrupt is disabled.
        DisabledInterruptIsDisabled = 0,
        /// Match. Interrupt is generated when MR1 matches the value in the TC.
        MatchInterruptIsGeneratedWhenMR1MatchesTheValueInTheTC = 1
    ],
    /// Reset on MR1
    MR1R OFFSET(4) NUMBITS(1) [
        /// Disabled. Feature disabled.
        DisabledFeatureDisabled = 0,
        /// Reset. TC will be reset if MR1 matches it.
        ResetTCWillBeResetIfMR1MatchesIt = 1
    ],
    /// Stop on MR1
    MR1S OFFSET(5) NUMBITS(1) [
        /// Disabled. Feature disabled.
        DisabledFeatureDisabled = 0,
        /// Stop. TC and PC will be stopped and TCR[0] will be set to 0 if MR1 matches the T
        StopTCAndPCWillBeStoppedAndTCR0WillBeSetTo0IfMR1MatchesTheTC = 1
    ],
    /// Interrupt on MR2
    MR2I OFFSET(6) NUMBITS(1) [
        /// Disabled. Interrupt is disabled
        DisabledInterruptIsDisabled = 0,
        /// Match. Interrupt is generated when MR2 matches the value in the TC.
        MatchInterruptIsGeneratedWhenMR2MatchesTheValueInTheTC = 1
    ],
    /// Reset on MR2
    MR2R OFFSET(7) NUMBITS(1) [
        /// Disabled. Feature disabled.
        DisabledFeatureDisabled = 0,
        /// Match. TC will be reset if MR2 matches it.
        MatchTCWillBeResetIfMR2MatchesIt = 1
    ],
    /// Stop on MR2.
    MR2S OFFSET(8) NUMBITS(1) [
        /// Disabled. Feature disabled.
        DisabledFeatureDisabled = 0,
        /// Stop. TC and PC will be stopped and TCR[0] will be set to 0 if MR2 matches the T
        StopTCAndPCWillBeStoppedAndTCR0WillBeSetTo0IfMR2MatchesTheTC = 1
    ],
    /// Interrupt on MR3
    MR3I OFFSET(9) NUMBITS(1) [
        /// Disabled. This interrupt is disabled.
        DisabledThisInterruptIsDisabled = 0,
        /// Interrupt. Interrupt is generated when MR3 matches the value in the TC.
        InterruptInterruptIsGeneratedWhenMR3MatchesTheValueInTheTC = 1
    ],
    /// Reset on MR3
    MR3R OFFSET(10) NUMBITS(1) [
        /// Disabled. Feature disabled.
        DisabledFeatureDisabled = 0,
        /// Match. TC will be reset if MR3 matches it.
        MatchTCWillBeResetIfMR3MatchesIt = 1
    ],
    /// Stop on MR3
    MR3S OFFSET(11) NUMBITS(1) [
        /// Disabled. Feature disabled.
        DisabledFeatureDisabled = 0,
        /// Stop. TC and PC will be stopped and TCR[0] will be set to 0 if MR3 matches the T
        StopTCAndPCWillBeStoppedAndTCR0WillBeSetTo0IfMR3MatchesTheTC = 1
    ]
],
CCR [
    /// Capture on CAPn.0 rising edge
    CAP0RE OFFSET(0) NUMBITS(1) [
        /// Disabled. This feature is disabled.
        DisabledThisFeatureIsDisabled = 0,
        /// Low to high. A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with t
        LowToHighASequenceOf0Then1OnCAPn0WillCauseCR0ToBeLoadedWithTheContentsOfTC = 1
    ],
    /// Capture on CAPn.0 falling edge
    CAP0FE OFFSET(1) NUMBITS(1) [
        /// Disabled. This feature is disabled.
        DisabledThisFeatureIsDisabled = 0,
        /// High to low. A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with t
        HighToLowASequenceOf1Then0OnCAPn0WillCauseCR0ToBeLoadedWithTheContentsOfTC = 1
    ],
    /// Interrupt on CAPn.0 event
    CAP0I OFFSET(2) NUMBITS(1) [
        /// Disabled. This feature is disabled.
        DisabledThisFeatureIsDisabled = 0,
        /// Load. A CR0 load due to a CAPn.0 event will generate an interrupt.
        LoadACR0LoadDueToACAPn0EventWillGenerateAnInterrupt = 1
    ],
    /// Capture on CAPn.1 rising edge
    CAP1RE OFFSET(3) NUMBITS(1) [
        /// Disabled. This feature is disabled.
        DisabledThisFeatureIsDisabled = 0,
        /// Low to high. A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with t
        LowToHighASequenceOf0Then1OnCAPn1WillCauseCR1ToBeLoadedWithTheContentsOfTC = 1
    ],
    /// Capture on CAPn.1 falling edge
    CAP1FE OFFSET(4) NUMBITS(1) [
        /// Disabled. This feature is disabled.
        DisabledThisFeatureIsDisabled = 0,
        /// High to low. A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with t
        HighToLowASequenceOf1Then0OnCAPn1WillCauseCR1ToBeLoadedWithTheContentsOfTC = 1
    ],
    /// Interrupt on CAPn.1 event
    CAP1I OFFSET(5) NUMBITS(1) [
        /// Disabled. This feature is disabled.
        DisabledThisFeatureIsDisabled = 0,
        /// Load. A CR1 load due to a CAPn.1 event will generate an interrupt.
        LoadACR1LoadDueToACAPn1EventWillGenerateAnInterrupt = 1
    ],
    /// Capture on CAPn.2 rising edge
    CAP2RE OFFSET(6) NUMBITS(1) [
        /// Disabled. This feature is disabled.
        DisabledThisFeatureIsDisabled = 0,
        /// Low to high. A sequence of 0 then 1 on CAPn.2 will cause CR2 to be loaded with t
        LowToHighASequenceOf0Then1OnCAPn2WillCauseCR2ToBeLoadedWithTheContentsOfTC = 1
    ],
    /// Capture on CAPn.2 falling edge:
    CAP2FE OFFSET(7) NUMBITS(1) [
        /// Disabled. This feature is disabled.
        DisabledThisFeatureIsDisabled = 0,
        /// High to low. A sequence of 1 then 0 on CAPn.2 will cause CR2 to be loaded with t
        HighToLowASequenceOf1Then0OnCAPn2WillCauseCR2ToBeLoadedWithTheContentsOfTC = 1
    ],
    /// Interrupt on CAPn.2 event
    CAP2I OFFSET(8) NUMBITS(1) [
        /// Disabled. This feature is disabled.
        DisabledThisFeatureIsDisabled = 0,
        /// Load. A CR2 load due to a CAPn.2 event will generate an interrupt.
        LoadACR2LoadDueToACAPn2EventWillGenerateAnInterrupt = 1
    ],
    /// Capture on CAPn.3 rising edge
    CAP3RE OFFSET(9) NUMBITS(1) [
        /// Disabled. This feature is disabled.
        DisabledThisFeatureIsDisabled = 0,
        /// Low to high. A sequence of 0 then 1 on CAPn.3 will cause CR3 to be loaded with t
        LowToHighASequenceOf0Then1OnCAPn3WillCauseCR3ToBeLoadedWithTheContentsOfTC = 1
    ],
    /// High to low. Capture on CAPn.3 falling edge
    CAP3FE OFFSET(10) NUMBITS(1) [
        /// Disabled. This feature is disabled.
        DisabledThisFeatureIsDisabled = 0,
        /// A sequence of 1 then 0 on CAPn.3 will cause CR3 to be loaded with the contents o
        ASequenceOf1Then0OnCAPn3WillCauseCR3ToBeLoadedWithTheContentsOfTC = 1
    ],
    /// Interrupt on CAPn.3 event:
    CAP3I OFFSET(11) NUMBITS(1) [
        /// Disabled. This feature is disabled.
        DisabledThisFeatureIsDisabled = 0,
        /// Load. A CR3 load due to a CAPn.3 event will generate an interrupt.
        LoadACR3LoadDueToACAPn3EventWillGenerateAnInterrupt = 1
    ]
],
EMR [
    /// External Match 0. When a match occurs between the TC and MR0, this bit can eithe
    EM0 OFFSET(0) NUMBITS(1) [],
    /// External Match 1. When a match occurs between the TC and MR1, this bit can eithe
    EM1 OFFSET(1) NUMBITS(1) [],
    /// External Match 2. When a match occurs between the TC and MR2, this bit can eithe
    EM2 OFFSET(2) NUMBITS(1) [],
    /// External Match 3. When a match occurs between the TC and MR3, this bit can eithe
    EM3 OFFSET(3) NUMBITS(1) [],
    /// External Match Control 0. Determines the functionality of External Match 0.
    EMC0 OFFSET(4) NUMBITS(2) [
        /// Do Nothing.
        DoNothing = 0,
        /// Clear. Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW
        ClearClearTheCorrespondingExternalMatchBitOutputTo0MATnMPinIsLOWIfPinnedOut = 1,
        /// Set. Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if
        SetSetTheCorrespondingExternalMatchBitOutputTo1MATnMPinIsHIGHIfPinnedOut = 2,
        /// Toggle. Toggle the corresponding External Match bit/output.
        ToggleToggleTheCorrespondingExternalMatchBitOutput = 3
    ],
    /// External Match Control 1. Determines the functionality of External Match 1.
    EMC1 OFFSET(6) NUMBITS(2) [
        /// Do Nothing.
        DoNothing = 0,
        /// Clear. Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW
        ClearClearTheCorrespondingExternalMatchBitOutputTo0MATnMPinIsLOWIfPinnedOut = 1,
        /// Set. Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if
        SetSetTheCorrespondingExternalMatchBitOutputTo1MATnMPinIsHIGHIfPinnedOut = 2,
        /// Toggle. Toggle the corresponding External Match bit/output.
        ToggleToggleTheCorrespondingExternalMatchBitOutput = 3
    ],
    /// External Match Control 2. Determines the functionality of External Match 2.
    EMC2 OFFSET(8) NUMBITS(2) [
        /// Do Nothing.
        DoNothing = 0,
        /// Clear. Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW
        ClearClearTheCorrespondingExternalMatchBitOutputTo0MATnMPinIsLOWIfPinnedOut = 1,
        /// Set. Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if
        SetSetTheCorrespondingExternalMatchBitOutputTo1MATnMPinIsHIGHIfPinnedOut = 2,
        /// Toggle. Toggle the corresponding External Match bit/output.
        ToggleToggleTheCorrespondingExternalMatchBitOutput = 3
    ],
    /// External Match Control 3. Determines the functionality of External Match 3.
    EMC3 OFFSET(10) NUMBITS(2) [
        /// Do Nothing.
        DoNothing = 0,
        /// Clear. Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW
        ClearClearTheCorrespondingExternalMatchBitOutputTo0MATnMPinIsLOWIfPinnedOut = 1,
        /// Set. Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if
        SetSetTheCorrespondingExternalMatchBitOutputTo1MATnMPinIsHIGHIfPinnedOut = 2,
        /// Toggle. Toggle the corresponding External Match bit/output.
        ToggleToggleTheCorrespondingExternalMatchBitOutput = 3
    ]
],
CTCR [
    /// Counter/Timer Mode This field selects which rising PCLK edges can increment Time
    CTMODE OFFSET(0) NUMBITS(2) [
        /// Timer Mode. Counts every rising PCLK edge
        TimerModeCountsEveryRisingPCLKEdge = 0,
        /// Counter Mode rising edge. TC is incremented on rising edges on the CAP input sel
        CounterModeRisingEdgeTCIsIncrementedOnRisingEdgesOnTheCAPInputSelectedByBits32 = 1,
        /// Counter Mode falling edge. TC is incremented on falling edges on the CAP input s
        COUNTER_MODE_FALLING = 2,
        /// Counter Mode edges. TC is incremented on both edges on the CAP input selected by
        CounterModeEdgesTCIsIncrementedOnBothEdgesOnTheCAPInputSelectedByBits32 = 3
    ],
    /// Count Input Select When bits 1:0 in this register are not 00, these bits select
    CINSEL OFFSET(2) NUMBITS(2) [
        /// CAP0. CAPn.0 for TIMERn
        CAP0CAPn0ForTIMERn = 0,
        /// CAP1. CAPn.1 for TIMERn
        CAP1CAPn1ForTIMERn = 1,
        /// CAP2. CAPn.2 for TIMERn
        CAP2CAPn2ForTIMERn = 2,
        /// CAP3. CAPn.3 for TIMERn
        CAP3CAPn3ForTIMERn = 3
    ]
]
];
const TIMER0_BASE: StaticRef<TimerRegisters> =
    unsafe { StaticRef::new(0x40084000 as *const TimerRegisters) };


const TIMER1_BASE: StaticRef<TimerRegisters> =
    unsafe { StaticRef::new(0x40085000 as *const TimerRegisters) };


const TIMER2_BASE: StaticRef<TimerRegisters> =
    unsafe { StaticRef::new(0x400C3000 as *const TimerRegisters) };


const TIMER3_BASE: StaticRef<TimerRegisters> =
    unsafe { StaticRef::new(0x400C4000 as *const TimerRegisters) };
