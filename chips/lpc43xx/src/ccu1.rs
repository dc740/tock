
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly, register_bitfields};

use cgu;
    /// Clock Control Unit (CCU)
#[repr(C)]
struct Ccu1Registers {
/// CCU1 power mode register
pm: ReadWrite<u32>,
/// CCU1 base clocks status register
base_stat: ReadOnly<u32, BASE_STAT::Register>,
_reserved0: [u8; 248],
/// CLK_APB3_BUS clock configuration register
clk_apb3_bus_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_APB3_BUS clock status register
clk_apb3_bus_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_APB3_I2C1 clock configuration register
clk_apb3_i2c1_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_APB3_I2C1 clock status register
clk_apb3_i2c1_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_APB3_DAC clock configuration register
clk_apb3_dac_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_APB3_DAC clock status register
clk_apb3_dac_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_APB3_ADC0 clock configuration register
clk_apb3_adc0_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_APB3_ADC0 clock status register
clk_apb3_adc0_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_APB3_ADC1 clock configuration register
clk_apb3_adc1_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_APB3_ADC1 clock status register
clk_apb3_adc1_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_APB3_CAN0 clock configuration register
clk_apb3_can0_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_APB3_CAN0 clock status register
clk_apb3_can0_stat: ReadOnly<u32, CLK_CFG::Register>,
_reserved1: [u8; 208],
/// CLK_APB1_BUS clock configuration register
clk_apb1_bus_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_APB1_BUS clock status register
clk_apb1_bus_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_APB1_MOTOCONPWM clock configuration register
clk_apb1_motoconpwm_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_APB1_MOTOCONPWM clock status register
clk_apb1_motoconpwm_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_ABP1_I2C0 clock configuration register
clk_apb1_i2c0_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_APB1_I2C0 clock status register
clk_apb1_i2c0_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_APB1_I2S clock configuration register
clk_apb1_i2s_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_APB1_I2S clock status register
clk_apb1_i2s_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_APB1_CAN1 clock configuration register
clk_apb1_can1_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_APB1_CAN1 clock status register
clk_apb1_can1_stat: ReadOnly<u32, CLK_CFG::Register>,
_reserved2: [u8; 216],
/// CLK_SPIFI clock configuration register
clk_spifi_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_APB1_SPIFI clock status register
clk_spifi_stat: ReadOnly<u32, CLK_CFG::Register>,
_reserved3: [u8; 248],
/// CLK_M4_BUS clock configuration register
clk_m4_bus_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_BUSclock status register
clk_m4_bus_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_SPIFI clock configuration register
clk_m4_spifi_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_SPIFI clock status register
clk_m4_spifi_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_GPIO clock configuration register
clk_m4_gpio_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_GPIO clock status register
clk_m4_gpio_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_LCD clock configuration register
clk_m4_lcd_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_LCD clock status register
clk_m4_lcd_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_ETHERNET clock configuration register
clk_m4_ethernet_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_ETHERNET clock status register
clk_m4_ethernet_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_USB0 clock configuration register
clk_m4_usb0_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_USB0  clock status register
clk_m4_usb0_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_EMC clock configuration register
clk_m4_emc_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_EMC clock status register
clk_m4_emc_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_SDIO clock configuration register
clk_m4_sdio_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_SDIO clock status register
clk_m4_sdio_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_DMA clock configuration register
clk_m4_dma_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_DMA clock status register
clk_m4_dma_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_M4CORE clock configuration register
clk_m4_m4core_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_M3CORE clock status register
clk_m4_m4core_stat: ReadOnly<u32, CLK_CFG::Register>,
_reserved4: [u8; 24],
/// CLK_M4_SCT clock configuration register
clk_m4_sct_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_SCT clock status register
clk_m4_sct_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_USB1 clock configuration register
clk_m4_usb1_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_USB1 clock status register
clk_m4_usb1_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_EMCDIV clock configuration register
clk_m4_emcdiv_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_EMCDIV clock status register
clk_m4_emcdiv_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_FLASHA clock configuration register
clk_m4_flasha_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_FLASHA clock status register
clk_m4_flasha_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_FLASHB clock configuration register
clk_m4_flashb_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_FLASHB clock status register
clk_m4_flashb_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M0APP_CFG clock configuration register
clk_m4_m0app_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_MOAPP clock status register
clk_m4_m0app_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_ADCHS_CFG clock configuration register
clk_m4_adchs_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_ADCHS clock status register
clk_m4_adchs_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_EEPROM_CFG clock configuration register
clk_m4_eeprom_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_EEPROM clock status register
clk_m4_eeprom_stat: ReadOnly<u32, CLK_CFG::Register>,
_reserved5: [u8; 88],
/// CLK_M4_WWDT clock configuration register
clk_m4_wwdt_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_WWDT clock status register
clk_m4_wwdt_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_USART0 clock configuration register
clk_m4_usart0_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_USART0 clock status register
clk_m4_usart0_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_UART1 clock configuration register
clk_m4_uart1_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_UART1 clock status register
clk_m4_uart1_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_SSP0 clock configuration register
clk_m4_ssp0_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_SSP0 clock status register
clk_m4_ssp0_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_TIMER0 clock configuration register
clk_m4_timer0_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_TIMER0 clock status register
clk_m4_timer0_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_TIMER1clock configuration register
clk_m4_timer1_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_TIMER1 clock status register
clk_m4_timer1_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_SCU clock configuration register
clk_m4_scu_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_SCU_XXX clock status register
clk_m4_scu_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_CREGclock configuration register
clk_m4_creg_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_CREG clock status register
clk_m4_creg_stat: ReadOnly<u32, CLK_CFG::Register>,
_reserved6: [u8; 192],
/// CLK_M4_RITIMER clock configuration register
clk_m4_ritimer_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_RITIMER clock status register
clk_m4_ritimer_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_USART2 clock configuration register
clk_m4_usart2_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_USART2 clock status register
clk_m4_usart2_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_USART3 clock configuration register
clk_m4_usart3_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_USART3 clock status register
clk_m4_usart3_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_TIMER2 clock configuration register
clk_m4_timer2_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_TIMER2 clock status register
clk_m4_timer2_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_TIMER3 clock configuration register
clk_m4_timer3_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_TIMER3 clock status register
clk_m4_timer3_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_SSP1 clock configuration register
clk_m4_ssp1_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_SSP1 clock status register
clk_m4_ssp1_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_M4_QEIclock configuration register
clk_m4_qei_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_M4_QEI clock status register
clk_m4_qei_stat: ReadOnly<u32, CLK_CFG::Register>,
_reserved7: [u8; 200],
/// CLK_PERIPH_BUS_CFG clock configuration register
clk_periph_bus_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_PERIPH_BUS_STAT clock status register
clk_periph_bus_stat: ReadOnly<u32, CLK_CFG::Register>,
_reserved8: [u8; 8],
/// CLK_PERIPH_CORE_CFG clock configuration register
clk_periph_core_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_CORE_BUS_STAT clock status register
clk_periph_core_stat: ReadOnly<u32, CLK_CFG::Register>,
/// CLK_PERIPH_SGPIO_CFG clock configuration register
clk_periph_sgpio_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_CORE_SGPIO_STAT clock status register
clk_periph_sgpio_stat: ReadOnly<u32, CLK_CFG::Register>,
_reserved9: [u8; 224],
/// CLK_M4_USB0 clock configuration register
clk_usb0_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_USB0 clock status register
clk_usb0_stat: ReadOnly<u32, CLK_CFG::Register>,
_reserved10: [u8; 248],
/// CLK_USB1 clock configuration register
clk_usb1_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_USB1 clock status register
clk_usb1_stat: ReadOnly<u32, CLK_CFG::Register>,
_reserved11: [u8; 248],
/// CLK_SPI clock configuration register
clk_spi_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_SPI clock status register
clk_spi_stat: ReadOnly<u32, CLK_CFG::Register>,
_reserved12: [u8; 248],
/// CLK_ADCHS clock configuration register
clk_adchs_cfg: ReadWrite<u32, CLK_CFG::Register>,
/// CLK_ADCHS clock status register
clk_adchs_stat: ReadOnly<u32, CLK_CFG::Register>,
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
CLK_CFG [
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
]
];
const CCU1_BASE: StaticRef<Ccu1Registers> =
    unsafe { StaticRef::new(0x40051000 as *const Ccu1Registers) };

/* UART 2 init */
pub fn uart2_init() {
    CCU1_BASE.clk_m4_usart2_cfg.write(CLK_CFG::AUTO::AutoIsEnabled + CLK_CFG::WAKEUP::WakeUpIsEnabled + CLK_CFG::RUN::ClockIsEnabled)
}

pub fn get_uart2_rate() -> u32 {
    let mut rate : u32;
    let mut div : u32;
    if CCU1_BASE.clk_m4_usart2_cfg.is_set(CLK_CFG::RUN::ClockIsEnabled) {
        //base clock is CLK_BASE_UART2: CGU_BASE.base_uart2_clk
        rate = cgu::get_clock_input_hz(cgu::get_uart2_base_clk())
        /* Get divider for this clock */
        if (((CCU1_BASE.clk_m4_usart2_cfg.get() >> 5) & 0x7) == 0) {
            div = 1;
        }
        else {
            div = 2;/* No other dividers supported */
        }
        rate = rate / div;
    }
    else {
        rate = 0;
    }

    return rate;
}