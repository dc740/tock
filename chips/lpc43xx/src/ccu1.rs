
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// Clock Control Unit (CCU)
#[repr(C)]
struct Ccu1Registers {
/// CCU1 power mode register
pm: ReadWrite<u32>,
/// CCU1 base clocks status register
base_stat: ReadOnly<u32, BASE_STAT::Register>,
_reserved0: [u8; 248],
/// CLK_APB3_BUS clock configuration register
clk_apb3_bus_cfg: ReadWrite<u32, CLK_APB3_BUS_CFG::Register>,
/// CLK_APB3_BUS clock status register
clk_apb3_bus_stat: ReadOnly<u32, CLK_APB3_BUS_STAT::Register>,
/// CLK_APB3_I2C1 clock configuration register
clk_apb3_i2c1_cfg: ReadWrite<u32, CLK_APB3_I2C1_CFG::Register>,
/// CLK_APB3_I2C1 clock status register
clk_apb3_i2c1_stat: ReadOnly<u32, CLK_APB3_I2C1_STAT::Register>,
/// CLK_APB3_DAC clock configuration register
clk_apb3_dac_cfg: ReadWrite<u32, CLK_APB3_DAC_CFG::Register>,
/// CLK_APB3_DAC clock status register
clk_apb3_dac_stat: ReadOnly<u32, CLK_APB3_DAC_STAT::Register>,
/// CLK_APB3_ADC0 clock configuration register
clk_apb3_adc0_cfg: ReadWrite<u32, CLK_APB3_ADC0_CFG::Register>,
/// CLK_APB3_ADC0 clock status register
clk_apb3_adc0_stat: ReadOnly<u32, CLK_APB3_ADC0_STAT::Register>,
/// CLK_APB3_ADC1 clock configuration register
clk_apb3_adc1_cfg: ReadWrite<u32, CLK_APB3_ADC1_CFG::Register>,
/// CLK_APB3_ADC1 clock status register
clk_apb3_adc1_stat: ReadOnly<u32, CLK_APB3_ADC1_STAT::Register>,
/// CLK_APB3_CAN0 clock configuration register
clk_apb3_can0_cfg: ReadWrite<u32, CLK_APB3_CAN0_CFG::Register>,
/// CLK_APB3_CAN0 clock status register
clk_apb3_can0_stat: ReadOnly<u32, CLK_APB3_CAN0_STAT::Register>,
_reserved1: [u8; 208],
/// CLK_APB1_BUS clock configuration register
clk_apb1_bus_cfg: ReadWrite<u32, CLK_APB1_BUS_CFG::Register>,
/// CLK_APB1_BUS clock status register
clk_apb1_bus_stat: ReadOnly<u32, CLK_APB1_BUS_STAT::Register>,
/// CLK_APB1_MOTOCONPWM clock configuration register
clk_apb1_motoconpwm_cfg: ReadWrite<u32, CLK_APB1_MOTOCONPWM_CFG::Register>,
/// CLK_APB1_MOTOCONPWM clock status register
clk_apb1_motoconpwm_stat: ReadOnly<u32, CLK_APB1_MOTOCONPWM_STAT::Register>,
/// CLK_ABP1_I2C0 clock configuration register
clk_apb1_i2c0_cfg: ReadWrite<u32, CLK_APB1_I2C0_CFG::Register>,
/// CLK_APB1_I2C0 clock status register
clk_apb1_i2c0_stat: ReadOnly<u32, CLK_APB1_I2C0_STAT::Register>,
/// CLK_APB1_I2S clock configuration register
clk_apb1_i2s_cfg: ReadWrite<u32, CLK_APB1_I2S_CFG::Register>,
/// CLK_APB1_I2S clock status register
clk_apb1_i2s_stat: ReadOnly<u32, CLK_APB1_I2S_STAT::Register>,
/// CLK_APB1_CAN1 clock configuration register
clk_apb1_can1_cfg: ReadWrite<u32, CLK_APB1_CAN1_CFG::Register>,
/// CLK_APB1_CAN1 clock status register
clk_apb1_can1_stat: ReadOnly<u32, CLK_APB1_CAN1_STAT::Register>,
_reserved2: [u8; 216],
/// CLK_SPIFI clock configuration register
clk_spifi_cfg: ReadWrite<u32, CLK_SPIFI_CFG::Register>,
/// CLK_APB1_SPIFI clock status register
clk_spifi_stat: ReadOnly<u32, CLK_SPIFI_STAT::Register>,
_reserved3: [u8; 248],
/// CLK_M4_BUS clock configuration register
clk_m4_bus_cfg: ReadWrite<u32, CLK_M4_BUS_CFG::Register>,
/// CLK_M4_BUSclock status register
clk_m4_bus_stat: ReadOnly<u32, CLK_M4_BUS_STAT::Register>,
/// CLK_M4_SPIFI clock configuration register
clk_m4_spifi_cfg: ReadWrite<u32, CLK_M4_SPIFI_CFG::Register>,
/// CLK_M4_SPIFI clock status register
clk_m4_spifi_stat: ReadOnly<u32, CLK_M4_SPIFI_STAT::Register>,
/// CLK_M4_GPIO clock configuration register
clk_m4_gpio_cfg: ReadWrite<u32, CLK_M4_GPIO_CFG::Register>,
/// CLK_M4_GPIO clock status register
clk_m4_gpio_stat: ReadOnly<u32, CLK_M4_GPIO_STAT::Register>,
/// CLK_M4_LCD clock configuration register
clk_m4_lcd_cfg: ReadWrite<u32, CLK_M4_LCD_CFG::Register>,
/// CLK_M4_LCD clock status register
clk_m4_lcd_stat: ReadOnly<u32, CLK_M4_LCD_STAT::Register>,
/// CLK_M4_ETHERNET clock configuration register
clk_m4_ethernet_cfg: ReadWrite<u32, CLK_M4_ETHERNET_CFG::Register>,
/// CLK_M4_ETHERNET clock status register
clk_m4_ethernet_stat: ReadOnly<u32, CLK_M4_ETHERNET_STAT::Register>,
/// CLK_M4_USB0 clock configuration register
clk_m4_usb0_cfg: ReadWrite<u32, CLK_M4_USB0_CFG::Register>,
/// CLK_M4_USB0  clock status register
clk_m4_usb0_stat: ReadOnly<u32, CLK_M4_USB0_STAT::Register>,
/// CLK_M4_EMC clock configuration register
clk_m4_emc_cfg: ReadWrite<u32, CLK_M4_EMC_CFG::Register>,
/// CLK_M4_EMC clock status register
clk_m4_emc_stat: ReadOnly<u32, CLK_M4_EMC_STAT::Register>,
/// CLK_M4_SDIO clock configuration register
clk_m4_sdio_cfg: ReadWrite<u32, CLK_M4_SDIO_CFG::Register>,
/// CLK_M4_SDIO clock status register
clk_m4_sdio_stat: ReadOnly<u32, CLK_M4_SDIO_STAT::Register>,
/// CLK_M4_DMA clock configuration register
clk_m4_dma_cfg: ReadWrite<u32, CLK_M4_DMA_CFG::Register>,
/// CLK_M4_DMA clock status register
clk_m4_dma_stat: ReadOnly<u32, CLK_M4_DMA_STAT::Register>,
/// CLK_M4_M4CORE clock configuration register
clk_m4_m4core_cfg: ReadWrite<u32, CLK_M4_M4CORE_CFG::Register>,
/// CLK_M4_M3CORE clock status register
clk_m4_m4core_stat: ReadOnly<u32, CLK_M4_M4CORE_STAT::Register>,
_reserved4: [u8; 24],
/// CLK_M4_SCT clock configuration register
clk_m4_sct_cfg: ReadWrite<u32, CLK_M4_SCT_CFG::Register>,
/// CLK_M4_SCT clock status register
clk_m4_sct_stat: ReadOnly<u32, CLK_M4_SCT_STAT::Register>,
/// CLK_M4_USB1 clock configuration register
clk_m4_usb1_cfg: ReadWrite<u32, CLK_M4_USB1_CFG::Register>,
/// CLK_M4_USB1 clock status register
clk_m4_usb1_stat: ReadOnly<u32, CLK_M4_USB1_STAT::Register>,
/// CLK_M4_EMCDIV clock configuration register
clk_m4_emcdiv_cfg: ReadWrite<u32, CLK_M4_EMCDIV_CFG::Register>,
/// CLK_M4_EMCDIV clock status register
clk_m4_emcdiv_stat: ReadOnly<u32, CLK_M4_EMCDIV_STAT::Register>,
/// CLK_M4_FLASHA clock configuration register
clk_m4_flasha_cfg: ReadWrite<u32, CLK_M4_FLASHA_CFG::Register>,
/// CLK_M4_FLASHA clock status register
clk_m4_flasha_stat: ReadOnly<u32, CLK_M4_FLASHA_STAT::Register>,
/// CLK_M4_FLASHB clock configuration register
clk_m4_flashb_cfg: ReadWrite<u32, CLK_M4_FLASHB_CFG::Register>,
/// CLK_M4_FLASHB clock status register
clk_m4_flashb_stat: ReadOnly<u32, CLK_M4_FLASHB_STAT::Register>,
/// CLK_M0APP_CFG clock configuration register
clk_m4_m0app_cfg: ReadWrite<u32, CLK_M4_M0APP_CFG::Register>,
/// CLK_M4_MOAPP clock status register
clk_m4_m0app_stat: ReadOnly<u32, CLK_M4_M0APP_STAT::Register>,
/// CLK_ADCHS_CFG clock configuration register
clk_m4_adchs_cfg: ReadWrite<u32, CLK_M4_ADCHS_CFG::Register>,
/// CLK_M4_ADCHS clock status register
clk_m4_adchs_stat: ReadOnly<u32, CLK_M4_ADCHS_STAT::Register>,
/// CLK_EEPROM_CFG clock configuration register
clk_m4_eeprom_cfg: ReadWrite<u32, CLK_M4_EEPROM_CFG::Register>,
/// CLK_M4_EEPROM clock status register
clk_m4_eeprom_stat: ReadOnly<u32, CLK_M4_EEPROM_STAT::Register>,
_reserved5: [u8; 88],
/// CLK_M4_WWDT clock configuration register
clk_m4_wwdt_cfg: ReadWrite<u32, CLK_M4_WWDT_CFG::Register>,
/// CLK_M4_WWDT clock status register
clk_m4_wwdt_stat: ReadOnly<u32, CLK_M4_WWDT_STAT::Register>,
/// CLK_M4_USART0 clock configuration register
clk_m4_usart0_cfg: ReadWrite<u32, CLK_M4_USART0_CFG::Register>,
/// CLK_M4_USART0 clock status register
clk_m4_usart0_stat: ReadOnly<u32, CLK_M4_USART0_STAT::Register>,
/// CLK_M4_UART1 clock configuration register
clk_m4_uart1_cfg: ReadWrite<u32, CLK_M4_UART1_CFG::Register>,
/// CLK_M4_UART1 clock status register
clk_m4_uart1_stat: ReadOnly<u32, CLK_M4_UART1_STAT::Register>,
/// CLK_M4_SSP0 clock configuration register
clk_m4_ssp0_cfg: ReadWrite<u32, CLK_M4_SSP0_CFG::Register>,
/// CLK_M4_SSP0 clock status register
clk_m4_ssp0_stat: ReadOnly<u32, CLK_M4_SSP0_STAT::Register>,
/// CLK_M4_TIMER0 clock configuration register
clk_m4_timer0_cfg: ReadWrite<u32, CLK_M4_TIMER0_CFG::Register>,
/// CLK_M4_TIMER0 clock status register
clk_m4_timer0_stat: ReadOnly<u32, CLK_M4_TIMER0_STAT::Register>,
/// CLK_M4_TIMER1clock configuration register
clk_m4_timer1_cfg: ReadWrite<u32, CLK_M4_TIMER1_CFG::Register>,
/// CLK_M4_TIMER1 clock status register
clk_m4_timer1_stat: ReadOnly<u32, CLK_M4_TIMER1_STAT::Register>,
/// CLK_M4_SCU clock configuration register
clk_m4_scu_cfg: ReadWrite<u32, CLK_M4_SCU_CFG::Register>,
/// CLK_SCU_XXX clock status register
clk_m4_scu_stat: ReadOnly<u32, CLK_M4_SCU_STAT::Register>,
/// CLK_M4_CREGclock configuration register
clk_m4_creg_cfg: ReadWrite<u32, CLK_M4_CREG_CFG::Register>,
/// CLK_M4_CREG clock status register
clk_m4_creg_stat: ReadOnly<u32, CLK_M4_CREG_STAT::Register>,
_reserved6: [u8; 192],
/// CLK_M4_RITIMER clock configuration register
clk_m4_ritimer_cfg: ReadWrite<u32, CLK_M4_RITIMER_CFG::Register>,
/// CLK_M4_RITIMER clock status register
clk_m4_ritimer_stat: ReadOnly<u32, CLK_M4_RITIMER_STAT::Register>,
/// CLK_M4_USART2 clock configuration register
clk_m4_usart2_cfg: ReadWrite<u32, CLK_M4_USART2_CFG::Register>,
/// CLK_M4_USART2 clock status register
clk_m4_usart2_stat: ReadOnly<u32, CLK_M4_USART2_STAT::Register>,
/// CLK_M4_USART3 clock configuration register
clk_m4_usart3_cfg: ReadWrite<u32, CLK_M4_USART3_CFG::Register>,
/// CLK_M4_USART3 clock status register
clk_m4_usart3_stat: ReadOnly<u32, CLK_M4_USART3_STAT::Register>,
/// CLK_M4_TIMER2 clock configuration register
clk_m4_timer2_cfg: ReadWrite<u32, CLK_M4_TIMER2_CFG::Register>,
/// CLK_M4_TIMER2 clock status register
clk_m4_timer2_stat: ReadOnly<u32, CLK_M4_TIMER2_STAT::Register>,
/// CLK_M4_TIMER3 clock configuration register
clk_m4_timer3_cfg: ReadWrite<u32, CLK_M4_TIMER3_CFG::Register>,
/// CLK_M4_TIMER3 clock status register
clk_m4_timer3_stat: ReadOnly<u32, CLK_M4_TIMER3_STAT::Register>,
/// CLK_M4_SSP1 clock configuration register
clk_m4_ssp1_cfg: ReadWrite<u32, CLK_M4_SSP1_CFG::Register>,
/// CLK_M4_SSP1 clock status register
clk_m4_ssp1_stat: ReadOnly<u32, CLK_M4_SSP1_STAT::Register>,
/// CLK_M4_QEIclock configuration register
clk_m4_qei_cfg: ReadWrite<u32, CLK_M4_QEI_CFG::Register>,
/// CLK_M4_QEI clock status register
clk_m4_qei_stat: ReadOnly<u32, CLK_M4_QEI_STAT::Register>,
_reserved7: [u8; 200],
/// CLK_PERIPH_BUS_CFG clock configuration register
clk_periph_bus_cfg: ReadWrite<u32, CLK_PERIPH_BUS_CFG::Register>,
/// CLK_PERIPH_BUS_STAT clock status register
clk_periph_bus_stat: ReadOnly<u32, CLK_PERIPH_BUS_STAT::Register>,
_reserved8: [u8; 8],
/// CLK_PERIPH_CORE_CFG clock configuration register
clk_periph_core_cfg: ReadWrite<u32, CLK_PERIPH_CORE_CFG::Register>,
/// CLK_CORE_BUS_STAT clock status register
clk_periph_core_stat: ReadOnly<u32, CLK_PERIPH_CORE_STAT::Register>,
/// CLK_PERIPH_SGPIO_CFG clock configuration register
clk_periph_sgpio_cfg: ReadWrite<u32, CLK_PERIPH_SGPIO_CFG::Register>,
/// CLK_CORE_SGPIO_STAT clock status register
clk_periph_sgpio_stat: ReadOnly<u32, CLK_PERIPH_SGPIO_STAT::Register>,
_reserved9: [u8; 224],
/// CLK_M4_USB0 clock configuration register
clk_usb0_cfg: ReadWrite<u32, CLK_USB0_CFG::Register>,
/// CLK_USB0 clock status register
clk_usb0_stat: ReadOnly<u32, CLK_USB0_STAT::Register>,
_reserved10: [u8; 248],
/// CLK_USB1 clock configuration register
clk_usb1_cfg: ReadWrite<u32, CLK_USB1_CFG::Register>,
/// CLK_USB1 clock status register
clk_usb1_stat: ReadOnly<u32, CLK_USB1_STAT::Register>,
_reserved11: [u8; 248],
/// CLK_SPI clock configuration register
clk_spi_cfg: ReadWrite<u32, CLK_SPI_CFG::Register>,
/// CLK_SPI clock status register
clk_spi_stat: ReadOnly<u32, CLK_SPI_STAT::Register>,
_reserved12: [u8; 248],
/// CLK_ADCHS clock configuration register
clk_adchs_cfg: ReadWrite<u32, CLK_ADCHS_CFG::Register>,
/// CLK_ADCHS clock status register
clk_adchs_stat: ReadOnly<u32, CLK_ADCHS_STAT::Register>,
}
register_bitfields![u32,
BASE_STAT [
    /// Base clock indicator for BASE_APB3_CLK 0 = All branch clocks switched off. 1 = A
    BASE_APB3_CLK_IND OFFSET(0) NUMBITS(1) [],
    /// Base clock indicator for BASE_APB1_CLK 0 = All branch clocks switched off. 1 = A
    BASE_APB1_CLK_IND OFFSET(1) NUMBITS(1) [],
    /// Base clock indicator for BASE_SPIFI_CLK 0 = All branch clocks switched off. 1 =
    BASE_SPIFI_CLK_IND OFFSET(2) NUMBITS(1) [],
    /// Base clock indicator for BASE_M3_CLK 0 = All branch clocks switched off. 1 = At
    BASE_M3_CLK_IND OFFSET(3) NUMBITS(1) [],
    /// Base clock indicator for BASE_USB0_CLK 0 = All branch clocks switched off. 1 = A
    BASE_USB0_CLK_IND OFFSET(7) NUMBITS(1) [],
    /// Base clock indicator for BASE_USB1_CLK 0 = All branch clocks switched off. 1 = a
    BASE_USB1_CLK_IND OFFSET(8) NUMBITS(1) []
],
CLK_APB3_BUS_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_APB3_I2C1_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_APB3_DAC_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_APB3_ADC0_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_APB3_ADC1_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_APB3_CAN0_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_APB1_BUS_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_APB1_MOTOCONPWM_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_APB1_I2C0_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_APB1_I2S_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_APB1_CAN1_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_SPIFI_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_BUS_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_SPIFI_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_GPIO_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_LCD_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_ETHERNET_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_USB0_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_EMC_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_SDIO_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_DMA_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_M4CORE_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_SCT_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_USB1_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_EMCDIV_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ],
    /// Clock divider value
    DIV OFFSET(5) NUMBITS(3) [
        /// No division. Divide by 1.
        NoDivisionDivideBy1 = 0,
        /// Divide by 2.
        DivideBy2 = 1
    ]
],
CLK_M4_FLASHA_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_FLASHB_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_M0APP_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_ADCHS_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_EEPROM_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_WWDT_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_USART0_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_UART1_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_SSP0_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_TIMER0_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_TIMER1_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_SCU_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_CREG_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_RITIMER_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_USART2_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_USART3_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_TIMER2_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_TIMER3_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_SSP1_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_M4_QEI_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_PERIPH_BUS_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_PERIPH_CORE_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_PERIPH_SGPIO_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_USB0_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_USB1_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_SPI_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_ADCHS_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_APB3_BUS_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_APB3_I2C1_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_APB3_DAC_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_APB3_ADC0_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_APB3_ADC1_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_APB3_CAN0_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_APB1_BUS_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_APB1_MOTOCONPWM_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_APB1_I2C0_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_APB1_I2S_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_APB1_CAN1_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_SPIFI_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_BUS_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_SPIFI_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_GPIO_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_LCD_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_ETHERNET_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_USB0_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_EMC_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_SDIO_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_DMA_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_M4CORE_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_SCT_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_USB1_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_EMCDIV_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_FLASHA_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_FLASHB_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_M0APP_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_ADCHS_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_EEPROM_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_WWDT_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_USART0_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_UART1_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_SSP0_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_TIMER0_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_TIMER1_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_SCU_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_CREG_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_RITIMER_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_USART2_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_USART3_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_TIMER2_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_TIMER3_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_SSP1_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_M4_QEI_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_PERIPH_BUS_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_PERIPH_CORE_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_PERIPH_SGPIO_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_USB0_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_USB1_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_SPI_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_ADCHS_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
]
];
const CCU1_BASE: StaticRef<Ccu1Registers> =
    unsafe { StaticRef::new(0x40051000 as *const Ccu1Registers) };
