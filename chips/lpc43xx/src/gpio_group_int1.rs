
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly, register_bitfields};
    /// GPIO group interrupt 0
#[repr(C)]
struct Gpio_Group_Int1Registers {
/// GPIO grouped interrupt control register
ctrl: ReadWrite<u32, CTRL::Register>,
_reserved0: [u8; 28],
/// GPIO grouped interrupt port  polarity register
port_pol0: ReadWrite<u32, PORT_POL0::Register>,
/// GPIO grouped interrupt port  polarity register
port_pol1: ReadWrite<u32, PORT_POL1::Register>,
/// GPIO grouped interrupt port  polarity register
port_pol2: ReadWrite<u32, PORT_POL2::Register>,
/// GPIO grouped interrupt port  polarity register
port_pol3: ReadWrite<u32, PORT_POL3::Register>,
/// GPIO grouped interrupt port  polarity register
port_pol4: ReadWrite<u32, PORT_POL4::Register>,
/// GPIO grouped interrupt port  polarity register
port_pol5: ReadWrite<u32, PORT_POL5::Register>,
/// GPIO grouped interrupt port  polarity register
port_pol6: ReadWrite<u32, PORT_POL6::Register>,
/// GPIO grouped interrupt port  polarity register
port_pol7: ReadWrite<u32, PORT_POL7::Register>,
/// GPIO grouped interrupt port m enable register
port_ena0: ReadWrite<u32, PORT_ENA0::Register>,
/// GPIO grouped interrupt port m enable register
port_ena1: ReadWrite<u32, PORT_ENA1::Register>,
/// GPIO grouped interrupt port m enable register
port_ena2: ReadWrite<u32, PORT_ENA2::Register>,
/// GPIO grouped interrupt port m enable register
port_ena3: ReadWrite<u32, PORT_ENA3::Register>,
/// GPIO grouped interrupt port m enable register
port_ena4: ReadWrite<u32, PORT_ENA4::Register>,
/// GPIO grouped interrupt port m enable register
port_ena5: ReadWrite<u32, PORT_ENA5::Register>,
/// GPIO grouped interrupt port m enable register
port_ena6: ReadWrite<u32, PORT_ENA6::Register>,
/// GPIO grouped interrupt port m enable register
port_ena7: ReadWrite<u32, PORT_ENA7::Register>,
}
register_bitfields![u32,
CTRL [
    /// Group interrupt status. This bit is cleared by writing a one to it. Writing zero
    INT OFFSET(0) NUMBITS(1) [
        /// No interrupt request is pending.
        NoInterruptRequestIsPending = 0,
        /// Interrupt request is active.
        InterruptRequestIsActive = 1
    ],
    /// Combine enabled inputs for group interrupt
    COMB OFFSET(1) NUMBITS(1) [
        /// OR functionality: A grouped interrupt is generated when any one of the enabled i
        OR_FUNCTIONALITY_A_ = 0,
        /// AND functionality: An interrupt is generated when all enabled bits are active (b
        AND_FUNCTIONALITY_A = 1
    ],
    /// Group interrupt trigger
    TRIG OFFSET(2) NUMBITS(1) [
        /// Edge-triggered
        EdgeTriggered = 0,
        /// Level-triggered
        LevelTriggered = 1
    ]
],
PORT_POL0 [
    /// Configure pin polarity of port  pins for group interrupt. Bit n corresponds to p
    POL_0 OFFSET(0) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_1 OFFSET(1) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_2 OFFSET(2) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_3 OFFSET(3) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_4 OFFSET(4) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_5 OFFSET(5) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_6 OFFSET(6) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_7 OFFSET(7) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_8 OFFSET(8) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_9 OFFSET(9) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_10 OFFSET(10) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_11 OFFSET(11) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_12 OFFSET(12) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_13 OFFSET(13) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_14 OFFSET(14) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_15 OFFSET(15) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_16 OFFSET(16) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_17 OFFSET(17) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_18 OFFSET(18) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_19 OFFSET(19) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_20 OFFSET(20) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_21 OFFSET(21) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_22 OFFSET(22) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_23 OFFSET(23) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_24 OFFSET(24) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_25 OFFSET(25) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_26 OFFSET(26) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_27 OFFSET(27) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_28 OFFSET(28) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_29 OFFSET(29) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_30 OFFSET(30) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_31 OFFSET(31) NUMBITS(1) []
],
PORT_POL1 [
    /// Configure pin polarity of port  pins for group interrupt. Bit n corresponds to p
    POL_0 OFFSET(0) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_1 OFFSET(1) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_2 OFFSET(2) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_3 OFFSET(3) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_4 OFFSET(4) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_5 OFFSET(5) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_6 OFFSET(6) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_7 OFFSET(7) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_8 OFFSET(8) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_9 OFFSET(9) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_10 OFFSET(10) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_11 OFFSET(11) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_12 OFFSET(12) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_13 OFFSET(13) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_14 OFFSET(14) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_15 OFFSET(15) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_16 OFFSET(16) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_17 OFFSET(17) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_18 OFFSET(18) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_19 OFFSET(19) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_20 OFFSET(20) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_21 OFFSET(21) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_22 OFFSET(22) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_23 OFFSET(23) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_24 OFFSET(24) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_25 OFFSET(25) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_26 OFFSET(26) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_27 OFFSET(27) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_28 OFFSET(28) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_29 OFFSET(29) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_30 OFFSET(30) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_31 OFFSET(31) NUMBITS(1) []
],
PORT_POL2 [
    /// Configure pin polarity of port  pins for group interrupt. Bit n corresponds to p
    POL_0 OFFSET(0) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_1 OFFSET(1) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_2 OFFSET(2) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_3 OFFSET(3) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_4 OFFSET(4) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_5 OFFSET(5) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_6 OFFSET(6) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_7 OFFSET(7) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_8 OFFSET(8) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_9 OFFSET(9) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_10 OFFSET(10) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_11 OFFSET(11) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_12 OFFSET(12) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_13 OFFSET(13) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_14 OFFSET(14) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_15 OFFSET(15) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_16 OFFSET(16) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_17 OFFSET(17) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_18 OFFSET(18) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_19 OFFSET(19) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_20 OFFSET(20) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_21 OFFSET(21) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_22 OFFSET(22) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_23 OFFSET(23) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_24 OFFSET(24) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_25 OFFSET(25) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_26 OFFSET(26) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_27 OFFSET(27) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_28 OFFSET(28) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_29 OFFSET(29) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_30 OFFSET(30) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_31 OFFSET(31) NUMBITS(1) []
],
PORT_POL3 [
    /// Configure pin polarity of port  pins for group interrupt. Bit n corresponds to p
    POL_0 OFFSET(0) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_1 OFFSET(1) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_2 OFFSET(2) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_3 OFFSET(3) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_4 OFFSET(4) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_5 OFFSET(5) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_6 OFFSET(6) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_7 OFFSET(7) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_8 OFFSET(8) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_9 OFFSET(9) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_10 OFFSET(10) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_11 OFFSET(11) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_12 OFFSET(12) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_13 OFFSET(13) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_14 OFFSET(14) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_15 OFFSET(15) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_16 OFFSET(16) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_17 OFFSET(17) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_18 OFFSET(18) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_19 OFFSET(19) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_20 OFFSET(20) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_21 OFFSET(21) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_22 OFFSET(22) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_23 OFFSET(23) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_24 OFFSET(24) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_25 OFFSET(25) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_26 OFFSET(26) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_27 OFFSET(27) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_28 OFFSET(28) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_29 OFFSET(29) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_30 OFFSET(30) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_31 OFFSET(31) NUMBITS(1) []
],
PORT_POL4 [
    /// Configure pin polarity of port  pins for group interrupt. Bit n corresponds to p
    POL_0 OFFSET(0) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_1 OFFSET(1) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_2 OFFSET(2) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_3 OFFSET(3) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_4 OFFSET(4) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_5 OFFSET(5) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_6 OFFSET(6) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_7 OFFSET(7) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_8 OFFSET(8) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_9 OFFSET(9) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_10 OFFSET(10) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_11 OFFSET(11) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_12 OFFSET(12) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_13 OFFSET(13) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_14 OFFSET(14) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_15 OFFSET(15) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_16 OFFSET(16) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_17 OFFSET(17) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_18 OFFSET(18) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_19 OFFSET(19) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_20 OFFSET(20) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_21 OFFSET(21) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_22 OFFSET(22) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_23 OFFSET(23) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_24 OFFSET(24) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_25 OFFSET(25) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_26 OFFSET(26) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_27 OFFSET(27) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_28 OFFSET(28) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_29 OFFSET(29) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_30 OFFSET(30) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_31 OFFSET(31) NUMBITS(1) []
],
PORT_POL5 [
    /// Configure pin polarity of port  pins for group interrupt. Bit n corresponds to p
    POL_0 OFFSET(0) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_1 OFFSET(1) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_2 OFFSET(2) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_3 OFFSET(3) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_4 OFFSET(4) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_5 OFFSET(5) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_6 OFFSET(6) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_7 OFFSET(7) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_8 OFFSET(8) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_9 OFFSET(9) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_10 OFFSET(10) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_11 OFFSET(11) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_12 OFFSET(12) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_13 OFFSET(13) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_14 OFFSET(14) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_15 OFFSET(15) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_16 OFFSET(16) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_17 OFFSET(17) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_18 OFFSET(18) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_19 OFFSET(19) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_20 OFFSET(20) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_21 OFFSET(21) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_22 OFFSET(22) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_23 OFFSET(23) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_24 OFFSET(24) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_25 OFFSET(25) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_26 OFFSET(26) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_27 OFFSET(27) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_28 OFFSET(28) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_29 OFFSET(29) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_30 OFFSET(30) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_31 OFFSET(31) NUMBITS(1) []
],
PORT_POL6 [
    /// Configure pin polarity of port  pins for group interrupt. Bit n corresponds to p
    POL_0 OFFSET(0) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_1 OFFSET(1) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_2 OFFSET(2) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_3 OFFSET(3) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_4 OFFSET(4) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_5 OFFSET(5) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_6 OFFSET(6) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_7 OFFSET(7) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_8 OFFSET(8) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_9 OFFSET(9) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_10 OFFSET(10) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_11 OFFSET(11) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_12 OFFSET(12) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_13 OFFSET(13) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_14 OFFSET(14) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_15 OFFSET(15) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_16 OFFSET(16) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_17 OFFSET(17) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_18 OFFSET(18) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_19 OFFSET(19) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_20 OFFSET(20) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_21 OFFSET(21) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_22 OFFSET(22) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_23 OFFSET(23) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_24 OFFSET(24) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_25 OFFSET(25) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_26 OFFSET(26) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_27 OFFSET(27) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_28 OFFSET(28) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_29 OFFSET(29) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_30 OFFSET(30) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_31 OFFSET(31) NUMBITS(1) []
],
PORT_POL7 [
    /// Configure pin polarity of port  pins for group interrupt. Bit n corresponds to p
    POL_0 OFFSET(0) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_1 OFFSET(1) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_2 OFFSET(2) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_3 OFFSET(3) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_4 OFFSET(4) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_5 OFFSET(5) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_6 OFFSET(6) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_7 OFFSET(7) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_8 OFFSET(8) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_9 OFFSET(9) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_10 OFFSET(10) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_11 OFFSET(11) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_12 OFFSET(12) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_13 OFFSET(13) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_14 OFFSET(14) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_15 OFFSET(15) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_16 OFFSET(16) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_17 OFFSET(17) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_18 OFFSET(18) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_19 OFFSET(19) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_20 OFFSET(20) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_21 OFFSET(21) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_22 OFFSET(22) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_23 OFFSET(23) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_24 OFFSET(24) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_25 OFFSET(25) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_26 OFFSET(26) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_27 OFFSET(27) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_28 OFFSET(28) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_29 OFFSET(29) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_30 OFFSET(30) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_31 OFFSET(31) NUMBITS(1) []
],
PORT_ENA0 [
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_0 OFFSET(0) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_1 OFFSET(1) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_2 OFFSET(2) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_3 OFFSET(3) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_4 OFFSET(4) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_5 OFFSET(5) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_6 OFFSET(6) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_7 OFFSET(7) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_8 OFFSET(8) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_9 OFFSET(9) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_10 OFFSET(10) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_11 OFFSET(11) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_12 OFFSET(12) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_13 OFFSET(13) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_14 OFFSET(14) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_15 OFFSET(15) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_16 OFFSET(16) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_17 OFFSET(17) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_18 OFFSET(18) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_19 OFFSET(19) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_20 OFFSET(20) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_21 OFFSET(21) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_22 OFFSET(22) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_23 OFFSET(23) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_24 OFFSET(24) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_25 OFFSET(25) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_26 OFFSET(26) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_27 OFFSET(27) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_28 OFFSET(28) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_29 OFFSET(29) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_30 OFFSET(30) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_31 OFFSET(31) NUMBITS(1) []
],
PORT_ENA1 [
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_0 OFFSET(0) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_1 OFFSET(1) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_2 OFFSET(2) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_3 OFFSET(3) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_4 OFFSET(4) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_5 OFFSET(5) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_6 OFFSET(6) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_7 OFFSET(7) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_8 OFFSET(8) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_9 OFFSET(9) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_10 OFFSET(10) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_11 OFFSET(11) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_12 OFFSET(12) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_13 OFFSET(13) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_14 OFFSET(14) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_15 OFFSET(15) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_16 OFFSET(16) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_17 OFFSET(17) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_18 OFFSET(18) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_19 OFFSET(19) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_20 OFFSET(20) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_21 OFFSET(21) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_22 OFFSET(22) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_23 OFFSET(23) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_24 OFFSET(24) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_25 OFFSET(25) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_26 OFFSET(26) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_27 OFFSET(27) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_28 OFFSET(28) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_29 OFFSET(29) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_30 OFFSET(30) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_31 OFFSET(31) NUMBITS(1) []
],
PORT_ENA2 [
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_0 OFFSET(0) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_1 OFFSET(1) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_2 OFFSET(2) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_3 OFFSET(3) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_4 OFFSET(4) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_5 OFFSET(5) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_6 OFFSET(6) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_7 OFFSET(7) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_8 OFFSET(8) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_9 OFFSET(9) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_10 OFFSET(10) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_11 OFFSET(11) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_12 OFFSET(12) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_13 OFFSET(13) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_14 OFFSET(14) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_15 OFFSET(15) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_16 OFFSET(16) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_17 OFFSET(17) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_18 OFFSET(18) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_19 OFFSET(19) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_20 OFFSET(20) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_21 OFFSET(21) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_22 OFFSET(22) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_23 OFFSET(23) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_24 OFFSET(24) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_25 OFFSET(25) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_26 OFFSET(26) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_27 OFFSET(27) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_28 OFFSET(28) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_29 OFFSET(29) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_30 OFFSET(30) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_31 OFFSET(31) NUMBITS(1) []
],
PORT_ENA3 [
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_0 OFFSET(0) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_1 OFFSET(1) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_2 OFFSET(2) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_3 OFFSET(3) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_4 OFFSET(4) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_5 OFFSET(5) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_6 OFFSET(6) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_7 OFFSET(7) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_8 OFFSET(8) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_9 OFFSET(9) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_10 OFFSET(10) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_11 OFFSET(11) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_12 OFFSET(12) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_13 OFFSET(13) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_14 OFFSET(14) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_15 OFFSET(15) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_16 OFFSET(16) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_17 OFFSET(17) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_18 OFFSET(18) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_19 OFFSET(19) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_20 OFFSET(20) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_21 OFFSET(21) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_22 OFFSET(22) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_23 OFFSET(23) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_24 OFFSET(24) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_25 OFFSET(25) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_26 OFFSET(26) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_27 OFFSET(27) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_28 OFFSET(28) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_29 OFFSET(29) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_30 OFFSET(30) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_31 OFFSET(31) NUMBITS(1) []
],
PORT_ENA4 [
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_0 OFFSET(0) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_1 OFFSET(1) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_2 OFFSET(2) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_3 OFFSET(3) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_4 OFFSET(4) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_5 OFFSET(5) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_6 OFFSET(6) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_7 OFFSET(7) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_8 OFFSET(8) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_9 OFFSET(9) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_10 OFFSET(10) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_11 OFFSET(11) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_12 OFFSET(12) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_13 OFFSET(13) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_14 OFFSET(14) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_15 OFFSET(15) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_16 OFFSET(16) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_17 OFFSET(17) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_18 OFFSET(18) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_19 OFFSET(19) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_20 OFFSET(20) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_21 OFFSET(21) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_22 OFFSET(22) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_23 OFFSET(23) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_24 OFFSET(24) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_25 OFFSET(25) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_26 OFFSET(26) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_27 OFFSET(27) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_28 OFFSET(28) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_29 OFFSET(29) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_30 OFFSET(30) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_31 OFFSET(31) NUMBITS(1) []
],
PORT_ENA5 [
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_0 OFFSET(0) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_1 OFFSET(1) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_2 OFFSET(2) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_3 OFFSET(3) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_4 OFFSET(4) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_5 OFFSET(5) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_6 OFFSET(6) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_7 OFFSET(7) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_8 OFFSET(8) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_9 OFFSET(9) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_10 OFFSET(10) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_11 OFFSET(11) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_12 OFFSET(12) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_13 OFFSET(13) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_14 OFFSET(14) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_15 OFFSET(15) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_16 OFFSET(16) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_17 OFFSET(17) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_18 OFFSET(18) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_19 OFFSET(19) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_20 OFFSET(20) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_21 OFFSET(21) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_22 OFFSET(22) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_23 OFFSET(23) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_24 OFFSET(24) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_25 OFFSET(25) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_26 OFFSET(26) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_27 OFFSET(27) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_28 OFFSET(28) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_29 OFFSET(29) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_30 OFFSET(30) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_31 OFFSET(31) NUMBITS(1) []
],
PORT_ENA6 [
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_0 OFFSET(0) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_1 OFFSET(1) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_2 OFFSET(2) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_3 OFFSET(3) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_4 OFFSET(4) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_5 OFFSET(5) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_6 OFFSET(6) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_7 OFFSET(7) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_8 OFFSET(8) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_9 OFFSET(9) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_10 OFFSET(10) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_11 OFFSET(11) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_12 OFFSET(12) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_13 OFFSET(13) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_14 OFFSET(14) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_15 OFFSET(15) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_16 OFFSET(16) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_17 OFFSET(17) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_18 OFFSET(18) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_19 OFFSET(19) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_20 OFFSET(20) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_21 OFFSET(21) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_22 OFFSET(22) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_23 OFFSET(23) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_24 OFFSET(24) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_25 OFFSET(25) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_26 OFFSET(26) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_27 OFFSET(27) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_28 OFFSET(28) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_29 OFFSET(29) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_30 OFFSET(30) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_31 OFFSET(31) NUMBITS(1) []
],
PORT_ENA7 [
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_0 OFFSET(0) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_1 OFFSET(1) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_2 OFFSET(2) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_3 OFFSET(3) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_4 OFFSET(4) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_5 OFFSET(5) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_6 OFFSET(6) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_7 OFFSET(7) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_8 OFFSET(8) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_9 OFFSET(9) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_10 OFFSET(10) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_11 OFFSET(11) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_12 OFFSET(12) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_13 OFFSET(13) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_14 OFFSET(14) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_15 OFFSET(15) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_16 OFFSET(16) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_17 OFFSET(17) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_18 OFFSET(18) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_19 OFFSET(19) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_20 OFFSET(20) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_21 OFFSET(21) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_22 OFFSET(22) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_23 OFFSET(23) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_24 OFFSET(24) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_25 OFFSET(25) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_26 OFFSET(26) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_27 OFFSET(27) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_28 OFFSET(28) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_29 OFFSET(29) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_30 OFFSET(30) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_31 OFFSET(31) NUMBITS(1) []
],
PORT_POL0 [
    /// Configure pin polarity of port  pins for group interrupt. Bit n corresponds to p
    POL_0 OFFSET(0) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_1 OFFSET(1) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_2 OFFSET(2) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_3 OFFSET(3) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_4 OFFSET(4) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_5 OFFSET(5) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_6 OFFSET(6) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_7 OFFSET(7) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_8 OFFSET(8) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_9 OFFSET(9) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_10 OFFSET(10) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_11 OFFSET(11) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_12 OFFSET(12) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_13 OFFSET(13) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_14 OFFSET(14) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_15 OFFSET(15) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_16 OFFSET(16) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_17 OFFSET(17) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_18 OFFSET(18) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_19 OFFSET(19) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_20 OFFSET(20) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_21 OFFSET(21) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_22 OFFSET(22) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_23 OFFSET(23) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_24 OFFSET(24) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_25 OFFSET(25) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_26 OFFSET(26) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_27 OFFSET(27) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_28 OFFSET(28) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_29 OFFSET(29) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_30 OFFSET(30) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_31 OFFSET(31) NUMBITS(1) []
],
PORT_POL1 [
    /// Configure pin polarity of port  pins for group interrupt. Bit n corresponds to p
    POL_0 OFFSET(0) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_1 OFFSET(1) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_2 OFFSET(2) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_3 OFFSET(3) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_4 OFFSET(4) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_5 OFFSET(5) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_6 OFFSET(6) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_7 OFFSET(7) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_8 OFFSET(8) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_9 OFFSET(9) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_10 OFFSET(10) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_11 OFFSET(11) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_12 OFFSET(12) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_13 OFFSET(13) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_14 OFFSET(14) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_15 OFFSET(15) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_16 OFFSET(16) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_17 OFFSET(17) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_18 OFFSET(18) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_19 OFFSET(19) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_20 OFFSET(20) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_21 OFFSET(21) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_22 OFFSET(22) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_23 OFFSET(23) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_24 OFFSET(24) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_25 OFFSET(25) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_26 OFFSET(26) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_27 OFFSET(27) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_28 OFFSET(28) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_29 OFFSET(29) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_30 OFFSET(30) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_31 OFFSET(31) NUMBITS(1) []
],
PORT_POL2 [
    /// Configure pin polarity of port  pins for group interrupt. Bit n corresponds to p
    POL_0 OFFSET(0) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_1 OFFSET(1) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_2 OFFSET(2) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_3 OFFSET(3) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_4 OFFSET(4) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_5 OFFSET(5) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_6 OFFSET(6) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_7 OFFSET(7) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_8 OFFSET(8) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_9 OFFSET(9) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_10 OFFSET(10) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_11 OFFSET(11) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_12 OFFSET(12) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_13 OFFSET(13) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_14 OFFSET(14) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_15 OFFSET(15) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_16 OFFSET(16) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_17 OFFSET(17) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_18 OFFSET(18) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_19 OFFSET(19) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_20 OFFSET(20) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_21 OFFSET(21) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_22 OFFSET(22) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_23 OFFSET(23) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_24 OFFSET(24) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_25 OFFSET(25) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_26 OFFSET(26) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_27 OFFSET(27) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_28 OFFSET(28) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_29 OFFSET(29) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_30 OFFSET(30) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_31 OFFSET(31) NUMBITS(1) []
],
PORT_POL3 [
    /// Configure pin polarity of port  pins for group interrupt. Bit n corresponds to p
    POL_0 OFFSET(0) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_1 OFFSET(1) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_2 OFFSET(2) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_3 OFFSET(3) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_4 OFFSET(4) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_5 OFFSET(5) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_6 OFFSET(6) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_7 OFFSET(7) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_8 OFFSET(8) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_9 OFFSET(9) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_10 OFFSET(10) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_11 OFFSET(11) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_12 OFFSET(12) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_13 OFFSET(13) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_14 OFFSET(14) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_15 OFFSET(15) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_16 OFFSET(16) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_17 OFFSET(17) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_18 OFFSET(18) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_19 OFFSET(19) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_20 OFFSET(20) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_21 OFFSET(21) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_22 OFFSET(22) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_23 OFFSET(23) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_24 OFFSET(24) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_25 OFFSET(25) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_26 OFFSET(26) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_27 OFFSET(27) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_28 OFFSET(28) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_29 OFFSET(29) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_30 OFFSET(30) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_31 OFFSET(31) NUMBITS(1) []
],
PORT_POL4 [
    /// Configure pin polarity of port  pins for group interrupt. Bit n corresponds to p
    POL_0 OFFSET(0) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_1 OFFSET(1) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_2 OFFSET(2) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_3 OFFSET(3) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_4 OFFSET(4) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_5 OFFSET(5) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_6 OFFSET(6) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_7 OFFSET(7) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_8 OFFSET(8) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_9 OFFSET(9) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_10 OFFSET(10) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_11 OFFSET(11) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_12 OFFSET(12) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_13 OFFSET(13) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_14 OFFSET(14) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_15 OFFSET(15) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_16 OFFSET(16) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_17 OFFSET(17) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_18 OFFSET(18) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_19 OFFSET(19) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_20 OFFSET(20) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_21 OFFSET(21) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_22 OFFSET(22) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_23 OFFSET(23) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_24 OFFSET(24) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_25 OFFSET(25) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_26 OFFSET(26) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_27 OFFSET(27) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_28 OFFSET(28) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_29 OFFSET(29) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_30 OFFSET(30) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_31 OFFSET(31) NUMBITS(1) []
],
PORT_POL5 [
    /// Configure pin polarity of port  pins for group interrupt. Bit n corresponds to p
    POL_0 OFFSET(0) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_1 OFFSET(1) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_2 OFFSET(2) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_3 OFFSET(3) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_4 OFFSET(4) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_5 OFFSET(5) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_6 OFFSET(6) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_7 OFFSET(7) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_8 OFFSET(8) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_9 OFFSET(9) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_10 OFFSET(10) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_11 OFFSET(11) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_12 OFFSET(12) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_13 OFFSET(13) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_14 OFFSET(14) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_15 OFFSET(15) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_16 OFFSET(16) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_17 OFFSET(17) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_18 OFFSET(18) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_19 OFFSET(19) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_20 OFFSET(20) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_21 OFFSET(21) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_22 OFFSET(22) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_23 OFFSET(23) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_24 OFFSET(24) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_25 OFFSET(25) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_26 OFFSET(26) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_27 OFFSET(27) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_28 OFFSET(28) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_29 OFFSET(29) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_30 OFFSET(30) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_31 OFFSET(31) NUMBITS(1) []
],
PORT_POL6 [
    /// Configure pin polarity of port  pins for group interrupt. Bit n corresponds to p
    POL_0 OFFSET(0) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_1 OFFSET(1) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_2 OFFSET(2) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_3 OFFSET(3) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_4 OFFSET(4) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_5 OFFSET(5) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_6 OFFSET(6) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_7 OFFSET(7) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_8 OFFSET(8) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_9 OFFSET(9) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_10 OFFSET(10) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_11 OFFSET(11) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_12 OFFSET(12) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_13 OFFSET(13) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_14 OFFSET(14) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_15 OFFSET(15) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_16 OFFSET(16) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_17 OFFSET(17) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_18 OFFSET(18) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_19 OFFSET(19) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_20 OFFSET(20) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_21 OFFSET(21) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_22 OFFSET(22) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_23 OFFSET(23) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_24 OFFSET(24) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_25 OFFSET(25) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_26 OFFSET(26) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_27 OFFSET(27) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_28 OFFSET(28) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_29 OFFSET(29) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_30 OFFSET(30) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_31 OFFSET(31) NUMBITS(1) []
],
PORT_POL7 [
    /// Configure pin polarity of port  pins for group interrupt. Bit n corresponds to p
    POL_0 OFFSET(0) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_1 OFFSET(1) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_2 OFFSET(2) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_3 OFFSET(3) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_4 OFFSET(4) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_5 OFFSET(5) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_6 OFFSET(6) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_7 OFFSET(7) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_8 OFFSET(8) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_9 OFFSET(9) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_10 OFFSET(10) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_11 OFFSET(11) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_12 OFFSET(12) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_13 OFFSET(13) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_14 OFFSET(14) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_15 OFFSET(15) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_16 OFFSET(16) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_17 OFFSET(17) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_18 OFFSET(18) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_19 OFFSET(19) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_20 OFFSET(20) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_21 OFFSET(21) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_22 OFFSET(22) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_23 OFFSET(23) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_24 OFFSET(24) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_25 OFFSET(25) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_26 OFFSET(26) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_27 OFFSET(27) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_28 OFFSET(28) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_29 OFFSET(29) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_30 OFFSET(30) NUMBITS(1) [],
    /// Configure pin polarity of port m pins for group interrupt. Bit n corresponds to
    POL_31 OFFSET(31) NUMBITS(1) []
],
PORT_ENA0 [
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_0 OFFSET(0) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_1 OFFSET(1) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_2 OFFSET(2) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_3 OFFSET(3) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_4 OFFSET(4) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_5 OFFSET(5) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_6 OFFSET(6) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_7 OFFSET(7) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_8 OFFSET(8) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_9 OFFSET(9) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_10 OFFSET(10) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_11 OFFSET(11) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_12 OFFSET(12) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_13 OFFSET(13) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_14 OFFSET(14) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_15 OFFSET(15) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_16 OFFSET(16) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_17 OFFSET(17) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_18 OFFSET(18) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_19 OFFSET(19) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_20 OFFSET(20) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_21 OFFSET(21) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_22 OFFSET(22) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_23 OFFSET(23) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_24 OFFSET(24) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_25 OFFSET(25) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_26 OFFSET(26) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_27 OFFSET(27) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_28 OFFSET(28) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_29 OFFSET(29) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_30 OFFSET(30) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_31 OFFSET(31) NUMBITS(1) []
],
PORT_ENA1 [
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_0 OFFSET(0) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_1 OFFSET(1) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_2 OFFSET(2) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_3 OFFSET(3) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_4 OFFSET(4) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_5 OFFSET(5) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_6 OFFSET(6) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_7 OFFSET(7) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_8 OFFSET(8) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_9 OFFSET(9) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_10 OFFSET(10) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_11 OFFSET(11) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_12 OFFSET(12) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_13 OFFSET(13) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_14 OFFSET(14) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_15 OFFSET(15) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_16 OFFSET(16) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_17 OFFSET(17) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_18 OFFSET(18) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_19 OFFSET(19) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_20 OFFSET(20) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_21 OFFSET(21) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_22 OFFSET(22) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_23 OFFSET(23) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_24 OFFSET(24) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_25 OFFSET(25) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_26 OFFSET(26) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_27 OFFSET(27) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_28 OFFSET(28) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_29 OFFSET(29) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_30 OFFSET(30) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_31 OFFSET(31) NUMBITS(1) []
],
PORT_ENA2 [
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_0 OFFSET(0) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_1 OFFSET(1) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_2 OFFSET(2) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_3 OFFSET(3) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_4 OFFSET(4) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_5 OFFSET(5) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_6 OFFSET(6) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_7 OFFSET(7) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_8 OFFSET(8) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_9 OFFSET(9) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_10 OFFSET(10) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_11 OFFSET(11) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_12 OFFSET(12) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_13 OFFSET(13) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_14 OFFSET(14) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_15 OFFSET(15) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_16 OFFSET(16) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_17 OFFSET(17) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_18 OFFSET(18) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_19 OFFSET(19) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_20 OFFSET(20) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_21 OFFSET(21) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_22 OFFSET(22) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_23 OFFSET(23) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_24 OFFSET(24) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_25 OFFSET(25) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_26 OFFSET(26) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_27 OFFSET(27) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_28 OFFSET(28) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_29 OFFSET(29) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_30 OFFSET(30) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_31 OFFSET(31) NUMBITS(1) []
],
PORT_ENA3 [
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_0 OFFSET(0) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_1 OFFSET(1) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_2 OFFSET(2) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_3 OFFSET(3) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_4 OFFSET(4) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_5 OFFSET(5) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_6 OFFSET(6) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_7 OFFSET(7) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_8 OFFSET(8) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_9 OFFSET(9) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_10 OFFSET(10) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_11 OFFSET(11) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_12 OFFSET(12) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_13 OFFSET(13) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_14 OFFSET(14) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_15 OFFSET(15) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_16 OFFSET(16) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_17 OFFSET(17) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_18 OFFSET(18) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_19 OFFSET(19) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_20 OFFSET(20) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_21 OFFSET(21) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_22 OFFSET(22) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_23 OFFSET(23) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_24 OFFSET(24) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_25 OFFSET(25) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_26 OFFSET(26) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_27 OFFSET(27) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_28 OFFSET(28) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_29 OFFSET(29) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_30 OFFSET(30) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_31 OFFSET(31) NUMBITS(1) []
],
PORT_ENA4 [
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_0 OFFSET(0) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_1 OFFSET(1) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_2 OFFSET(2) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_3 OFFSET(3) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_4 OFFSET(4) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_5 OFFSET(5) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_6 OFFSET(6) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_7 OFFSET(7) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_8 OFFSET(8) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_9 OFFSET(9) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_10 OFFSET(10) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_11 OFFSET(11) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_12 OFFSET(12) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_13 OFFSET(13) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_14 OFFSET(14) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_15 OFFSET(15) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_16 OFFSET(16) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_17 OFFSET(17) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_18 OFFSET(18) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_19 OFFSET(19) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_20 OFFSET(20) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_21 OFFSET(21) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_22 OFFSET(22) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_23 OFFSET(23) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_24 OFFSET(24) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_25 OFFSET(25) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_26 OFFSET(26) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_27 OFFSET(27) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_28 OFFSET(28) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_29 OFFSET(29) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_30 OFFSET(30) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_31 OFFSET(31) NUMBITS(1) []
],
PORT_ENA5 [
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_0 OFFSET(0) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_1 OFFSET(1) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_2 OFFSET(2) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_3 OFFSET(3) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_4 OFFSET(4) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_5 OFFSET(5) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_6 OFFSET(6) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_7 OFFSET(7) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_8 OFFSET(8) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_9 OFFSET(9) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_10 OFFSET(10) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_11 OFFSET(11) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_12 OFFSET(12) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_13 OFFSET(13) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_14 OFFSET(14) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_15 OFFSET(15) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_16 OFFSET(16) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_17 OFFSET(17) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_18 OFFSET(18) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_19 OFFSET(19) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_20 OFFSET(20) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_21 OFFSET(21) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_22 OFFSET(22) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_23 OFFSET(23) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_24 OFFSET(24) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_25 OFFSET(25) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_26 OFFSET(26) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_27 OFFSET(27) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_28 OFFSET(28) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_29 OFFSET(29) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_30 OFFSET(30) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_31 OFFSET(31) NUMBITS(1) []
],
PORT_ENA6 [
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_0 OFFSET(0) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_1 OFFSET(1) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_2 OFFSET(2) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_3 OFFSET(3) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_4 OFFSET(4) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_5 OFFSET(5) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_6 OFFSET(6) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_7 OFFSET(7) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_8 OFFSET(8) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_9 OFFSET(9) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_10 OFFSET(10) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_11 OFFSET(11) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_12 OFFSET(12) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_13 OFFSET(13) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_14 OFFSET(14) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_15 OFFSET(15) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_16 OFFSET(16) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_17 OFFSET(17) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_18 OFFSET(18) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_19 OFFSET(19) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_20 OFFSET(20) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_21 OFFSET(21) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_22 OFFSET(22) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_23 OFFSET(23) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_24 OFFSET(24) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_25 OFFSET(25) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_26 OFFSET(26) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_27 OFFSET(27) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_28 OFFSET(28) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_29 OFFSET(29) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_30 OFFSET(30) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_31 OFFSET(31) NUMBITS(1) []
],
PORT_ENA7 [
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_0 OFFSET(0) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_1 OFFSET(1) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_2 OFFSET(2) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_3 OFFSET(3) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_4 OFFSET(4) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_5 OFFSET(5) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_6 OFFSET(6) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_7 OFFSET(7) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_8 OFFSET(8) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_9 OFFSET(9) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_10 OFFSET(10) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_11 OFFSET(11) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_12 OFFSET(12) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_13 OFFSET(13) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_14 OFFSET(14) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_15 OFFSET(15) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_16 OFFSET(16) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_17 OFFSET(17) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_18 OFFSET(18) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_19 OFFSET(19) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_20 OFFSET(20) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_21 OFFSET(21) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_22 OFFSET(22) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_23 OFFSET(23) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_24 OFFSET(24) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_25 OFFSET(25) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_26 OFFSET(26) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_27 OFFSET(27) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_28 OFFSET(28) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_29 OFFSET(29) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_30 OFFSET(30) NUMBITS(1) [],
    /// Enable port m pin for group interrupt. Bit n corresponds to pin GPIOm[n] of port
    ENA_31 OFFSET(31) NUMBITS(1) []
]
];
const GPIO_GROUP_INT1_BASE: StaticRef<Gpio_Group_Int1Registers> =
    unsafe { StaticRef::new(0x40089000 as *const Gpio_Group_Int1Registers) };
