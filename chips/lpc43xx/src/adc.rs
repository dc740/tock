
use kernel::common::cells::OptionalCell;
use kernel::common::registers::{ ReadOnly, ReadWrite, register_bitfields, FieldValue};
use kernel::common::StaticRef;
use kernel::hil;
use kernel::ReturnCode;

use crate::{ccu1,nvic};

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

const ADC_MAX_SAMPLE_RATE : u32 = 400000;
pub static mut ADC0: Adc = Adc::new(ADC0_BASE, 0);
pub static mut ADC1: Adc = Adc::new(ADC1_BASE, 1);

pub struct Adc {
    registers: StaticRef<AdcRegisters>,
    client: OptionalCell<&'static dyn hil::adc::Client>,
    adc_idx:u8,
}
#[derive(Copy, Clone, Debug)]
pub enum ChannelSetting {
    Disable = 0,
    Enable = 1,
}
#[derive(Copy, Clone, Debug)]
pub enum AdcChannel {
    AnalogInput0 = 0,
    AnalogInput1 = 1,
    AnalogInput2 = 2,
    AnalogInput3 = 3,
    AnalogInput4 = 4,
    AnalogInput5 = 5,
    AnalogInput6 = 6,
    AnalogInput7 = 7
}

impl Adc {
    const fn new(registers: StaticRef<AdcRegisters>, adc_idx:u8) -> Adc {
        Adc {
            registers,
            // state: Cell::new(State::Idle),
            client: OptionalCell::empty(),
            adc_idx,
        }
    }
    pub fn enable_interrupt(&self) {
        //enable  IRQ
        unsafe {
            let n;
            if self.adc_idx == 0{
                n = cortexm4::nvic::Nvic::new(nvic::ADC0);
            } else {
                n = cortexm4::nvic::Nvic::new(nvic::ADC1);
            }
            n.clear_pending();
            n.enable();
        }
    }
    
    pub fn disable_interrupt(&self) {
        //disable  IRQ
        unsafe {
            let n;
            if self.adc_idx == 0{
                n = cortexm4::nvic::Nvic::new(nvic::ADC0);
            } else {
                n = cortexm4::nvic::Nvic::new(nvic::ADC1);
            }
            n.clear_pending();
            n.disable();
        }
    }
    
    /**
     * Enables or disables a channel interrupt for the current ADC.
     */
    pub fn channel_set(&self, channel : u32,  setting : ChannelSetting) {
        let regs = &*self.registers;
        let register_setting = FieldValue::<u32, INTEN::Register>::new(0x1, channel as usize, setting as u32);
        regs.inten.modify(register_setting);
    }
    
    pub fn set_client(&self, client: &'static dyn hil::adc::Client) {
        self.client.set(client);
    }

    pub fn handle_interrupt(&self) {
        let regs = &*self.registers;
        self.disable_interrupt();
        let channel = regs.gdr.read(GDR::CHN);
        self.channel_set(channel, ChannelSetting::Disable);
        
        let val = regs.dr[channel as usize].read(DR::V_VREF) as u16;
        self.client.map(|client| {
            client.sample_ready(val);
        });
    }
    
    pub fn init_adc(&self, channel : u8) {
        let regs = &*self.registers;
        ccu1::adc_clock_init(channel as u8);
        //disable ALL channels first
        regs.inten.set(0);
        let currently_enabled = regs.cr.read(CR::SEL);
        let clk_val;
        if currently_enabled != 0 {
            clk_val = regs.cr.read(CR::CLKDIV);
        } else {
            clk_val = self.get_adc_clk_div(ADC_MAX_SAMPLE_RATE, 11);
        }
        regs.cr.write(CR::PDN::TheADConverterIsOperational
                      + CR::CLKS::_11Clocks10Bits
                      + CR::SEL.val(currently_enabled | (1 << channel))
                      + CR::CLKDIV.val(clk_val)
                      + CR::BURST::ConversionsAreSoftwareControlledAndRequire11Clocks);
        self.enable_interrupt();
        self.channel_set(channel as u32, ChannelSetting::Enable);
    }
    
    pub fn get_adc_clk_div(&self, rate : u32, clocks : u32) -> u32 {
        /* The APB clock (PCLK_ADC0) is divided by (CLKDIV+1) to produce the clock for
           A/D converter, which should be less than or equal to 4.5MHz.
           A fully conversion requires (bits_accuracy+1) of these clocks.
           ADC Clock = PCLK_ADC0 / (CLKDIV + 1);
           ADC rate = ADC clock / (the number of clocks required for each conversion);
         */
        let adc_block_freq = ccu1::get_adc_rate(self.adc_idx);
        let full_rate = rate * clocks;
        ((adc_block_freq * 2 + full_rate) / (full_rate * 2)) - 1
    }
}

/// Implements an ADC capable reading ADC samples on any channel.
impl hil::adc::Adc for Adc {
    type Channel = AdcChannel;

    fn sample(&self, channel: &Self::Channel) -> ReturnCode {
        self.init_adc(*channel as u8);
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


//NOT IMPLEMENTED. I just needed the interface
/// Implements an ADC capable of continuous sampling
impl hil::adc::AdcHighSpeed for Adc {
    /// Capture buffered samples from the ADC continuously at a given
    /// frequency, calling the client whenever a buffer fills up. The client is
    /// then expected to either stop sampling or provide an additional buffer
    /// to sample into. Note that due to hardware constraints the maximum
    /// frequency range of the ADC is from 187 kHz to 23 Hz (although its
    /// precision is limited at higher frequencies due to aliasing).
    ///
    /// - `channel`: the ADC channel to sample
    /// - `frequency`: frequency to sample at
    /// - `buffer1`: first buffer to fill with samples
    /// - `length1`: number of samples to collect (up to buffer length)
    /// - `buffer2`: second buffer to fill once the first is full
    /// - `length2`: number of samples to collect (up to buffer length)
    fn sample_highspeed(
        &self,
        _channel: &Self::Channel,
        _frequency: u32,
        buffer1: &'static mut [u16],
        _length1: usize,
        buffer2: &'static mut [u16],
        _length2: usize,
    ) -> (
        ReturnCode,
        Option<&'static mut [u16]>,
        Option<&'static mut [u16]>,
    ) {
        (ReturnCode::EINVAL, Some(buffer1), Some(buffer2))
    }

    /// Provide a new buffer to send on-going buffered continuous samples to.
    /// This is expected to be called after the `samples_ready` callback.
    ///
    /// - `buf`: buffer to fill with samples
    /// - `length`: number of samples to collect (up to buffer length)
    fn provide_buffer(
        &self,
        buf: &'static mut [u16],
        _length: usize,
    ) -> (ReturnCode, Option<&'static mut [u16]>) {
            (ReturnCode::EINVAL, Some(buf))
    }

    /// Reclaim buffers after the ADC is stopped.
    /// This is expected to be called after `stop_sampling`.
    fn retrieve_buffers(
        &self,
    ) -> (
        ReturnCode,
        Option<&'static mut [u16]>,
        Option<&'static mut [u16]>,
    ) {
            (ReturnCode::EINVAL, None, None)
    }
}
