
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// Motor Control PWM (MOTOCONPWM)
#[repr(C)]
struct McpwmRegisters {
/// PWM Control read address
con: ReadOnly<u32, CON::Register>,
/// PWM Control set address
con_set: WriteOnly<u32, CON_SET::Register>,
/// PWM Control clear address
con_clr: WriteOnly<u32, CON_CLR::Register>,
/// Capture Control read address
capcon: ReadOnly<u32, CAPCON::Register>,
/// Capture Control set address
capcon_set: WriteOnly<u32, CAPCON_SET::Register>,
/// Event Control clear address
capcon_clr: WriteOnly<u32, CAPCON_CLR::Register>,
/// Timer Counter register
tc0: ReadWrite<u32>,
/// Timer Counter register
tc1: ReadWrite<u32>,
/// Timer Counter register
tc2: ReadWrite<u32>,
/// Limit register
lim0: ReadWrite<u32>,
/// Limit register
lim1: ReadWrite<u32>,
/// Limit register
lim2: ReadWrite<u32>,
/// Match register
mat0: ReadWrite<u32>,
/// Match register
mat1: ReadWrite<u32>,
/// Match register
mat2: ReadWrite<u32>,
/// Dead time register
dt: ReadWrite<u32, DT::Register>,
/// Communication Pattern register
ccp: ReadWrite<u32, CCP::Register>,
/// Capture register
cap0: ReadOnly<u32>,
/// Capture register
cap1: ReadOnly<u32>,
/// Capture register
cap2: ReadOnly<u32>,
/// Interrupt Enable read address
inten: ReadOnly<u32, INTEN::Register>,
/// Interrupt Enable set address
inten_set: WriteOnly<u32, INTEN_SET::Register>,
/// Interrupt Enable clear address
inten_clr: WriteOnly<u32, INTEN_CLR::Register>,
/// Count Control read address
cntcon: ReadOnly<u32, CNTCON::Register>,
/// Count Control set address
cntcon_set: WriteOnly<u32, CNTCON_SET::Register>,
/// Count Control clear address
cntcon_clr: WriteOnly<u32, CNTCON_CLR::Register>,
/// Interrupt flags read address
intf: ReadOnly<u32, INTF::Register>,
/// Interrupt flags set address
intf_set: WriteOnly<u32, INTF_SET::Register>,
/// Interrupt flags clear address
intf_clr: WriteOnly<u32, INTF_CLR::Register>,
/// Capture clear address
cap_clr: WriteOnly<u32, CAP_CLR::Register>,
}
register_bitfields![u32,
CON [
    /// Stops/starts timer channel 0.
    RUN0 OFFSET(0) NUMBITS(1) [
        /// Stop.
        Stop = 0,
        /// Run.
        Run = 1
    ],
    /// Edge/center aligned operation for channel 0.
    CENTER0 OFFSET(1) NUMBITS(1) [
        /// Edge-aligned.
        EdgeAligned = 0,
        /// Center-aligned.
        CenterAligned = 1
    ],
    /// Selects polarity of the MCOA0 and MCOB0 pins.
    POLA0 OFFSET(2) NUMBITS(1) [
        /// Passive state is LOW, active state is HIGH.
        PassiveStateIsLOWActiveStateIsHIGH = 0,
        /// Passive state is HIGH, active state is LOW.
        PassiveStateIsHIGHActiveStateIsLOW = 1
    ],
    /// Controls the dead-time feature for channel 0.
    DTE0 OFFSET(3) NUMBITS(1) [
        /// Dead-time disabled.
        DeadTimeDisabled = 0,
        /// Dead-time enabled.
        DeadTimeEnabled = 1
    ],
    /// Enable/disable updates of functional registers for channel 0 (see Section 24.8.2
    DISUP0 OFFSET(4) NUMBITS(1) [
        /// Functional registers are updated from the write registers at the end of each PWM
        FunctionalRegistersAreUpdatedFromTheWriteRegistersAtTheEndOfEachPWMCycle = 0,
        /// Functional registers remain the same as long as the timer is running.
        FunctionalRegistersRemainTheSameAsLongAsTheTimerIsRunning = 1
    ],
    /// Stops/starts timer channel 1.
    RUN1 OFFSET(8) NUMBITS(1) [
        /// Stop.
        Stop = 0,
        /// Run.
        Run = 1
    ],
    /// Edge/center aligned operation for channel 1.
    CENTER1 OFFSET(9) NUMBITS(1) [
        /// Edge-aligned.
        EdgeAligned = 0,
        /// Center-aligned.
        CenterAligned = 1
    ],
    /// Selects polarity of the MCOA1 and MCOB1 pins.
    POLA1 OFFSET(10) NUMBITS(1) [
        /// Passive state is LOW, active state is HIGH.
        PassiveStateIsLOWActiveStateIsHIGH = 0,
        /// Passive state is HIGH, active state is LOW.
        PassiveStateIsHIGHActiveStateIsLOW = 1
    ],
    /// Controls the dead-time feature for channel 1.
    DTE1 OFFSET(11) NUMBITS(1) [
        /// Dead-time disabled.
        DeadTimeDisabled = 0,
        /// Dead-time enabled.
        DeadTimeEnabled = 1
    ],
    /// Enable/disable updates of functional registers for channel 1 (see Section 24.8.2
    DISUP1 OFFSET(12) NUMBITS(1) [
        /// Functional registers are updated from the write registers at the end of each PWM
        FunctionalRegistersAreUpdatedFromTheWriteRegistersAtTheEndOfEachPWMCycle = 0,
        /// Functional registers remain the same as long as the timer is running.
        FunctionalRegistersRemainTheSameAsLongAsTheTimerIsRunning = 1
    ],
    /// Stops/starts timer channel 2.
    RUN2 OFFSET(16) NUMBITS(1) [
        /// Stop.
        Stop = 0,
        /// Run.
        Run = 1
    ],
    /// Edge/center aligned operation for channel 2.
    CENTER2 OFFSET(17) NUMBITS(1) [
        /// Edge-aligned.
        EdgeAligned = 0,
        /// Center-aligned.
        CenterAligned = 1
    ],
    /// Selects polarity of the MCOA2 and MCOB2 pins.
    POLA2 OFFSET(18) NUMBITS(1) [
        /// Passive state is LOW, active state is HIGH.
        PassiveStateIsLOWActiveStateIsHIGH = 0,
        /// Passive state is HIGH, active state is LOW.
        PassiveStateIsHIGHActiveStateIsLOW = 1
    ],
    /// Controls the dead-time feature for channel 1.
    DTE2 OFFSET(19) NUMBITS(1) [
        /// Dead-time disabled.
        DeadTimeDisabled = 0,
        /// Dead-time enabled.
        DeadTimeEnabled = 1
    ],
    /// Enable/disable updates of functional registers for channel 2 (see Section 24.8.2
    DISUP2 OFFSET(20) NUMBITS(1) [
        /// Functional registers are updated from the write registers at the end of each PWM
        FunctionalRegistersAreUpdatedFromTheWriteRegistersAtTheEndOfEachPWMCycle = 0,
        /// Functional registers remain the same as long as the timer is running.
        FunctionalRegistersRemainTheSameAsLongAsTheTimerIsRunning = 1
    ],
    /// Controls the polarity of the MCOB outputs for all 3 channels. This bit is typica
    INVBDC OFFSET(29) NUMBITS(1) [
        /// The MCOB outputs have opposite polarity from the MCOA outputs (aside from dead t
        TheMCOBOutputsHaveOppositePolarityFromTheMCOAOutputsAsideFromDeadTime = 0,
        /// The MCOB outputs have the same basic polarity as the MCOA outputs. (see Section
        TheMCOBOutputsHaveTheSameBasicPolarityAsTheMCOAOutputsSeeSection2486 = 1
    ],
    /// 3-phase AC mode select (see Section 24.8.7).
    ACMODE OFFSET(30) NUMBITS(1) [
        /// 3-phase AC-mode off: Each PWM channel uses its own timer-counter and period regi
        _3PhaseACModeOffEachPWMChannelUsesItsOwnTimerCounterAndPeriodRegister = 0,
        /// 3-phase AC-mode on: All PWM channels use the timer-counter and period register o
        _3PhaseACModeOnAllPWMChannelsUseTheTimerCounterAndPeriodRegisterOfChannel0 = 1
    ],
    /// 3-phase DC mode select (see Section 24.8.6).
    DCMODE OFFSET(31) NUMBITS(1) [
        /// 3-phase DC mode off: PWM channels are independent (unless bit ACMODE = 1)
        _3PhaseDCModeOffPWMChannelsAreIndependentUnlessBitACMODE1 = 0,
        /// 3-phase DC mode on: The internal MCOA0 output is routed through the CP register
        _3_PHASE_DC_MODE_ON_ = 1
    ]
],
CON_SET [
    /// Writing a one sets the corresponding bit in the CON register.
    RUN0_SET OFFSET(0) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CON register.
    CENTER0_SET OFFSET(1) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CON register.
    POLA0_SET OFFSET(2) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CON register.
    DTE0_SET OFFSET(3) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CON register.
    DISUP0_SET OFFSET(4) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CON register.
    RUN1_SET OFFSET(8) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CON register.
    CENTER1_SET OFFSET(9) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CON register.
    POLA1_SET OFFSET(10) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CON register.
    DTE1_SET OFFSET(11) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CON register.
    DISUP1_SET OFFSET(12) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CON register.
    RUN2_SET OFFSET(16) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CON register.
    CENTER2_SET OFFSET(17) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CON register.
    POLA2_SET OFFSET(18) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CON register.
    DTE2_SET OFFSET(19) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CON register.
    DISUP2_SET OFFSET(20) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CON register.
    INVBDC_SET OFFSET(29) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CON register.
    ACMODE_SET OFFSET(30) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CON register.
    DCMODE_SET OFFSET(31) NUMBITS(1) []
],
CON_CLR [
    /// Writing a one clears the corresponding bit in the CON register.
    RUN0_CLR OFFSET(0) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CON register.
    CENTER0_CLR OFFSET(1) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CON register.
    POLA0_CLR OFFSET(2) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CON register.
    DTE0_CLR OFFSET(3) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CON register.
    DISUP0_CLR OFFSET(4) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CON register.
    RUN1_CLR OFFSET(8) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CON register.
    CENTER1_CLR OFFSET(9) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CON register.
    POLA1_CLR OFFSET(10) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CON register.
    DTE1_CLR OFFSET(11) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CON register.
    DISUP1_CLR OFFSET(12) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CON register.
    RUN2_CLR OFFSET(16) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CON register.
    CENTER2_CLR OFFSET(17) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CON register.
    POLA2_CLR OFFSET(18) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CON register.
    DTE2_CLR OFFSET(19) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CON register.
    DISUP2_CLR OFFSET(20) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CON register.
    INVBDC_CLR OFFSET(29) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CON register.
    ACMOD_CLR OFFSET(30) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CON register.
    DCMODE_CLR OFFSET(31) NUMBITS(1) []
],
CAPCON [
    /// A 1 in this bit enables a channel 0 capture event on a rising edge on MCI0.
    CAP0MCI0_RE OFFSET(0) NUMBITS(1) [],
    /// A 1 in this bit enables a channel 0 capture event on a falling edge on MCI0.
    CAP0MCI0_FE OFFSET(1) NUMBITS(1) [],
    /// A 1 in this bit enables a channel 0 capture event on a rising edge on MCI1.
    CAP0MCI1_RE OFFSET(2) NUMBITS(1) [],
    /// A 1 in this bit enables a channel 0 capture event on a falling edge on MCI1.
    CAP0MCI1_FE OFFSET(3) NUMBITS(1) [],
    /// A 1 in this bit enables a channel 0 capture event on a rising edge on MCI2.
    CAP0MCI2_RE OFFSET(4) NUMBITS(1) [],
    /// A 1 in this bit enables a channel 0 capture event on a falling edge on MCI2.
    CAP0MCI2_FE OFFSET(5) NUMBITS(1) [],
    /// A 1 in this bit enables a channel 1 capture event on a rising edge on MCI0.
    CAP1MCI0_RE OFFSET(6) NUMBITS(1) [],
    /// A 1 in this bit enables a channel 1 capture event on a falling edge on MCI0.
    CAP1MCI0_FE OFFSET(7) NUMBITS(1) [],
    /// A 1 in this bit enables a channel 1 capture event on a rising edge on MCI1.
    CAP1MCI1_RE OFFSET(8) NUMBITS(1) [],
    /// A 1 in this bit enables a channel 1 capture event on a falling edge on MCI1.
    CAP1MCI1_FE OFFSET(9) NUMBITS(1) [],
    /// A 1 in this bit enables a channel 1 capture event on a rising edge on MCI2.
    CAP1MCI2_RE OFFSET(10) NUMBITS(1) [],
    /// A 1 in this bit enables a channel 1 capture event on a falling edge on MCI2.
    CAP1MCI2_FE OFFSET(11) NUMBITS(1) [],
    /// A 1 in this bit enables a channel 2 capture event on a rising edge on MCI0.
    CAP2MCI0_RE OFFSET(12) NUMBITS(1) [],
    /// A 1 in this bit enables a channel 2 capture event on a falling edge on MCI0.
    CAP2MCI0_FE OFFSET(13) NUMBITS(1) [],
    /// A 1 in this bit enables a channel 2 capture event on a rising edge on MCI1.
    CAP2MCI1_RE OFFSET(14) NUMBITS(1) [],
    /// A 1 in this bit enables a channel 2 capture event on a falling edge on MCI1.
    CAP2MCI1_FE OFFSET(15) NUMBITS(1) [],
    /// A 1 in this bit enables a channel 2 capture event on a rising edge on MCI2.
    CAP2MCI2_RE OFFSET(16) NUMBITS(1) [],
    /// A 1 in this bit enables a channel 2 capture event on a falling edge on MCI2.
    CAP2MCI2_FE OFFSET(17) NUMBITS(1) [],
    /// If this bit is 1, TC0 is reset by a channel 0 capture event.
    RT0 OFFSET(18) NUMBITS(1) [],
    /// If this bit is 1, TC1 is reset by a channel 1 capture event.
    RT1 OFFSET(19) NUMBITS(1) [],
    /// If this bit is 1, TC2 is reset by a channel 2 capture event.
    RT2 OFFSET(20) NUMBITS(1) []
],
CAPCON_SET [
    /// Writing a one sets the corresponding bits in the CAPCON register.
    CAP0MCI0_RE_SET OFFSET(0) NUMBITS(1) [],
    /// Writing a one sets the corresponding bits in the CAPCON register.
    CAP0MCI0_FE_SET OFFSET(1) NUMBITS(1) [],
    /// Writing a one sets the corresponding bits in the CAPCON register.
    CAP0MCI1_RE_SET OFFSET(2) NUMBITS(1) [],
    /// Writing a one sets the corresponding bits in the CAPCON register.
    CAP0MCI1_FE_SET OFFSET(3) NUMBITS(1) [],
    /// Writing a one sets the corresponding bits in the CAPCON register.
    CAP0MCI2_RE_SET OFFSET(4) NUMBITS(1) [],
    /// Writing a one sets the corresponding bits in the CAPCON register.
    CAP0MCI2_FE_SET OFFSET(5) NUMBITS(1) [],
    /// Writing a one sets the corresponding bits in the CAPCON register.
    CAP1MCI0_RE_SET OFFSET(6) NUMBITS(1) [],
    /// Writing a one sets the corresponding bits in the CAPCON register.
    CAP1MCI0_FE_SET OFFSET(7) NUMBITS(1) [],
    /// Writing a one sets the corresponding bits in the CAPCON register.
    CAP1MCI1_RE_SET OFFSET(8) NUMBITS(1) [],
    /// Writing a one sets the corresponding bits in the CAPCON register.
    CAP1MCI1_FE_SET OFFSET(9) NUMBITS(1) [],
    /// Writing a one sets the corresponding bits in the CAPCON register.
    CAP1MCI2_RE_SET OFFSET(10) NUMBITS(1) [],
    /// Writing a one sets the corresponding bits in the CAPCON register.
    CAP1MCI2_FE_SET OFFSET(11) NUMBITS(1) [],
    /// Writing a one sets the corresponding bits in the CAPCON register.
    CAP2MCI0_RE_SET OFFSET(12) NUMBITS(1) [],
    /// Writing a one sets the corresponding bits in the CAPCON register.
    CAP2MCI0_FE_SET OFFSET(13) NUMBITS(1) [],
    /// Writing a one sets the corresponding bits in the CAPCON register.
    CAP2MCI1_RE_SET OFFSET(14) NUMBITS(1) [],
    /// Writing a one sets the corresponding bits in the CAPCON register.
    CAP2MCI1_FE_SET OFFSET(15) NUMBITS(1) [],
    /// Writing a one sets the corresponding bits in the CAPCON register.
    CAP2MCI2_RE_SET OFFSET(16) NUMBITS(1) [],
    /// Writing a one sets the corresponding bits in the CAPCON register.
    CAP2MCI2_FE_SET OFFSET(17) NUMBITS(1) [],
    /// Writing a one sets the corresponding bits in the CAPCON register.
    RT0_SET OFFSET(18) NUMBITS(1) [],
    /// Writing a one sets the corresponding bits in the CAPCON register.
    RT1_SET OFFSET(19) NUMBITS(1) [],
    /// Writing a one sets the corresponding bits in the CAPCON register.
    RT2_SET OFFSET(20) NUMBITS(1) []
],
CAPCON_CLR [
    /// Writing a one clears the corresponding bits in the CAPCON register.
    CAP0MCI0_RE_CLR OFFSET(0) NUMBITS(1) [],
    /// Writing a one clears the corresponding bits in the CAPCON register.
    CAP0MCI0_FE_CLR OFFSET(1) NUMBITS(1) [],
    /// Writing a one clears the corresponding bits in the CAPCON register.
    CAP0MCI1_RE_CLR OFFSET(2) NUMBITS(1) [],
    /// Writing a one clears the corresponding bits in the CAPCON register.
    CAP0MCI1_FE_CLR OFFSET(3) NUMBITS(1) [],
    /// Writing a one clears the corresponding bits in the CAPCON register.
    CAP0MCI2_RE_CLR OFFSET(4) NUMBITS(1) [],
    /// Writing a one clears the corresponding bits in the CAPCON register.
    CAP0MCI2_FE_CLR OFFSET(5) NUMBITS(1) [],
    /// Writing a one clears the corresponding bits in the CAPCON register.
    CAP1MCI0_RE_CLR OFFSET(6) NUMBITS(1) [],
    /// Writing a one clears the corresponding bits in the CAPCON register.
    CAP1MCI0_FE_CLR OFFSET(7) NUMBITS(1) [],
    /// Writing a one clears the corresponding bits in the CAPCON register.
    CAP1MCI1_RE_CLR OFFSET(8) NUMBITS(1) [],
    /// Writing a one clears the corresponding bits in the CAPCON register.
    CAP1MCI1_FE_CLR OFFSET(9) NUMBITS(1) [],
    /// Writing a one clears the corresponding bits in the CAPCON register.
    CAP1MCI2_RE_CLR OFFSET(10) NUMBITS(1) [],
    /// Writing a one clears the corresponding bits in the CAPCON register.
    CAP1MCI2_FE_CLR OFFSET(11) NUMBITS(1) [],
    /// Writing a one clears the corresponding bits in the CAPCON register.
    CAP2MCI0_RE_CLR OFFSET(12) NUMBITS(1) [],
    /// Writing a one clears the corresponding bits in the CAPCON register.
    CAP2MCI0_FE_CLR OFFSET(13) NUMBITS(1) [],
    /// Writing a one clears the corresponding bits in the CAPCON register.
    CAP2MCI1_RE_CLR OFFSET(14) NUMBITS(1) [],
    /// Writing a one clears the corresponding bits in the CAPCON register.
    CAP2MCI1_FE_CLR OFFSET(15) NUMBITS(1) [],
    /// Writing a one clears the corresponding bits in the CAPCON register.
    CAP2MCI2_RE_CLR OFFSET(16) NUMBITS(1) [],
    /// Writing a one clears the corresponding bits in the CAPCON register.
    CAP2MCI2_FE_CLR OFFSET(17) NUMBITS(1) [],
    /// Writing a one clears the corresponding bits in the CAPCON register.
    RT0_CLR OFFSET(18) NUMBITS(1) [],
    /// Writing a one clears the corresponding bits in the CAPCON register.
    RT1_CLR OFFSET(19) NUMBITS(1) [],
    /// Writing a one clears the corresponding bits in the CAPCON register.
    RT2_CLR OFFSET(20) NUMBITS(1) []
],
DT [
    /// Dead time for channel 0.[1]
    DT0 OFFSET(0) NUMBITS(10) [],
    /// Dead time for channel 1.[2]
    DT1 OFFSET(10) NUMBITS(10) [],
    /// Dead time for channel 2.[2]
    DT2 OFFSET(20) NUMBITS(10) []
],
CCP [
    /// Communication pattern output A, channel 0.
    CCPA0 OFFSET(0) NUMBITS(1) [
        /// MCOA0 passive.
        MCOA0Passive = 0,
        /// internal MCOA0.
        InternalMCOA0 = 1
    ],
    /// Communication pattern output B, channel 0.
    CCPB0 OFFSET(1) NUMBITS(1) [
        /// MCOB0 passive.
        MCOB0Passive = 0,
        /// MCOB0 tracks internal MCOA0.
        MCOB0TracksInternalMCOA0 = 1
    ],
    /// Communication pattern output A, channel 1.
    CCPA1 OFFSET(2) NUMBITS(1) [
        /// MCOA1 passive.
        MCOA1Passive = 0,
        /// MCOA1 tracks internal MCOA0.
        MCOA1TracksInternalMCOA0 = 1
    ],
    /// Communication pattern output B, channel 1.
    CCPB1 OFFSET(3) NUMBITS(1) [
        /// MCOB1 passive.
        MCOB1Passive = 0,
        /// MCOB1 tracks internal MCOA0.
        MCOB1TracksInternalMCOA0 = 1
    ],
    /// Communication pattern output A, channel 2.
    CCPA2 OFFSET(4) NUMBITS(1) [
        /// MCOA2 passive.
        MCOA2Passive = 0,
        /// MCOA2 tracks internal MCOA0.
        MCOA2TracksInternalMCOA0 = 1
    ],
    /// Communication pattern output B, channel 2.
    CCPB2 OFFSET(5) NUMBITS(1) [
        /// MCOB2 passive.
        MCOB2Passive = 0,
        /// MCOB2 tracks internal MCOA0.
        MCOB2TracksInternalMCOA0 = 1
    ]
],
INTEN [
    /// Limit interrupt for channel 0.
    ILIM0 OFFSET(0) NUMBITS(1) [
        /// Interrupt disabled.
        InterruptDisabled = 0,
        /// Interrupt enabled.
        InterruptEnabled = 1
    ],
    /// Match interrupt for channel 0.
    IMAT0 OFFSET(1) NUMBITS(1) [
        /// Interrupt disabled.
        InterruptDisabled = 0,
        /// Interrupt enabled.
        InterruptEnabled = 1
    ],
    /// Capture interrupt for channel 0.
    ICAP0 OFFSET(2) NUMBITS(1) [
        /// Interrupt disabled.
        InterruptDisabled = 0,
        /// Interrupt enabled.
        InterruptEnabled = 1
    ],
    /// Limit interrupt for channel 1.
    ILIM1 OFFSET(4) NUMBITS(1) [
        /// Interrupt disabled.
        InterruptDisabled = 0,
        /// Interrupt enabled.
        InterruptEnabled = 1
    ],
    /// Match interrupt for channel 1.
    IMAT1 OFFSET(5) NUMBITS(1) [
        /// Interrupt disabled.
        InterruptDisabled = 0,
        /// Interrupt enabled.
        InterruptEnabled = 1
    ],
    /// Capture interrupt for channel 1.
    ICAP1 OFFSET(6) NUMBITS(1) [
        /// Interrupt disabled.
        InterruptDisabled = 0,
        /// Interrupt enabled.
        InterruptEnabled = 1
    ],
    /// Limit interrupt for channel 2.
    ILIM2 OFFSET(8) NUMBITS(1) [
        /// Interrupt disabled.
        InterruptDisabled = 0,
        /// Interrupt enabled.
        InterruptEnabled = 1
    ],
    /// Match interrupt for channel 2.
    IMAT2 OFFSET(9) NUMBITS(1) [
        /// Interrupt disabled.
        InterruptDisabled = 0,
        /// Interrupt enabled.
        InterruptEnabled = 1
    ],
    /// Capture interrupt for channel 2.
    ICAP2 OFFSET(10) NUMBITS(1) [
        /// Interrupt disabled.
        InterruptDisabled = 0,
        /// Interrupt enabled.
        InterruptEnabled = 1
    ],
    /// Fast abort interrupt.
    ABORT OFFSET(15) NUMBITS(1) [
        /// Interrupt disabled.
        InterruptDisabled = 0,
        /// Interrupt enabled.
        InterruptEnabled = 1
    ]
],
INTEN_SET [
    /// Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt.
    ILIM0_SET OFFSET(0) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt.
    IMAT0_SET OFFSET(1) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt.
    ICAP0_SET OFFSET(2) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt.
    ILIM1_SET OFFSET(4) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt.
    IMAT1_SET OFFSET(5) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt.
    ICAP1_SET OFFSET(6) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt.
    ILIM2_SET OFFSET(9) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt.
    IMAT2_SET OFFSET(10) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt.
    ICAP2_SET OFFSET(11) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt.
    ABORT_SET OFFSET(15) NUMBITS(1) []
],
INTEN_CLR [
    /// Writing a one clears the corresponding bit in INTEN, thus disabling the interrup
    ILIM0_CLR OFFSET(0) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in INTEN, thus disabling the interrup
    IMAT0_CLR OFFSET(1) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in INTEN, thus disabling the interrup
    ICAP0_CLR OFFSET(2) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in INTEN, thus disabling the interrup
    ILIM1_CLR OFFSET(4) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in INTEN, thus disabling the interrup
    IMAT1_CLR OFFSET(5) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in INTEN, thus disabling the interrup
    ICAP1_CLR OFFSET(6) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in INTEN, thus disabling the interrup
    ILIM2_CLR OFFSET(8) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in INTEN, thus disabling the interrup
    IMAT2_CLR OFFSET(9) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in INTEN, thus disabling the interrup
    ICAP2_CLR OFFSET(10) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in INTEN, thus disabling the interrup
    ABORT_CLR OFFSET(15) NUMBITS(1) []
],
INTF [
    /// Limit interrupt flag for channel 0.
    ILIM0_F OFFSET(0) NUMBITS(1) [
        /// This interrupt source is not contributing to the MCPWM interrupt request.
        ThisInterruptSourceIsNotContributingToTheMCPWMInterruptRequest = 0,
        /// If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interr
        IF_THE_CORRESPONDING = 1
    ],
    /// Match interrupt flag for channel 0.
    IMAT0_F OFFSET(1) NUMBITS(1) [
        /// This interrupt source is not contributing to the MCPWM interrupt request.
        ThisInterruptSourceIsNotContributingToTheMCPWMInterruptRequest = 0,
        /// If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interr
        IF_THE_CORRESPONDING = 1
    ],
    /// Capture interrupt flag for channel 0.
    ICAP0_F OFFSET(2) NUMBITS(1) [
        /// This interrupt source is not contributing to the MCPWM interrupt request.
        ThisInterruptSourceIsNotContributingToTheMCPWMInterruptRequest = 0,
        /// If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interr
        IF_THE_CORRESPONDING = 1
    ],
    /// Limit interrupt flag for channel 1.
    ILIM1_F OFFSET(4) NUMBITS(1) [
        /// This interrupt source is not contributing to the MCPWM interrupt request.
        ThisInterruptSourceIsNotContributingToTheMCPWMInterruptRequest = 0,
        /// If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interr
        IF_THE_CORRESPONDING = 1
    ],
    /// Match interrupt flag for channel 1.
    IMAT1_F OFFSET(5) NUMBITS(1) [
        /// This interrupt source is not contributing to the MCPWM interrupt request.
        ThisInterruptSourceIsNotContributingToTheMCPWMInterruptRequest = 0,
        /// If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interr
        IF_THE_CORRESPONDING = 1
    ],
    /// Capture interrupt flag for channel 1.
    ICAP1_F OFFSET(6) NUMBITS(1) [
        /// This interrupt source is not contributing to the MCPWM interrupt request.
        ThisInterruptSourceIsNotContributingToTheMCPWMInterruptRequest = 0,
        /// If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interr
        IF_THE_CORRESPONDING = 1
    ],
    /// Limit interrupt flag for channel 2.
    ILIM2_F OFFSET(8) NUMBITS(1) [
        /// This interrupt source is not contributing to the MCPWM interrupt request.
        ThisInterruptSourceIsNotContributingToTheMCPWMInterruptRequest = 0,
        /// If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interr
        IF_THE_CORRESPONDING = 1
    ],
    /// Match interrupt flag for channel 2.
    IMAT2_F OFFSET(9) NUMBITS(1) [
        /// This interrupt source is not contributing to the MCPWM interrupt request.
        ThisInterruptSourceIsNotContributingToTheMCPWMInterruptRequest = 0,
        /// If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interr
        IF_THE_CORRESPONDING = 1
    ],
    /// Capture interrupt flag for channel 2.
    ICAP2_F OFFSET(10) NUMBITS(1) [
        /// This interrupt source is not contributing to the MCPWM interrupt request.
        ThisInterruptSourceIsNotContributingToTheMCPWMInterruptRequest = 0,
        /// If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interr
        IF_THE_CORRESPONDING = 1
    ],
    /// Fast abort interrupt flag.
    ABORT_F OFFSET(15) NUMBITS(1) [
        /// This interrupt source is not contributing to the MCPWM interrupt request.
        ThisInterruptSourceIsNotContributingToTheMCPWMInterruptRequest = 0,
        /// If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interr
        IF_THE_CORRESPONDING = 1
    ]
],
INTF_SET [
    /// Writing a one sets the corresponding bit in the INTF register, thus possibly sim
    ILIM0_F_SET OFFSET(0) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the INTF register, thus possibly sim
    IMAT0_F_SET OFFSET(1) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the INTF register, thus possibly sim
    ICAP0_F_SET OFFSET(2) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the INTF register, thus possibly sim
    ILIM1_F_SET OFFSET(4) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the INTF register, thus possibly sim
    IMAT1_F_SET OFFSET(5) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the INTF register, thus possibly sim
    ICAP1_F_SET OFFSET(6) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the INTF register, thus possibly sim
    ILIM2_F_SET OFFSET(8) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the INTF register, thus possibly sim
    IMAT2_F_SET OFFSET(9) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the INTF register, thus possibly sim
    ICAP2_F_SET OFFSET(10) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the INTF register, thus possibly sim
    ABORT_F_SET OFFSET(15) NUMBITS(1) []
],
INTF_CLR [
    /// Writing a one clears the corresponding bit in the INTF register, thus clearing t
    ILIM0_F_CLR OFFSET(0) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in INTEN, thus disabling the interrup
    IMAT0_F_CLR OFFSET(1) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in INTEN, thus disabling the interrup
    ICAP0_F_CLR OFFSET(2) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in INTEN, thus disabling the interrup
    ILIM1_F_CLR OFFSET(4) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in INTEN, thus disabling the interrup
    IMAT1_F_CLR OFFSET(5) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in INTEN, thus disabling the interrup
    ICAP1_F_CLR OFFSET(6) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in INTEN, thus disabling the interrup
    ILIM2_F_CLR OFFSET(8) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in INTEN, thus disabling the interrup
    IMAT2_F_CLR OFFSET(9) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in INTEN, thus disabling the interrup
    ICAP2_F_CLR OFFSET(10) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in INTEN, thus disabling the interrup
    ABORT_F_CLR OFFSET(15) NUMBITS(1) []
],
CNTCON [
    /// Counter 0 rising edge mode, channel 0.
    TC0MCI0_RE OFFSET(0) NUMBITS(1) [
        /// A rising edge on MCI0 does not affect counter 0.
        ARisingEdgeOnMCI0DoesNotAffectCounter0 = 0,
        /// If MODE0 is 1, counter 0 advances on a rising edge on MCI0.
        IfMODE0Is1Counter0AdvancesOnARisingEdgeOnMCI0 = 1
    ],
    /// Counter 0 falling edge mode, channel 0.
    TC0MCI0_FE OFFSET(1) NUMBITS(1) [
        /// A falling edge on MCI0 does not affect counter 0.
        AFallingEdgeOnMCI0DoesNotAffectCounter0 = 0,
        /// If MODE0 is 1, counter 0 advances on a falling edge on MCI0.
        IfMODE0Is1Counter0AdvancesOnAFallingEdgeOnMCI0 = 1
    ],
    /// Counter 0 rising edge mode, channel 1.
    TC0MCI1_RE OFFSET(2) NUMBITS(1) [
        /// A rising edge on MCI1 does not affect counter 0.
        ARisingEdgeOnMCI1DoesNotAffectCounter0 = 0,
        /// If MODE0 is 1, counter 0 advances on a rising edge on MCI1.
        IfMODE0Is1Counter0AdvancesOnARisingEdgeOnMCI1 = 1
    ],
    /// Counter 0 falling edge mode, channel 1.
    TC0MCI1_FE OFFSET(3) NUMBITS(1) [
        /// A falling edge on MCI1 does not affect counter 0.
        AFallingEdgeOnMCI1DoesNotAffectCounter0 = 0,
        /// If MODE0 is 1, counter 0 advances on a falling edge on MCI1.
        IfMODE0Is1Counter0AdvancesOnAFallingEdgeOnMCI1 = 1
    ],
    /// Counter 0 rising edge mode, channel 2.
    TC0MCI2_RE OFFSET(4) NUMBITS(1) [
        /// A rising edge on MCI0 does not affect counter 0.
        ARisingEdgeOnMCI0DoesNotAffectCounter0 = 0,
        /// If MODE0 is 1, counter 0 advances on a rising edge on MCI2.
        IfMODE0Is1Counter0AdvancesOnARisingEdgeOnMCI2 = 1
    ],
    /// Counter 0 falling edge mode, channel 2.
    TC0MCI2_FE OFFSET(5) NUMBITS(1) [
        /// A falling edge on MCI0 does not affect counter 0.
        AFallingEdgeOnMCI0DoesNotAffectCounter0 = 0,
        /// If MODE0 is 1, counter 0 advances on a falling edge on MCI2.
        IfMODE0Is1Counter0AdvancesOnAFallingEdgeOnMCI2 = 1
    ],
    /// Counter 1 rising edge mode, channel 0.
    TC1MCI0_RE OFFSET(6) NUMBITS(1) [
        /// A rising edge on MCI0 does not affect counter 1.
        ARisingEdgeOnMCI0DoesNotAffectCounter1 = 0,
        /// If MODE1 is 1, counter 1 advances on a rising edge on MCI0.
        IfMODE1Is1Counter1AdvancesOnARisingEdgeOnMCI0 = 1
    ],
    /// Counter 1 falling edge mode, channel 0.
    TC1MCI0_FE OFFSET(7) NUMBITS(1) [
        /// A falling edge on MCI0 does not affect counter 1.
        AFallingEdgeOnMCI0DoesNotAffectCounter1 = 0,
        /// If MODE1 is 1, counter 1 advances on a falling edge on MCI0.
        IfMODE1Is1Counter1AdvancesOnAFallingEdgeOnMCI0 = 1
    ],
    /// Counter 1 rising edge mode, channel 1.
    TC1MCI1_RE OFFSET(8) NUMBITS(1) [
        /// A rising edge on MCI1 does not affect counter 1.
        ARisingEdgeOnMCI1DoesNotAffectCounter1 = 0,
        /// If MODE1 is 1, counter 1 advances on a rising edge on MCI1.
        IfMODE1Is1Counter1AdvancesOnARisingEdgeOnMCI1 = 1
    ],
    /// Counter 1 falling edge mode, channel 1.
    TC1MCI1_FE OFFSET(9) NUMBITS(1) [
        /// A falling edge on MCI0 does not affect counter 1.
        AFallingEdgeOnMCI0DoesNotAffectCounter1 = 0,
        /// If MODE1 is 1, counter 1 advances on a falling edge on MCI1.
        IfMODE1Is1Counter1AdvancesOnAFallingEdgeOnMCI1 = 1
    ],
    /// Counter 1 rising edge mode, channel 2.
    TC1MCI2_RE OFFSET(10) NUMBITS(1) [
        /// A rising edge on MCI2 does not affect counter 1.
        ARisingEdgeOnMCI2DoesNotAffectCounter1 = 0,
        /// If MODE1 is 1, counter 1 advances on a rising edge on MCI2.
        IfMODE1Is1Counter1AdvancesOnARisingEdgeOnMCI2 = 1
    ],
    /// Counter 1 falling edge mode, channel 2.
    TC1MCI2_FE OFFSET(11) NUMBITS(1) [
        /// A falling edge on MCI2 does not affect counter 1.
        AFallingEdgeOnMCI2DoesNotAffectCounter1 = 0,
        /// If MODE1 is 1, counter 1 advances on a falling edge on MCI2.
        IfMODE1Is1Counter1AdvancesOnAFallingEdgeOnMCI2 = 1
    ],
    /// Counter 2 rising edge mode, channel 0.
    TC2MCI0_RE OFFSET(12) NUMBITS(1) [
        /// A rising edge on MCI0 does not affect counter 2.
        ARisingEdgeOnMCI0DoesNotAffectCounter2 = 0,
        /// If MODE2 is 1, counter 2 advances on a rising edge on MCI0.
        IfMODE2Is1Counter2AdvancesOnARisingEdgeOnMCI0 = 1
    ],
    /// Counter 2 falling edge mode, channel 0.
    TC2MCI0_FE OFFSET(13) NUMBITS(1) [
        /// A falling edge on MCI0 does not affect counter 2.
        AFallingEdgeOnMCI0DoesNotAffectCounter2 = 0,
        /// If MODE2 is 1, counter 2 advances on a falling edge on MCI0.
        IfMODE2Is1Counter2AdvancesOnAFallingEdgeOnMCI0 = 1
    ],
    /// Counter 2 rising edge mode, channel 1.
    TC2MCI1_RE OFFSET(14) NUMBITS(1) [
        /// A rising edge on MCI1 does not affect counter 2.
        ARisingEdgeOnMCI1DoesNotAffectCounter2 = 0,
        /// If MODE2 is 1, counter 2 advances on a rising edge on MCI1.
        IfMODE2Is1Counter2AdvancesOnARisingEdgeOnMCI1 = 1
    ],
    /// Counter 2 falling edge mode, channel 1.
    TC2MCI1_FE OFFSET(15) NUMBITS(1) [
        /// A falling edge on MCI1 does not affect counter 2.
        AFallingEdgeOnMCI1DoesNotAffectCounter2 = 0,
        /// If MODE2 is 1, counter 2 advances on a falling edge on MCI1.
        IfMODE2Is1Counter2AdvancesOnAFallingEdgeOnMCI1 = 1
    ],
    /// Counter 2 rising edge mode, channel 2.
    TC2MCI2_RE OFFSET(16) NUMBITS(1) [
        /// A rising edge on MCI2 does not affect counter 2.
        ARisingEdgeOnMCI2DoesNotAffectCounter2 = 0,
        /// If MODE2 is 1, counter 2 advances on a rising edge on MCI2.
        IfMODE2Is1Counter2AdvancesOnARisingEdgeOnMCI2 = 1
    ],
    /// Counter 2 falling edge mode, channel 2.
    TC2MCI2_FE OFFSET(17) NUMBITS(1) [
        /// A falling edge on MCI2 does not affect counter 2.
        AFallingEdgeOnMCI2DoesNotAffectCounter2 = 0,
        /// If MODE2 is 1, counter 2 advances on a falling edge on MCI2.
        IfMODE2Is1Counter2AdvancesOnAFallingEdgeOnMCI2 = 1
    ],
    /// Channel 0 counter/timer mode.
    CNTR0 OFFSET(29) NUMBITS(1) [
        /// Channel 0 is in timer mode.
        Channel0IsInTimerMode = 0,
        /// Channel 0 is in counter mode.
        Channel0IsInCounterMode = 1
    ],
    /// Channel 1 counter/timer mode.
    CNTR1 OFFSET(30) NUMBITS(1) [
        /// Channel 1 is in timer mode.
        Channel1IsInTimerMode = 0,
        /// Channel 1 is in counter mode.
        Channel1IsInCounterMode = 1
    ],
    /// Channel 2 counter/timer mode.
    CNTR2 OFFSET(31) NUMBITS(1) [
        /// Channel 2 is in timer mode.
        Channel2IsInTimerMode = 0,
        /// Channel 2 is in counter mode.
        Channel2IsInCounterMode = 1
    ]
],
CNTCON_SET [
    /// Writing a one sets the corresponding bit in the CNTCON register.
    TC0MCI0_RE_SET OFFSET(0) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CNTCON register.
    TC0MCI0_FE_SET OFFSET(1) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CNTCON register.
    TC0MCI1_RE_SET OFFSET(2) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CNTCON register.
    TC0MCI1_FE_SET OFFSET(3) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CNTCON register.
    TC0MCI2_RE_SET OFFSET(4) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CNTCON register.
    TC0MCI2_FE_SET OFFSET(5) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CNTCON register.
    TC1MCI0_RE_SET OFFSET(6) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CNTCON register.
    TC1MCI0_FE_SET OFFSET(7) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CNTCON register.
    TC1MCI1_RE_SET OFFSET(8) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CNTCON register.
    TC1MCI1_FE_SET OFFSET(9) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CNTCON register.
    TC1MCI2_RE_SET OFFSET(10) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CNTCON register.
    TC1MCI2_FE_SET OFFSET(11) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CNTCON register.
    TC2MCI0_RE_SET OFFSET(12) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CNTCON register.
    TC2MCI0_FE_SET OFFSET(13) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CNTCON register.
    TC2MCI1_RE_SET OFFSET(14) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CNTCON register.
    TC2MCI1_FE_SET OFFSET(15) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CNTCON register.
    TC2MCI2_RE_SET OFFSET(16) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CNTCON register.
    TC2MCI2_FE_SET OFFSET(17) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CNTCON register.
    CNTR0_SET OFFSET(29) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CNTCON register.
    CNTR1_SET OFFSET(30) NUMBITS(1) [],
    /// Writing a one sets the corresponding bit in the CNTCON register.
    CNTR2_SET OFFSET(31) NUMBITS(1) []
],
CNTCON_CLR [
    /// Writing a one clears the corresponding bit in the CNTCON register.
    TC0MCI0_RE_CLR OFFSET(0) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CNTCON register.
    TC0MCI0_FE_CLR OFFSET(1) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CNTCON register.
    TC0MCI1_RE_CLR OFFSET(2) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CNTCON register.
    TC0MCI1_FE_CLR OFFSET(3) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CNTCON register.
    TC0MCI2_RE OFFSET(4) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CNTCON register.
    TC0MCI2_FE_CLR OFFSET(5) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CNTCON register.
    TC1MCI0_RE_CLR OFFSET(6) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CNTCON register.
    TC1MCI0_FE_CLR OFFSET(7) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CNTCON register.
    TC1MCI1_RE_CLR OFFSET(8) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CNTCON register.
    TC1MCI1_FE_CLR OFFSET(9) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CNTCON register.
    TC1MCI2_RE_CLR OFFSET(10) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CNTCON register.
    TC1MCI2_FE_CLR OFFSET(11) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CNTCON register.
    TC2MCI0_RE_CLR OFFSET(12) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CNTCON register.
    TC2MCI0_FE_CLR OFFSET(13) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CNTCON register.
    TC2MCI1_RE_CLR OFFSET(14) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CNTCON register.
    TC2MCI1_FE_CLR OFFSET(15) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CNTCON register.
    TC2MCI2_RE_CLR OFFSET(16) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CNTCON register.
    TC2MCI2_FE_CLR OFFSET(17) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CNTCON register.
    CNTR0_CLR OFFSET(29) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CNTCON register.
    CNTR1_CLR OFFSET(30) NUMBITS(1) [],
    /// Writing a one clears the corresponding bit in the CNTCON register.
    CNTR2_CLR OFFSET(31) NUMBITS(1) []
],
CAP_CLR [
    /// Writing a 1 to this bit clears the CAP0 register.
    CAP_CLR0 OFFSET(0) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the CAP1 register.
    CAP_CLR1 OFFSET(1) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the CAP2 register.
    CAP_CLR2 OFFSET(2) NUMBITS(1) []
]
];
const MCPWM_BASE: StaticRef<McpwmRegisters> =
    unsafe { StaticRef::new(0x400A0000 as *const McpwmRegisters) };
