
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// Configuration Registers (CREG)
#[repr(C)]
struct CregRegisters {
_reserved0: [u8; 4],
/// Chip configuration register 32 kHz oscillator output and BOD control register.
creg0: ReadWrite<u32, CREG0::Register>,
_reserved1: [u8; 248],
/// ARM Cortex-M4 memory mapping
m4memmap: ReadWrite<u32>,
_reserved2: [u8; 20],
/// Chip configuration register 5. Controls JTAG access.
creg5: ReadWrite<u32, CREG5::Register>,
/// DMA mux control
dmamux: ReadWrite<u32, DMAMUX::Register>,
/// Flash accelerator configuration register for flash bank A
flashcfga: ReadWrite<u32, FLASHCFGA::Register>,
/// Flash accelerator configuration register for flash bank B
flashcfgb: ReadWrite<u32, FLASHCFGB::Register>,
/// ETB RAM configuration
etbcfg: ReadWrite<u32>,
/// Chip configuration register 6. Controls multiple functions : Ethernet interface,
creg6: ReadWrite<u32, CREG6::Register>,
/// Cortex-M4 TXEV event clear
m4txevent: ReadWrite<u32>,
_reserved3: [u8; 204],
/// Part ID
chipid: ReadOnly<u32>,
_reserved4: [u8; 260],
/// ARM Cortex-M0SUB memory mapping
m0submemmap: ReadWrite<u32>,
_reserved5: [u8; 8],
/// Cortex-M0SUB TXEV event clear
m0subtxevent: ReadWrite<u32>,
_reserved6: [u8; 232],
/// Cortex-M0APP TXEV event clear
m0apptxevent: ReadWrite<u32>,
/// ARM Cortex-M0APP memory mapping
m0appmemmap: ReadWrite<u32>,
_reserved7: [u8; 248],
/// USB0 frame length adjust register
usb0fladj: ReadWrite<u32>,
_reserved8: [u8; 252],
/// USB1 frame length adjust register
usb1fladj: ReadWrite<u32>,
}
register_bitfields![u32,
CREG0 [
    /// Enable 1 kHz output.
    EN1KHZ OFFSET(0) NUMBITS(1) [
        /// 1 kHz output disabled.
        _1KHzOutputDisabled = 0,
        /// 1 kHz output enabled.
        _1KHzOutputEnabled = 1
    ],
    /// Enable 32 kHz output
    EN32KHZ OFFSET(1) NUMBITS(1) [
        /// 32 kHz output disabled.
        _32KHzOutputDisabled = 0,
        /// 32 kHz output enabled.
        _32KHzOutputEnabled = 1
    ],
    /// 32 kHz oscillator reset
    RESET32KHZ OFFSET(2) NUMBITS(1) [
        /// Clear reset.
        ClearReset = 0,
        /// Reset active.
        ResetActive = 1
    ],
    /// 32 kHz power control.
    PD32KHZ OFFSET(3) NUMBITS(1) [
        /// Powered.
        Powered = 0,
        /// Powered-down.
        PoweredDown = 1
    ],
    /// USB0 PHY power control.
    USB0PHY OFFSET(5) NUMBITS(1) [
        /// Enable USB0 PHY power.
        EnableUSB0PHYPower = 0,
        /// Disable USB0 PHY. PHY powered down.
        DisableUSB0PHYPHYPoweredDown = 1
    ],
    /// RTC_ALARM pin output control
    ALARMCTRL OFFSET(6) NUMBITS(2) [
        /// RTC alarm.
        RTCAlarm = 0,
        /// Event router event.
        EventRouterEvent = 1,
        /// Reserved.
        Reserved = 2,
        /// Inactive.
        Inactive = 3
    ],
    /// BOD trip level to generate an interrupt. See the LPC43xx data sheets for the tri
    BODLVL1 OFFSET(8) NUMBITS(2) [
        /// Level 0 interrupt
        Level0Interrupt = 0,
        /// Level 1 interrupt
        Level1Interrupt = 1,
        /// Level 2 interrupt
        Level2Interrupt = 2,
        /// Level 3 interrupt
        Level3Interrupt = 3
    ],
    /// BOD trip level to generate a reset. See the LPC43xx data sheets for the trip val
    BODLVL2 OFFSET(10) NUMBITS(2) [
        /// Level 0 reset
        Level0Reset = 0,
        /// Level 1 reset
        Level1Reset = 1,
        /// Level 2 reset
        Level2Reset = 2,
        /// Level 3 reset
        Level3Reset = 3
    ],
    /// SAMPLE pin input/output control
    SAMPLECTRL OFFSET(12) NUMBITS(2) [
        /// Reserved
        Reserved = 0,
        /// Sample output from the event monitor/recorder.
        SampleOutputFromTheEventMonitorRecorder = 1,
        /// Output from the event router.
        OutputFromTheEventRouter = 2,
        /// Reserved.
        Reserved = 3
    ],
    /// WAKEUP0 pin input/output control
    WAKEUP0CTRL OFFSET(14) NUMBITS(2) [
        /// Input to the event router.
        InputToTheEventRouter = 0,
        /// Output from the event router.
        OutputFromTheEventRouter = 1,
        /// Reserved.
        Reserved = 2
    ],
    /// WAKEUP1 pin input/output control
    WAKEUP1CTRL OFFSET(16) NUMBITS(2) [
        /// Input to event router.
        InputToEventRouter = 0,
        /// Output from the event router.
        OutputFromTheEventRouter = 1,
        /// Reserved
        Reserved = 2
    ]
],
CREG5 [
    /// JTAG debug disable for M0SUB co-processor. If this bit is set to 1, it can be ch
    M0SUBTAPSEL OFFSET(10) NUMBITS(1) [
        /// No effect.
        NoEffect = 0,
        /// Disable JTAG debug. Once JTAG is disabled, JTAG access remains disabled until th
        DISABLE_JTAG_DEBUG = 1
    ],
    /// JTAG debug disable for M4 main processor. If this bit is set to 1, it can be cha
    M4TAPSEL OFFSET(11) NUMBITS(1) [
        /// No effect.
        NoEffect = 0,
        /// Disable JTAG debug. Once JTAG is disabled, JTAG access remains disabled until th
        DISABLE_JTAG_DEBUG = 1
    ],
    /// JTAG debug disable for M0APPco-processor. If this bit is set to 1, it can be cha
    M0APPTAPSEL OFFSET(12) NUMBITS(1) [
        /// No effect.
        NoEffect = 0,
        /// Disable JTAG debug. Once JTAG is disabled, JTAG access remains disabled until th
        DISABLE_JTAG_DEBUG = 1
    ]
],
DMAMUX [
    /// Select DMA to peripheral connection for DMA peripheral 0.
    DMAMUXPER0 OFFSET(0) NUMBITS(2) [
        /// SPIFI
        SPIFI = 0,
        /// SCT CTOUT_2
        SCTCTOUT_2 = 1,
        /// SGPIO14
        SGPIO14 = 2,
        /// Timer3 match 1
        Timer3Match1 = 3
    ],
    /// Select DMA to peripheral connection for DMA peripheral 1
    DMAMUXPER1 OFFSET(2) NUMBITS(2) [
        /// Timer0 match 0
        Timer0Match0 = 0,
        /// USART0 transmit
        USART0Transmit = 1,
        /// Reserved
        Reserved = 2
    ],
    /// Select DMA to peripheral connection for DMA peripheral 2.
    DMAMUXPER2 OFFSET(4) NUMBITS(2) [
        /// Timer0 match 1
        Timer0Match1 = 0,
        /// USART0 receive
        USART0Receive = 1,
        /// Reserved
        Reserved = 2
    ],
    /// Select DMA to peripheral connection for DMA peripheral 3.
    DMAMUXPER3 OFFSET(6) NUMBITS(2) [
        /// Timer1 match 0
        Timer1Match0 = 0,
        /// UART1 transmit
        UART1Transmit = 1,
        /// I2S1 DMA request 1
        I2S1DMARequest1 = 2,
        /// SSP1 transmit
        SSP1Transmit = 3
    ],
    /// Select DMA to peripheral connection for DMA peripheral 4.
    DMAMUXPER4 OFFSET(8) NUMBITS(2) [
        /// Timer1 match 1
        Timer1Match1 = 0,
        /// UART1 receive
        UART1Receive = 1,
        /// I2S1 DMA request 2
        I2S1DMARequest2 = 2,
        /// SSP1 receive
        SSP1Receive = 3
    ],
    /// Select DMA to peripheral connection for DMA peripheral 5.
    DMAMUXPER5 OFFSET(10) NUMBITS(2) [
        /// Timer2 match 0
        Timer2Match0 = 0,
        /// USART2 transmit
        USART2Transmit = 1,
        /// SSP1 transmit
        SSP1Transmit = 2,
        /// SGPIO15
        SGPIO15 = 3
    ],
    /// Selects DMA to peripheral connection for DMA peripheral 6.
    DMAMUXPER6 OFFSET(12) NUMBITS(2) [
        /// Timer2 match 1
        Timer2Match1 = 0,
        /// USART2 receive
        USART2Receive = 1,
        /// SSP1 receive
        SSP1Receive = 2,
        /// SGPIO14
        SGPIO14 = 3
    ],
    /// Selects DMA to peripheral connection for DMA peripheral 7.
    DMAMUXPER7 OFFSET(14) NUMBITS(2) [
        /// Timer3 match 0
        Timer3Match0 = 0,
        /// USART3 transmit
        USART3Transmit = 1,
        /// SCT DMA request 0
        SCTDMARequest0 = 2,
        /// ADCHS write
        ADCHSWrite = 3
    ],
    /// Select DMA to peripheral connection for DMA peripheral 8.
    DMAMUXPER8 OFFSET(16) NUMBITS(2) [
        /// Timer3 match 1
        Timer3Match1 = 0,
        /// USART3 receive
        USART3Receive = 1,
        /// SCT DMA request 1
        SCTDMARequest1 = 2,
        /// ADCHS read
        ADCHSRead = 3
    ],
    /// Select DMA to peripheral connection for DMA peripheral 9.
    DMAMUXPER9 OFFSET(18) NUMBITS(2) [
        /// SSP0 receive
        SSP0Receive = 0,
        /// I2S0 DMA request 1
        I2S0DMARequest1 = 1,
        /// SCT DMA request 1
        SCTDMARequest1 = 2,
        /// Reserved
        Reserved = 3
    ],
    /// Select DMA to peripheral connection for DMA peripheral 10.
    DMAMUXPER10 OFFSET(20) NUMBITS(2) [
        /// SSP0 transmit
        SSP0Transmit = 0,
        /// I2S0 DMA request 2
        I2S0DMARequest2 = 1,
        /// SCT DMA request 0
        SCTDMARequest0 = 2,
        /// Reserved
        Reserved = 3
    ],
    /// Selects DMA to peripheral connection for DMA peripheral 11.
    DMAMUXPER11 OFFSET(22) NUMBITS(2) [
        /// SSP1 receive
        SSP1Receive = 0,
        /// SGPIO14
        SGPIO14 = 1,
        /// USART0 transmit
        USART0Transmit = 2,
        /// Reserved
        Reserved = 3
    ],
    /// Select DMA to peripheral connection for DMA peripheral 12.
    DMAMUXPER12 OFFSET(24) NUMBITS(2) [
        /// SSP1 transmit
        SSP1Transmit = 0,
        /// SGPIO15
        SGPIO15 = 1,
        /// USART0 receive
        USART0Receive = 2,
        /// Reserved
        Reserved = 3
    ],
    /// Select DMA to peripheral connection for DMA peripheral 13.
    DMAMUXPER13 OFFSET(26) NUMBITS(2) [
        /// ADC0
        ADC0 = 0,
        /// Reserved
        Reserved = 1,
        /// SSP1 receive
        SSP1Receive = 2,
        /// USART3 receive
        USART3Receive = 3
    ],
    /// Select DMA to peripheral connection for DMA peripheral 14.
    DMAMUXPER14 OFFSET(28) NUMBITS(2) [
        /// ADC1
        ADC1 = 0,
        /// Reserved
        Reserved = 1,
        /// SSP1 transmit
        SSP1Transmit = 2,
        /// USART3 transmit
        USART3Transmit = 3
    ],
    /// Select DMA to peripheral connection for DMA peripheral 15.
    DMAMUXPER15 OFFSET(30) NUMBITS(2) [
        /// DAC
        DAC = 0,
        /// SCT CTOUT_3
        SCTCTOUT_3 = 1,
        /// SGPIO15
        SGPIO15 = 2,
        /// Timer3 match 0
        Timer3Match0 = 3
    ]
],
FLASHCFGA [
    /// Flash access time. The value of this field plus 1 gives the number of BASE_M4_CL
    FLASHTIM OFFSET(12) NUMBITS(4) [
        /// 1 BASE_M4_CLK clock. Use for BASE_M4_CLK up to 21 MHz.
        _1BASE_M4_CLKClockUseForBASE_M4_CLKUpTo21MHz = 0,
        /// 2 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 43 MHz.
        _2BASE_M4_CLKClocksUseForBASE_M4_CLKUpTo43MHz = 1,
        /// 3 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 64 MHz.
        _3BASE_M4_CLKClocksUseForBASE_M4_CLKUpTo64MHz = 2,
        /// 4 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 86 MHz.
        _4BASE_M4_CLKClocksUseForBASE_M4_CLKUpTo86MHz = 3,
        /// 5 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 107 MHz.
        _5BASE_M4_CLKClocksUseForBASE_M4_CLKUpTo107MHz = 4,
        /// 6 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 129 MHz.
        _6BASE_M4_CLKClocksUseForBASE_M4_CLKUpTo129MHz = 5,
        /// 7 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 150 MHz.
        _7BASE_M4_CLKClocksUseForBASE_M4_CLKUpTo150MHz = 6,
        /// 8 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 172 MHz.
        _8BASE_M4_CLKClocksUseForBASE_M4_CLKUpTo172MHz = 7,
        /// 9 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 193 MHz.
        _9BASE_M4_CLKClocksUseForBASE_M4_CLKUpTo193MHz = 8,
        /// 10 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 204 MHz. Safe setting for all a
        _10_BASE_M4_CLK_CLOCK = 9
    ],
    /// Flash bank A power control
    POW OFFSET(31) NUMBITS(1) [
        /// Power-down
        PowerDown = 0,
        /// Active (Default)
        ActiveDefault = 1
    ]
],
FLASHCFGB [
    /// Flash access time. The value of this field plus 1 gives the number of BASE_M4_CL
    FLASHTIM OFFSET(12) NUMBITS(4) [
        /// 1 BASE_M4_CLK clock. Use for BASE_M4_CLK up to 21 MHz.
        _1BASE_M4_CLKClockUseForBASE_M4_CLKUpTo21MHz = 0,
        /// 2 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 43 MHz.
        _2BASE_M4_CLKClocksUseForBASE_M4_CLKUpTo43MHz = 1,
        /// 3 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 64 MHz.
        _3BASE_M4_CLKClocksUseForBASE_M4_CLKUpTo64MHz = 2,
        /// 4 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 86 MHz.
        _4BASE_M4_CLKClocksUseForBASE_M4_CLKUpTo86MHz = 3,
        /// 5 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 107 MHz.
        _5BASE_M4_CLKClocksUseForBASE_M4_CLKUpTo107MHz = 4,
        /// 6 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 129 MHz.
        _6BASE_M4_CLKClocksUseForBASE_M4_CLKUpTo129MHz = 5,
        /// 7 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 150 MHz.
        _7BASE_M4_CLKClocksUseForBASE_M4_CLKUpTo150MHz = 6,
        /// 8 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 172 MHz.
        _8BASE_M4_CLKClocksUseForBASE_M4_CLKUpTo172MHz = 7,
        /// 9 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 193 MHz.
        _9BASE_M4_CLKClocksUseForBASE_M4_CLKUpTo193MHz = 8,
        /// 10 BASE_M4_CLK clocks. Use for BASE_M4_CLK up to 204 MHz. Safe setting for all a
        _10_BASE_M4_CLK_CLOCK = 9
    ],
    /// Flash bank A power control
    POW OFFSET(31) NUMBITS(1) [
        /// Power-down
        PowerDown = 0,
        /// Active (Default)
        ActiveDefault = 1
    ]
],
CREG6 [
    /// Selects the Ethernet mode. Reset the ethernet after changing the PHY interface.
    ETHMODE OFFSET(0) NUMBITS(3) [
        /// MII
        MII = 0,
        /// RMII
        RMII = 4
    ],
    /// Selects the functionality of the SCT outputs.
    CTOUTCTRL OFFSET(4) NUMBITS(1) [
        /// Combine SCT and timer match outputs. SCT outputs are Red with timer outputs.
        CombineSCTAndTimerMatchOutputsSCTOutputsAreRedWithTimerOutputs = 0,
        /// SCT outputs only. SCT outputs are used without timer match outputs.
        SCTOutputsOnlySCTOutputsAreUsedWithoutTimerMatchOutputs = 1
    ],
    /// I2S0_TX_SCK input select
    I2S0_TX_SCK_IN_SEL OFFSET(12) NUMBITS(1) [
        /// I2S Register. I2S clock selected as defined by the I2S transmit mode register Ta
        I2SRegisterI2SClockSelectedAsDefinedByTheI2STransmitModeRegisterTable960 = 0,
        /// BASE_AUDIO_CLK for I2S transmit clock MCLK input and MCLK output. The I2S must b
        BASE_AUDIO_CLK_FOR_I = 1
    ],
    /// I2S0_RX_SCK input select
    I2S0_RX_SCK_IN_SEL OFFSET(13) NUMBITS(1) [
        /// I2S Register. I2S clock selected as defined by the I2S receive mode register Tab
        I2SRegisterI2SClockSelectedAsDefinedByTheI2SReceiveModeRegisterTable961 = 0,
        /// BASE_AUDIO_CLK for I2S receive clock MCLK input and MCLK output. The I2S must be
        BASE_AUDIO_CLK_FOR_I = 1
    ],
    /// I2S1_TX_SCK input select
    I2S1_TX_SCK_IN_SEL OFFSET(14) NUMBITS(1) [
        /// I2S register. I2S clock selected as defined by the I2S transmit mode register Ta
        I2SRegisterI2SClockSelectedAsDefinedByTheI2STransmitModeRegisterTable960 = 0,
        /// BASE_AUDIO_CLK for I2S transmit clock MCLK input and MCLK output. The I2S must b
        BASE_AUDIO_CLK_FOR_I = 1
    ],
    /// I2S1_RX_SCK input select
    I2S1_RX_SCK_IN_SEL OFFSET(15) NUMBITS(1) [
        /// I2S register. I2S clock selected as defined by the I2S receive mode register Tab
        I2SRegisterI2SClockSelectedAsDefinedByTheI2SReceiveModeRegisterTable961 = 0,
        /// BASE_AUDIO_CLK for I2S receive clock MCLK input and MCLK output. The I2S must be
        BASE_AUDIO_CLK_FOR_I = 1
    ],
    /// EMC_CLK divided clock select (see Section 21.1).
    EMC_CLK_SEL OFFSET(16) NUMBITS(1) [
        /// Divide by 1. EMC_CLK_DIV not divided.
        DivideBy1EMC_CLK_DIVNotDivided = 0,
        /// Divide by 2. EMC_CLK_DIV divided by 2.
        DivideBy2EMC_CLK_DIVDividedBy2 = 1
    ]
]
];
const CREG_BASE: StaticRef<CregRegisters> =
    unsafe { StaticRef::new(0x40043000 as *const CregRegisters) };
