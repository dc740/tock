
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// SPI
#[repr(C)]
struct SpiRegisters {
/// SPI Control Register. This register controls the operation of the SPI.
cr: ReadWrite<u32, CR::Register>,
/// SPI Status Register. This register shows the status of the SPI.
sr: ReadOnly<u32, SR::Register>,
/// SPI Data Register. This bi-directional register provides the transmit and receiv
dr: ReadWrite<u32, DR::Register>,
/// SPI Clock Counter Register. This register controls the frequency of a master's S
ccr: ReadWrite<u32>,
/// SPI Test Control register. For functional testing only.
tcr: ReadWrite<u32>,
/// SPI Test Status register. For functional testing only.
tsr: ReadWrite<u32, TSR::Register>,
_reserved0: [u8; 4],
/// SPI Interrupt Flag. This register contains the interrupt flag for the SPI interf
int: ReadWrite<u32>,
}
register_bitfields![u32,
CR [
    /// The SPI controller sends and receives 8 bits of data per transfer.
    BITENABLE OFFSET(2) NUMBITS(1) [
        /// The SPI controller sends and receives the number of bits selected by bits 11:8.
        TheSPIControllerSendsAndReceivesTheNumberOfBitsSelectedByBits118 = 1
    ],
    /// Clock phase control determines the relationship between the data and the clock o
    CPHA OFFSET(3) NUMBITS(1) [
        /// Data is sampled on the first clock edge of SCK. A transfer starts and ends with
        FIRST_EDGE = 0,
        /// Data is sampled on the second clock edge of the SCK. A transfer starts with the
        SECOND_EDGE = 1
    ],
    /// Clock polarity control.
    CPOL OFFSET(4) NUMBITS(1) [
        /// SCK is active high.
        SCKIsActiveHigh = 0,
        /// SCK is active low.
        SCKIsActiveLow = 1
    ],
    /// Master mode select.
    MSTR OFFSET(5) NUMBITS(1) [
        /// The SPI operates in Slave mode.
        TheSPIOperatesInSlaveMode = 0,
        /// The SPI operates in Master mode.
        TheSPIOperatesInMasterMode = 1
    ],
    /// LSB First controls which direction each byte is shifted when transferred.
    LSBF OFFSET(6) NUMBITS(1) [
        /// SPI data is transferred MSB (bit 7) first.
        SPIDataIsTransferredMSBBit7First = 0,
        /// SPI data is transferred LSB (bit 0) first.
        SPIDataIsTransferredLSBBit0First = 1
    ],
    /// Serial peripheral interrupt enable.
    SPIE OFFSET(7) NUMBITS(1) [
        /// SPI interrupts are inhibited.
        SPIInterruptsAreInhibited = 0,
        /// A hardware interrupt is generated each time the SPIF or MODF bits are activated.
        AHardwareInterruptIsGeneratedEachTimeTheSPIFOrMODFBitsAreActivated = 1
    ],
    /// When bit 2 of this register is 1, this field controls the number of bits per tra
    BITS OFFSET(8) NUMBITS(4) [
        /// 8 bits per transfer
        _8BitsPerTransfer = 8,
        /// 9 bits per transfer
        _9BitsPerTransfer = 9,
        /// 10 bits per transfer
        _10BitsPerTransfer = 10,
        /// 11 bits per transfer
        _11BitsPerTransfer = 11,
        /// 12 bits per transfer
        _12BitsPerTransfer = 12,
        /// 13 bits per transfer
        _13BitsPerTransfer = 13,
        /// 14 bits per transfer
        _14BitsPerTransfer = 14,
        /// 15 bits per transfer
        _15BitsPerTransfer = 15,
        /// 16 bits per transfer
        _16BitsPerTransfer = 0
    ]
],
SR [
    /// Slave abort. When 1, this bit indicates that a slave abort has occurred. This bi
    ABRT OFFSET(3) NUMBITS(1) [],
    /// Mode fault. when 1, this bit indicates that a Mode fault error has occurred. Thi
    MODF OFFSET(4) NUMBITS(1) [],
    /// Read overrun. When 1, this bit indicates that a read overrun has occurred. This
    ROVR OFFSET(5) NUMBITS(1) [],
    /// Write collision. When 1, this bit indicates that a write collision has occurred.
    WCOL OFFSET(6) NUMBITS(1) [],
    /// SPI transfer complete flag. When 1, this bit indicates when a SPI data transfer
    SPIF OFFSET(7) NUMBITS(1) []
],
DR [
    /// SPI Bi-directional data port.
    DATALOW OFFSET(0) NUMBITS(8) [],
    /// If bit 2 of the SPCR is 1 and bits 11:8 are other than 1000, some or all of thes
    DATAHIGH OFFSET(8) NUMBITS(8) []
],
TSR [
    /// Slave abort.
    ABRT OFFSET(3) NUMBITS(1) [],
    /// Mode fault.
    MODF OFFSET(4) NUMBITS(1) [],
    /// Read overrun.
    ROVR OFFSET(5) NUMBITS(1) [],
    /// Write collision.
    WCOL OFFSET(6) NUMBITS(1) [],
    /// SPI transfer complete flag.
    SPIF OFFSET(7) NUMBITS(1) []
]
];
const SPI_BASE: StaticRef<SpiRegisters> =
    unsafe { StaticRef::new(0x40100000 as *const SpiRegisters) };
