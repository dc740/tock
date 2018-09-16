
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// SSP0/1
#[repr(C)]
struct SspRegisters {
/// Control Register 0. Selects the serial clock rate, bus type, and data size.
cr0: ReadWrite<u32, CR0::Register>,
/// Control Register 1. Selects master/slave and other modes.
cr1: ReadWrite<u32, CR1::Register>,
/// Data Register. Writes fill the transmit FIFO, and reads empty the receive FIFO.
dr: ReadWrite<u32>,
/// Status Register
sr: ReadOnly<u32, SR::Register>,
/// Clock Prescale Register
cpsr: ReadWrite<u32>,
/// Interrupt Mask Set and Clear Register
imsc: ReadWrite<u32, IMSC::Register>,
/// Raw Interrupt Status Register
ris: ReadOnly<u32, RIS::Register>,
/// Masked Interrupt Status Register
mis: ReadOnly<u32, MIS::Register>,
/// SSPICR Interrupt Clear Register
icr: WriteOnly<u32, ICR::Register>,
/// SSP0 DMA control register
dmacr: ReadWrite<u32, DMACR::Register>,
}
register_bitfields![u32,
CR0 [
    /// Data Size Select. This field controls the number of bits transferred in each fra
    DSS OFFSET(0) NUMBITS(4) [
        /// 4-bit transfer
        _4BitTransfer = 3,
        /// 5-bit transfer
        _5BitTransfer = 4,
        /// 6-bit transfer
        _6BitTransfer = 5,
        /// 7-bit transfer
        _7BitTransfer = 6,
        /// 8-bit transfer
        _8BitTransfer = 7,
        /// 9-bit transfer
        _9BitTransfer = 8,
        /// 10-bit transfer
        _10BitTransfer = 9,
        /// 11-bit transfer
        _11BitTransfer = 10,
        /// 12-bit transfer
        _12BitTransfer = 11,
        /// 13-bit transfer
        _13BitTransfer = 12,
        /// 14-bit transfer
        _14BitTransfer = 13,
        /// 15-bit transfer
        _15BitTransfer = 14,
        /// 16-bit transfer
        _16BitTransfer = 15
    ],
    /// Frame Format.
    FRF OFFSET(4) NUMBITS(2) [
        /// SPI
        SPI = 0,
        /// TI
        TI = 1,
        /// Microwire
        Microwire = 2,
        /// This combination is not supported and should not be used.
        ThisCombinationIsNotSupportedAndShouldNotBeUsed = 3
    ],
    /// Clock Out Polarity. This bit is only used in SPI mode.
    CPOL OFFSET(6) NUMBITS(1) [
        /// SSP controller maintains the bus clock low between frames.
        SSPControllerMaintainsTheBusClockLowBetweenFrames = 0,
        /// SSP controller maintains the bus clock high between frames.
        SSPControllerMaintainsTheBusClockHighBetweenFrames = 1
    ],
    /// Clock Out Phase. This bit is only used in SPI mode.
    CPHA OFFSET(7) NUMBITS(1) [
        /// SSP controller captures serial data on the first clock transition of the frame,
        FIRST_CLOCK = 0,
        /// SSP controller captures serial data on the second clock transition of the frame,
        SECOND_CLOCK = 1
    ],
    /// Serial Clock Rate. The number of prescaler-output clocks per bit on the bus, min
    SCR OFFSET(8) NUMBITS(8) []
],
CR1 [
    /// Loop Back Mode.
    LBM OFFSET(0) NUMBITS(1) [
        /// During normal operation.
        DuringNormalOperation = 0,
        /// Serial input is taken from the serial output (MOSI or MISO) rather than the seri
        OUPTU = 1
    ],
    /// SSP Enable.
    SSE OFFSET(1) NUMBITS(1) [
        /// The SSP controller is disabled.
        TheSSPControllerIsDisabled = 0,
        /// The SSP controller will interact with other devices on the serial bus. Software
        ENABLED = 1
    ],
    /// Master/Slave Mode.This bit can only be written when the SSE bit is 0.
    MS OFFSET(2) NUMBITS(1) [
        /// The SSP controller acts as a master on the bus, driving the SCLK, MOSI, and SSEL
        MASTER = 0,
        /// The SSP controller acts as a slave on the bus, driving MISO line and receiving S
        SLAVE = 1
    ],
    /// Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is
    SOD OFFSET(3) NUMBITS(1) []
],
SR [
    /// Transmit FIFO Empty. This bit is 1 is the Transmit FIFO is empty, 0 if not.
    TFE OFFSET(0) NUMBITS(1) [],
    /// Transmit FIFO Not Full. This bit is 0 if the Tx FIFO is full, 1 if not.
    TNF OFFSET(1) NUMBITS(1) [],
    /// Receive FIFO Not Empty. This bit is 0 if the Receive FIFO is empty, 1 if not.
    RNE OFFSET(2) NUMBITS(1) [],
    /// Receive FIFO Full. This bit is 1 if the Receive FIFO is full, 0 if not.
    RFF OFFSET(3) NUMBITS(1) [],
    /// Busy. This bit is 0 if the SSPn controller is idle, or 1 if it is currently send
    BSY OFFSET(4) NUMBITS(1) []
],
IMSC [
    /// Software should set this bit to enable interrupt when a Receive Overrun occurs,
    RORIM OFFSET(0) NUMBITS(1) [],
    /// Software should set this bit to enable interrupt when a Receive Time-out conditi
    RTIM OFFSET(1) NUMBITS(1) [],
    /// Software should set this bit to enable interrupt when the Rx FIFO is at least ha
    RXIM OFFSET(2) NUMBITS(1) [],
    /// Software should set this bit to enable interrupt when the Tx FIFO is at least ha
    TXIM OFFSET(3) NUMBITS(1) []
],
RIS [
    /// This bit is 1 if another frame was completely received while the RxFIFO was full
    RORRIS OFFSET(0) NUMBITS(1) [],
    /// This bit is 1 if the Rx FIFO is not empty, and has not been read for a time-out
    RTRIS OFFSET(1) NUMBITS(1) [],
    /// This bit is 1 if the Rx FIFO is at least half full.
    RXRIS OFFSET(2) NUMBITS(1) [],
    /// This bit is 1 if the Tx FIFO is at least half empty.
    TXRIS OFFSET(3) NUMBITS(1) []
],
MIS [
    /// This bit is 1 if another frame was completely received while the RxFIFO was full
    RORMIS OFFSET(0) NUMBITS(1) [],
    /// This bit is 1 if the Rx FIFO is not empty, has not been read for a time-out peri
    RTMIS OFFSET(1) NUMBITS(1) [],
    /// This bit is 1 if the Rx FIFO is at least half full, and this interrupt is enable
    RXMIS OFFSET(2) NUMBITS(1) [],
    /// This bit is 1 if the Tx FIFO is at least half empty, and this interrupt is enabl
    TXMIS OFFSET(3) NUMBITS(1) []
],
ICR [
    /// Writing a 1 to this bit clears the   frame was received when RxFIFO was full int
    RORIC OFFSET(0) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the Rx FIFO was not empty and has not been read f
    RTIC OFFSET(1) NUMBITS(1) []
],
DMACR [
    /// Receive DMA Enable. When this bit is set to one 1, DMA for the receive FIFO is e
    RXDMAE OFFSET(0) NUMBITS(1) [],
    /// Transmit DMA Enable. When this bit is set to one 1, DMA for the transmit FIFO is
    TXDMAE OFFSET(1) NUMBITS(1) []
]
];
const SSP0_BASE: StaticRef<SspRegisters> =
    unsafe { StaticRef::new(0x40083000 as *const SspRegisters) };


const SSP1_BASE: StaticRef<SspRegisters> =
    unsafe { StaticRef::new(0x400C5000 as *const SspRegisters) };
