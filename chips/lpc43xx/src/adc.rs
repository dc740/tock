
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly, register_bitfields};
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
dr: [ReadOnly<u32, DR::Register>; 8],
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
DR [
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


pub static mut ADC0: Adc = Adc::new(ADC0_BASE, 0);
pub static mut ADC1: Adc = Adc::new(ADC1_BASE, 1);

pub struct Adc {
    registers: StaticRef<AdcRegisters>,
    client: OptionalCell<&'static dyn hil::adc::Client>,
    buffer_idx:usize,
}

#[derive(Copy, Clone, Debug)]
pub enum AdcChannel {
    AnalogInput0 = 1,
    AnalogInput1 = 2,
    AnalogInput2 = 3,
    AnalogInput3 = 4,
    AnalogInput4 = 5,
    AnalogInput5 = 6,
    AnalogInput6 = 7,
    AnalogInput7 = 8,
    VDD = 9,
    VDDHDIV5 = 0xD,
}

impl Adc {
    const fn new(registers: StaticRef<AdcRegisters>, buffer_idx:usize) -> Adc {
        Adc {
            registers,
            // state: Cell::new(State::Idle),
            client: OptionalCell::empty(),
            buffer_idx,
        }
    }
    pub fn disable_interrupt(&self) {
        let regs = &*self.registers;
        //disable  IRQ
        unsafe {
            let n;
            if self.buffer_idx == 0{
                n = cortexm4::nvic::Nvic::new(nvic::ADC0);
            } else {
                n = cortexm4::nvic::Nvic::new(nvic::ADC1);
            }
            n.clear_pending();
            n.disable();
        }
        let channel_disable = FieldValue::<u32, ()>::new(0x1, self.channel as usize, 0x0)
        regs.inten.modify(channel_disable);
    }
    pub fn set_client(&self, client: &'static dyn hil::adc::Client) {
        self.client.set(client);
    }

    pub fn handle_interrupt(&self) {
        let regs = &*self.registers;
        self.disable_interrupt();
        //we can internally store a channel, or check the register
        //to see which channel sent the interrupt. So lets do that.
        let channel = regs.gdr.read(GDR::CHN);
        let val = regs.dr[channel as usize].read(DR::V_VREF) as u16;
        self.client.map(|client| {
            client.sample_ready(val);
        });
    }
}

/// Implements an ADC capable reading ADC samples on any channel.
impl hil::adc::Adc for Adc {
    type Channel = AdcChannel;

    fn sample(&self, channel: &Self::Channel) -> ReturnCode {
        let regs = &*self.registers;
        self.enable_interrupt();
        
        ReturnCode::SUCCESS
    }

    fn sample_continuous(&self, _channel: &Self::Channel, _frequency: u32) -> ReturnCode {
        ReturnCode::FAIL
    }

    fn stop_sampling(&self) -> ReturnCode {
        ReturnCode::FAIL
    }

    fn get_resolution_bits(&self) -> usize {
        10
    }

    fn get_voltage_reference_mv(&self) -> Option<usize> {
        Some(3300)
    }
}

