
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// 10-bit Analog-to-Digital Converter (ADC)
#[repr(C)]
struct AdcRegisters {
/// A/D Control Register. The AD0CR register must be written to select the operating
cr: ReadWrite<u32, CR::Register>,
/// A/D Global Data Register. Contains the result of the most recent A/D conversion.
gdr: ReadOnly<u32, GDR::Register>,
_reserved0: [u8; 4],
/// A/D Interrupt Enable Register. This register contains enable bits that allow the
inten: ReadWrite<u32, INTEN::Register>,
/// A/D Channel Data Register. This register contains the result of the most recent
dr_0: ReadOnly<u32, DR[0]::Register>,
/// A/D Channel Data Register. This register contains the result of the most recent
dr_1: ReadOnly<u32, DR[1]::Register>,
/// A/D Channel Data Register. This register contains the result of the most recent
dr_2: ReadOnly<u32, DR[2]::Register>,
/// A/D Channel Data Register. This register contains the result of the most recent
dr_3: ReadOnly<u32, DR[3]::Register>,
/// A/D Channel Data Register. This register contains the result of the most recent
dr_4: ReadOnly<u32, DR[4]::Register>,
/// A/D Channel Data Register. This register contains the result of the most recent
dr_5: ReadOnly<u32, DR[5]::Register>,
/// A/D Channel Data Register. This register contains the result of the most recent
dr_6: ReadOnly<u32, DR[6]::Register>,
/// A/D Channel Data Register. This register contains the result of the most recent
dr_7: ReadOnly<u32, DR[7]::Register>,
/// A/D Status Register. This register contains DONE and OVERRUN flags for all of th
stat: ReadOnly<u32, STAT::Register>,
}
register_bitfields![u32,
CR [
    /// Selects which of the ADC[7:0] pins are to be sampled and converted. Bit 0 select
    SEL OFFSET(0) NUMBITS(8) [],
    /// The ADC clock is divided by the CLKDIV value plus one to produce the clock for t
    CLKDIV OFFSET(8) NUMBITS(8) [],
    /// Burst mode
    BURST OFFSET(16) NUMBITS(1) [
        /// Conversions are software controlled and require 11 clocks.
        ConversionsAreSoftwareControlledAndRequire11Clocks = 0,
        /// The AD converter does repeated conversions at the rate selected by the CLKS fiel
        BURST = 1
    ],
    /// This field selects the number of clocks used for each conversion in Burst mode,
    CLKS OFFSET(17) NUMBITS(3) [
        /// 11 clocks / 10 bits
        _11Clocks10Bits = 0,
        /// 10 clocks / 9 bits
        _10Clocks9Bits = 1,
        /// 9 clocks / 8 bits
        _9Clocks8Bits = 2,
        /// 8 clocks / 7 bits
        _8Clocks7Bits = 3,
        /// 7 clocks / 6 bits
        _7Clocks6Bits = 4,
        /// 6 clocks / 5 bits
        _6Clocks5Bits = 5,
        /// 5 clocks / 4 bits
        _5Clocks4Bits = 6,
        /// 4 clocks / 3 bits
        _4Clocks3Bits = 7
    ],
    /// Power mode
    PDN OFFSET(21) NUMBITS(1) [
        /// The A/D converter is in Power-down mode.
        TheADConverterIsInPowerDownMode = 0,
        /// The A/D converter is operational.
        TheADConverterIsOperational = 1
    ],
    /// When the BURST bit is 0, these bits control whether and when an A/D conversion i
    START OFFSET(24) NUMBITS(3) [
        /// No start (this value should be used when clearing PDN to 0).
        NoStartThisValueShouldBeUsedWhenClearingPDNTo0 = 0,
        /// Start conversion now.
        StartConversionNow = 1,
        /// Start conversion when the edge selected by bit 27 occurs on CTOUT_15 (combined t
        StartConversionWhenTheEdgeSelectedByBit27OccursOnCTOUT_15CombinedTimerOutput15 = 2,
        /// Start conversion when the edge selected by bit 27 occurs on CTOUT_8 (combined ti
        StartConversionWhenTheEdgeSelectedByBit27OccursOnCTOUT_8CombinedTimerOutput8 = 3,
        /// Start conversion when the edge selected by bit 27 occurs on ADCTRIG0 input.
        StartConversionWhenTheEdgeSelectedByBit27OccursOnADCTRIG0Input = 4,
        /// Start conversion when the edge selected by bit 27 occurs on ADCTRIG1 input.
        StartConversionWhenTheEdgeSelectedByBit27OccursOnADCTRIG1Input = 5,
        /// Start conversion when the edge selected by bit 27 occurs on Motocon PWM output M
        StartConversionWhenTheEdgeSelectedByBit27OccursOnMotoconPWMOutputMCOA2 = 6,
        /// Reserved.
        Reserved = 7
    ],
    /// This bit is significant only when the START field contains 0x2 -0x6. In these ca
    EDGE OFFSET(27) NUMBITS(1) [
        /// Start conversion on a rising edge on the selected signal.
        StartConversionOnARisingEdgeOnTheSelectedSignal = 0,
        /// Start conversion on a falling edge on the selected signal.
        StartConversionOnAFallingEdgeOnTheSelectedSignal = 1
    ]
],
GDR [
    /// When DONE is 1, this field contains a binary fraction representing the voltage o
    V_VREF OFFSET(6) NUMBITS(10) [],
    /// These bits contain the channel from which the LS bits were converted.
    CHN OFFSET(24) NUMBITS(3) [],
    /// This bit is 1 in burst mode if the results of one or more conversions was (were)
    OVERRUN OFFSET(30) NUMBITS(1) [],
    /// This bit is set to 1 when an analog-to-digital conversion completes. It is clear
    DONE OFFSET(31) NUMBITS(1) []
],
INTEN [
    /// These bits allow control over which A/D channels generate interrupts for convers
    ADINTEN OFFSET(0) NUMBITS(8) [],
    /// When 1, enables the global DONE flag in ADDR to generate an interrupt. When 0, o
    ADGINTEN OFFSET(8) NUMBITS(1) []
],
STAT [
    /// These bits mirror the DONE status flags that appear in the result register for e
    DONE OFFSET(0) NUMBITS(8) [],
    /// These bits mirror the OVERRRUN status flags that appear in the result register f
    OVERUN OFFSET(8) NUMBITS(8) [],
    /// This bit is the A/D interrupt flag. It is one when any of the individual A/D cha
    ADINT OFFSET(16) NUMBITS(1) []
],
DR[0] [
    /// When DONE is 1, this field contains a binary fraction representing the voltage o
    V_VREF OFFSET(6) NUMBITS(10) [],
    /// This bit is 1 in burst mode if the results of one or more conversions was (were)
    OVERRUN OFFSET(30) NUMBITS(1) [],
    /// This bit is set to 1 when an A/D conversion completes. It is cleared when this r
    DONE OFFSET(31) NUMBITS(1) []
],
DR[1] [
    /// When DONE is 1, this field contains a binary fraction representing the voltage o
    V_VREF OFFSET(6) NUMBITS(10) [],
    /// This bit is 1 in burst mode if the results of one or more conversions was (were)
    OVERRUN OFFSET(30) NUMBITS(1) [],
    /// This bit is set to 1 when an A/D conversion completes. It is cleared when this r
    DONE OFFSET(31) NUMBITS(1) []
],
DR[2] [
    /// When DONE is 1, this field contains a binary fraction representing the voltage o
    V_VREF OFFSET(6) NUMBITS(10) [],
    /// This bit is 1 in burst mode if the results of one or more conversions was (were)
    OVERRUN OFFSET(30) NUMBITS(1) [],
    /// This bit is set to 1 when an A/D conversion completes. It is cleared when this r
    DONE OFFSET(31) NUMBITS(1) []
],
DR[3] [
    /// When DONE is 1, this field contains a binary fraction representing the voltage o
    V_VREF OFFSET(6) NUMBITS(10) [],
    /// This bit is 1 in burst mode if the results of one or more conversions was (were)
    OVERRUN OFFSET(30) NUMBITS(1) [],
    /// This bit is set to 1 when an A/D conversion completes. It is cleared when this r
    DONE OFFSET(31) NUMBITS(1) []
],
DR[4] [
    /// When DONE is 1, this field contains a binary fraction representing the voltage o
    V_VREF OFFSET(6) NUMBITS(10) [],
    /// This bit is 1 in burst mode if the results of one or more conversions was (were)
    OVERRUN OFFSET(30) NUMBITS(1) [],
    /// This bit is set to 1 when an A/D conversion completes. It is cleared when this r
    DONE OFFSET(31) NUMBITS(1) []
],
DR[5] [
    /// When DONE is 1, this field contains a binary fraction representing the voltage o
    V_VREF OFFSET(6) NUMBITS(10) [],
    /// This bit is 1 in burst mode if the results of one or more conversions was (were)
    OVERRUN OFFSET(30) NUMBITS(1) [],
    /// This bit is set to 1 when an A/D conversion completes. It is cleared when this r
    DONE OFFSET(31) NUMBITS(1) []
],
DR[6] [
    /// When DONE is 1, this field contains a binary fraction representing the voltage o
    V_VREF OFFSET(6) NUMBITS(10) [],
    /// This bit is 1 in burst mode if the results of one or more conversions was (were)
    OVERRUN OFFSET(30) NUMBITS(1) [],
    /// This bit is set to 1 when an A/D conversion completes. It is cleared when this r
    DONE OFFSET(31) NUMBITS(1) []
],
DR[7] [
    /// When DONE is 1, this field contains a binary fraction representing the voltage o
    V_VREF OFFSET(6) NUMBITS(10) [],
    /// This bit is 1 in burst mode if the results of one or more conversions was (were)
    OVERRUN OFFSET(30) NUMBITS(1) [],
    /// This bit is set to 1 when an A/D conversion completes. It is cleared when this r
    DONE OFFSET(31) NUMBITS(1) []
]
];
const ADC0_BASE: StaticRef<AdcRegisters> =
    unsafe { StaticRef::new(0x400E3000 as *const AdcRegisters) };


const ADC1_BASE: StaticRef<AdcRegisters> =
    unsafe { StaticRef::new(0x400E4000 as *const AdcRegisters) };
