
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly, register_bitfields};
    /// Reset Generation Unit (RGU)
#[repr(C)]
struct RguRegisters {
_reserved0: [u8; 256],
/// Reset control register 0
reset_ctrl0: WriteOnly<u32, RESET_CTRL0::Register>,
/// Reset control register 1
reset_ctrl1: WriteOnly<u32, RESET_CTRL1::Register>,
_reserved1: [u8; 8],
/// Reset status register 0
reset_status0: ReadWrite<u32, RESET_STATUS0::Register>,
/// Reset status register 1
reset_status1: ReadWrite<u32, RESET_STATUS1::Register>,
/// Reset status register 2
reset_status2: ReadWrite<u32, RESET_STATUS2::Register>,
/// Reset status register 3
reset_status3: ReadWrite<u32, RESET_STATUS3::Register>,
_reserved2: [u8; 48],
/// Reset active status register 0
reset_active_status0: ReadOnly<u32, RESET_ACTIVE_STATUS0::Register>,
/// Reset active status register 1
reset_active_status1: ReadOnly<u32, RESET_ACTIVE_STATUS1::Register>,
_reserved3: [u8; 684],
/// Reset external status register 1 for PERIPH_RST
reset_ext_stat1: ReadWrite<u32>,
/// Reset external status register 2 for MASTER_RST
reset_ext_stat2: ReadWrite<u32>,
_reserved4: [u8; 8],
/// Reset external status register 5 for CREG_RST
reset_ext_stat5: ReadWrite<u32>,
_reserved5: [u8; 8],
/// Reset external status register
reset_ext_stat8: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat9: ReadWrite<u32>,
_reserved6: [u8; 8],
/// Reset external status register
reset_ext_stat12: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat13: ReadWrite<u32>,
_reserved7: [u8; 8],
/// Reset external status register
reset_ext_stat16: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat17: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat18: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat19: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat20: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat21: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat22: ReadWrite<u32>,
_reserved8: [u8; 8],
/// Reset external status register
reset_ext_stat25: ReadWrite<u32>,
_reserved9: [u8; 4],
/// Reset external status register
reset_ext_stat27: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat28: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat29: ReadWrite<u32>,
_reserved10: [u8; 8],
/// Reset external status register
reset_ext_stat32: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat33: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat34: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat35: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat36: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat37: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat38: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat39: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat40: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat41: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat42: ReadWrite<u32>,
_reserved11: [u8; 4],
/// Reset external status register
reset_ext_stat44: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat45: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat46: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat47: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat48: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat49: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat50: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat51: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat52: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat53: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat54: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat55: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat56: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat57: ReadWrite<u32>,
/// Reset external status register
reset_ext_stat58: ReadWrite<u32>,
_reserved12: [u8; 4],
/// Reset external status register
reset_ext_stat60: ReadWrite<u32>,
}
register_bitfields![u32,
RESET_CTRL0 [
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    CORE_RST OFFSET(0) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    PERIPH_RST OFFSET(1) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    MASTER_RST OFFSET(2) NUMBITS(1) [],
    /// Writing a one to this bit has no effect.
    WWDT_RST OFFSET(4) NUMBITS(1) [],
    /// Writing a one to this bit has no effect.
    CREG_RST OFFSET(5) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    BUS_RST OFFSET(8) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    SCU_RST OFFSET(9) NUMBITS(1) [],
    /// Writing a one activates the reset. Writing a 0 clears the reset. This bit must b
    M0_SUB_RST OFFSET(12) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    M4_RST OFFSET(13) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    LCD_RST OFFSET(16) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    USB0_RST OFFSET(17) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    USB1_RST OFFSET(18) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    DMA_RST OFFSET(19) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    SDIO_RST OFFSET(20) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    EMC_RST OFFSET(21) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    ETHERNET_RST OFFSET(22) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    FLASHA_RST OFFSET(25) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    EEPROM_RST OFFSET(27) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    GPIO_RST OFFSET(28) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    FLASHB_RST OFFSET(29) NUMBITS(1) []
],
RESET_CTRL1 [
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    TIMER0_RST OFFSET(0) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    TIMER1_RST OFFSET(1) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    TIMER2_RST OFFSET(2) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    TIMER3_RST OFFSET(3) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    RITIMER_RST OFFSET(4) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    SCT_RST OFFSET(5) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    MOTOCONPWM_RST OFFSET(6) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    QEI_RST OFFSET(7) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    ADC0_RST OFFSET(8) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    ADC1_RST OFFSET(9) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    DAC_RST OFFSET(10) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    UART0_RST OFFSET(12) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    UART1_RST OFFSET(13) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    UART2_RST OFFSET(14) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    UART3_RST OFFSET(15) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    I2C0_RST OFFSET(16) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    I2C1_RST OFFSET(17) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    SSP0_RST OFFSET(18) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    SSP1_RST OFFSET(19) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    I2S_RST OFFSET(20) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    SPIFI_RST OFFSET(21) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    CAN1_RST OFFSET(22) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    CAN0_RST OFFSET(23) NUMBITS(1) [],
    /// Writing a one activates the reset. Writing a 0 clears the reset. This bit must b
    M0APP_RST OFFSET(24) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    SGPIO_RST OFFSET(25) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    SPI_RST OFFSET(26) NUMBITS(1) [],
    /// Writing a one activates the reset. This bit is automatically cleared to 0 after
    ADCHS_RST OFFSET(28) NUMBITS(1) []
],
RESET_STATUS0 [
    /// Status of the PERIPH_RST reset generator output 00 = No reset activated 01 = Res
    PERIPH_RST OFFSET(2) NUMBITS(2) [],
    /// Status of the MASTER_RST reset generator output 00 = No reset activated 01 = Res
    MASTER_RST OFFSET(4) NUMBITS(2) [],
    /// Status of the WWDT_RST reset generator output 00 = No reset activated 01 = Reset
    WWDT_RST OFFSET(8) NUMBITS(2) [],
    /// Status of the CREG_RST reset generator output 00 = No reset activated 01 = Reset
    CREG_RST OFFSET(10) NUMBITS(2) [],
    /// Status of the BUS_RST reset generator output 00 = No reset activated 01 = Reset
    BUS_RST OFFSET(16) NUMBITS(2) [],
    /// Status of the SCU_RST reset generator output 00 = No reset activated 01 = Reset
    SCU_RST OFFSET(18) NUMBITS(2) [],
    /// Status of the M0SUB_RST reset generator output 00 = No reset activated 01 = Rese
    M0SUB_RST OFFSET(24) NUMBITS(2) [],
    /// Status of the M4_RST reset generator output 00 = No reset activated 01 = Reset o
    M4_RST OFFSET(26) NUMBITS(2) []
],
RESET_STATUS1 [
    /// Status of the LCD_RST reset generator output 00 = No reset activated 01 = Reset
    LCD_RST OFFSET(0) NUMBITS(2) [],
    /// Status of the USB0_RST reset generator output 00 = No reset activated 01 = Reset
    USB0_RST OFFSET(2) NUMBITS(2) [],
    /// Status of the USB1_RST reset generator output 00 = No reset activated 01 = Reset
    USB1_RST OFFSET(4) NUMBITS(2) [],
    /// Status of the DMA_RST reset generator output 00 = No reset activated 01 = Reset
    DMA_RST OFFSET(6) NUMBITS(2) [],
    /// Status of the SDIO_RST reset generator output 00 = No reset activated 01 = Reset
    SDIO_RST OFFSET(8) NUMBITS(2) [],
    /// Status of the EMC_RST reset generator output 00 = No reset activated 01 = Reset
    EMC_RST OFFSET(10) NUMBITS(2) [],
    /// Status of the ETHERNET_RST reset generator output 00 = No reset activated 01 = R
    ETHERNET_RST OFFSET(12) NUMBITS(2) [],
    /// Status of the FLASHA_RST reset generator output 00 = No reset activated 01 = Res
    FLASHA_RST OFFSET(18) NUMBITS(2) [],
    /// Status of the EEPROM_RST reset generator output 00 = No reset activated 01 = Res
    EEPROM_RST OFFSET(22) NUMBITS(2) [],
    /// Status of the GPIO_RST reset generator output 00 = No reset activated 01 = Reset
    GPIO_RST OFFSET(24) NUMBITS(2) [],
    /// Status of the FLASHB_RST reset generator output 00 = No reset activated 01 = Res
    FLASHB_RST OFFSET(26) NUMBITS(2) []
],
RESET_STATUS2 [
    /// Status of the TIMER0_RST reset generator output 00 = No reset activated 01 = Res
    TIMER0_RST OFFSET(0) NUMBITS(2) [],
    /// Status of the TIMER1_RST reset generator output 00 = No reset activated 01 = Res
    TIMER1_RST OFFSET(2) NUMBITS(2) [],
    /// Status of the TIMER2_RST reset generator output 00 = No reset activated 01 = Res
    TIMER2_RST OFFSET(4) NUMBITS(2) [],
    /// Status of the TIMER3_RST reset generator output 00 = No reset activated 01 = Res
    TIMER3_RST OFFSET(6) NUMBITS(2) [],
    /// Status of the RITIMER_RST reset generator output 00 = No reset activated 01 = Re
    RITIMER_RST OFFSET(8) NUMBITS(2) [],
    /// Status of the SCT_RST reset generator output 00 = No reset activated 01 = Reset
    SCT_RST OFFSET(10) NUMBITS(2) [],
    /// Status of the MOTOCONPWM_RST reset generator output 00 = No reset activated 01 =
    MOTOCONPWM_RST OFFSET(12) NUMBITS(2) [],
    /// Status of the QEI_RST reset generator output 00 = No reset activated 01 = Reset
    QEI_RST OFFSET(14) NUMBITS(2) [],
    /// Status of the ADC0_RST reset generator output 00 = No reset activated 01 = Reset
    ADC0_RST OFFSET(16) NUMBITS(2) [],
    /// Status of the ADC1_RST reset generator output 00 = No reset activated 01 = Reset
    ADC1_RST OFFSET(18) NUMBITS(2) [],
    /// Status of the DAC_RST reset generator output 00 = No reset activated 01 = Reset
    DAC_RST OFFSET(20) NUMBITS(2) [],
    /// Status of the UART0_RST reset generator output 00 = No reset activated 01 = Rese
    UART0_RST OFFSET(24) NUMBITS(2) [],
    /// Status of the UART1_RST reset generator output 00 = No reset activated 01 = Rese
    UART1_RST OFFSET(26) NUMBITS(2) [],
    /// Status of the UART2_RST reset generator output 00 = No reset activated 01 = Rese
    UART2_RST OFFSET(28) NUMBITS(2) [],
    /// Status of the UART3_RST reset generator output 00 = No reset activated 01 = Rese
    UART3_RST OFFSET(30) NUMBITS(2) []
],
RESET_STATUS3 [
    /// Status of the I2C0_RST reset generator output 00 = No reset activated 01 = Reset
    I2C0_RST OFFSET(0) NUMBITS(2) [],
    /// Status of the I2C1_RST reset generator output 00 = No reset activated 01 = Reset
    I2C1_RST OFFSET(2) NUMBITS(2) [],
    /// Status of the SSP0_RST reset generator output 00 = No reset activated 01 = Reset
    SSP0_RST OFFSET(4) NUMBITS(2) [],
    /// Status of the SSP1_RST reset generator output 00 = No reset activated 01 = Reset
    SSP1_RST OFFSET(6) NUMBITS(2) [],
    /// Status of the I2S_RST reset generator output 00 = No reset activated 01 = Reset
    I2S_RST OFFSET(8) NUMBITS(2) [],
    /// Status of the SPIFI_RST reset generator output 00 = No reset activated 01 = Rese
    SPIFI_RST OFFSET(10) NUMBITS(2) [],
    /// Status of the CAN1_RST reset generator output 00 = No reset activated 01 = Reset
    CAN1_RST OFFSET(12) NUMBITS(2) [],
    /// Status of the CAN0_RST reset generator output 00 = No reset activated 01 = Reset
    CAN0_RST OFFSET(14) NUMBITS(2) [],
    /// Status of the M0APP_RST reset generator output 00 = No reset activated 01 = Rese
    M0APP_RST OFFSET(16) NUMBITS(2) [],
    /// Status of the SGPIO_RST reset generator output 00 = No reset activated 01 = Rese
    SGPIO_RST OFFSET(18) NUMBITS(2) [],
    /// Status of the SPI_RST reset generator output 00 = No reset activated 01 = Reset
    SPI_RST OFFSET(20) NUMBITS(2) [],
    /// Status of the ADCHS_RST reset generator output 00 = No reset activated 01 = Rese
    ADCHS_RST OFFSET(24) NUMBITS(2) []
],
RESET_ACTIVE_STATUS0 [
    /// Current status of the CORE_RST 0 = Reset asserted 1 = No reset
    CORE_RST OFFSET(0) NUMBITS(1) [],
    /// Current status of the PERIPH_RST 0 = Reset asserted 1 = No reset
    PERIPH_RST OFFSET(1) NUMBITS(1) [],
    /// Current status of the MASTER_RST 0 = Reset asserted 1 = No reset
    MASTER_RST OFFSET(2) NUMBITS(1) [],
    /// Current status of the WWDT_RS 0 = Reset asserted 1 = No reset
    WWDT_RST OFFSET(4) NUMBITS(1) [],
    /// Current status of the CREG_RST 0 = Reset asserted 1 = No reset
    CREG_RST OFFSET(5) NUMBITS(1) [],
    /// Current status of the BUS_RST 0 = Reset asserted 1 = No reset
    BUS_RST OFFSET(8) NUMBITS(1) [],
    /// Current status of the SCU_RST 0 = Reset asserted 1 = No reset
    SCU_RST OFFSET(9) NUMBITS(1) [],
    /// Current status of the M0SUB_RST 0 = Reset asserted 1 = No reset
    M0SUB_RST OFFSET(12) NUMBITS(1) [],
    /// Current status of the M4_RST 0 = Reset asserted 1 = No reset
    M4_RST OFFSET(13) NUMBITS(1) [],
    /// Current status of the LCD_RST 0 = Reset asserted 1 = No reset
    LCD_RST OFFSET(16) NUMBITS(1) [],
    /// Current status of the USB0_RST 0 = Reset asserted 1 = No reset
    USB0_RST OFFSET(17) NUMBITS(1) [],
    /// Current status of the USB1_RST 0 = Reset asserted 1 = No reset
    USB1_RST OFFSET(18) NUMBITS(1) [],
    /// Current status of the DMA_RST 0 = Reset asserted 1 = No reset
    DMA_RST OFFSET(19) NUMBITS(1) [],
    /// Current status of the SDIO_RST 0 = Reset asserted 1 = No reset
    SDIO_RST OFFSET(20) NUMBITS(1) [],
    /// Current status of the EMC_RST 0 = Reset asserted 1 = No reset
    EMC_RST OFFSET(21) NUMBITS(1) [],
    /// Current status of the ETHERNET_RST 0 = Reset asserted 1 = No reset
    ETHERNET_RST OFFSET(22) NUMBITS(1) [],
    /// Current status of the FLASHA_RST 0 = Reset asserted 1 = No reset
    FLASHA_RST OFFSET(25) NUMBITS(1) [],
    /// Current status of the EEPROM_RST 0 = Reset asserted 1 = No reset
    EEPROM_RST OFFSET(27) NUMBITS(1) [],
    /// Current status of the GPIO_RST 0 = Reset asserted 1 = No reset
    GPIO_RST OFFSET(28) NUMBITS(1) [],
    /// Current status of the FLASHB_RST 0 = Reset asserted 1 = No reset
    FLASHB_RST OFFSET(29) NUMBITS(1) []
],
RESET_ACTIVE_STATUS1 [
    /// Current status of the TIMER0_RST 0 = Reset asserted 1 = No reset
    TIMER0_RST OFFSET(0) NUMBITS(1) [],
    /// Current status of the TIMER1_RST 0 = Reset asserted 1 = No reset
    TIMER1_RST OFFSET(1) NUMBITS(1) [],
    /// Current status of the TIMER2_RST 0 = Reset asserted 1 = No reset
    TIMER2_RST OFFSET(2) NUMBITS(1) [],
    /// Current status of the TIMER3_RST 0 = Reset asserted 1 = No reset
    TIMER3_RST OFFSET(3) NUMBITS(1) [],
    /// Current status of the RITIMER_RST 0 = Reset asserted 1 = No reset
    RITIMER_RST OFFSET(4) NUMBITS(1) [],
    /// Current status of the SCT_RST 0 = Reset asserted 1 = No reset
    SCT_RST OFFSET(5) NUMBITS(1) [],
    /// Current status of the MOTOCONPWM_RST 0 = Reset asserted 1 = No reset
    MOTOCONPWM_RST OFFSET(6) NUMBITS(1) [],
    /// Current status of the QEI_RST 0 = Reset asserted 1 = No reset
    QEI_RST OFFSET(7) NUMBITS(1) [],
    /// Current status of the ADC0_RST 0 = Reset asserted 1 = No reset
    ADC0_RST OFFSET(8) NUMBITS(1) [],
    /// Current status of the ADC1_RST 0 = Reset asserted 1 = No reset
    ADC1_RST OFFSET(9) NUMBITS(1) [],
    /// Current status of the DAC_RST 0 = Reset asserted 1 = No reset
    DAC_RST OFFSET(10) NUMBITS(1) [],
    /// Current status of the UART0_RST 0 = Reset asserted 1 = No reset
    UART0_RST OFFSET(12) NUMBITS(1) [],
    /// Current status of the UART1_RST 0 = Reset asserted 1 = No reset
    UART1_RST OFFSET(13) NUMBITS(1) [],
    /// Current status of the UART2_RST 0 = Reset asserted 1 = No reset
    UART2_RST OFFSET(14) NUMBITS(1) [],
    /// Current status of the UART3_RST 0 = Reset asserted 1 = No reset
    UART3_RST OFFSET(15) NUMBITS(1) [],
    /// Current status of the I2C0_RST 0 = Reset asserted 1 = No reset
    I2C0_RST OFFSET(16) NUMBITS(1) [],
    /// Current status of the I2C1_RST 0 = Reset asserted 1 = No reset
    I2C1_RST OFFSET(17) NUMBITS(1) [],
    /// Current status of the SSP0_RST 0 = Reset asserted 1 = No reset
    SSP0_RST OFFSET(18) NUMBITS(1) [],
    /// Current status of the SSP1_RST 0 = Reset asserted 1 = No reset
    SSP1_RST OFFSET(19) NUMBITS(1) [],
    /// Current status of the I2S_RST 0 = Reset asserted 1 = No reset
    I2S_RST OFFSET(20) NUMBITS(1) [],
    /// Current status of the SPIFI_RST 0 = Reset asserted 1 = No reset
    SPIFI_RST OFFSET(21) NUMBITS(1) [],
    /// Current status of the CAN1_RST 0 = Reset asserted 1 = No reset
    CAN1_RST OFFSET(22) NUMBITS(1) [],
    /// Current status of the CAN0_RST 0 = Reset asserted 1 = No reset
    CAN0_RST OFFSET(23) NUMBITS(1) [],
    /// Current status of the M0APP_RST 0 = Reset asserted 1 = No reset
    M0APP_RST OFFSET(24) NUMBITS(1) [],
    /// Current status of the SGPIO_RST 0 = Reset asserted 1 = No reset
    SGPIO_RST OFFSET(25) NUMBITS(1) [],
    /// Current status of the SPI_RST 0 = Reset asserted 1 = No reset
    SPI_RST OFFSET(26) NUMBITS(1) [],
    /// Current status of the ADCHS_RST 0 = Reset asserted 1 = No reset
    ADCHS_RST OFFSET(28) NUMBITS(1) []
]
];
const RGU_BASE: StaticRef<RguRegisters> =
    unsafe { StaticRef::new(0x40053000 as *const RguRegisters) };
