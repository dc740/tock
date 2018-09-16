
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// I2S interface
#[repr(C)]
struct I2SRegisters {
/// I2S Digital Audio Output Register. Contains control bits for the I2S transmit ch
dao: ReadWrite<u32, DAO::Register>,
/// I2S Digital Audio Input Register. Contains control bits for the I2S receive chan
dai: ReadWrite<u32, DAI::Register>,
/// I2S Transmit FIFO. Access register for the 8 x 32-bit transmitter FIFO.
txfifo: WriteOnly<u32>,
/// I2S Receive FIFO. Access register for the 8 x 32-bit receiver FIFO.
rxfifo: ReadOnly<u32>,
/// I2S Status Feedback Register. Contains status information about the I2S interfac
state: ReadOnly<u32, STATE::Register>,
/// I2S DMA Configuration Register 1. Contains control information for DMA request 1
dma1: ReadWrite<u32, DMA1::Register>,
/// I2S DMA Configuration Register 2. Contains control information for DMA request 2
dma2: ReadWrite<u32, DMA2::Register>,
/// I2S Interrupt Request Control Register. Contains bits that control how the I2S i
irq: ReadWrite<u32, IRQ::Register>,
/// I2S Transmit MCLK divider. This register determines the I2S TX MCLK rate by spec
txrate: ReadWrite<u32, TXRATE::Register>,
/// I2S Receive MCLK divider. This register determines the I2S RX MCLK rate by speci
rxrate: ReadWrite<u32, RXRATE::Register>,
/// I2S Transmit bit rate divider. This register determines the I2S transmit bit rat
txbitrate: ReadWrite<u32>,
/// I2S Receive bit rate divider. This register determines the I2S receive bit rate
rxbitrate: ReadWrite<u32>,
/// I2S Transmit mode control.
txmode: ReadWrite<u32, TXMODE::Register>,
/// I2S Receive mode control.
rxmode: ReadWrite<u32, RXMODE::Register>,
}
register_bitfields![u32,
DAO [
    /// Selects the number of bytes in data as follows:
    WORDWIDTH OFFSET(0) NUMBITS(2) [
        /// 8-bit data
        _8BitData = 0,
        /// 16-bit data
        _16BitData = 1,
        /// Reserved, do not use this setting
        ReservedDoNotUseThisSetting = 2,
        /// 32-bit data
        _32BitData = 3
    ],
    /// When 1, data is of monaural format. When 0, the data is in stereo format.
    MONO OFFSET(2) NUMBITS(1) [],
    /// When 1, disables accesses on FIFOs, places the transmit channel in mute mode.
    STOP OFFSET(3) NUMBITS(1) [],
    /// When 1, asynchronously resets the transmit channel and FIFO.
    RESET OFFSET(4) NUMBITS(1) [],
    /// When 0, the interface is in master mode. When 1, the interface is in slave mode.
    WS_SEL OFFSET(5) NUMBITS(1) [],
    /// Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31.
    WS_HALFPERIOD OFFSET(6) NUMBITS(9) [],
    /// When 1, the transmit channel sends only zeroes.
    MUTE OFFSET(15) NUMBITS(1) []
],
DAI [
    /// Selects the number of bytes in data as follows:
    WORDWIDTH OFFSET(0) NUMBITS(2) [
        /// 8-bit data
        _8BitData = 0,
        /// 16-bit data
        _16BitData = 1,
        /// Reserved, do not use this setting
        ReservedDoNotUseThisSetting = 2,
        /// 32-bit data
        _32BitData = 3
    ],
    /// When 1, data is of monaural format. When 0, the data is in stereo format.
    MONO OFFSET(2) NUMBITS(1) [],
    /// When 1, disables accesses on FIFOs, places the transmit channel in mute mode.
    STOP OFFSET(3) NUMBITS(1) [],
    /// When 1, asynchronously reset the transmit channel and FIFO.
    RESET OFFSET(4) NUMBITS(1) [],
    /// When 0, the interface is in master mode. When 1, the interface is in slave mode.
    WS_SEL OFFSET(5) NUMBITS(1) [],
    /// Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31.
    WS_HALFPERIOD OFFSET(6) NUMBITS(9) []
],
STATE [
    /// This bit reflects the presence of Receive Interrupt or Transmit Interrupt. This
    IRQ OFFSET(0) NUMBITS(1) [],
    /// This bit reflects the presence of Receive or Transmit DMA Request 1. This is det
    DMAREQ1 OFFSET(1) NUMBITS(1) [],
    /// This bit reflects the presence of Receive or Transmit DMA Request 2. This is det
    DMAREQ2 OFFSET(2) NUMBITS(1) [],
    /// Reflects the current level of the Receive FIFO.
    RX_LEVEL OFFSET(8) NUMBITS(4) [],
    /// Reflects the current level of the Transmit FIFO.
    TX_LEVEL OFFSET(16) NUMBITS(4) []
],
DMA1 [
    /// When 1, enables DMA1 for I2S receive.
    RX_DMA1_ENABLE OFFSET(0) NUMBITS(1) [],
    /// When 1, enables DMA1 for I2S transmit.
    TX_DMA1_ENABLE OFFSET(1) NUMBITS(1) [],
    /// Set the FIFO level that triggers a receive DMA request on DMA1.
    RX_DEPTH_DMA1 OFFSET(8) NUMBITS(4) [],
    /// Set the FIFO level that triggers a transmit DMA request on DMA1.
    TX_DEPTH_DMA1 OFFSET(16) NUMBITS(4) []
],
DMA2 [
    /// When 1, enables DMA1 for I2S receive.
    RX_DMA2_ENABLE OFFSET(0) NUMBITS(1) [],
    /// When 1, enables DMA1 for I2S transmit.
    TX_DMA2_ENABLE OFFSET(1) NUMBITS(1) [],
    /// Set the FIFO level that triggers a receive DMA request on DMA2.
    RX_DEPTH_DMA2 OFFSET(8) NUMBITS(4) [],
    /// Set the FIFO level that triggers a transmit DMA request on DMA2.
    TX_DEPTH_DMA2 OFFSET(16) NUMBITS(4) []
],
IRQ [
    /// When 1, enables I2S receive interrupt.
    RX_IRQ_ENABLE OFFSET(0) NUMBITS(1) [],
    /// When 1, enables I2S transmit interrupt.
    TX_IRQ_ENABLE OFFSET(1) NUMBITS(1) [],
    /// Set the FIFO level on which to create an irq request.
    RX_DEPTH_IRQ OFFSET(8) NUMBITS(4) [],
    /// Set the FIFO level on which to create an irq request.
    TX_DEPTH_IRQ OFFSET(16) NUMBITS(4) []
],
TXRATE [
    /// I2S transmit MCLK rate denominator. This value is used to divide PCLK to produce
    Y_DIVIDER OFFSET(0) NUMBITS(8) [],
    /// I2S transmit MCLK rate numerator. This value is used to multiply PCLK by to prod
    X_DIVIDER OFFSET(8) NUMBITS(8) []
],
RXRATE [
    /// I2S receive MCLK rate denominator. This value is used to divide PCLK to produce
    Y_DIVIDER OFFSET(0) NUMBITS(8) [],
    /// I2S receive MCLK rate numerator. This value is used to multiply PCLK by to produ
    X_DIVIDER OFFSET(8) NUMBITS(8) []
],
TXMODE [
    /// Clock source selection for the transmit bit clock divider.
    TXCLKSEL OFFSET(0) NUMBITS(2) [
        /// Select the TX fractional rate divider clock output as the source
        SelectTheTXFractionalRateDividerClockOutputAsTheSource = 0,
        /// Reserved
        Reserved = 1,
        /// Select the RX_MCLK signal as the TX_MCLK clock source
        SelectTheRX_MCLKSignalAsTheTX_MCLKClockSource = 2
    ],
    /// Transmit 4-pin mode selection. When 1, enables 4-pin mode.
    TX4PIN OFFSET(2) NUMBITS(1) [],
    /// Enable for the TX_MCLK output. When 0, output of TX_MCLK is not enabled. When 1,
    TXMCENA OFFSET(3) NUMBITS(1) []
],
RXMODE [
    /// Clock source selection for the receive bit clock divider.
    RXCLKSEL OFFSET(0) NUMBITS(2) [
        /// Select the RX fractional rate divider clock output as the source
        SelectTheRXFractionalRateDividerClockOutputAsTheSource = 0,
        /// Reserved
        Reserved = 1,
        /// Select the TX_MCLK signal as the RX_MCLK clock source
        SelectTheTX_MCLKSignalAsTheRX_MCLKClockSource = 2
    ],
    /// Receive 4-pin mode selection. When 1, enables 4-pin mode.
    RX4PIN OFFSET(2) NUMBITS(1) [],
    /// Enable for the RX_MCLK output. When 0, output of RX_MCLK is not enabled. When 1,
    RXMCENA OFFSET(3) NUMBITS(1) []
]
];
const I2S0_BASE: StaticRef<I2SRegisters> =
    unsafe { StaticRef::new(0x400A2000 as *const I2SRegisters) };


const I2S1_BASE: StaticRef<I2SRegisters> =
    unsafe { StaticRef::new(0x400A3000 as *const I2SRegisters) };
