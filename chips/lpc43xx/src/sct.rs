
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// State Configurable Timer (SCT) with dither engine
#[repr(C)]
struct SctRegisters {
/// SCT configuration register
config: ReadWrite<u32, CONFIG::Register>,
/// SCT control register
ctrl: ReadWrite<u32, CTRL::Register>,
/// SCT limit register
limit: ReadWrite<u32, LIMIT::Register>,
/// SCT halt condition register
halt: ReadWrite<u32, HALT::Register>,
/// SCT stop condition register
stop: ReadWrite<u32, STOP::Register>,
/// SCT start condition register
start: ReadWrite<u32, START::Register>,
/// SCT dither condition register
dither: ReadWrite<u32, DITHER::Register>,
_reserved0: [u8; 36],
/// SCT counter register
count: ReadWrite<u32, COUNT::Register>,
/// SCT state register
state: ReadWrite<u32, STATE::Register>,
/// SCT input register
input: ReadOnly<u32, INPUT::Register>,
/// SCT match/capture registers mode register
regmode: ReadWrite<u32, REGMODE::Register>,
/// SCT output register
output: ReadWrite<u32>,
/// SCT output counter direction control register
outputdirctrl: ReadWrite<u32, OUTPUTDIRCTRL::Register>,
/// SCT conflict resolution register
res: ReadWrite<u32, RES::Register>,
/// SCT DMA request 0 register
dmareq0: ReadWrite<u32, DMAREQ0::Register>,
/// SCT DMA request 1 register
dmareq1: ReadWrite<u32, DMAREQ1::Register>,
_reserved1: [u8; 140],
/// SCT event enable register
even: ReadWrite<u32, EVEN::Register>,
/// SCT event flag register
evflag: ReadWrite<u32, EVFLAG::Register>,
/// SCT conflict enable register
conen: ReadWrite<u32, CONEN::Register>,
/// SCT conflict flag register
conflag: ReadWrite<u32, CONFLAG::Register>,
/// SCT match value register of match channels 0 to 15; REGMOD0 to REGMODE15 = 0
match0: ReadWrite<u32, MATCH0::Register>,
/// SCT match value register of match channels 0 to 15; REGMOD0 to REGMODE15 = 0
match1: ReadWrite<u32, MATCH1::Register>,
/// SCT match value register of match channels 0 to 15; REGMOD0 to REGMODE15 = 0
match2: ReadWrite<u32, MATCH2::Register>,
/// SCT match value register of match channels 0 to 15; REGMOD0 to REGMODE15 = 0
match3: ReadWrite<u32, MATCH3::Register>,
/// SCT match value register of match channels 0 to 15; REGMOD0 to REGMODE15 = 0
match4: ReadWrite<u32, MATCH4::Register>,
/// SCT match value register of match channels 0 to 15; REGMOD0 to REGMODE15 = 0
match5: ReadWrite<u32, MATCH5::Register>,
/// SCT match value register of match channels 0 to 15; REGMOD0 to REGMODE15 = 0
match6: ReadWrite<u32, MATCH6::Register>,
/// SCT match value register of match channels 0 to 15; REGMOD0 to REGMODE15 = 0
match7: ReadWrite<u32, MATCH7::Register>,
/// SCT match value register of match channels 0 to 15; REGMOD0 to REGMODE15 = 0
match8: ReadWrite<u32, MATCH8::Register>,
/// SCT match value register of match channels 0 to 15; REGMOD0 to REGMODE15 = 0
match9: ReadWrite<u32, MATCH9::Register>,
/// SCT match value register of match channels 0 to 15; REGMOD0 to REGMODE15 = 0
match10: ReadWrite<u32, MATCH10::Register>,
/// SCT match value register of match channels 0 to 15; REGMOD0 to REGMODE15 = 0
match11: ReadWrite<u32, MATCH11::Register>,
/// SCT match value register of match channels 0 to 15; REGMOD0 to REGMODE15 = 0
match12: ReadWrite<u32, MATCH12::Register>,
/// SCT match value register of match channels 0 to 15; REGMOD0 to REGMODE15 = 0
match13: ReadWrite<u32, MATCH13::Register>,
/// SCT match value register of match channels 0 to 15; REGMOD0 to REGMODE15 = 0
match14: ReadWrite<u32, MATCH14::Register>,
/// SCT match value register of match channels 0 to 15; REGMOD0 to REGMODE15 = 0
match15: ReadWrite<u32, MATCH15::Register>,
/// Fractional match registers 0 to 5 for SCT match value registers 0 to 5.
fracmat0: ReadWrite<u32, FRACMAT0::Register>,
/// Fractional match registers 0 to 5 for SCT match value registers 0 to 5.
fracmat1: ReadWrite<u32, FRACMAT1::Register>,
/// Fractional match registers 0 to 5 for SCT match value registers 0 to 5.
fracmat2: ReadWrite<u32, FRACMAT2::Register>,
/// Fractional match registers 0 to 5 for SCT match value registers 0 to 5.
fracmat3: ReadWrite<u32, FRACMAT3::Register>,
/// Fractional match registers 0 to 5 for SCT match value registers 0 to 5.
fracmat4: ReadWrite<u32, FRACMAT4::Register>,
/// Fractional match registers 0 to 5 for SCT match value registers 0 to 5.
fracmat5: ReadWrite<u32, FRACMAT5::Register>,
_reserved2: [u8; 168],
/// SCT match reload value register 0 to 15; REGMOD0 = 0 to REGMODE15 = 0
matchrel0: ReadWrite<u32, MATCHREL0::Register>,
/// SCT match reload value register 0 to 15; REGMOD0 = 0 to REGMODE15 = 0
matchrel1: ReadWrite<u32, MATCHREL1::Register>,
/// SCT match reload value register 0 to 15; REGMOD0 = 0 to REGMODE15 = 0
matchrel2: ReadWrite<u32, MATCHREL2::Register>,
/// SCT match reload value register 0 to 15; REGMOD0 = 0 to REGMODE15 = 0
matchrel3: ReadWrite<u32, MATCHREL3::Register>,
/// SCT match reload value register 0 to 15; REGMOD0 = 0 to REGMODE15 = 0
matchrel4: ReadWrite<u32, MATCHREL4::Register>,
/// SCT match reload value register 0 to 15; REGMOD0 = 0 to REGMODE15 = 0
matchrel5: ReadWrite<u32, MATCHREL5::Register>,
/// SCT match reload value register 0 to 15; REGMOD0 = 0 to REGMODE15 = 0
matchrel6: ReadWrite<u32, MATCHREL6::Register>,
/// SCT match reload value register 0 to 15; REGMOD0 = 0 to REGMODE15 = 0
matchrel7: ReadWrite<u32, MATCHREL7::Register>,
/// SCT match reload value register 0 to 15; REGMOD0 = 0 to REGMODE15 = 0
matchrel8: ReadWrite<u32, MATCHREL8::Register>,
/// SCT match reload value register 0 to 15; REGMOD0 = 0 to REGMODE15 = 0
matchrel9: ReadWrite<u32, MATCHREL9::Register>,
/// SCT match reload value register 0 to 15; REGMOD0 = 0 to REGMODE15 = 0
matchrel10: ReadWrite<u32, MATCHREL10::Register>,
/// SCT match reload value register 0 to 15; REGMOD0 = 0 to REGMODE15 = 0
matchrel11: ReadWrite<u32, MATCHREL11::Register>,
/// SCT match reload value register 0 to 15; REGMOD0 = 0 to REGMODE15 = 0
matchrel12: ReadWrite<u32, MATCHREL12::Register>,
/// SCT match reload value register 0 to 15; REGMOD0 = 0 to REGMODE15 = 0
matchrel13: ReadWrite<u32, MATCHREL13::Register>,
/// SCT match reload value register 0 to 15; REGMOD0 = 0 to REGMODE15 = 0
matchrel14: ReadWrite<u32, MATCHREL14::Register>,
/// SCT match reload value register 0 to 15; REGMOD0 = 0 to REGMODE15 = 0
matchrel15: ReadWrite<u32, MATCHREL15::Register>,
/// Fractional match reload registers 0 to 5 for SCT match value registers 0 to 5.
fracmatrel0: ReadWrite<u32, FRACMATREL0::Register>,
/// Fractional match reload registers 0 to 5 for SCT match value registers 0 to 5.
fracmatrel1: ReadWrite<u32, FRACMATREL1::Register>,
/// Fractional match reload registers 0 to 5 for SCT match value registers 0 to 5.
fracmatrel2: ReadWrite<u32, FRACMATREL2::Register>,
/// Fractional match reload registers 0 to 5 for SCT match value registers 0 to 5.
fracmatrel3: ReadWrite<u32, FRACMATREL3::Register>,
/// Fractional match reload registers 0 to 5 for SCT match value registers 0 to 5.
fracmatrel4: ReadWrite<u32, FRACMATREL4::Register>,
/// Fractional match reload registers 0 to 5 for SCT match value registers 0 to 5.
fracmatrel5: ReadWrite<u32, FRACMATREL5::Register>,
_reserved3: [u8; 168],
/// SCT event state register 0
ev0_state: ReadWrite<u32, EV0_STATE::Register>,
/// SCT event control register 0
ev0_ctrl: ReadWrite<u32, EV0_CTRL::Register>,
/// SCT event state register 0
ev1_state: ReadWrite<u32, EV1_STATE::Register>,
/// SCT event control register 0
ev1_ctrl: ReadWrite<u32, EV1_CTRL::Register>,
/// SCT event state register 0
ev2_state: ReadWrite<u32, EV2_STATE::Register>,
/// SCT event control register 0
ev2_ctrl: ReadWrite<u32, EV2_CTRL::Register>,
/// SCT event state register 0
ev3_state: ReadWrite<u32, EV3_STATE::Register>,
/// SCT event control register 0
ev3_ctrl: ReadWrite<u32, EV3_CTRL::Register>,
/// SCT event state register 0
ev4_state: ReadWrite<u32, EV4_STATE::Register>,
/// SCT event control register 0
ev4_ctrl: ReadWrite<u32, EV4_CTRL::Register>,
/// SCT event state register 0
ev5_state: ReadWrite<u32, EV5_STATE::Register>,
/// SCT event control register 0
ev5_ctrl: ReadWrite<u32, EV5_CTRL::Register>,
/// SCT event state register 0
ev6_state: ReadWrite<u32, EV6_STATE::Register>,
/// SCT event control register 0
ev6_ctrl: ReadWrite<u32, EV6_CTRL::Register>,
/// SCT event state register 0
ev7_state: ReadWrite<u32, EV7_STATE::Register>,
/// SCT event control register 0
ev7_ctrl: ReadWrite<u32, EV7_CTRL::Register>,
/// SCT event state register 0
ev8_state: ReadWrite<u32, EV8_STATE::Register>,
/// SCT event control register 0
ev8_ctrl: ReadWrite<u32, EV8_CTRL::Register>,
/// SCT event state register 0
ev9_state: ReadWrite<u32, EV9_STATE::Register>,
/// SCT event control register 0
ev9_ctrl: ReadWrite<u32, EV9_CTRL::Register>,
/// SCT event state register 0
ev10_state: ReadWrite<u32, EV10_STATE::Register>,
/// SCT event control register 0
ev10_ctrl: ReadWrite<u32, EV10_CTRL::Register>,
/// SCT event state register 0
ev11_state: ReadWrite<u32, EV11_STATE::Register>,
/// SCT event control register 0
ev11_ctrl: ReadWrite<u32, EV11_CTRL::Register>,
/// SCT event state register 0
ev12_state: ReadWrite<u32, EV12_STATE::Register>,
/// SCT event control register 0
ev12_ctrl: ReadWrite<u32, EV12_CTRL::Register>,
/// SCT event state register 0
ev13_state: ReadWrite<u32, EV13_STATE::Register>,
/// SCT event control register 0
ev13_ctrl: ReadWrite<u32, EV13_CTRL::Register>,
/// SCT event state register 0
ev14_state: ReadWrite<u32, EV14_STATE::Register>,
/// SCT event control register 0
ev14_ctrl: ReadWrite<u32, EV14_CTRL::Register>,
/// SCT event state register 0
ev15_state: ReadWrite<u32, EV15_STATE::Register>,
/// SCT event control register 0
ev15_ctrl: ReadWrite<u32, EV15_CTRL::Register>,
_reserved4: [u8; 384],
/// SCT output 0 set register
out0_set: ReadWrite<u32, OUT0_SET::Register>,
/// SCT output 0 clear register
out0_clr: ReadWrite<u32, OUT0_CLR::Register>,
/// SCT output 0 set register
out1_set: ReadWrite<u32, OUT1_SET::Register>,
/// SCT output 0 clear register
out1_clr: ReadWrite<u32, OUT1_CLR::Register>,
/// SCT output 0 set register
out2_set: ReadWrite<u32, OUT2_SET::Register>,
/// SCT output 0 clear register
out2_clr: ReadWrite<u32, OUT2_CLR::Register>,
/// SCT output 0 set register
out3_set: ReadWrite<u32, OUT3_SET::Register>,
/// SCT output 0 clear register
out3_clr: ReadWrite<u32, OUT3_CLR::Register>,
/// SCT output 0 set register
out4_set: ReadWrite<u32, OUT4_SET::Register>,
/// SCT output 0 clear register
out4_clr: ReadWrite<u32, OUT4_CLR::Register>,
/// SCT output 0 set register
out5_set: ReadWrite<u32, OUT5_SET::Register>,
/// SCT output 0 clear register
out5_clr: ReadWrite<u32, OUT5_CLR::Register>,
/// SCT output 0 set register
out6_set: ReadWrite<u32, OUT6_SET::Register>,
/// SCT output 0 clear register
out6_clr: ReadWrite<u32, OUT6_CLR::Register>,
/// SCT output 0 set register
out7_set: ReadWrite<u32, OUT7_SET::Register>,
/// SCT output 0 clear register
out7_clr: ReadWrite<u32, OUT7_CLR::Register>,
/// SCT output 0 set register
out8_set: ReadWrite<u32, OUT8_SET::Register>,
/// SCT output 0 clear register
out8_clr: ReadWrite<u32, OUT8_CLR::Register>,
/// SCT output 0 set register
out9_set: ReadWrite<u32, OUT9_SET::Register>,
/// SCT output 0 clear register
out9_clr: ReadWrite<u32, OUT9_CLR::Register>,
/// SCT output 0 set register
out10_set: ReadWrite<u32, OUT10_SET::Register>,
/// SCT output 0 clear register
out10_clr: ReadWrite<u32, OUT10_CLR::Register>,
/// SCT output 0 set register
out11_set: ReadWrite<u32, OUT11_SET::Register>,
/// SCT output 0 clear register
out11_clr: ReadWrite<u32, OUT11_CLR::Register>,
/// SCT output 0 set register
out12_set: ReadWrite<u32, OUT12_SET::Register>,
/// SCT output 0 clear register
out12_clr: ReadWrite<u32, OUT12_CLR::Register>,
/// SCT output 0 set register
out13_set: ReadWrite<u32, OUT13_SET::Register>,
/// SCT output 0 clear register
out13_clr: ReadWrite<u32, OUT13_CLR::Register>,
/// SCT output 0 set register
out14_set: ReadWrite<u32, OUT14_SET::Register>,
/// SCT output 0 clear register
out14_clr: ReadWrite<u32, OUT14_CLR::Register>,
/// SCT output 0 set register
out15_set: ReadWrite<u32, OUT15_SET::Register>,
/// SCT output 0 clear register
out15_clr: ReadWrite<u32, OUT15_CLR::Register>,
}
register_bitfields![u32,
CONFIG [
    /// SCT operation
    UNIFY OFFSET(0) NUMBITS(1) [
        /// 16-bit.The SCT operates as two 16-bit counters named L and H.
        _16BitTheSCTOperatesAsTwo16BitCountersNamedLAndH = 0,
        /// 32-bit. The SCT operates as a unified 32-bit counter.
        _32BitTheSCTOperatesAsAUnified32BitCounter = 1
    ],
    /// SCT clock mode
    CLKMODE OFFSET(1) NUMBITS(2) [
        /// Bus clock. The bus clock clocks the SCT and prescalers.
        BusClockTheBusClockClocksTheSCTAndPrescalers = 0,
        /// Prescaled bus clock. The SCT clock is the bus clock, but the prescalers are  ena
        PRESCALED_BUS_CLOCK = 1,
        /// SCT Input. The input selected by  CKSEL clocks the SCT and prescalers. The input
        SCT_INPUT = 2,
        /// Reserved.
        Reserved = 3
    ],
    /// SCT clock select
    CKSEL OFFSET(3) NUMBITS(4) [
        /// Rising edges on input 0.
        RisingEdgesOnInput0 = 0,
        /// Falling edges on input 0.
        FallingEdgesOnInput0 = 1,
        /// Rising edges on input 1.
        RisingEdgesOnInput1 = 2,
        /// Falling edges on input 1.
        FallingEdgesOnInput1 = 3,
        /// Rising edges on input 2.
        RisingEdgesOnInput2 = 4,
        /// Falling edges on input 2.
        FallingEdgesOnInput2 = 5,
        /// Rising edges on input 3.
        RisingEdgesOnInput3 = 6,
        /// Falling edges on input 3.
        FallingEdgesOnInput3 = 7,
        /// Rising edges on input 4.
        RisingEdgesOnInput4 = 8,
        /// Falling edges on input 4.
        FallingEdgesOnInput4 = 9,
        /// Rising edges on input 5.
        RisingEdgesOnInput5 = 10,
        /// Falling edges on input 5.
        FallingEdgesOnInput5 = 11,
        /// Rising edges on input 6.
        RisingEdgesOnInput6 = 12,
        /// Falling edges on input 6.
        FallingEdgesOnInput6 = 13,
        /// Rising edges on input 7.
        RisingEdgesOnInput7 = 14,
        /// Falling edges on input 7.
        FallingEdgesOnInput7 = 15
    ],
    /// A 1 in this bit prevents the lower match and fractional match registers from bei
    NORELAOD_L OFFSET(7) NUMBITS(1) [],
    /// A 1 in this bit prevents the higher match and fractional match registers from be
    NORELOAD_H OFFSET(8) NUMBITS(1) [],
    /// Synchronization for input n (bit 9 = input 0, bit 10 = input 1,..., bit 16 = inp
    INSYNC OFFSET(9) NUMBITS(8) [],
    /// A one in this bit causes a match on match register 0 to be treated  as a de-fact
    AUTOLIMIT_L OFFSET(17) NUMBITS(1) [],
    /// A one in this bit will cause a match on match register 0 to be treated  as a de-
    AUTOLIMIT_H OFFSET(18) NUMBITS(1) []
],
CTRL [
    /// This bit is 1 when the L or unified counter is counting down. Hardware sets this
    DOWN_L OFFSET(0) NUMBITS(1) [],
    /// When this bit is 1 and HALT is 0, the L or unified counter does not run but I/O
    STOP_L OFFSET(1) NUMBITS(1) [],
    /// When this bit is 1, the L or unified counter does not run and no events can occu
    HALT_L OFFSET(2) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the L or unified counter. This bit always reads a
    CLRCTR_L OFFSET(3) NUMBITS(1) [],
    /// L or unified counter direction select
    BIDIR_L OFFSET(4) NUMBITS(1) [
        /// Up. The counter counts up to its limit condition, then is cleared to zero.
        UpTheCounterCountsUpToItsLimitConditionThenIsClearedToZero = 0,
        /// Up-down. The counter counts up to its limit, then counts down to a limit conditi
        UpDownTheCounterCountsUpToItsLimitThenCountsDownToALimitConditionOrTo0 = 1
    ],
    /// Specifies the factor by which the SCT clock is prescaled to produce the  L or un
    PRE_L OFFSET(5) NUMBITS(8) [],
    /// This bit is 1 when the H counter is counting down. Hardware sets this bit   when
    DOWN_H OFFSET(16) NUMBITS(1) [],
    /// When this bit is 1 and HALT is 0, the H counter does not run but I/O  events rel
    STOP_H OFFSET(17) NUMBITS(1) [],
    /// When this bit is 1, the H counter does not run and no events can occur.  A reset
    HALT_H OFFSET(18) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the H counter. This bit always reads as 0.
    CLRCTR_H OFFSET(19) NUMBITS(1) [],
    /// Direction select
    BIDIR_H OFFSET(20) NUMBITS(1) [
        /// Up. The H counter counts up to its limit condition, then is cleared to zero.
        UpTheHCounterCountsUpToItsLimitConditionThenIsClearedToZero = 0,
        /// Up-down. The H counter counts up to its limit, then counts down to a limit condi
        UpDownTheHCounterCountsUpToItsLimitThenCountsDownToALimitConditionOrTo0 = 1
    ],
    /// Specifies the factor by which the SCT clock is prescaled to produce the  H count
    PRE_H OFFSET(21) NUMBITS(8) []
],
LIMIT [
    /// If bit n is one, event n is used as a counter limit event for the L or unified c
    LIMMSK_L OFFSET(0) NUMBITS(16) [],
    /// If bit n is one, event n is used as a counter limit event for the H counter (eve
    LIMMSK_H OFFSET(16) NUMBITS(16) []
],
HALT [
    /// If bit n is one, event n sets the HALT_L bit in the CTRL register (event 0 = bit
    HALTMSK_L OFFSET(0) NUMBITS(16) [],
    /// If bit n is one, event n sets the HALT_H bit in the CTRL register (event 0 = bit
    HALTMSK_H OFFSET(16) NUMBITS(16) []
],
STOP [
    /// If bit n is one, event n sets the STOP_L bit in the CTRL register (event 0 = bit
    STOPMSK_L OFFSET(0) NUMBITS(16) [],
    /// If bit n is one, event n sets the STOP_H bit in the CTRL register (event 0 = bit
    STOPMSK_H OFFSET(16) NUMBITS(16) []
],
START [
    /// If bit n is one, event n clears the STOP_L bit in the CTRL register (event 0 = b
    STARTMSK_L OFFSET(0) NUMBITS(16) [],
    /// If bit n is one, event n clears the STOP_H bit in the CTRL register (event 0 = b
    STARTMSK_H OFFSET(16) NUMBITS(16) []
],
DITHER [
    /// If bit n is one, the event n causes the dither engine to advance to the next ele
    DITHMSK_L OFFSET(0) NUMBITS(16) [],
    /// If bit n is one, the event n causes the dither engine to advance to the next ele
    DITHMSK_H OFFSET(16) NUMBITS(16) []
],
COUNT [
    /// When UNIFY = 0, read or write the 16-bit L counter value. When UNIFY = 1, read o
    CTR_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit H counter value. When UNIFY = 1, read o
    CTR_H OFFSET(16) NUMBITS(16) []
],
STATE [
    /// State variable.
    STATE_L OFFSET(0) NUMBITS(5) [],
    /// State variable.
    STATE_H OFFSET(16) NUMBITS(5) []
],
INPUT [
    /// Real-time status of input 0.
    AIN0 OFFSET(0) NUMBITS(1) [],
    /// Real-time status of input 1.
    AIN1 OFFSET(1) NUMBITS(1) [],
    /// Real-time status of input 2.
    AIN2 OFFSET(2) NUMBITS(1) [],
    /// Real-time status of input 3.
    AIN3 OFFSET(3) NUMBITS(1) [],
    /// Real-time status of input 4.
    AIN4 OFFSET(4) NUMBITS(1) [],
    /// Real-time status of input 5.
    AIN5 OFFSET(5) NUMBITS(1) [],
    /// Real-time status of input 6.
    AIN6 OFFSET(6) NUMBITS(1) [],
    /// Real-time status of input 7.
    AIN7 OFFSET(7) NUMBITS(1) [],
    /// Input 0 state synchronized to the SCT clock.
    SIN0 OFFSET(16) NUMBITS(1) [],
    /// Input 1 state synchronized to the SCT clock.
    SIN1 OFFSET(17) NUMBITS(1) [],
    /// Input 2 state synchronized to the SCT clock.
    SIN2 OFFSET(18) NUMBITS(1) [],
    /// Input 3 state synchronized to the SCT clock.
    SIN3 OFFSET(19) NUMBITS(1) [],
    /// Input 4 state synchronized to the SCT clock.
    SIN4 OFFSET(20) NUMBITS(1) [],
    /// Input 5 state synchronized to the SCT clock.
    SIN5 OFFSET(21) NUMBITS(1) [],
    /// Input 6 state synchronized to the SCT clock.
    SIN6 OFFSET(22) NUMBITS(1) [],
    /// Input 7 state synchronized to the SCT clock.
    SIN7 OFFSET(23) NUMBITS(1) []
],
REGMODE [
    /// Each bit controls one pair of match/capture registers (register 0 = bit 0, regis
    REGMOD_L OFFSET(0) NUMBITS(16) [],
    /// Each bit controls one pair of match/capture registers (register 0 = bit 16, regi
    REGMOD_H OFFSET(16) NUMBITS(16) []
],
OUTPUTDIRCTRL [
    /// Set/clear operation on output 0. Value 0x3 is reserved. Do not program this valu
    SETCLR0 OFFSET(0) NUMBITS(2) [
        /// Independent. Set and clear do not depend on any counter.
        IndependentSetAndClearDoNotDependOnAnyCounter = 0,
        /// L counter. Set and clear are reversed when counter L or the unified counter is c
        LCounterSetAndClearAreReversedWhenCounterLOrTheUnifiedCounterIsCountingDown = 1,
        /// H counter. Set and clear are reversed when counter H is counting down. Do not us
        HCounterSetAndClearAreReversedWhenCounterHIsCountingDownDoNotUseIfUNIFY1 = 2
    ],
    /// Set/clear operation on output 1. Value 0x3 is reserved. Do not program this valu
    SETCLR1 OFFSET(2) NUMBITS(2) [
        /// Independent. Set and clear do not depend on any counter.
        IndependentSetAndClearDoNotDependOnAnyCounter = 0,
        /// L counter. Set and clear are reversed when counter L or the unified counter is c
        LCounterSetAndClearAreReversedWhenCounterLOrTheUnifiedCounterIsCountingDown = 1,
        /// H counter. Set and clear are reversed when counter H is counting down. Do not us
        HCounterSetAndClearAreReversedWhenCounterHIsCountingDownDoNotUseIfUNIFY1 = 2
    ],
    /// Set/clear operation on output 2. Value 0x3 is reserved. Do not program this valu
    SETCLR2 OFFSET(4) NUMBITS(2) [
        /// Independent. Set and clear do not depend on any counter.
        IndependentSetAndClearDoNotDependOnAnyCounter = 0,
        /// L counter. Set and clear are reversed when counter L or the unified counter is c
        LCounterSetAndClearAreReversedWhenCounterLOrTheUnifiedCounterIsCountingDown = 1,
        /// H counter. Set and clear are reversed when counter H is counting down. Do not us
        HCounterSetAndClearAreReversedWhenCounterHIsCountingDownDoNotUseIfUNIFY1 = 2
    ],
    /// Set/clear operation on output 3. Value 0x3 is reserved. Do not program this valu
    SETCLR3 OFFSET(6) NUMBITS(2) [
        /// Independent. Set and clear do not depend on any counter.
        IndependentSetAndClearDoNotDependOnAnyCounter = 0,
        /// L counter. Set and clear are reversed when counter L or the unified counter is c
        LCounterSetAndClearAreReversedWhenCounterLOrTheUnifiedCounterIsCountingDown = 1,
        /// H counter. Set and clear are reversed when counter H is counting down. Do not us
        HCounterSetAndClearAreReversedWhenCounterHIsCountingDownDoNotUseIfUNIFY1 = 2
    ],
    /// Set/clear operation on output 4. Value 0x3 is reserved. Do not program this valu
    SETCLR4 OFFSET(8) NUMBITS(2) [
        /// Independent. Set and clear do not depend on any counter.
        IndependentSetAndClearDoNotDependOnAnyCounter = 0,
        /// L counter. Set and clear are reversed when counter L or the unified counter is c
        LCounterSetAndClearAreReversedWhenCounterLOrTheUnifiedCounterIsCountingDown = 1,
        /// H counter. Set and clear are reversed when counter H is counting down. Do not us
        HCounterSetAndClearAreReversedWhenCounterHIsCountingDownDoNotUseIfUNIFY1 = 2
    ],
    /// Set/clear operation on output 5. Value 0x3 is reserved. Do not program this valu
    SETCLR5 OFFSET(10) NUMBITS(2) [
        /// Independent. Set and clear do not depend on any counter.
        IndependentSetAndClearDoNotDependOnAnyCounter = 0,
        /// L counter. Set and clear are reversed when counter L or the unified counter is c
        LCounterSetAndClearAreReversedWhenCounterLOrTheUnifiedCounterIsCountingDown = 1,
        /// H counter. Set and clear are reversed when counter H is counting down. Do not us
        HCounterSetAndClearAreReversedWhenCounterHIsCountingDownDoNotUseIfUNIFY1 = 2
    ],
    /// Set/clear operation on output 6. Value 0x3 is reserved. Do not program this valu
    SETCLR6 OFFSET(12) NUMBITS(2) [
        /// Independent. Set and clear do not depend on any counter.
        IndependentSetAndClearDoNotDependOnAnyCounter = 0,
        /// L counter. Set and clear are reversed when counter L or the unified counter is c
        LCounterSetAndClearAreReversedWhenCounterLOrTheUnifiedCounterIsCountingDown = 1,
        /// H counter. Set and clear are reversed when counter H is counting down. Do not us
        HCounterSetAndClearAreReversedWhenCounterHIsCountingDownDoNotUseIfUNIFY1 = 2
    ],
    /// Set/clear operation on output 7. Value 0x3 is reserved. Do not program this valu
    SETCLR7 OFFSET(14) NUMBITS(2) [
        /// Independent. Set and clear do not depend on any counter.
        IndependentSetAndClearDoNotDependOnAnyCounter = 0,
        /// L counter. Set and clear are reversed when counter L or the unified counter is c
        LCounterSetAndClearAreReversedWhenCounterLOrTheUnifiedCounterIsCountingDown = 1,
        /// H counter. Set and clear are reversed when counter H is counting down. Do not us
        HCounterSetAndClearAreReversedWhenCounterHIsCountingDownDoNotUseIfUNIFY1 = 2
    ],
    /// Set/clear operation on output 8. Value 0x3 is reserved. Do not program this valu
    SETCLR8 OFFSET(16) NUMBITS(2) [
        /// Independent. Set and clear do not depend on any counter.
        IndependentSetAndClearDoNotDependOnAnyCounter = 0,
        /// L counter. Set and clear are reversed when counter L or the unified counter is c
        LCounterSetAndClearAreReversedWhenCounterLOrTheUnifiedCounterIsCountingDown = 1,
        /// H counter. Set and clear are reversed when counter H is counting down. Do not us
        HCounterSetAndClearAreReversedWhenCounterHIsCountingDownDoNotUseIfUNIFY1 = 2
    ],
    /// Set/clear operation on output 9. Value 0x3 is reserved. Do not program this valu
    SETCLR9 OFFSET(18) NUMBITS(2) [
        /// Independent. Set and clear do not depend on any counter.
        IndependentSetAndClearDoNotDependOnAnyCounter = 0,
        /// L counter. Set and clear are reversed when counter L or the unified counter is c
        LCounterSetAndClearAreReversedWhenCounterLOrTheUnifiedCounterIsCountingDown = 1,
        /// H counter. Set and clear are reversed when counter H is counting down. Do not us
        HCounterSetAndClearAreReversedWhenCounterHIsCountingDownDoNotUseIfUNIFY1 = 2
    ],
    /// Set/clear operation on output 5. Value 0x3 is reserved. Do not program this valu
    SETCLR10 OFFSET(20) NUMBITS(2) [
        /// Independent. Set and clear do not depend on any counter.
        IndependentSetAndClearDoNotDependOnAnyCounter = 0,
        /// L counter. Set and clear are reversed when counter L or the unified counter is c
        LCounterSetAndClearAreReversedWhenCounterLOrTheUnifiedCounterIsCountingDown = 1,
        /// H counter. Set and clear are reversed when counter H is counting down. Do not us
        HCounterSetAndClearAreReversedWhenCounterHIsCountingDownDoNotUseIfUNIFY1 = 2
    ],
    /// Set/clear operation on output 11. Value 0x3 is reserved. Do not program this val
    SETCLR11 OFFSET(22) NUMBITS(2) [
        /// Independent. Set and clear do not depend on any counter.
        IndependentSetAndClearDoNotDependOnAnyCounter = 0,
        /// L counter. Set and clear are reversed when counter L or the unified counter is c
        LCounterSetAndClearAreReversedWhenCounterLOrTheUnifiedCounterIsCountingDown = 1,
        /// H counter. Set and clear are reversed when counter H is counting down. Do not us
        HCounterSetAndClearAreReversedWhenCounterHIsCountingDownDoNotUseIfUNIFY1 = 2
    ],
    /// Set/clear operation on output 12. Value 0x3 is reserved. Do not program this val
    SETCLR12 OFFSET(24) NUMBITS(2) [
        /// Independent. Set and clear do not depend on any counter.
        IndependentSetAndClearDoNotDependOnAnyCounter = 0,
        /// L counter. Set and clear are reversed when counter L or the unified counter is c
        LCounterSetAndClearAreReversedWhenCounterLOrTheUnifiedCounterIsCountingDown = 1,
        /// H counter. Set and clear are reversed when counter H is counting down. Do not us
        HCounterSetAndClearAreReversedWhenCounterHIsCountingDownDoNotUseIfUNIFY1 = 2
    ],
    /// Set/clear operation on output 13. Value 0x3 is reserved. Do not program this val
    SETCLR13 OFFSET(26) NUMBITS(2) [
        /// Independent. Set and clear do not depend on any counter.
        IndependentSetAndClearDoNotDependOnAnyCounter = 0,
        /// L counter. Set and clear are reversed when counter L or the unified counter is c
        LCounterSetAndClearAreReversedWhenCounterLOrTheUnifiedCounterIsCountingDown = 1,
        /// H counter. Set and clear are reversed when counter H is counting down. Do not us
        HCounterSetAndClearAreReversedWhenCounterHIsCountingDownDoNotUseIfUNIFY1 = 2
    ],
    /// Set/clear operation on output 14. Value 0x3 is reserved. Do not program this val
    SETCLR14 OFFSET(28) NUMBITS(2) [
        /// Independent. Set and clear do not depend on any counter.
        IndependentSetAndClearDoNotDependOnAnyCounter = 0,
        /// L counter. Set and clear are reversed when counter L or the unified counter is c
        LCounterSetAndClearAreReversedWhenCounterLOrTheUnifiedCounterIsCountingDown = 1,
        /// H counter. Set and clear are reversed when counter H is counting down. Do not us
        HCounterSetAndClearAreReversedWhenCounterHIsCountingDownDoNotUseIfUNIFY1 = 2
    ],
    /// Set/clear operation on output 15. Value 0x3 is reserved. Do not program this val
    SETCLR15 OFFSET(30) NUMBITS(2) [
        /// Independent. Set and clear do not depend on any counter.
        IndependentSetAndClearDoNotDependOnAnyCounter = 0,
        /// L counter. Set and clear are reversed when counter L or the unified counter is c
        LCounterSetAndClearAreReversedWhenCounterLOrTheUnifiedCounterIsCountingDown = 1,
        /// H counter. Set and clear are reversed when counter H is counting down. Do not us
        HCounterSetAndClearAreReversedWhenCounterHIsCountingDownDoNotUseIfUNIFY1 = 2
    ]
],
RES [
    /// Effect of simultaneous set and clear on output 0.
    O0RES OFFSET(0) NUMBITS(2) [
        /// No change.
        NoChange = 0,
        /// Set output (or clear based on the SETCLR0 field).
        SetOutputOrClearBasedOnTheSETCLR0Field = 1,
        /// Clear output (or set based on the SETCLR0 field).
        ClearOutputOrSetBasedOnTheSETCLR0Field = 2,
        /// Toggle output.
        ToggleOutput = 3
    ],
    /// Effect of simultaneous set and clear on output 1.
    O1RES OFFSET(2) NUMBITS(2) [
        /// No change.
        NoChange = 0,
        /// Set output (or clear based on the SETCLR1 field).
        SetOutputOrClearBasedOnTheSETCLR1Field = 1,
        /// Clear output (or set based on the SETCLR1 field).
        ClearOutputOrSetBasedOnTheSETCLR1Field = 2,
        /// Toggle output.
        ToggleOutput = 3
    ],
    /// Effect of simultaneous set and clear on output 2.
    O2RES OFFSET(4) NUMBITS(2) [
        /// No change.
        NoChange = 0,
        /// Set output (or clear based on the SETCLR2 field).
        SetOutputOrClearBasedOnTheSETCLR2Field = 1,
        /// Clear output n (or set based on the SETCLR2 field).
        ClearOutputNOrSetBasedOnTheSETCLR2Field = 2,
        /// Toggle output.
        ToggleOutput = 3
    ],
    /// Effect of simultaneous set and clear on output 3.
    O3RES OFFSET(6) NUMBITS(2) [
        /// No change.
        NoChange = 0,
        /// Set output (or clear based on the SETCLR3 field).
        SetOutputOrClearBasedOnTheSETCLR3Field = 1,
        /// Clear output (or set based on the SETCLR3 field).
        ClearOutputOrSetBasedOnTheSETCLR3Field = 2,
        /// Toggle output.
        ToggleOutput = 3
    ],
    /// Effect of simultaneous set and clear on output 4.
    O4RES OFFSET(8) NUMBITS(2) [
        /// No change.
        NoChange = 0,
        /// Set output (or clear based on the SETCLR4 field).
        SetOutputOrClearBasedOnTheSETCLR4Field = 1,
        /// Clear output (or set based on the SETCLR4 field).
        ClearOutputOrSetBasedOnTheSETCLR4Field = 2,
        /// Toggle output.
        ToggleOutput = 3
    ],
    /// Effect of simultaneous set and clear on output 5.
    O5RES OFFSET(10) NUMBITS(2) [
        /// No change.
        NoChange = 0,
        /// Set output (or clear based on the SETCLR5 field).
        SetOutputOrClearBasedOnTheSETCLR5Field = 1,
        /// Clear output (or set based on the SETCLR5 field).
        ClearOutputOrSetBasedOnTheSETCLR5Field = 2,
        /// Toggle output.
        ToggleOutput = 3
    ],
    /// Effect of simultaneous set and clear on output 6.
    O6RES OFFSET(12) NUMBITS(2) [
        /// No change.
        NoChange = 0,
        /// Set output (or clear based on the SETCLR6 field).
        SetOutputOrClearBasedOnTheSETCLR6Field = 1,
        /// Clear output (or set based on the SETCLR6 field).
        ClearOutputOrSetBasedOnTheSETCLR6Field = 2,
        /// Toggle output.
        ToggleOutput = 3
    ],
    /// Effect of simultaneous set and clear on output 7.
    O7RES OFFSET(14) NUMBITS(2) [
        /// No change.
        NoChange = 0,
        /// Set output (or clear based on the SETCLR7 field).
        SetOutputOrClearBasedOnTheSETCLR7Field = 1,
        /// Clear output (or set based on the SETCLR7 field).
        ClearOutputOrSetBasedOnTheSETCLR7Field = 2,
        /// Toggle output.
        ToggleOutput = 3
    ],
    /// Effect of simultaneous set and clear on output 8.
    O8RES OFFSET(16) NUMBITS(2) [
        /// No change.
        NoChange = 0,
        /// Set output (or clear based on the SETCLR8 field).
        SetOutputOrClearBasedOnTheSETCLR8Field = 1,
        /// Clear output (or set based on the SETCLR8 field).
        ClearOutputOrSetBasedOnTheSETCLR8Field = 2,
        /// Toggle output.
        ToggleOutput = 3
    ],
    /// Effect of simultaneous set and clear on output 9.
    O9RES OFFSET(18) NUMBITS(2) [
        /// No change.
        NoChange = 0,
        /// Set output (or clear based on the SETCLR9 field).
        SetOutputOrClearBasedOnTheSETCLR9Field = 1,
        /// Clear output (or set based on the SETCLR9 field).
        ClearOutputOrSetBasedOnTheSETCLR9Field = 2,
        /// Toggle output.
        ToggleOutput = 3
    ],
    /// Effect of simultaneous set and clear on output 10.
    O10RES OFFSET(20) NUMBITS(2) [
        /// No change.
        NoChange = 0,
        /// Set output (or clear based on the SETCLR10 field).
        SetOutputOrClearBasedOnTheSETCLR10Field = 1,
        /// Clear output (or set based on the SETCLR10 field).
        ClearOutputOrSetBasedOnTheSETCLR10Field = 2,
        /// Toggle output.
        ToggleOutput = 3
    ],
    /// Effect of simultaneous set and clear on output 11.
    O11RES OFFSET(22) NUMBITS(2) [
        /// No change.
        NoChange = 0,
        /// Set output (or clear based on the SETCLR11 field).
        SetOutputOrClearBasedOnTheSETCLR11Field = 1,
        /// Clear output (or set based on the SETCLR11 field).
        ClearOutputOrSetBasedOnTheSETCLR11Field = 2,
        /// Toggle output.
        ToggleOutput = 3
    ],
    /// Effect of simultaneous set and clear on output 12.
    O12RES OFFSET(24) NUMBITS(2) [
        /// No change.
        NoChange = 0,
        /// Set output (or clear based on the SETCLR12 field).
        SetOutputOrClearBasedOnTheSETCLR12Field = 1,
        /// Clear output (or set based on the SETCLR12 field).
        ClearOutputOrSetBasedOnTheSETCLR12Field = 2,
        /// Toggle output.
        ToggleOutput = 3
    ],
    /// Effect of simultaneous set and clear on output 13.
    O13RES OFFSET(26) NUMBITS(2) [
        /// No change.
        NoChange = 0,
        /// Set output (or clear based on the SETCLR13 field).
        SetOutputOrClearBasedOnTheSETCLR13Field = 1,
        /// Clear output (or set based on the SETCLR13 field).
        ClearOutputOrSetBasedOnTheSETCLR13Field = 2,
        /// Toggle output.
        ToggleOutput = 3
    ],
    /// Effect of simultaneous set and clear on output 14.
    O14RES OFFSET(28) NUMBITS(2) [
        /// No change.
        NoChange = 0,
        /// Set output (or clear based on the SETCLR14 field).
        SetOutputOrClearBasedOnTheSETCLR14Field = 1,
        /// Clear output (or set based on the SETCLR14 field).
        ClearOutputOrSetBasedOnTheSETCLR14Field = 2,
        /// Toggle output.
        ToggleOutput = 3
    ],
    /// Effect of simultaneous set and clear on output 15.
    O15RES OFFSET(30) NUMBITS(2) [
        /// No change.
        NoChange = 0,
        /// Set output (or clear based on the SETCLR15 field).
        SetOutputOrClearBasedOnTheSETCLR15Field = 1,
        /// Clear output (or set based on the SETCLR15 field).
        ClearOutputOrSetBasedOnTheSETCLR15Field = 2,
        /// Toggle output.
        ToggleOutput = 3
    ]
],
DMAREQ0 [
    /// If bit n is one, event n sets DMA request 0 (event 0 = bit 0, event 1 = bit 1,..
    DEV_00 OFFSET(0) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 0 (event 0 = bit 0, event 1 = bit 1,..
    DEV_01 OFFSET(1) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 0 (event 0 = bit 0, event 1 = bit 1,..
    DEV_02 OFFSET(2) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 0 (event 0 = bit 0, event 1 = bit 1,..
    DEV_03 OFFSET(3) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 0 (event 0 = bit 0, event 1 = bit 1,..
    DEV_04 OFFSET(4) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 0 (event 0 = bit 0, event 1 = bit 1,..
    DEV_05 OFFSET(5) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 0 (event 0 = bit 0, event 1 = bit 1,..
    DEV_06 OFFSET(6) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 0 (event 0 = bit 0, event 1 = bit 1,..
    DEV_07 OFFSET(7) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 0 (event 0 = bit 0, event 1 = bit 1,..
    DEV_08 OFFSET(8) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 0 (event 0 = bit 0, event 1 = bit 1,..
    DEV_09 OFFSET(9) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 0 (event 0 = bit 0, event 1 = bit 1,..
    DEV_010 OFFSET(10) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 0 (event 0 = bit 0, event 1 = bit 1,..
    DEV_011 OFFSET(11) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 0 (event 0 = bit 0, event 1 = bit 1,..
    DEV_012 OFFSET(12) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 0 (event 0 = bit 0, event 1 = bit 1,..
    DEV_013 OFFSET(13) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 0 (event 0 = bit 0, event 1 = bit 1,..
    DEV_014 OFFSET(14) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 0 (event 0 = bit 0, event 1 = bit 1,..
    DEV_015 OFFSET(15) NUMBITS(1) [],
    /// A 1 in this bit makes the SCT set DMA request 0 when it loads the  Match_L/Unifi
    DRL0 OFFSET(30) NUMBITS(1) [],
    /// This read-only bit indicates the state of DMA Request 0
    DRQ0 OFFSET(31) NUMBITS(1) []
],
DMAREQ1 [
    /// If bit n is one, event n sets DMA request 1 (event 0 = bit 0, event 1 = bit 1,..
    DEV_10 OFFSET(0) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 1 (event 0 = bit 0, event 1 = bit 1,..
    DEV_11 OFFSET(1) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 1 (event 0 = bit 0, event 1 = bit 1,..
    DEV_12 OFFSET(2) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 1 (event 0 = bit 0, event 1 = bit 1,..
    DEV_13 OFFSET(3) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 1 (event 0 = bit 0, event 1 = bit 1,..
    DEV_14 OFFSET(4) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 1 (event 0 = bit 0, event 1 = bit 1,..
    DEV_15 OFFSET(5) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 1 (event 0 = bit 0, event 1 = bit 1,..
    DEV_16 OFFSET(6) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 1 (event 0 = bit 0, event 1 = bit 1,..
    DEV_17 OFFSET(7) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 1 (event 0 = bit 0, event 1 = bit 1,..
    DEV_18 OFFSET(8) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 1 (event 0 = bit 0, event 1 = bit 1,..
    DEV_19 OFFSET(9) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 1 (event 0 = bit 0, event 1 = bit 1,..
    DEV_110 OFFSET(10) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 1 (event 0 = bit 0, event 1 = bit 1,..
    DEV_111 OFFSET(11) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 1 (event 0 = bit 0, event 1 = bit 1,..
    DEV_112 OFFSET(12) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 1 (event 0 = bit 0, event 1 = bit 1,..
    DEV_113 OFFSET(13) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 1 (event 0 = bit 0, event 1 = bit 1,..
    DEV_114 OFFSET(14) NUMBITS(1) [],
    /// If bit n is one, event n sets DMA request 1 (event 0 = bit 0, event 1 = bit 1,..
    DEV_115 OFFSET(15) NUMBITS(1) [],
    /// A 1 in this bit makes the SCT set DMA request 1 when it loads the  Match L/Unifi
    DRL1 OFFSET(30) NUMBITS(1) [],
    /// This read-only bit indicates the state of DMA Request 1.
    DRQ1 OFFSET(31) NUMBITS(1) []
],
EVEN [
    /// The SCT requests interrupt when bit n of this register and the event flag regist
    IEN0 OFFSET(0) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the event flag regist
    IEN1 OFFSET(1) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the event flag regist
    IEN2 OFFSET(2) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the event flag regist
    IEN3 OFFSET(3) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the event flag regist
    IEN4 OFFSET(4) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the event flag regist
    IEN5 OFFSET(5) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the event flag regist
    IEN6 OFFSET(6) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the event flag regist
    IEN7 OFFSET(7) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the event flag regist
    IEN8 OFFSET(8) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the event flag regist
    IEN9 OFFSET(9) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the event flag regist
    IEN10 OFFSET(10) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the event flag regist
    IEN11 OFFSET(11) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the event flag regist
    IEN12 OFFSET(12) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the event flag regist
    IEN13 OFFSET(13) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the event flag regist
    IEN14 OFFSET(14) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the event flag regist
    IEN15 OFFSET(15) NUMBITS(1) []
],
EVFLAG [
    /// Bit n is one if event n has occurred since reset or a 1 was last written to this
    FLAG0 OFFSET(0) NUMBITS(1) [],
    /// Bit n is one if event n has occurred since reset or a 1 was last written to this
    FLAG1 OFFSET(1) NUMBITS(1) [],
    /// Bit n is one if event n has occurred since reset or a 1 was last written to this
    FLAG2 OFFSET(2) NUMBITS(1) [],
    /// Bit n is one if event n has occurred since reset or a 1 was last written to this
    FLAG3 OFFSET(3) NUMBITS(1) [],
    /// Bit n is one if event n has occurred since reset or a 1 was last written to this
    FLAG4 OFFSET(4) NUMBITS(1) [],
    /// Bit n is one if event n has occurred since reset or a 1 was last written to this
    FLAG5 OFFSET(5) NUMBITS(1) [],
    /// Bit n is one if event n has occurred since reset or a 1 was last written to this
    FLAG6 OFFSET(6) NUMBITS(1) [],
    /// Bit n is one if event n has occurred since reset or a 1 was last written to this
    FLAG7 OFFSET(7) NUMBITS(1) [],
    /// Bit n is one if event n has occurred since reset or a 1 was last written to this
    FLAG8 OFFSET(8) NUMBITS(1) [],
    /// Bit n is one if event n has occurred since reset or a 1 was last written to this
    FLAG9 OFFSET(9) NUMBITS(1) [],
    /// Bit n is one if event n has occurred since reset or a 1 was last written to this
    FLAG10 OFFSET(10) NUMBITS(1) [],
    /// Bit n is one if event n has occurred since reset or a 1 was last written to this
    FLAG11 OFFSET(11) NUMBITS(1) [],
    /// Bit n is one if event n has occurred since reset or a 1 was last written to this
    FLAG12 OFFSET(12) NUMBITS(1) [],
    /// Bit n is one if event n has occurred since reset or a 1 was last written to this
    FLAG13 OFFSET(13) NUMBITS(1) [],
    /// Bit n is one if event n has occurred since reset or a 1 was last written to this
    FLAG14 OFFSET(14) NUMBITS(1) [],
    /// Bit n is one if event n has occurred since reset or a 1 was last written to this
    FLAG15 OFFSET(15) NUMBITS(1) []
],
CONEN [
    /// The SCT requests interrupt when bit n of this register and the SCT conflict flag
    NCEN0 OFFSET(0) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the SCT conflict flag
    NCEN1 OFFSET(1) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the SCT conflict flag
    NCEN2 OFFSET(2) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the SCT conflict flag
    NCEN3 OFFSET(3) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the SCT conflict flag
    NCEN4 OFFSET(4) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the SCT conflict flag
    NCEN5 OFFSET(5) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the SCT conflict flag
    NCEN6 OFFSET(6) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the SCT conflict flag
    NCEN7 OFFSET(7) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the SCT conflict flag
    NCEN8 OFFSET(8) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the SCT conflict flag
    NCEN9 OFFSET(9) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the SCT conflict flag
    NCEN10 OFFSET(10) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the SCT conflict flag
    NCEN11 OFFSET(11) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the SCT conflict flag
    NCEN12 OFFSET(12) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the SCT conflict flag
    NCEN13 OFFSET(13) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the SCT conflict flag
    NCEN14 OFFSET(14) NUMBITS(1) [],
    /// The SCT requests interrupt when bit n of this register and the SCT conflict flag
    NCEN15 OFFSET(15) NUMBITS(1) []
],
CONFLAG [
    /// Bit n is one if a no-change conflict event occurred on output n since  reset or
    NCFLAG0 OFFSET(0) NUMBITS(1) [],
    /// Bit n is one if a no-change conflict event occurred on output n since  reset or
    NCFLAG1 OFFSET(1) NUMBITS(1) [],
    /// Bit n is one if a no-change conflict event occurred on output n since  reset or
    NCFLAG2 OFFSET(2) NUMBITS(1) [],
    /// Bit n is one if a no-change conflict event occurred on output n since  reset or
    NCFLAG3 OFFSET(3) NUMBITS(1) [],
    /// Bit n is one if a no-change conflict event occurred on output n since  reset or
    NCFLAG4 OFFSET(4) NUMBITS(1) [],
    /// Bit n is one if a no-change conflict event occurred on output n since  reset or
    NCFLAG5 OFFSET(5) NUMBITS(1) [],
    /// Bit n is one if a no-change conflict event occurred on output n since  reset or
    NCFLAG6 OFFSET(6) NUMBITS(1) [],
    /// Bit n is one if a no-change conflict event occurred on output n since  reset or
    NCFLAG7 OFFSET(7) NUMBITS(1) [],
    /// Bit n is one if a no-change conflict event occurred on output n since  reset or
    NCFLAG8 OFFSET(8) NUMBITS(1) [],
    /// Bit n is one if a no-change conflict event occurred on output n since  reset or
    NCFLAG9 OFFSET(9) NUMBITS(1) [],
    /// Bit n is one if a no-change conflict event occurred on output n since  reset or
    NCFLAG10 OFFSET(10) NUMBITS(1) [],
    /// Bit n is one if a no-change conflict event occurred on output n since  reset or
    NCFLAG11 OFFSET(11) NUMBITS(1) [],
    /// Bit n is one if a no-change conflict event occurred on output n since  reset or
    NCFLAG12 OFFSET(12) NUMBITS(1) [],
    /// Bit n is one if a no-change conflict event occurred on output n since  reset or
    NCFLAG13 OFFSET(13) NUMBITS(1) [],
    /// Bit n is one if a no-change conflict event occurred on output n since  reset or
    NCFLAG14 OFFSET(14) NUMBITS(1) [],
    /// Bit n is one if a no-change conflict event occurred on output n since  reset or
    NCFLAG15 OFFSET(15) NUMBITS(1) [],
    /// The most recent bus error from this SCT involved writing CTR  L/Unified, STATE L
    BUSERRL OFFSET(30) NUMBITS(1) [],
    /// The most recent bus error from this SCT involved writing CTR H,  STATE H, MATCH
    BUSERRH OFFSET(31) NUMBITS(1) []
],
MATCH0 [
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the L counter.
    MATCH_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the H counter.
    MATCH_H OFFSET(16) NUMBITS(16) []
],
MATCH1 [
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the L counter.
    MATCH_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the H counter.
    MATCH_H OFFSET(16) NUMBITS(16) []
],
MATCH2 [
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the L counter.
    MATCH_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the H counter.
    MATCH_H OFFSET(16) NUMBITS(16) []
],
MATCH3 [
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the L counter.
    MATCH_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the H counter.
    MATCH_H OFFSET(16) NUMBITS(16) []
],
MATCH4 [
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the L counter.
    MATCH_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the H counter.
    MATCH_H OFFSET(16) NUMBITS(16) []
],
MATCH5 [
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the L counter.
    MATCH_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the H counter.
    MATCH_H OFFSET(16) NUMBITS(16) []
],
MATCH6 [
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the L counter.
    MATCH_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the H counter.
    MATCH_H OFFSET(16) NUMBITS(16) []
],
MATCH7 [
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the L counter.
    MATCH_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the H counter.
    MATCH_H OFFSET(16) NUMBITS(16) []
],
MATCH8 [
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the L counter.
    MATCH_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the H counter.
    MATCH_H OFFSET(16) NUMBITS(16) []
],
MATCH9 [
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the L counter.
    MATCH_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the H counter.
    MATCH_H OFFSET(16) NUMBITS(16) []
],
MATCH10 [
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the L counter.
    MATCH_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the H counter.
    MATCH_H OFFSET(16) NUMBITS(16) []
],
MATCH11 [
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the L counter.
    MATCH_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the H counter.
    MATCH_H OFFSET(16) NUMBITS(16) []
],
MATCH12 [
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the L counter.
    MATCH_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the H counter.
    MATCH_H OFFSET(16) NUMBITS(16) []
],
MATCH13 [
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the L counter.
    MATCH_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the H counter.
    MATCH_H OFFSET(16) NUMBITS(16) []
],
MATCH14 [
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the L counter.
    MATCH_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the H counter.
    MATCH_H OFFSET(16) NUMBITS(16) []
],
MATCH15 [
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the L counter.
    MATCH_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit value to be  compared to the H counter.
    MATCH_H OFFSET(16) NUMBITS(16) []
],
FRACMAT0 [
    /// When UNIFY = 0, read or write the 4-bit value specifying the dither pattern to b
    FRACMAT_L OFFSET(0) NUMBITS(4) [],
    /// When UNIFY = 0, read or write 4-bit value specifying the dither pattern to be ap
    FRACMAT_H OFFSET(16) NUMBITS(4) []
],
FRACMAT1 [
    /// When UNIFY = 0, read or write the 4-bit value specifying the dither pattern to b
    FRACMAT_L OFFSET(0) NUMBITS(4) [],
    /// When UNIFY = 0, read or write 4-bit value specifying the dither pattern to be ap
    FRACMAT_H OFFSET(16) NUMBITS(4) []
],
FRACMAT2 [
    /// When UNIFY = 0, read or write the 4-bit value specifying the dither pattern to b
    FRACMAT_L OFFSET(0) NUMBITS(4) [],
    /// When UNIFY = 0, read or write 4-bit value specifying the dither pattern to be ap
    FRACMAT_H OFFSET(16) NUMBITS(4) []
],
FRACMAT3 [
    /// When UNIFY = 0, read or write the 4-bit value specifying the dither pattern to b
    FRACMAT_L OFFSET(0) NUMBITS(4) [],
    /// When UNIFY = 0, read or write 4-bit value specifying the dither pattern to be ap
    FRACMAT_H OFFSET(16) NUMBITS(4) []
],
FRACMAT4 [
    /// When UNIFY = 0, read or write the 4-bit value specifying the dither pattern to b
    FRACMAT_L OFFSET(0) NUMBITS(4) [],
    /// When UNIFY = 0, read or write 4-bit value specifying the dither pattern to be ap
    FRACMAT_H OFFSET(16) NUMBITS(4) []
],
FRACMAT5 [
    /// When UNIFY = 0, read or write the 4-bit value specifying the dither pattern to b
    FRACMAT_L OFFSET(0) NUMBITS(4) [],
    /// When UNIFY = 0, read or write 4-bit value specifying the dither pattern to be ap
    FRACMAT_H OFFSET(16) NUMBITS(4) []
],
CAP0 [
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_H OFFSET(16) NUMBITS(16) []
],
CAP1 [
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_H OFFSET(16) NUMBITS(16) []
],
CAP2 [
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_H OFFSET(16) NUMBITS(16) []
],
CAP3 [
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_H OFFSET(16) NUMBITS(16) []
],
CAP4 [
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_H OFFSET(16) NUMBITS(16) []
],
CAP5 [
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_H OFFSET(16) NUMBITS(16) []
],
CAP6 [
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_H OFFSET(16) NUMBITS(16) []
],
CAP7 [
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_H OFFSET(16) NUMBITS(16) []
],
CAP8 [
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_H OFFSET(16) NUMBITS(16) []
],
CAP9 [
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_H OFFSET(16) NUMBITS(16) []
],
CAP10 [
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_H OFFSET(16) NUMBITS(16) []
],
CAP11 [
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_H OFFSET(16) NUMBITS(16) []
],
CAP12 [
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_H OFFSET(16) NUMBITS(16) []
],
CAP13 [
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_H OFFSET(16) NUMBITS(16) []
],
CAP14 [
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_H OFFSET(16) NUMBITS(16) []
],
CAP15 [
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read the 16-bit counter value at which this register was last ca
    CAP_H OFFSET(16) NUMBITS(16) []
],
MATCHREL0 [
    /// When UNIFY = 0, read or write the 16-bit value to be loaded into the MATCHn_L re
    RELOAD_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit to be loaded into the MATCHn_H register
    RELOAD_H OFFSET(16) NUMBITS(16) []
],
MATCHREL1 [
    /// When UNIFY = 0, read or write the 16-bit value to be loaded into the MATCHn_L re
    RELOAD_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit to be loaded into the MATCHn_H register
    RELOAD_H OFFSET(16) NUMBITS(16) []
],
MATCHREL2 [
    /// When UNIFY = 0, read or write the 16-bit value to be loaded into the MATCHn_L re
    RELOAD_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit to be loaded into the MATCHn_H register
    RELOAD_H OFFSET(16) NUMBITS(16) []
],
MATCHREL3 [
    /// When UNIFY = 0, read or write the 16-bit value to be loaded into the MATCHn_L re
    RELOAD_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit to be loaded into the MATCHn_H register
    RELOAD_H OFFSET(16) NUMBITS(16) []
],
MATCHREL4 [
    /// When UNIFY = 0, read or write the 16-bit value to be loaded into the MATCHn_L re
    RELOAD_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit to be loaded into the MATCHn_H register
    RELOAD_H OFFSET(16) NUMBITS(16) []
],
MATCHREL5 [
    /// When UNIFY = 0, read or write the 16-bit value to be loaded into the MATCHn_L re
    RELOAD_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit to be loaded into the MATCHn_H register
    RELOAD_H OFFSET(16) NUMBITS(16) []
],
MATCHREL6 [
    /// When UNIFY = 0, read or write the 16-bit value to be loaded into the MATCHn_L re
    RELOAD_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit to be loaded into the MATCHn_H register
    RELOAD_H OFFSET(16) NUMBITS(16) []
],
MATCHREL7 [
    /// When UNIFY = 0, read or write the 16-bit value to be loaded into the MATCHn_L re
    RELOAD_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit to be loaded into the MATCHn_H register
    RELOAD_H OFFSET(16) NUMBITS(16) []
],
MATCHREL8 [
    /// When UNIFY = 0, read or write the 16-bit value to be loaded into the MATCHn_L re
    RELOAD_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit to be loaded into the MATCHn_H register
    RELOAD_H OFFSET(16) NUMBITS(16) []
],
MATCHREL9 [
    /// When UNIFY = 0, read or write the 16-bit value to be loaded into the MATCHn_L re
    RELOAD_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit to be loaded into the MATCHn_H register
    RELOAD_H OFFSET(16) NUMBITS(16) []
],
MATCHREL10 [
    /// When UNIFY = 0, read or write the 16-bit value to be loaded into the MATCHn_L re
    RELOAD_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit to be loaded into the MATCHn_H register
    RELOAD_H OFFSET(16) NUMBITS(16) []
],
MATCHREL11 [
    /// When UNIFY = 0, read or write the 16-bit value to be loaded into the MATCHn_L re
    RELOAD_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit to be loaded into the MATCHn_H register
    RELOAD_H OFFSET(16) NUMBITS(16) []
],
MATCHREL12 [
    /// When UNIFY = 0, read or write the 16-bit value to be loaded into the MATCHn_L re
    RELOAD_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit to be loaded into the MATCHn_H register
    RELOAD_H OFFSET(16) NUMBITS(16) []
],
MATCHREL13 [
    /// When UNIFY = 0, read or write the 16-bit value to be loaded into the MATCHn_L re
    RELOAD_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit to be loaded into the MATCHn_H register
    RELOAD_H OFFSET(16) NUMBITS(16) []
],
MATCHREL14 [
    /// When UNIFY = 0, read or write the 16-bit value to be loaded into the MATCHn_L re
    RELOAD_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit to be loaded into the MATCHn_H register
    RELOAD_H OFFSET(16) NUMBITS(16) []
],
MATCHREL15 [
    /// When UNIFY = 0, read or write the 16-bit value to be loaded into the MATCHn_L re
    RELOAD_L OFFSET(0) NUMBITS(16) [],
    /// When UNIFY = 0, read or write the 16-bit to be loaded into the MATCHn_H register
    RELOAD_H OFFSET(16) NUMBITS(16) []
],
FRACMATREL0 [
    /// When UNIFY = 0, read or write the 4-bit value to be loaded into the FRACMATn_L r
    RELFRAC_L OFFSET(0) NUMBITS(4) [],
    /// When UNIFY = 0, read or write the 4-bit value to be loaded into the FRACMATn_H r
    RELFRAC_H OFFSET(16) NUMBITS(4) []
],
FRACMATREL1 [
    /// When UNIFY = 0, read or write the 4-bit value to be loaded into the FRACMATn_L r
    RELFRAC_L OFFSET(0) NUMBITS(4) [],
    /// When UNIFY = 0, read or write the 4-bit value to be loaded into the FRACMATn_H r
    RELFRAC_H OFFSET(16) NUMBITS(4) []
],
FRACMATREL2 [
    /// When UNIFY = 0, read or write the 4-bit value to be loaded into the FRACMATn_L r
    RELFRAC_L OFFSET(0) NUMBITS(4) [],
    /// When UNIFY = 0, read or write the 4-bit value to be loaded into the FRACMATn_H r
    RELFRAC_H OFFSET(16) NUMBITS(4) []
],
FRACMATREL3 [
    /// When UNIFY = 0, read or write the 4-bit value to be loaded into the FRACMATn_L r
    RELFRAC_L OFFSET(0) NUMBITS(4) [],
    /// When UNIFY = 0, read or write the 4-bit value to be loaded into the FRACMATn_H r
    RELFRAC_H OFFSET(16) NUMBITS(4) []
],
FRACMATREL4 [
    /// When UNIFY = 0, read or write the 4-bit value to be loaded into the FRACMATn_L r
    RELFRAC_L OFFSET(0) NUMBITS(4) [],
    /// When UNIFY = 0, read or write the 4-bit value to be loaded into the FRACMATn_H r
    RELFRAC_H OFFSET(16) NUMBITS(4) []
],
FRACMATREL5 [
    /// When UNIFY = 0, read or write the 4-bit value to be loaded into the FRACMATn_L r
    RELFRAC_L OFFSET(0) NUMBITS(4) [],
    /// When UNIFY = 0, read or write the 4-bit value to be loaded into the FRACMATn_H r
    RELFRAC_H OFFSET(16) NUMBITS(4) []
],
CAPCTRL0 [
    /// If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) r
    CAPCON_L OFFSET(0) NUMBITS(16) [],
    /// If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (ev
    CAPCON_H OFFSET(16) NUMBITS(16) []
],
CAPCTRL1 [
    /// If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) r
    CAPCON_L OFFSET(0) NUMBITS(16) [],
    /// If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (ev
    CAPCON_H OFFSET(16) NUMBITS(16) []
],
CAPCTRL2 [
    /// If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) r
    CAPCON_L OFFSET(0) NUMBITS(16) [],
    /// If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (ev
    CAPCON_H OFFSET(16) NUMBITS(16) []
],
CAPCTRL3 [
    /// If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) r
    CAPCON_L OFFSET(0) NUMBITS(16) [],
    /// If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (ev
    CAPCON_H OFFSET(16) NUMBITS(16) []
],
CAPCTRL4 [
    /// If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) r
    CAPCON_L OFFSET(0) NUMBITS(16) [],
    /// If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (ev
    CAPCON_H OFFSET(16) NUMBITS(16) []
],
CAPCTRL5 [
    /// If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) r
    CAPCON_L OFFSET(0) NUMBITS(16) [],
    /// If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (ev
    CAPCON_H OFFSET(16) NUMBITS(16) []
],
CAPCTRL6 [
    /// If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) r
    CAPCON_L OFFSET(0) NUMBITS(16) [],
    /// If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (ev
    CAPCON_H OFFSET(16) NUMBITS(16) []
],
CAPCTRL7 [
    /// If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) r
    CAPCON_L OFFSET(0) NUMBITS(16) [],
    /// If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (ev
    CAPCON_H OFFSET(16) NUMBITS(16) []
],
CAPCTRL8 [
    /// If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) r
    CAPCON_L OFFSET(0) NUMBITS(16) [],
    /// If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (ev
    CAPCON_H OFFSET(16) NUMBITS(16) []
],
CAPCTRL9 [
    /// If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) r
    CAPCON_L OFFSET(0) NUMBITS(16) [],
    /// If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (ev
    CAPCON_H OFFSET(16) NUMBITS(16) []
],
CAPCTRL10 [
    /// If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) r
    CAPCON_L OFFSET(0) NUMBITS(16) [],
    /// If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (ev
    CAPCON_H OFFSET(16) NUMBITS(16) []
],
CAPCTRL11 [
    /// If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) r
    CAPCON_L OFFSET(0) NUMBITS(16) [],
    /// If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (ev
    CAPCON_H OFFSET(16) NUMBITS(16) []
],
CAPCTRL12 [
    /// If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) r
    CAPCON_L OFFSET(0) NUMBITS(16) [],
    /// If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (ev
    CAPCON_H OFFSET(16) NUMBITS(16) []
],
CAPCTRL13 [
    /// If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) r
    CAPCON_L OFFSET(0) NUMBITS(16) [],
    /// If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (ev
    CAPCON_H OFFSET(16) NUMBITS(16) []
],
CAPCTRL14 [
    /// If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) r
    CAPCON_L OFFSET(0) NUMBITS(16) [],
    /// If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (ev
    CAPCON_H OFFSET(16) NUMBITS(16) []
],
CAPCTRL15 [
    /// If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) r
    CAPCON_L OFFSET(0) NUMBITS(16) [],
    /// If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (ev
    CAPCON_H OFFSET(16) NUMBITS(16) []
],
EV0_STATE [
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK0 OFFSET(0) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK1 OFFSET(1) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK2 OFFSET(2) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK3 OFFSET(3) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK4 OFFSET(4) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK5 OFFSET(5) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK6 OFFSET(6) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK7 OFFSET(7) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK8 OFFSET(8) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK9 OFFSET(9) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK10 OFFSET(10) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK11 OFFSET(11) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK12 OFFSET(12) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK13 OFFSET(13) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK14 OFFSET(14) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK15 OFFSET(15) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK16 OFFSET(16) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK17 OFFSET(17) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK18 OFFSET(18) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK19 OFFSET(19) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK20 OFFSET(20) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK21 OFFSET(21) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK22 OFFSET(22) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK23 OFFSET(23) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK24 OFFSET(24) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK25 OFFSET(25) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK26 OFFSET(26) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK27 OFFSET(27) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK28 OFFSET(28) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK29 OFFSET(29) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK30 OFFSET(30) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK31 OFFSET(31) NUMBITS(1) []
],
EV1_STATE [
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK0 OFFSET(0) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK1 OFFSET(1) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK2 OFFSET(2) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK3 OFFSET(3) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK4 OFFSET(4) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK5 OFFSET(5) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK6 OFFSET(6) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK7 OFFSET(7) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK8 OFFSET(8) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK9 OFFSET(9) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK10 OFFSET(10) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK11 OFFSET(11) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK12 OFFSET(12) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK13 OFFSET(13) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK14 OFFSET(14) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK15 OFFSET(15) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK16 OFFSET(16) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK17 OFFSET(17) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK18 OFFSET(18) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK19 OFFSET(19) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK20 OFFSET(20) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK21 OFFSET(21) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK22 OFFSET(22) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK23 OFFSET(23) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK24 OFFSET(24) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK25 OFFSET(25) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK26 OFFSET(26) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK27 OFFSET(27) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK28 OFFSET(28) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK29 OFFSET(29) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK30 OFFSET(30) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK31 OFFSET(31) NUMBITS(1) []
],
EV2_STATE [
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK0 OFFSET(0) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK1 OFFSET(1) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK2 OFFSET(2) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK3 OFFSET(3) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK4 OFFSET(4) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK5 OFFSET(5) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK6 OFFSET(6) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK7 OFFSET(7) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK8 OFFSET(8) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK9 OFFSET(9) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK10 OFFSET(10) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK11 OFFSET(11) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK12 OFFSET(12) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK13 OFFSET(13) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK14 OFFSET(14) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK15 OFFSET(15) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK16 OFFSET(16) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK17 OFFSET(17) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK18 OFFSET(18) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK19 OFFSET(19) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK20 OFFSET(20) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK21 OFFSET(21) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK22 OFFSET(22) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK23 OFFSET(23) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK24 OFFSET(24) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK25 OFFSET(25) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK26 OFFSET(26) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK27 OFFSET(27) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK28 OFFSET(28) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK29 OFFSET(29) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK30 OFFSET(30) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK31 OFFSET(31) NUMBITS(1) []
],
EV3_STATE [
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK0 OFFSET(0) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK1 OFFSET(1) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK2 OFFSET(2) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK3 OFFSET(3) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK4 OFFSET(4) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK5 OFFSET(5) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK6 OFFSET(6) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK7 OFFSET(7) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK8 OFFSET(8) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK9 OFFSET(9) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK10 OFFSET(10) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK11 OFFSET(11) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK12 OFFSET(12) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK13 OFFSET(13) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK14 OFFSET(14) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK15 OFFSET(15) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK16 OFFSET(16) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK17 OFFSET(17) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK18 OFFSET(18) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK19 OFFSET(19) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK20 OFFSET(20) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK21 OFFSET(21) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK22 OFFSET(22) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK23 OFFSET(23) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK24 OFFSET(24) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK25 OFFSET(25) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK26 OFFSET(26) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK27 OFFSET(27) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK28 OFFSET(28) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK29 OFFSET(29) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK30 OFFSET(30) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK31 OFFSET(31) NUMBITS(1) []
],
EV4_STATE [
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK0 OFFSET(0) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK1 OFFSET(1) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK2 OFFSET(2) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK3 OFFSET(3) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK4 OFFSET(4) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK5 OFFSET(5) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK6 OFFSET(6) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK7 OFFSET(7) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK8 OFFSET(8) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK9 OFFSET(9) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK10 OFFSET(10) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK11 OFFSET(11) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK12 OFFSET(12) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK13 OFFSET(13) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK14 OFFSET(14) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK15 OFFSET(15) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK16 OFFSET(16) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK17 OFFSET(17) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK18 OFFSET(18) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK19 OFFSET(19) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK20 OFFSET(20) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK21 OFFSET(21) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK22 OFFSET(22) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK23 OFFSET(23) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK24 OFFSET(24) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK25 OFFSET(25) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK26 OFFSET(26) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK27 OFFSET(27) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK28 OFFSET(28) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK29 OFFSET(29) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK30 OFFSET(30) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK31 OFFSET(31) NUMBITS(1) []
],
EV5_STATE [
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK0 OFFSET(0) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK1 OFFSET(1) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK2 OFFSET(2) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK3 OFFSET(3) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK4 OFFSET(4) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK5 OFFSET(5) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK6 OFFSET(6) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK7 OFFSET(7) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK8 OFFSET(8) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK9 OFFSET(9) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK10 OFFSET(10) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK11 OFFSET(11) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK12 OFFSET(12) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK13 OFFSET(13) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK14 OFFSET(14) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK15 OFFSET(15) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK16 OFFSET(16) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK17 OFFSET(17) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK18 OFFSET(18) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK19 OFFSET(19) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK20 OFFSET(20) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK21 OFFSET(21) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK22 OFFSET(22) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK23 OFFSET(23) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK24 OFFSET(24) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK25 OFFSET(25) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK26 OFFSET(26) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK27 OFFSET(27) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK28 OFFSET(28) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK29 OFFSET(29) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK30 OFFSET(30) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK31 OFFSET(31) NUMBITS(1) []
],
EV6_STATE [
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK0 OFFSET(0) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK1 OFFSET(1) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK2 OFFSET(2) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK3 OFFSET(3) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK4 OFFSET(4) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK5 OFFSET(5) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK6 OFFSET(6) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK7 OFFSET(7) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK8 OFFSET(8) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK9 OFFSET(9) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK10 OFFSET(10) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK11 OFFSET(11) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK12 OFFSET(12) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK13 OFFSET(13) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK14 OFFSET(14) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK15 OFFSET(15) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK16 OFFSET(16) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK17 OFFSET(17) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK18 OFFSET(18) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK19 OFFSET(19) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK20 OFFSET(20) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK21 OFFSET(21) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK22 OFFSET(22) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK23 OFFSET(23) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK24 OFFSET(24) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK25 OFFSET(25) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK26 OFFSET(26) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK27 OFFSET(27) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK28 OFFSET(28) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK29 OFFSET(29) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK30 OFFSET(30) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK31 OFFSET(31) NUMBITS(1) []
],
EV7_STATE [
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK0 OFFSET(0) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK1 OFFSET(1) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK2 OFFSET(2) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK3 OFFSET(3) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK4 OFFSET(4) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK5 OFFSET(5) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK6 OFFSET(6) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK7 OFFSET(7) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK8 OFFSET(8) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK9 OFFSET(9) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK10 OFFSET(10) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK11 OFFSET(11) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK12 OFFSET(12) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK13 OFFSET(13) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK14 OFFSET(14) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK15 OFFSET(15) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK16 OFFSET(16) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK17 OFFSET(17) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK18 OFFSET(18) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK19 OFFSET(19) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK20 OFFSET(20) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK21 OFFSET(21) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK22 OFFSET(22) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK23 OFFSET(23) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK24 OFFSET(24) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK25 OFFSET(25) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK26 OFFSET(26) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK27 OFFSET(27) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK28 OFFSET(28) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK29 OFFSET(29) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK30 OFFSET(30) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK31 OFFSET(31) NUMBITS(1) []
],
EV8_STATE [
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK0 OFFSET(0) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK1 OFFSET(1) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK2 OFFSET(2) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK3 OFFSET(3) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK4 OFFSET(4) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK5 OFFSET(5) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK6 OFFSET(6) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK7 OFFSET(7) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK8 OFFSET(8) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK9 OFFSET(9) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK10 OFFSET(10) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK11 OFFSET(11) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK12 OFFSET(12) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK13 OFFSET(13) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK14 OFFSET(14) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK15 OFFSET(15) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK16 OFFSET(16) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK17 OFFSET(17) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK18 OFFSET(18) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK19 OFFSET(19) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK20 OFFSET(20) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK21 OFFSET(21) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK22 OFFSET(22) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK23 OFFSET(23) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK24 OFFSET(24) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK25 OFFSET(25) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK26 OFFSET(26) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK27 OFFSET(27) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK28 OFFSET(28) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK29 OFFSET(29) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK30 OFFSET(30) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK31 OFFSET(31) NUMBITS(1) []
],
EV9_STATE [
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK0 OFFSET(0) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK1 OFFSET(1) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK2 OFFSET(2) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK3 OFFSET(3) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK4 OFFSET(4) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK5 OFFSET(5) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK6 OFFSET(6) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK7 OFFSET(7) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK8 OFFSET(8) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK9 OFFSET(9) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK10 OFFSET(10) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK11 OFFSET(11) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK12 OFFSET(12) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK13 OFFSET(13) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK14 OFFSET(14) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK15 OFFSET(15) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK16 OFFSET(16) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK17 OFFSET(17) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK18 OFFSET(18) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK19 OFFSET(19) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK20 OFFSET(20) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK21 OFFSET(21) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK22 OFFSET(22) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK23 OFFSET(23) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK24 OFFSET(24) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK25 OFFSET(25) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK26 OFFSET(26) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK27 OFFSET(27) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK28 OFFSET(28) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK29 OFFSET(29) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK30 OFFSET(30) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK31 OFFSET(31) NUMBITS(1) []
],
EV10_STATE [
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK0 OFFSET(0) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK1 OFFSET(1) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK2 OFFSET(2) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK3 OFFSET(3) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK4 OFFSET(4) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK5 OFFSET(5) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK6 OFFSET(6) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK7 OFFSET(7) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK8 OFFSET(8) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK9 OFFSET(9) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK10 OFFSET(10) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK11 OFFSET(11) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK12 OFFSET(12) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK13 OFFSET(13) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK14 OFFSET(14) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK15 OFFSET(15) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK16 OFFSET(16) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK17 OFFSET(17) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK18 OFFSET(18) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK19 OFFSET(19) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK20 OFFSET(20) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK21 OFFSET(21) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK22 OFFSET(22) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK23 OFFSET(23) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK24 OFFSET(24) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK25 OFFSET(25) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK26 OFFSET(26) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK27 OFFSET(27) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK28 OFFSET(28) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK29 OFFSET(29) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK30 OFFSET(30) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK31 OFFSET(31) NUMBITS(1) []
],
EV11_STATE [
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK0 OFFSET(0) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK1 OFFSET(1) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK2 OFFSET(2) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK3 OFFSET(3) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK4 OFFSET(4) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK5 OFFSET(5) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK6 OFFSET(6) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK7 OFFSET(7) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK8 OFFSET(8) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK9 OFFSET(9) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK10 OFFSET(10) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK11 OFFSET(11) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK12 OFFSET(12) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK13 OFFSET(13) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK14 OFFSET(14) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK15 OFFSET(15) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK16 OFFSET(16) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK17 OFFSET(17) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK18 OFFSET(18) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK19 OFFSET(19) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK20 OFFSET(20) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK21 OFFSET(21) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK22 OFFSET(22) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK23 OFFSET(23) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK24 OFFSET(24) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK25 OFFSET(25) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK26 OFFSET(26) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK27 OFFSET(27) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK28 OFFSET(28) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK29 OFFSET(29) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK30 OFFSET(30) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK31 OFFSET(31) NUMBITS(1) []
],
EV12_STATE [
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK0 OFFSET(0) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK1 OFFSET(1) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK2 OFFSET(2) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK3 OFFSET(3) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK4 OFFSET(4) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK5 OFFSET(5) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK6 OFFSET(6) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK7 OFFSET(7) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK8 OFFSET(8) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK9 OFFSET(9) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK10 OFFSET(10) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK11 OFFSET(11) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK12 OFFSET(12) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK13 OFFSET(13) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK14 OFFSET(14) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK15 OFFSET(15) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK16 OFFSET(16) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK17 OFFSET(17) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK18 OFFSET(18) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK19 OFFSET(19) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK20 OFFSET(20) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK21 OFFSET(21) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK22 OFFSET(22) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK23 OFFSET(23) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK24 OFFSET(24) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK25 OFFSET(25) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK26 OFFSET(26) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK27 OFFSET(27) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK28 OFFSET(28) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK29 OFFSET(29) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK30 OFFSET(30) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK31 OFFSET(31) NUMBITS(1) []
],
EV13_STATE [
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK0 OFFSET(0) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK1 OFFSET(1) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK2 OFFSET(2) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK3 OFFSET(3) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK4 OFFSET(4) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK5 OFFSET(5) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK6 OFFSET(6) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK7 OFFSET(7) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK8 OFFSET(8) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK9 OFFSET(9) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK10 OFFSET(10) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK11 OFFSET(11) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK12 OFFSET(12) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK13 OFFSET(13) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK14 OFFSET(14) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK15 OFFSET(15) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK16 OFFSET(16) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK17 OFFSET(17) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK18 OFFSET(18) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK19 OFFSET(19) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK20 OFFSET(20) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK21 OFFSET(21) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK22 OFFSET(22) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK23 OFFSET(23) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK24 OFFSET(24) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK25 OFFSET(25) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK26 OFFSET(26) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK27 OFFSET(27) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK28 OFFSET(28) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK29 OFFSET(29) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK30 OFFSET(30) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK31 OFFSET(31) NUMBITS(1) []
],
EV14_STATE [
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK0 OFFSET(0) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK1 OFFSET(1) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK2 OFFSET(2) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK3 OFFSET(3) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK4 OFFSET(4) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK5 OFFSET(5) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK6 OFFSET(6) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK7 OFFSET(7) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK8 OFFSET(8) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK9 OFFSET(9) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK10 OFFSET(10) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK11 OFFSET(11) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK12 OFFSET(12) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK13 OFFSET(13) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK14 OFFSET(14) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK15 OFFSET(15) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK16 OFFSET(16) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK17 OFFSET(17) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK18 OFFSET(18) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK19 OFFSET(19) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK20 OFFSET(20) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK21 OFFSET(21) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK22 OFFSET(22) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK23 OFFSET(23) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK24 OFFSET(24) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK25 OFFSET(25) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK26 OFFSET(26) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK27 OFFSET(27) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK28 OFFSET(28) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK29 OFFSET(29) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK30 OFFSET(30) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK31 OFFSET(31) NUMBITS(1) []
],
EV15_STATE [
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK0 OFFSET(0) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK1 OFFSET(1) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK2 OFFSET(2) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK3 OFFSET(3) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK4 OFFSET(4) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK5 OFFSET(5) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK6 OFFSET(6) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK7 OFFSET(7) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK8 OFFSET(8) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK9 OFFSET(9) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK10 OFFSET(10) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK11 OFFSET(11) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK12 OFFSET(12) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK13 OFFSET(13) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK14 OFFSET(14) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK15 OFFSET(15) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK16 OFFSET(16) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK17 OFFSET(17) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK18 OFFSET(18) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK19 OFFSET(19) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK20 OFFSET(20) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK21 OFFSET(21) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK22 OFFSET(22) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK23 OFFSET(23) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK24 OFFSET(24) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK25 OFFSET(25) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK26 OFFSET(26) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK27 OFFSET(27) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK28 OFFSET(28) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK29 OFFSET(29) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK30 OFFSET(30) NUMBITS(1) [],
    /// If bit m is one, event n (n= 0 to 15) happens in state m of the counter selected
    STATEMSK31 OFFSET(31) NUMBITS(1) []
],
EV0_CTRL [
    /// Selects the Match register associated with this event (if any). A  match can occ
    MATCHSEL OFFSET(0) NUMBITS(4) [],
    /// Select L/H counter. Do not set this bit if UNIFY = 1.
    HEVENT OFFSET(4) NUMBITS(1) [
        /// L state. Selects the L state and the L match register selected by MATCHSEL.
        LStateSelectsTheLStateAndTheLMatchRegisterSelectedByMATCHSEL = 0,
        /// H state. Selects the H state and the H match register selected by MATCHSEL.
        HStateSelectsTheHStateAndTheHMatchRegisterSelectedByMATCHSEL = 1
    ],
    /// Input/output select
    OUTSEL OFFSET(5) NUMBITS(1) [
        /// Input. Selects the input selected by IOSEL.
        InputSelectsTheInputSelectedByIOSEL = 0,
        /// Output. Selects the output selected by IOSEL.
        OutputSelectsTheOutputSelectedByIOSEL = 1
    ],
    /// Selects the input or output signal associated with this event (if  any). Do not
    IOSEL OFFSET(6) NUMBITS(4) [],
    /// Selects the I/O condition for event n. (The detection of edges on outputs lags t
    IOCOND OFFSET(10) NUMBITS(2) [
        /// LOW
        LOW = 0,
        /// Rise
        Rise = 1,
        /// Fall
        Fall = 2,
        /// HIGH
        HIGH = 3
    ],
    /// Selects how the specified match and I/O condition are used and combined.
    COMBMODE OFFSET(12) NUMBITS(2) [
        /// OR. The event occurs when either the specified match or I/O condition occurs.
        ORTheEventOccursWhenEitherTheSpecifiedMatchOrIOConditionOccurs = 0,
        /// MATCH. Uses the specified match only.
        MATCHUsesTheSpecifiedMatchOnly = 1,
        /// IO. Uses the specified I/O condition only.
        IOUsesTheSpecifiedIOConditionOnly = 2,
        /// AND. The event occurs when the specified match and I/O condition occur simultane
        ANDTheEventOccursWhenTheSpecifiedMatchAndIOConditionOccurSimultaneously = 3
    ],
    /// This bit controls how the STATEV value modifies the state  selected by HEVENT wh
    STATELD OFFSET(14) NUMBITS(1) [
        /// STATEV value is added into STATE (the carry-out is ignored).
        STATEVValueIsAddedIntoSTATETheCarryOutIsIgnored = 0,
        /// STATEV value is loaded into STATE.
        STATEVValueIsLoadedIntoSTATE = 1
    ],
    /// This value is loaded into or added to the state selected by  HEVENT, depending o
    STATEV OFFSET(15) NUMBITS(5) [],
    /// If this bit is one and the COMBMODE field specifies a match  component to the tr
    MATCHMEM OFFSET(20) NUMBITS(1) [],
    /// Direction qualifier for event generation. This field only applies when the count
    DIRECTION OFFSET(21) NUMBITS(2) [
        /// Direction independent. This event is triggered regardless of the count direction
        DirectionIndependentThisEventIsTriggeredRegardlessOfTheCountDirection = 0,
        /// Counting up. This event is triggered only during up-counting when BIDIR = 1.
        CountingUpThisEventIsTriggeredOnlyDuringUpCountingWhenBIDIR1 = 1,
        /// Counting down. This event is triggered only during down-counting when BIDIR = 1.
        CountingDownThisEventIsTriggeredOnlyDuringDownCountingWhenBIDIR1 = 2
    ]
],
EV1_CTRL [
    /// Selects the Match register associated with this event (if any). A  match can occ
    MATCHSEL OFFSET(0) NUMBITS(4) [],
    /// Select L/H counter. Do not set this bit if UNIFY = 1.
    HEVENT OFFSET(4) NUMBITS(1) [
        /// L state. Selects the L state and the L match register selected by MATCHSEL.
        LStateSelectsTheLStateAndTheLMatchRegisterSelectedByMATCHSEL = 0,
        /// H state. Selects the H state and the H match register selected by MATCHSEL.
        HStateSelectsTheHStateAndTheHMatchRegisterSelectedByMATCHSEL = 1
    ],
    /// Input/output select
    OUTSEL OFFSET(5) NUMBITS(1) [
        /// Input. Selects the input selected by IOSEL.
        InputSelectsTheInputSelectedByIOSEL = 0,
        /// Output. Selects the output selected by IOSEL.
        OutputSelectsTheOutputSelectedByIOSEL = 1
    ],
    /// Selects the input or output signal associated with this event (if  any). Do not
    IOSEL OFFSET(6) NUMBITS(4) [],
    /// Selects the I/O condition for event n. (The detection of edges on outputs lags t
    IOCOND OFFSET(10) NUMBITS(2) [
        /// LOW
        LOW = 0,
        /// Rise
        Rise = 1,
        /// Fall
        Fall = 2,
        /// HIGH
        HIGH = 3
    ],
    /// Selects how the specified match and I/O condition are used and combined.
    COMBMODE OFFSET(12) NUMBITS(2) [
        /// OR. The event occurs when either the specified match or I/O condition occurs.
        ORTheEventOccursWhenEitherTheSpecifiedMatchOrIOConditionOccurs = 0,
        /// MATCH. Uses the specified match only.
        MATCHUsesTheSpecifiedMatchOnly = 1,
        /// IO. Uses the specified I/O condition only.
        IOUsesTheSpecifiedIOConditionOnly = 2,
        /// AND. The event occurs when the specified match and I/O condition occur simultane
        ANDTheEventOccursWhenTheSpecifiedMatchAndIOConditionOccurSimultaneously = 3
    ],
    /// This bit controls how the STATEV value modifies the state  selected by HEVENT wh
    STATELD OFFSET(14) NUMBITS(1) [
        /// STATEV value is added into STATE (the carry-out is ignored).
        STATEVValueIsAddedIntoSTATETheCarryOutIsIgnored = 0,
        /// STATEV value is loaded into STATE.
        STATEVValueIsLoadedIntoSTATE = 1
    ],
    /// This value is loaded into or added to the state selected by  HEVENT, depending o
    STATEV OFFSET(15) NUMBITS(5) [],
    /// If this bit is one and the COMBMODE field specifies a match  component to the tr
    MATCHMEM OFFSET(20) NUMBITS(1) [],
    /// Direction qualifier for event generation. This field only applies when the count
    DIRECTION OFFSET(21) NUMBITS(2) [
        /// Direction independent. This event is triggered regardless of the count direction
        DirectionIndependentThisEventIsTriggeredRegardlessOfTheCountDirection = 0,
        /// Counting up. This event is triggered only during up-counting when BIDIR = 1.
        CountingUpThisEventIsTriggeredOnlyDuringUpCountingWhenBIDIR1 = 1,
        /// Counting down. This event is triggered only during down-counting when BIDIR = 1.
        CountingDownThisEventIsTriggeredOnlyDuringDownCountingWhenBIDIR1 = 2
    ]
],
EV2_CTRL [
    /// Selects the Match register associated with this event (if any). A  match can occ
    MATCHSEL OFFSET(0) NUMBITS(4) [],
    /// Select L/H counter. Do not set this bit if UNIFY = 1.
    HEVENT OFFSET(4) NUMBITS(1) [
        /// L state. Selects the L state and the L match register selected by MATCHSEL.
        LStateSelectsTheLStateAndTheLMatchRegisterSelectedByMATCHSEL = 0,
        /// H state. Selects the H state and the H match register selected by MATCHSEL.
        HStateSelectsTheHStateAndTheHMatchRegisterSelectedByMATCHSEL = 1
    ],
    /// Input/output select
    OUTSEL OFFSET(5) NUMBITS(1) [
        /// Input. Selects the input selected by IOSEL.
        InputSelectsTheInputSelectedByIOSEL = 0,
        /// Output. Selects the output selected by IOSEL.
        OutputSelectsTheOutputSelectedByIOSEL = 1
    ],
    /// Selects the input or output signal associated with this event (if  any). Do not
    IOSEL OFFSET(6) NUMBITS(4) [],
    /// Selects the I/O condition for event n. (The detection of edges on outputs lags t
    IOCOND OFFSET(10) NUMBITS(2) [
        /// LOW
        LOW = 0,
        /// Rise
        Rise = 1,
        /// Fall
        Fall = 2,
        /// HIGH
        HIGH = 3
    ],
    /// Selects how the specified match and I/O condition are used and combined.
    COMBMODE OFFSET(12) NUMBITS(2) [
        /// OR. The event occurs when either the specified match or I/O condition occurs.
        ORTheEventOccursWhenEitherTheSpecifiedMatchOrIOConditionOccurs = 0,
        /// MATCH. Uses the specified match only.
        MATCHUsesTheSpecifiedMatchOnly = 1,
        /// IO. Uses the specified I/O condition only.
        IOUsesTheSpecifiedIOConditionOnly = 2,
        /// AND. The event occurs when the specified match and I/O condition occur simultane
        ANDTheEventOccursWhenTheSpecifiedMatchAndIOConditionOccurSimultaneously = 3
    ],
    /// This bit controls how the STATEV value modifies the state  selected by HEVENT wh
    STATELD OFFSET(14) NUMBITS(1) [
        /// STATEV value is added into STATE (the carry-out is ignored).
        STATEVValueIsAddedIntoSTATETheCarryOutIsIgnored = 0,
        /// STATEV value is loaded into STATE.
        STATEVValueIsLoadedIntoSTATE = 1
    ],
    /// This value is loaded into or added to the state selected by  HEVENT, depending o
    STATEV OFFSET(15) NUMBITS(5) [],
    /// If this bit is one and the COMBMODE field specifies a match  component to the tr
    MATCHMEM OFFSET(20) NUMBITS(1) [],
    /// Direction qualifier for event generation. This field only applies when the count
    DIRECTION OFFSET(21) NUMBITS(2) [
        /// Direction independent. This event is triggered regardless of the count direction
        DirectionIndependentThisEventIsTriggeredRegardlessOfTheCountDirection = 0,
        /// Counting up. This event is triggered only during up-counting when BIDIR = 1.
        CountingUpThisEventIsTriggeredOnlyDuringUpCountingWhenBIDIR1 = 1,
        /// Counting down. This event is triggered only during down-counting when BIDIR = 1.
        CountingDownThisEventIsTriggeredOnlyDuringDownCountingWhenBIDIR1 = 2
    ]
],
EV3_CTRL [
    /// Selects the Match register associated with this event (if any). A  match can occ
    MATCHSEL OFFSET(0) NUMBITS(4) [],
    /// Select L/H counter. Do not set this bit if UNIFY = 1.
    HEVENT OFFSET(4) NUMBITS(1) [
        /// L state. Selects the L state and the L match register selected by MATCHSEL.
        LStateSelectsTheLStateAndTheLMatchRegisterSelectedByMATCHSEL = 0,
        /// H state. Selects the H state and the H match register selected by MATCHSEL.
        HStateSelectsTheHStateAndTheHMatchRegisterSelectedByMATCHSEL = 1
    ],
    /// Input/output select
    OUTSEL OFFSET(5) NUMBITS(1) [
        /// Input. Selects the input selected by IOSEL.
        InputSelectsTheInputSelectedByIOSEL = 0,
        /// Output. Selects the output selected by IOSEL.
        OutputSelectsTheOutputSelectedByIOSEL = 1
    ],
    /// Selects the input or output signal associated with this event (if  any). Do not
    IOSEL OFFSET(6) NUMBITS(4) [],
    /// Selects the I/O condition for event n. (The detection of edges on outputs lags t
    IOCOND OFFSET(10) NUMBITS(2) [
        /// LOW
        LOW = 0,
        /// Rise
        Rise = 1,
        /// Fall
        Fall = 2,
        /// HIGH
        HIGH = 3
    ],
    /// Selects how the specified match and I/O condition are used and combined.
    COMBMODE OFFSET(12) NUMBITS(2) [
        /// OR. The event occurs when either the specified match or I/O condition occurs.
        ORTheEventOccursWhenEitherTheSpecifiedMatchOrIOConditionOccurs = 0,
        /// MATCH. Uses the specified match only.
        MATCHUsesTheSpecifiedMatchOnly = 1,
        /// IO. Uses the specified I/O condition only.
        IOUsesTheSpecifiedIOConditionOnly = 2,
        /// AND. The event occurs when the specified match and I/O condition occur simultane
        ANDTheEventOccursWhenTheSpecifiedMatchAndIOConditionOccurSimultaneously = 3
    ],
    /// This bit controls how the STATEV value modifies the state  selected by HEVENT wh
    STATELD OFFSET(14) NUMBITS(1) [
        /// STATEV value is added into STATE (the carry-out is ignored).
        STATEVValueIsAddedIntoSTATETheCarryOutIsIgnored = 0,
        /// STATEV value is loaded into STATE.
        STATEVValueIsLoadedIntoSTATE = 1
    ],
    /// This value is loaded into or added to the state selected by  HEVENT, depending o
    STATEV OFFSET(15) NUMBITS(5) [],
    /// If this bit is one and the COMBMODE field specifies a match  component to the tr
    MATCHMEM OFFSET(20) NUMBITS(1) [],
    /// Direction qualifier for event generation. This field only applies when the count
    DIRECTION OFFSET(21) NUMBITS(2) [
        /// Direction independent. This event is triggered regardless of the count direction
        DirectionIndependentThisEventIsTriggeredRegardlessOfTheCountDirection = 0,
        /// Counting up. This event is triggered only during up-counting when BIDIR = 1.
        CountingUpThisEventIsTriggeredOnlyDuringUpCountingWhenBIDIR1 = 1,
        /// Counting down. This event is triggered only during down-counting when BIDIR = 1.
        CountingDownThisEventIsTriggeredOnlyDuringDownCountingWhenBIDIR1 = 2
    ]
],
EV4_CTRL [
    /// Selects the Match register associated with this event (if any). A  match can occ
    MATCHSEL OFFSET(0) NUMBITS(4) [],
    /// Select L/H counter. Do not set this bit if UNIFY = 1.
    HEVENT OFFSET(4) NUMBITS(1) [
        /// L state. Selects the L state and the L match register selected by MATCHSEL.
        LStateSelectsTheLStateAndTheLMatchRegisterSelectedByMATCHSEL = 0,
        /// H state. Selects the H state and the H match register selected by MATCHSEL.
        HStateSelectsTheHStateAndTheHMatchRegisterSelectedByMATCHSEL = 1
    ],
    /// Input/output select
    OUTSEL OFFSET(5) NUMBITS(1) [
        /// Input. Selects the input selected by IOSEL.
        InputSelectsTheInputSelectedByIOSEL = 0,
        /// Output. Selects the output selected by IOSEL.
        OutputSelectsTheOutputSelectedByIOSEL = 1
    ],
    /// Selects the input or output signal associated with this event (if  any). Do not
    IOSEL OFFSET(6) NUMBITS(4) [],
    /// Selects the I/O condition for event n. (The detection of edges on outputs lags t
    IOCOND OFFSET(10) NUMBITS(2) [
        /// LOW
        LOW = 0,
        /// Rise
        Rise = 1,
        /// Fall
        Fall = 2,
        /// HIGH
        HIGH = 3
    ],
    /// Selects how the specified match and I/O condition are used and combined.
    COMBMODE OFFSET(12) NUMBITS(2) [
        /// OR. The event occurs when either the specified match or I/O condition occurs.
        ORTheEventOccursWhenEitherTheSpecifiedMatchOrIOConditionOccurs = 0,
        /// MATCH. Uses the specified match only.
        MATCHUsesTheSpecifiedMatchOnly = 1,
        /// IO. Uses the specified I/O condition only.
        IOUsesTheSpecifiedIOConditionOnly = 2,
        /// AND. The event occurs when the specified match and I/O condition occur simultane
        ANDTheEventOccursWhenTheSpecifiedMatchAndIOConditionOccurSimultaneously = 3
    ],
    /// This bit controls how the STATEV value modifies the state  selected by HEVENT wh
    STATELD OFFSET(14) NUMBITS(1) [
        /// STATEV value is added into STATE (the carry-out is ignored).
        STATEVValueIsAddedIntoSTATETheCarryOutIsIgnored = 0,
        /// STATEV value is loaded into STATE.
        STATEVValueIsLoadedIntoSTATE = 1
    ],
    /// This value is loaded into or added to the state selected by  HEVENT, depending o
    STATEV OFFSET(15) NUMBITS(5) [],
    /// If this bit is one and the COMBMODE field specifies a match  component to the tr
    MATCHMEM OFFSET(20) NUMBITS(1) [],
    /// Direction qualifier for event generation. This field only applies when the count
    DIRECTION OFFSET(21) NUMBITS(2) [
        /// Direction independent. This event is triggered regardless of the count direction
        DirectionIndependentThisEventIsTriggeredRegardlessOfTheCountDirection = 0,
        /// Counting up. This event is triggered only during up-counting when BIDIR = 1.
        CountingUpThisEventIsTriggeredOnlyDuringUpCountingWhenBIDIR1 = 1,
        /// Counting down. This event is triggered only during down-counting when BIDIR = 1.
        CountingDownThisEventIsTriggeredOnlyDuringDownCountingWhenBIDIR1 = 2
    ]
],
EV5_CTRL [
    /// Selects the Match register associated with this event (if any). A  match can occ
    MATCHSEL OFFSET(0) NUMBITS(4) [],
    /// Select L/H counter. Do not set this bit if UNIFY = 1.
    HEVENT OFFSET(4) NUMBITS(1) [
        /// L state. Selects the L state and the L match register selected by MATCHSEL.
        LStateSelectsTheLStateAndTheLMatchRegisterSelectedByMATCHSEL = 0,
        /// H state. Selects the H state and the H match register selected by MATCHSEL.
        HStateSelectsTheHStateAndTheHMatchRegisterSelectedByMATCHSEL = 1
    ],
    /// Input/output select
    OUTSEL OFFSET(5) NUMBITS(1) [
        /// Input. Selects the input selected by IOSEL.
        InputSelectsTheInputSelectedByIOSEL = 0,
        /// Output. Selects the output selected by IOSEL.
        OutputSelectsTheOutputSelectedByIOSEL = 1
    ],
    /// Selects the input or output signal associated with this event (if  any). Do not
    IOSEL OFFSET(6) NUMBITS(4) [],
    /// Selects the I/O condition for event n. (The detection of edges on outputs lags t
    IOCOND OFFSET(10) NUMBITS(2) [
        /// LOW
        LOW = 0,
        /// Rise
        Rise = 1,
        /// Fall
        Fall = 2,
        /// HIGH
        HIGH = 3
    ],
    /// Selects how the specified match and I/O condition are used and combined.
    COMBMODE OFFSET(12) NUMBITS(2) [
        /// OR. The event occurs when either the specified match or I/O condition occurs.
        ORTheEventOccursWhenEitherTheSpecifiedMatchOrIOConditionOccurs = 0,
        /// MATCH. Uses the specified match only.
        MATCHUsesTheSpecifiedMatchOnly = 1,
        /// IO. Uses the specified I/O condition only.
        IOUsesTheSpecifiedIOConditionOnly = 2,
        /// AND. The event occurs when the specified match and I/O condition occur simultane
        ANDTheEventOccursWhenTheSpecifiedMatchAndIOConditionOccurSimultaneously = 3
    ],
    /// This bit controls how the STATEV value modifies the state  selected by HEVENT wh
    STATELD OFFSET(14) NUMBITS(1) [
        /// STATEV value is added into STATE (the carry-out is ignored).
        STATEVValueIsAddedIntoSTATETheCarryOutIsIgnored = 0,
        /// STATEV value is loaded into STATE.
        STATEVValueIsLoadedIntoSTATE = 1
    ],
    /// This value is loaded into or added to the state selected by  HEVENT, depending o
    STATEV OFFSET(15) NUMBITS(5) [],
    /// If this bit is one and the COMBMODE field specifies a match  component to the tr
    MATCHMEM OFFSET(20) NUMBITS(1) [],
    /// Direction qualifier for event generation. This field only applies when the count
    DIRECTION OFFSET(21) NUMBITS(2) [
        /// Direction independent. This event is triggered regardless of the count direction
        DirectionIndependentThisEventIsTriggeredRegardlessOfTheCountDirection = 0,
        /// Counting up. This event is triggered only during up-counting when BIDIR = 1.
        CountingUpThisEventIsTriggeredOnlyDuringUpCountingWhenBIDIR1 = 1,
        /// Counting down. This event is triggered only during down-counting when BIDIR = 1.
        CountingDownThisEventIsTriggeredOnlyDuringDownCountingWhenBIDIR1 = 2
    ]
],
EV6_CTRL [
    /// Selects the Match register associated with this event (if any). A  match can occ
    MATCHSEL OFFSET(0) NUMBITS(4) [],
    /// Select L/H counter. Do not set this bit if UNIFY = 1.
    HEVENT OFFSET(4) NUMBITS(1) [
        /// L state. Selects the L state and the L match register selected by MATCHSEL.
        LStateSelectsTheLStateAndTheLMatchRegisterSelectedByMATCHSEL = 0,
        /// H state. Selects the H state and the H match register selected by MATCHSEL.
        HStateSelectsTheHStateAndTheHMatchRegisterSelectedByMATCHSEL = 1
    ],
    /// Input/output select
    OUTSEL OFFSET(5) NUMBITS(1) [
        /// Input. Selects the input selected by IOSEL.
        InputSelectsTheInputSelectedByIOSEL = 0,
        /// Output. Selects the output selected by IOSEL.
        OutputSelectsTheOutputSelectedByIOSEL = 1
    ],
    /// Selects the input or output signal associated with this event (if  any). Do not
    IOSEL OFFSET(6) NUMBITS(4) [],
    /// Selects the I/O condition for event n. (The detection of edges on outputs lags t
    IOCOND OFFSET(10) NUMBITS(2) [
        /// LOW
        LOW = 0,
        /// Rise
        Rise = 1,
        /// Fall
        Fall = 2,
        /// HIGH
        HIGH = 3
    ],
    /// Selects how the specified match and I/O condition are used and combined.
    COMBMODE OFFSET(12) NUMBITS(2) [
        /// OR. The event occurs when either the specified match or I/O condition occurs.
        ORTheEventOccursWhenEitherTheSpecifiedMatchOrIOConditionOccurs = 0,
        /// MATCH. Uses the specified match only.
        MATCHUsesTheSpecifiedMatchOnly = 1,
        /// IO. Uses the specified I/O condition only.
        IOUsesTheSpecifiedIOConditionOnly = 2,
        /// AND. The event occurs when the specified match and I/O condition occur simultane
        ANDTheEventOccursWhenTheSpecifiedMatchAndIOConditionOccurSimultaneously = 3
    ],
    /// This bit controls how the STATEV value modifies the state  selected by HEVENT wh
    STATELD OFFSET(14) NUMBITS(1) [
        /// STATEV value is added into STATE (the carry-out is ignored).
        STATEVValueIsAddedIntoSTATETheCarryOutIsIgnored = 0,
        /// STATEV value is loaded into STATE.
        STATEVValueIsLoadedIntoSTATE = 1
    ],
    /// This value is loaded into or added to the state selected by  HEVENT, depending o
    STATEV OFFSET(15) NUMBITS(5) [],
    /// If this bit is one and the COMBMODE field specifies a match  component to the tr
    MATCHMEM OFFSET(20) NUMBITS(1) [],
    /// Direction qualifier for event generation. This field only applies when the count
    DIRECTION OFFSET(21) NUMBITS(2) [
        /// Direction independent. This event is triggered regardless of the count direction
        DirectionIndependentThisEventIsTriggeredRegardlessOfTheCountDirection = 0,
        /// Counting up. This event is triggered only during up-counting when BIDIR = 1.
        CountingUpThisEventIsTriggeredOnlyDuringUpCountingWhenBIDIR1 = 1,
        /// Counting down. This event is triggered only during down-counting when BIDIR = 1.
        CountingDownThisEventIsTriggeredOnlyDuringDownCountingWhenBIDIR1 = 2
    ]
],
EV7_CTRL [
    /// Selects the Match register associated with this event (if any). A  match can occ
    MATCHSEL OFFSET(0) NUMBITS(4) [],
    /// Select L/H counter. Do not set this bit if UNIFY = 1.
    HEVENT OFFSET(4) NUMBITS(1) [
        /// L state. Selects the L state and the L match register selected by MATCHSEL.
        LStateSelectsTheLStateAndTheLMatchRegisterSelectedByMATCHSEL = 0,
        /// H state. Selects the H state and the H match register selected by MATCHSEL.
        HStateSelectsTheHStateAndTheHMatchRegisterSelectedByMATCHSEL = 1
    ],
    /// Input/output select
    OUTSEL OFFSET(5) NUMBITS(1) [
        /// Input. Selects the input selected by IOSEL.
        InputSelectsTheInputSelectedByIOSEL = 0,
        /// Output. Selects the output selected by IOSEL.
        OutputSelectsTheOutputSelectedByIOSEL = 1
    ],
    /// Selects the input or output signal associated with this event (if  any). Do not
    IOSEL OFFSET(6) NUMBITS(4) [],
    /// Selects the I/O condition for event n. (The detection of edges on outputs lags t
    IOCOND OFFSET(10) NUMBITS(2) [
        /// LOW
        LOW = 0,
        /// Rise
        Rise = 1,
        /// Fall
        Fall = 2,
        /// HIGH
        HIGH = 3
    ],
    /// Selects how the specified match and I/O condition are used and combined.
    COMBMODE OFFSET(12) NUMBITS(2) [
        /// OR. The event occurs when either the specified match or I/O condition occurs.
        ORTheEventOccursWhenEitherTheSpecifiedMatchOrIOConditionOccurs = 0,
        /// MATCH. Uses the specified match only.
        MATCHUsesTheSpecifiedMatchOnly = 1,
        /// IO. Uses the specified I/O condition only.
        IOUsesTheSpecifiedIOConditionOnly = 2,
        /// AND. The event occurs when the specified match and I/O condition occur simultane
        ANDTheEventOccursWhenTheSpecifiedMatchAndIOConditionOccurSimultaneously = 3
    ],
    /// This bit controls how the STATEV value modifies the state  selected by HEVENT wh
    STATELD OFFSET(14) NUMBITS(1) [
        /// STATEV value is added into STATE (the carry-out is ignored).
        STATEVValueIsAddedIntoSTATETheCarryOutIsIgnored = 0,
        /// STATEV value is loaded into STATE.
        STATEVValueIsLoadedIntoSTATE = 1
    ],
    /// This value is loaded into or added to the state selected by  HEVENT, depending o
    STATEV OFFSET(15) NUMBITS(5) [],
    /// If this bit is one and the COMBMODE field specifies a match  component to the tr
    MATCHMEM OFFSET(20) NUMBITS(1) [],
    /// Direction qualifier for event generation. This field only applies when the count
    DIRECTION OFFSET(21) NUMBITS(2) [
        /// Direction independent. This event is triggered regardless of the count direction
        DirectionIndependentThisEventIsTriggeredRegardlessOfTheCountDirection = 0,
        /// Counting up. This event is triggered only during up-counting when BIDIR = 1.
        CountingUpThisEventIsTriggeredOnlyDuringUpCountingWhenBIDIR1 = 1,
        /// Counting down. This event is triggered only during down-counting when BIDIR = 1.
        CountingDownThisEventIsTriggeredOnlyDuringDownCountingWhenBIDIR1 = 2
    ]
],
EV8_CTRL [
    /// Selects the Match register associated with this event (if any). A  match can occ
    MATCHSEL OFFSET(0) NUMBITS(4) [],
    /// Select L/H counter. Do not set this bit if UNIFY = 1.
    HEVENT OFFSET(4) NUMBITS(1) [
        /// L state. Selects the L state and the L match register selected by MATCHSEL.
        LStateSelectsTheLStateAndTheLMatchRegisterSelectedByMATCHSEL = 0,
        /// H state. Selects the H state and the H match register selected by MATCHSEL.
        HStateSelectsTheHStateAndTheHMatchRegisterSelectedByMATCHSEL = 1
    ],
    /// Input/output select
    OUTSEL OFFSET(5) NUMBITS(1) [
        /// Input. Selects the input selected by IOSEL.
        InputSelectsTheInputSelectedByIOSEL = 0,
        /// Output. Selects the output selected by IOSEL.
        OutputSelectsTheOutputSelectedByIOSEL = 1
    ],
    /// Selects the input or output signal associated with this event (if  any). Do not
    IOSEL OFFSET(6) NUMBITS(4) [],
    /// Selects the I/O condition for event n. (The detection of edges on outputs lags t
    IOCOND OFFSET(10) NUMBITS(2) [
        /// LOW
        LOW = 0,
        /// Rise
        Rise = 1,
        /// Fall
        Fall = 2,
        /// HIGH
        HIGH = 3
    ],
    /// Selects how the specified match and I/O condition are used and combined.
    COMBMODE OFFSET(12) NUMBITS(2) [
        /// OR. The event occurs when either the specified match or I/O condition occurs.
        ORTheEventOccursWhenEitherTheSpecifiedMatchOrIOConditionOccurs = 0,
        /// MATCH. Uses the specified match only.
        MATCHUsesTheSpecifiedMatchOnly = 1,
        /// IO. Uses the specified I/O condition only.
        IOUsesTheSpecifiedIOConditionOnly = 2,
        /// AND. The event occurs when the specified match and I/O condition occur simultane
        ANDTheEventOccursWhenTheSpecifiedMatchAndIOConditionOccurSimultaneously = 3
    ],
    /// This bit controls how the STATEV value modifies the state  selected by HEVENT wh
    STATELD OFFSET(14) NUMBITS(1) [
        /// STATEV value is added into STATE (the carry-out is ignored).
        STATEVValueIsAddedIntoSTATETheCarryOutIsIgnored = 0,
        /// STATEV value is loaded into STATE.
        STATEVValueIsLoadedIntoSTATE = 1
    ],
    /// This value is loaded into or added to the state selected by  HEVENT, depending o
    STATEV OFFSET(15) NUMBITS(5) [],
    /// If this bit is one and the COMBMODE field specifies a match  component to the tr
    MATCHMEM OFFSET(20) NUMBITS(1) [],
    /// Direction qualifier for event generation. This field only applies when the count
    DIRECTION OFFSET(21) NUMBITS(2) [
        /// Direction independent. This event is triggered regardless of the count direction
        DirectionIndependentThisEventIsTriggeredRegardlessOfTheCountDirection = 0,
        /// Counting up. This event is triggered only during up-counting when BIDIR = 1.
        CountingUpThisEventIsTriggeredOnlyDuringUpCountingWhenBIDIR1 = 1,
        /// Counting down. This event is triggered only during down-counting when BIDIR = 1.
        CountingDownThisEventIsTriggeredOnlyDuringDownCountingWhenBIDIR1 = 2
    ]
],
EV9_CTRL [
    /// Selects the Match register associated with this event (if any). A  match can occ
    MATCHSEL OFFSET(0) NUMBITS(4) [],
    /// Select L/H counter. Do not set this bit if UNIFY = 1.
    HEVENT OFFSET(4) NUMBITS(1) [
        /// L state. Selects the L state and the L match register selected by MATCHSEL.
        LStateSelectsTheLStateAndTheLMatchRegisterSelectedByMATCHSEL = 0,
        /// H state. Selects the H state and the H match register selected by MATCHSEL.
        HStateSelectsTheHStateAndTheHMatchRegisterSelectedByMATCHSEL = 1
    ],
    /// Input/output select
    OUTSEL OFFSET(5) NUMBITS(1) [
        /// Input. Selects the input selected by IOSEL.
        InputSelectsTheInputSelectedByIOSEL = 0,
        /// Output. Selects the output selected by IOSEL.
        OutputSelectsTheOutputSelectedByIOSEL = 1
    ],
    /// Selects the input or output signal associated with this event (if  any). Do not
    IOSEL OFFSET(6) NUMBITS(4) [],
    /// Selects the I/O condition for event n. (The detection of edges on outputs lags t
    IOCOND OFFSET(10) NUMBITS(2) [
        /// LOW
        LOW = 0,
        /// Rise
        Rise = 1,
        /// Fall
        Fall = 2,
        /// HIGH
        HIGH = 3
    ],
    /// Selects how the specified match and I/O condition are used and combined.
    COMBMODE OFFSET(12) NUMBITS(2) [
        /// OR. The event occurs when either the specified match or I/O condition occurs.
        ORTheEventOccursWhenEitherTheSpecifiedMatchOrIOConditionOccurs = 0,
        /// MATCH. Uses the specified match only.
        MATCHUsesTheSpecifiedMatchOnly = 1,
        /// IO. Uses the specified I/O condition only.
        IOUsesTheSpecifiedIOConditionOnly = 2,
        /// AND. The event occurs when the specified match and I/O condition occur simultane
        ANDTheEventOccursWhenTheSpecifiedMatchAndIOConditionOccurSimultaneously = 3
    ],
    /// This bit controls how the STATEV value modifies the state  selected by HEVENT wh
    STATELD OFFSET(14) NUMBITS(1) [
        /// STATEV value is added into STATE (the carry-out is ignored).
        STATEVValueIsAddedIntoSTATETheCarryOutIsIgnored = 0,
        /// STATEV value is loaded into STATE.
        STATEVValueIsLoadedIntoSTATE = 1
    ],
    /// This value is loaded into or added to the state selected by  HEVENT, depending o
    STATEV OFFSET(15) NUMBITS(5) [],
    /// If this bit is one and the COMBMODE field specifies a match  component to the tr
    MATCHMEM OFFSET(20) NUMBITS(1) [],
    /// Direction qualifier for event generation. This field only applies when the count
    DIRECTION OFFSET(21) NUMBITS(2) [
        /// Direction independent. This event is triggered regardless of the count direction
        DirectionIndependentThisEventIsTriggeredRegardlessOfTheCountDirection = 0,
        /// Counting up. This event is triggered only during up-counting when BIDIR = 1.
        CountingUpThisEventIsTriggeredOnlyDuringUpCountingWhenBIDIR1 = 1,
        /// Counting down. This event is triggered only during down-counting when BIDIR = 1.
        CountingDownThisEventIsTriggeredOnlyDuringDownCountingWhenBIDIR1 = 2
    ]
],
EV10_CTRL [
    /// Selects the Match register associated with this event (if any). A  match can occ
    MATCHSEL OFFSET(0) NUMBITS(4) [],
    /// Select L/H counter. Do not set this bit if UNIFY = 1.
    HEVENT OFFSET(4) NUMBITS(1) [
        /// L state. Selects the L state and the L match register selected by MATCHSEL.
        LStateSelectsTheLStateAndTheLMatchRegisterSelectedByMATCHSEL = 0,
        /// H state. Selects the H state and the H match register selected by MATCHSEL.
        HStateSelectsTheHStateAndTheHMatchRegisterSelectedByMATCHSEL = 1
    ],
    /// Input/output select
    OUTSEL OFFSET(5) NUMBITS(1) [
        /// Input. Selects the input selected by IOSEL.
        InputSelectsTheInputSelectedByIOSEL = 0,
        /// Output. Selects the output selected by IOSEL.
        OutputSelectsTheOutputSelectedByIOSEL = 1
    ],
    /// Selects the input or output signal associated with this event (if  any). Do not
    IOSEL OFFSET(6) NUMBITS(4) [],
    /// Selects the I/O condition for event n. (The detection of edges on outputs lags t
    IOCOND OFFSET(10) NUMBITS(2) [
        /// LOW
        LOW = 0,
        /// Rise
        Rise = 1,
        /// Fall
        Fall = 2,
        /// HIGH
        HIGH = 3
    ],
    /// Selects how the specified match and I/O condition are used and combined.
    COMBMODE OFFSET(12) NUMBITS(2) [
        /// OR. The event occurs when either the specified match or I/O condition occurs.
        ORTheEventOccursWhenEitherTheSpecifiedMatchOrIOConditionOccurs = 0,
        /// MATCH. Uses the specified match only.
        MATCHUsesTheSpecifiedMatchOnly = 1,
        /// IO. Uses the specified I/O condition only.
        IOUsesTheSpecifiedIOConditionOnly = 2,
        /// AND. The event occurs when the specified match and I/O condition occur simultane
        ANDTheEventOccursWhenTheSpecifiedMatchAndIOConditionOccurSimultaneously = 3
    ],
    /// This bit controls how the STATEV value modifies the state  selected by HEVENT wh
    STATELD OFFSET(14) NUMBITS(1) [
        /// STATEV value is added into STATE (the carry-out is ignored).
        STATEVValueIsAddedIntoSTATETheCarryOutIsIgnored = 0,
        /// STATEV value is loaded into STATE.
        STATEVValueIsLoadedIntoSTATE = 1
    ],
    /// This value is loaded into or added to the state selected by  HEVENT, depending o
    STATEV OFFSET(15) NUMBITS(5) [],
    /// If this bit is one and the COMBMODE field specifies a match  component to the tr
    MATCHMEM OFFSET(20) NUMBITS(1) [],
    /// Direction qualifier for event generation. This field only applies when the count
    DIRECTION OFFSET(21) NUMBITS(2) [
        /// Direction independent. This event is triggered regardless of the count direction
        DirectionIndependentThisEventIsTriggeredRegardlessOfTheCountDirection = 0,
        /// Counting up. This event is triggered only during up-counting when BIDIR = 1.
        CountingUpThisEventIsTriggeredOnlyDuringUpCountingWhenBIDIR1 = 1,
        /// Counting down. This event is triggered only during down-counting when BIDIR = 1.
        CountingDownThisEventIsTriggeredOnlyDuringDownCountingWhenBIDIR1 = 2
    ]
],
EV11_CTRL [
    /// Selects the Match register associated with this event (if any). A  match can occ
    MATCHSEL OFFSET(0) NUMBITS(4) [],
    /// Select L/H counter. Do not set this bit if UNIFY = 1.
    HEVENT OFFSET(4) NUMBITS(1) [
        /// L state. Selects the L state and the L match register selected by MATCHSEL.
        LStateSelectsTheLStateAndTheLMatchRegisterSelectedByMATCHSEL = 0,
        /// H state. Selects the H state and the H match register selected by MATCHSEL.
        HStateSelectsTheHStateAndTheHMatchRegisterSelectedByMATCHSEL = 1
    ],
    /// Input/output select
    OUTSEL OFFSET(5) NUMBITS(1) [
        /// Input. Selects the input selected by IOSEL.
        InputSelectsTheInputSelectedByIOSEL = 0,
        /// Output. Selects the output selected by IOSEL.
        OutputSelectsTheOutputSelectedByIOSEL = 1
    ],
    /// Selects the input or output signal associated with this event (if  any). Do not
    IOSEL OFFSET(6) NUMBITS(4) [],
    /// Selects the I/O condition for event n. (The detection of edges on outputs lags t
    IOCOND OFFSET(10) NUMBITS(2) [
        /// LOW
        LOW = 0,
        /// Rise
        Rise = 1,
        /// Fall
        Fall = 2,
        /// HIGH
        HIGH = 3
    ],
    /// Selects how the specified match and I/O condition are used and combined.
    COMBMODE OFFSET(12) NUMBITS(2) [
        /// OR. The event occurs when either the specified match or I/O condition occurs.
        ORTheEventOccursWhenEitherTheSpecifiedMatchOrIOConditionOccurs = 0,
        /// MATCH. Uses the specified match only.
        MATCHUsesTheSpecifiedMatchOnly = 1,
        /// IO. Uses the specified I/O condition only.
        IOUsesTheSpecifiedIOConditionOnly = 2,
        /// AND. The event occurs when the specified match and I/O condition occur simultane
        ANDTheEventOccursWhenTheSpecifiedMatchAndIOConditionOccurSimultaneously = 3
    ],
    /// This bit controls how the STATEV value modifies the state  selected by HEVENT wh
    STATELD OFFSET(14) NUMBITS(1) [
        /// STATEV value is added into STATE (the carry-out is ignored).
        STATEVValueIsAddedIntoSTATETheCarryOutIsIgnored = 0,
        /// STATEV value is loaded into STATE.
        STATEVValueIsLoadedIntoSTATE = 1
    ],
    /// This value is loaded into or added to the state selected by  HEVENT, depending o
    STATEV OFFSET(15) NUMBITS(5) [],
    /// If this bit is one and the COMBMODE field specifies a match  component to the tr
    MATCHMEM OFFSET(20) NUMBITS(1) [],
    /// Direction qualifier for event generation. This field only applies when the count
    DIRECTION OFFSET(21) NUMBITS(2) [
        /// Direction independent. This event is triggered regardless of the count direction
        DirectionIndependentThisEventIsTriggeredRegardlessOfTheCountDirection = 0,
        /// Counting up. This event is triggered only during up-counting when BIDIR = 1.
        CountingUpThisEventIsTriggeredOnlyDuringUpCountingWhenBIDIR1 = 1,
        /// Counting down. This event is triggered only during down-counting when BIDIR = 1.
        CountingDownThisEventIsTriggeredOnlyDuringDownCountingWhenBIDIR1 = 2
    ]
],
EV12_CTRL [
    /// Selects the Match register associated with this event (if any). A  match can occ
    MATCHSEL OFFSET(0) NUMBITS(4) [],
    /// Select L/H counter. Do not set this bit if UNIFY = 1.
    HEVENT OFFSET(4) NUMBITS(1) [
        /// L state. Selects the L state and the L match register selected by MATCHSEL.
        LStateSelectsTheLStateAndTheLMatchRegisterSelectedByMATCHSEL = 0,
        /// H state. Selects the H state and the H match register selected by MATCHSEL.
        HStateSelectsTheHStateAndTheHMatchRegisterSelectedByMATCHSEL = 1
    ],
    /// Input/output select
    OUTSEL OFFSET(5) NUMBITS(1) [
        /// Input. Selects the input selected by IOSEL.
        InputSelectsTheInputSelectedByIOSEL = 0,
        /// Output. Selects the output selected by IOSEL.
        OutputSelectsTheOutputSelectedByIOSEL = 1
    ],
    /// Selects the input or output signal associated with this event (if  any). Do not
    IOSEL OFFSET(6) NUMBITS(4) [],
    /// Selects the I/O condition for event n. (The detection of edges on outputs lags t
    IOCOND OFFSET(10) NUMBITS(2) [
        /// LOW
        LOW = 0,
        /// Rise
        Rise = 1,
        /// Fall
        Fall = 2,
        /// HIGH
        HIGH = 3
    ],
    /// Selects how the specified match and I/O condition are used and combined.
    COMBMODE OFFSET(12) NUMBITS(2) [
        /// OR. The event occurs when either the specified match or I/O condition occurs.
        ORTheEventOccursWhenEitherTheSpecifiedMatchOrIOConditionOccurs = 0,
        /// MATCH. Uses the specified match only.
        MATCHUsesTheSpecifiedMatchOnly = 1,
        /// IO. Uses the specified I/O condition only.
        IOUsesTheSpecifiedIOConditionOnly = 2,
        /// AND. The event occurs when the specified match and I/O condition occur simultane
        ANDTheEventOccursWhenTheSpecifiedMatchAndIOConditionOccurSimultaneously = 3
    ],
    /// This bit controls how the STATEV value modifies the state  selected by HEVENT wh
    STATELD OFFSET(14) NUMBITS(1) [
        /// STATEV value is added into STATE (the carry-out is ignored).
        STATEVValueIsAddedIntoSTATETheCarryOutIsIgnored = 0,
        /// STATEV value is loaded into STATE.
        STATEVValueIsLoadedIntoSTATE = 1
    ],
    /// This value is loaded into or added to the state selected by  HEVENT, depending o
    STATEV OFFSET(15) NUMBITS(5) [],
    /// If this bit is one and the COMBMODE field specifies a match  component to the tr
    MATCHMEM OFFSET(20) NUMBITS(1) [],
    /// Direction qualifier for event generation. This field only applies when the count
    DIRECTION OFFSET(21) NUMBITS(2) [
        /// Direction independent. This event is triggered regardless of the count direction
        DirectionIndependentThisEventIsTriggeredRegardlessOfTheCountDirection = 0,
        /// Counting up. This event is triggered only during up-counting when BIDIR = 1.
        CountingUpThisEventIsTriggeredOnlyDuringUpCountingWhenBIDIR1 = 1,
        /// Counting down. This event is triggered only during down-counting when BIDIR = 1.
        CountingDownThisEventIsTriggeredOnlyDuringDownCountingWhenBIDIR1 = 2
    ]
],
EV13_CTRL [
    /// Selects the Match register associated with this event (if any). A  match can occ
    MATCHSEL OFFSET(0) NUMBITS(4) [],
    /// Select L/H counter. Do not set this bit if UNIFY = 1.
    HEVENT OFFSET(4) NUMBITS(1) [
        /// L state. Selects the L state and the L match register selected by MATCHSEL.
        LStateSelectsTheLStateAndTheLMatchRegisterSelectedByMATCHSEL = 0,
        /// H state. Selects the H state and the H match register selected by MATCHSEL.
        HStateSelectsTheHStateAndTheHMatchRegisterSelectedByMATCHSEL = 1
    ],
    /// Input/output select
    OUTSEL OFFSET(5) NUMBITS(1) [
        /// Input. Selects the input selected by IOSEL.
        InputSelectsTheInputSelectedByIOSEL = 0,
        /// Output. Selects the output selected by IOSEL.
        OutputSelectsTheOutputSelectedByIOSEL = 1
    ],
    /// Selects the input or output signal associated with this event (if  any). Do not
    IOSEL OFFSET(6) NUMBITS(4) [],
    /// Selects the I/O condition for event n. (The detection of edges on outputs lags t
    IOCOND OFFSET(10) NUMBITS(2) [
        /// LOW
        LOW = 0,
        /// Rise
        Rise = 1,
        /// Fall
        Fall = 2,
        /// HIGH
        HIGH = 3
    ],
    /// Selects how the specified match and I/O condition are used and combined.
    COMBMODE OFFSET(12) NUMBITS(2) [
        /// OR. The event occurs when either the specified match or I/O condition occurs.
        ORTheEventOccursWhenEitherTheSpecifiedMatchOrIOConditionOccurs = 0,
        /// MATCH. Uses the specified match only.
        MATCHUsesTheSpecifiedMatchOnly = 1,
        /// IO. Uses the specified I/O condition only.
        IOUsesTheSpecifiedIOConditionOnly = 2,
        /// AND. The event occurs when the specified match and I/O condition occur simultane
        ANDTheEventOccursWhenTheSpecifiedMatchAndIOConditionOccurSimultaneously = 3
    ],
    /// This bit controls how the STATEV value modifies the state  selected by HEVENT wh
    STATELD OFFSET(14) NUMBITS(1) [
        /// STATEV value is added into STATE (the carry-out is ignored).
        STATEVValueIsAddedIntoSTATETheCarryOutIsIgnored = 0,
        /// STATEV value is loaded into STATE.
        STATEVValueIsLoadedIntoSTATE = 1
    ],
    /// This value is loaded into or added to the state selected by  HEVENT, depending o
    STATEV OFFSET(15) NUMBITS(5) [],
    /// If this bit is one and the COMBMODE field specifies a match  component to the tr
    MATCHMEM OFFSET(20) NUMBITS(1) [],
    /// Direction qualifier for event generation. This field only applies when the count
    DIRECTION OFFSET(21) NUMBITS(2) [
        /// Direction independent. This event is triggered regardless of the count direction
        DirectionIndependentThisEventIsTriggeredRegardlessOfTheCountDirection = 0,
        /// Counting up. This event is triggered only during up-counting when BIDIR = 1.
        CountingUpThisEventIsTriggeredOnlyDuringUpCountingWhenBIDIR1 = 1,
        /// Counting down. This event is triggered only during down-counting when BIDIR = 1.
        CountingDownThisEventIsTriggeredOnlyDuringDownCountingWhenBIDIR1 = 2
    ]
],
EV14_CTRL [
    /// Selects the Match register associated with this event (if any). A  match can occ
    MATCHSEL OFFSET(0) NUMBITS(4) [],
    /// Select L/H counter. Do not set this bit if UNIFY = 1.
    HEVENT OFFSET(4) NUMBITS(1) [
        /// L state. Selects the L state and the L match register selected by MATCHSEL.
        LStateSelectsTheLStateAndTheLMatchRegisterSelectedByMATCHSEL = 0,
        /// H state. Selects the H state and the H match register selected by MATCHSEL.
        HStateSelectsTheHStateAndTheHMatchRegisterSelectedByMATCHSEL = 1
    ],
    /// Input/output select
    OUTSEL OFFSET(5) NUMBITS(1) [
        /// Input. Selects the input selected by IOSEL.
        InputSelectsTheInputSelectedByIOSEL = 0,
        /// Output. Selects the output selected by IOSEL.
        OutputSelectsTheOutputSelectedByIOSEL = 1
    ],
    /// Selects the input or output signal associated with this event (if  any). Do not
    IOSEL OFFSET(6) NUMBITS(4) [],
    /// Selects the I/O condition for event n. (The detection of edges on outputs lags t
    IOCOND OFFSET(10) NUMBITS(2) [
        /// LOW
        LOW = 0,
        /// Rise
        Rise = 1,
        /// Fall
        Fall = 2,
        /// HIGH
        HIGH = 3
    ],
    /// Selects how the specified match and I/O condition are used and combined.
    COMBMODE OFFSET(12) NUMBITS(2) [
        /// OR. The event occurs when either the specified match or I/O condition occurs.
        ORTheEventOccursWhenEitherTheSpecifiedMatchOrIOConditionOccurs = 0,
        /// MATCH. Uses the specified match only.
        MATCHUsesTheSpecifiedMatchOnly = 1,
        /// IO. Uses the specified I/O condition only.
        IOUsesTheSpecifiedIOConditionOnly = 2,
        /// AND. The event occurs when the specified match and I/O condition occur simultane
        ANDTheEventOccursWhenTheSpecifiedMatchAndIOConditionOccurSimultaneously = 3
    ],
    /// This bit controls how the STATEV value modifies the state  selected by HEVENT wh
    STATELD OFFSET(14) NUMBITS(1) [
        /// STATEV value is added into STATE (the carry-out is ignored).
        STATEVValueIsAddedIntoSTATETheCarryOutIsIgnored = 0,
        /// STATEV value is loaded into STATE.
        STATEVValueIsLoadedIntoSTATE = 1
    ],
    /// This value is loaded into or added to the state selected by  HEVENT, depending o
    STATEV OFFSET(15) NUMBITS(5) [],
    /// If this bit is one and the COMBMODE field specifies a match  component to the tr
    MATCHMEM OFFSET(20) NUMBITS(1) [],
    /// Direction qualifier for event generation. This field only applies when the count
    DIRECTION OFFSET(21) NUMBITS(2) [
        /// Direction independent. This event is triggered regardless of the count direction
        DirectionIndependentThisEventIsTriggeredRegardlessOfTheCountDirection = 0,
        /// Counting up. This event is triggered only during up-counting when BIDIR = 1.
        CountingUpThisEventIsTriggeredOnlyDuringUpCountingWhenBIDIR1 = 1,
        /// Counting down. This event is triggered only during down-counting when BIDIR = 1.
        CountingDownThisEventIsTriggeredOnlyDuringDownCountingWhenBIDIR1 = 2
    ]
],
EV15_CTRL [
    /// Selects the Match register associated with this event (if any). A  match can occ
    MATCHSEL OFFSET(0) NUMBITS(4) [],
    /// Select L/H counter. Do not set this bit if UNIFY = 1.
    HEVENT OFFSET(4) NUMBITS(1) [
        /// L state. Selects the L state and the L match register selected by MATCHSEL.
        LStateSelectsTheLStateAndTheLMatchRegisterSelectedByMATCHSEL = 0,
        /// H state. Selects the H state and the H match register selected by MATCHSEL.
        HStateSelectsTheHStateAndTheHMatchRegisterSelectedByMATCHSEL = 1
    ],
    /// Input/output select
    OUTSEL OFFSET(5) NUMBITS(1) [
        /// Input. Selects the input selected by IOSEL.
        InputSelectsTheInputSelectedByIOSEL = 0,
        /// Output. Selects the output selected by IOSEL.
        OutputSelectsTheOutputSelectedByIOSEL = 1
    ],
    /// Selects the input or output signal associated with this event (if  any). Do not
    IOSEL OFFSET(6) NUMBITS(4) [],
    /// Selects the I/O condition for event n. (The detection of edges on outputs lags t
    IOCOND OFFSET(10) NUMBITS(2) [
        /// LOW
        LOW = 0,
        /// Rise
        Rise = 1,
        /// Fall
        Fall = 2,
        /// HIGH
        HIGH = 3
    ],
    /// Selects how the specified match and I/O condition are used and combined.
    COMBMODE OFFSET(12) NUMBITS(2) [
        /// OR. The event occurs when either the specified match or I/O condition occurs.
        ORTheEventOccursWhenEitherTheSpecifiedMatchOrIOConditionOccurs = 0,
        /// MATCH. Uses the specified match only.
        MATCHUsesTheSpecifiedMatchOnly = 1,
        /// IO. Uses the specified I/O condition only.
        IOUsesTheSpecifiedIOConditionOnly = 2,
        /// AND. The event occurs when the specified match and I/O condition occur simultane
        ANDTheEventOccursWhenTheSpecifiedMatchAndIOConditionOccurSimultaneously = 3
    ],
    /// This bit controls how the STATEV value modifies the state  selected by HEVENT wh
    STATELD OFFSET(14) NUMBITS(1) [
        /// STATEV value is added into STATE (the carry-out is ignored).
        STATEVValueIsAddedIntoSTATETheCarryOutIsIgnored = 0,
        /// STATEV value is loaded into STATE.
        STATEVValueIsLoadedIntoSTATE = 1
    ],
    /// This value is loaded into or added to the state selected by  HEVENT, depending o
    STATEV OFFSET(15) NUMBITS(5) [],
    /// If this bit is one and the COMBMODE field specifies a match  component to the tr
    MATCHMEM OFFSET(20) NUMBITS(1) [],
    /// Direction qualifier for event generation. This field only applies when the count
    DIRECTION OFFSET(21) NUMBITS(2) [
        /// Direction independent. This event is triggered regardless of the count direction
        DirectionIndependentThisEventIsTriggeredRegardlessOfTheCountDirection = 0,
        /// Counting up. This event is triggered only during up-counting when BIDIR = 1.
        CountingUpThisEventIsTriggeredOnlyDuringUpCountingWhenBIDIR1 = 1,
        /// Counting down. This event is triggered only during down-counting when BIDIR = 1.
        CountingDownThisEventIsTriggeredOnlyDuringDownCountingWhenBIDIR1 = 2
    ]
],
OUT0_SET [
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET15 OFFSET(15) NUMBITS(1) []
],
OUT1_SET [
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET15 OFFSET(15) NUMBITS(1) []
],
OUT2_SET [
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET15 OFFSET(15) NUMBITS(1) []
],
OUT3_SET [
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET15 OFFSET(15) NUMBITS(1) []
],
OUT4_SET [
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET15 OFFSET(15) NUMBITS(1) []
],
OUT5_SET [
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET15 OFFSET(15) NUMBITS(1) []
],
OUT6_SET [
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET15 OFFSET(15) NUMBITS(1) []
],
OUT7_SET [
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET15 OFFSET(15) NUMBITS(1) []
],
OUT8_SET [
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET15 OFFSET(15) NUMBITS(1) []
],
OUT9_SET [
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET15 OFFSET(15) NUMBITS(1) []
],
OUT10_SET [
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET15 OFFSET(15) NUMBITS(1) []
],
OUT11_SET [
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET15 OFFSET(15) NUMBITS(1) []
],
OUT12_SET [
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET15 OFFSET(15) NUMBITS(1) []
],
OUT13_SET [
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET15 OFFSET(15) NUMBITS(1) []
],
OUT14_SET [
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET15 OFFSET(15) NUMBITS(1) []
],
OUT15_SET [
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x
    SET15 OFFSET(15) NUMBITS(1) []
],
OUT0_CLR [
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR15 OFFSET(15) NUMBITS(1) []
],
OUT1_CLR [
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR15 OFFSET(15) NUMBITS(1) []
],
OUT2_CLR [
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR15 OFFSET(15) NUMBITS(1) []
],
OUT3_CLR [
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR15 OFFSET(15) NUMBITS(1) []
],
OUT4_CLR [
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR15 OFFSET(15) NUMBITS(1) []
],
OUT5_CLR [
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR15 OFFSET(15) NUMBITS(1) []
],
OUT6_CLR [
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR15 OFFSET(15) NUMBITS(1) []
],
OUT7_CLR [
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR15 OFFSET(15) NUMBITS(1) []
],
OUT8_CLR [
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR15 OFFSET(15) NUMBITS(1) []
],
OUT9_CLR [
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR15 OFFSET(15) NUMBITS(1) []
],
OUT10_CLR [
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR15 OFFSET(15) NUMBITS(1) []
],
OUT11_CLR [
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR15 OFFSET(15) NUMBITS(1) []
],
OUT12_CLR [
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR15 OFFSET(15) NUMBITS(1) []
],
OUT13_CLR [
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR15 OFFSET(15) NUMBITS(1) []
],
OUT14_CLR [
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR15 OFFSET(15) NUMBITS(1) []
],
OUT15_CLR [
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR0 OFFSET(0) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR1 OFFSET(1) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR2 OFFSET(2) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR3 OFFSET(3) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR4 OFFSET(4) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR5 OFFSET(5) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR6 OFFSET(6) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR7 OFFSET(7) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR8 OFFSET(8) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR9 OFFSET(9) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR10 OFFSET(10) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR11 OFFSET(11) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR12 OFFSET(12) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR13 OFFSET(13) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR14 OFFSET(14) NUMBITS(1) [],
    /// A 1 in bit m selects event m to clear output n (or set it if SETCLRn = 0x1 or 0x
    CLR15 OFFSET(15) NUMBITS(1) []
]
];
const SCT_BASE: StaticRef<SctRegisters> =
    unsafe { StaticRef::new(0x40000000 as *const SctRegisters) };
