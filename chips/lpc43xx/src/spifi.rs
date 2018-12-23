
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly, register_bitfields};
    /// SPI Flash Interface (SPIFI)
#[repr(C)]
struct SpifiRegisters {
/// SPIFI control register
ctrl: ReadWrite<u32, CTRL::Register>,
/// SPIFI command register
cmd: ReadWrite<u32, CMD::Register>,
/// SPIFI address register
addr: ReadWrite<u32>,
/// SPIFI intermediate data register
idata: ReadWrite<u32>,
/// SPIFI cache limit register
climit: ReadWrite<u32>,
/// SPIFI data register
data: ReadWrite<u32>,
/// SPIFI memory command register
mcmd: ReadWrite<u32, MCMD::Register>,
/// SPIFI status register
stat: ReadWrite<u32, STAT::Register>,
}
register_bitfields![u32,
CTRL [
    /// This field contains the number of serial clock periods without the processor rea
    TIMEOUT OFFSET(0) NUMBITS(16) [],
    /// This field controls the minimum CS high time, expressed as a number of serial cl
    CSHIGH OFFSET(16) NUMBITS(4) [],
    /// This bit allows conditioning of memory mode prefetches based on the AHB HPROT (i
    D_PRFTCH_DIS OFFSET(21) NUMBITS(1) [],
    /// If this bit is 1 when a command ends, the SPIFI will assert its interrupt reques
    INTEN OFFSET(22) NUMBITS(1) [],
    /// SPI Mode 3 select.
    MODE3 OFFSET(23) NUMBITS(1) [
        /// SCK LOW. The SPIFI drives SCK low after the rising edge at which the last bit of
        SCK_LOW = 0,
        /// SCK HIGH. the SPIFI keeps SCK high after the rising edge for the last bit of eac
        SCK_HIGH = 1
    ],
    /// Cache prefetching enable. The SPIFI includes an internal cache. A 1 in this bit
    PRFTCH_DIS OFFSET(27) NUMBITS(1) [
        /// Enable. Cache prefetching enabled.
        EnableCachePrefetchingEnabled = 0,
        /// Disable. Disables prefetching of cache lines.
        DisableDisablesPrefetchingOfCacheLines = 1
    ],
    /// Select dual protocol.
    DUAL OFFSET(28) NUMBITS(1) [
        /// Quad protocol. This protocol uses IO3:0.
        QuadProtocolThisProtocolUsesIO30 = 0,
        /// Dual protocol. This protocol uses IO1:0.
        DualProtocolThisProtocolUsesIO10 = 1
    ],
    /// Select active clock edge for input data.
    RFCLK OFFSET(29) NUMBITS(1) [
        /// Rising edge. Read data is sampled on rising edges on the clock, as in classic SP
        RisingEdgeReadDataIsSampledOnRisingEdgesOnTheClockAsInClassicSPIOperation = 0,
        /// Falling edge. Read data is sampled on falling edges of the clock, allowing a ful
        FALLING_EDGE = 1
    ],
    /// Feedback clock select.
    FBCLK OFFSET(30) NUMBITS(1) [
        /// Internal clock. The SPIFI samples read data using an internal clock.
        InternalClockTheSPIFISamplesReadDataUsingAnInternalClock = 0,
        /// Feedback clock. Read data is sampled using a feedback clock from the SCK pin. Th
        FEEDBACK_CLOCK = 1
    ],
    /// A 1 in this bit enables the DMA Request output from the SPIFI. Set this bit only
    DMAEN OFFSET(31) NUMBITS(1) []
],
CMD [
    /// Except when the POLL bit in this register is 1, this field controls how many dat
    DATALEN OFFSET(0) NUMBITS(14) [],
    /// This bit should be written as 1 only with an opcode that a) contains an input da
    POLL OFFSET(14) NUMBITS(1) [],
    /// If the DATALEN field is not zero, this bit controls the direction of the data:
    DOUT OFFSET(15) NUMBITS(1) [
        /// Input from serial flash.
        InputFromSerialFlash = 0,
        /// Output to serial flash.
        OutputToSerialFlash = 1
    ],
    /// This field controls how many intermediate bytes precede the data. (Each such byt
    INTLEN OFFSET(16) NUMBITS(3) [],
    /// This field controls how the fields of the command are sent.
    FIELDFORM OFFSET(19) NUMBITS(2) [
        /// All serial. All fields of the command are serial.
        AllSerialAllFieldsOfTheCommandAreSerial = 0,
        /// Quad/dual data. Data field is quad/dual, other fields are serial.
        QuadDualDataDataFieldIsQuadDualOtherFieldsAreSerial = 1,
        /// Serial opcode. Opcode field is serial. Other fields are quad/dual.
        SerialOpcodeOpcodeFieldIsSerialOtherFieldsAreQuadDual = 2,
        /// All quad/dual. All fields of the command are in quad/dual format.
        AllQuadDualAllFieldsOfTheCommandAreInQuadDualFormat = 3
    ],
    /// This field controls the opcode and address fields.
    FRAMEFORM OFFSET(21) NUMBITS(3) [
        /// Reserved.
        Reserved = 0,
        /// Opcode. Opcode only, no address.
        OpcodeOpcodeOnlyNoAddress = 1,
        /// Opcode one byte. Opcode, least significant byte of address.
        OpcodeOneByteOpcodeLeastSignificantByteOfAddress = 2,
        /// Opcode two bytes. Opcode, two least significant bytes of address.
        OpcodeTwoBytesOpcodeTwoLeastSignificantBytesOfAddress = 3,
        /// Opcode three bytes. Opcode, three least significant bytes of address.
        OpcodeThreeBytesOpcodeThreeLeastSignificantBytesOfAddress = 4,
        /// Opcode four bytes. Opcode, 4 bytes of address.
        OpcodeFourBytesOpcode4BytesOfAddress = 5,
        /// No opcode three bytes. No opcode, 3 least significant bytes of address.
        NoOpcodeThreeBytesNoOpcode3LeastSignificantBytesOfAddress = 6,
        /// No opcode four bytes. No opcode, 4 bytes of address.
        NoOpcodeFourBytesNoOpcode4BytesOfAddress = 7
    ],
    /// The opcode of the command (not used for some FRAMEFORM values).
    OPCODE OFFSET(24) NUMBITS(8) []
],
MCMD [
    /// This bit should be written as 0.
    POLL OFFSET(14) NUMBITS(1) [],
    /// This bit should be written as 0.
    DOUT OFFSET(15) NUMBITS(1) [],
    /// This field controls how many intermediate bytes precede the data. (Each such byt
    INTLEN OFFSET(16) NUMBITS(3) [],
    /// This field controls how the fields of the command are sent.
    FIELDFORM OFFSET(19) NUMBITS(2) [
        /// All serial. All fields of the command are serial.
        AllSerialAllFieldsOfTheCommandAreSerial = 0,
        /// Quad/dual data. Data field is quad/dual, other fields are serial.
        QuadDualDataDataFieldIsQuadDualOtherFieldsAreSerial = 1,
        /// Serial opcode. Opcode field is serial. Other fields are quad/dual.
        SerialOpcodeOpcodeFieldIsSerialOtherFieldsAreQuadDual = 2,
        /// All quad/dual. All fields of the command are in quad/dual format.
        AllQuadDualAllFieldsOfTheCommandAreInQuadDualFormat = 3
    ],
    /// This field controls the opcode and address fields.
    FRAMEFORM OFFSET(21) NUMBITS(3) [
        /// Reserved.
        Reserved = 0,
        /// Opcode one byte. Opcode, least-significant byte of address.
        OpcodeOneByteOpcodeLeastSignificantByteOfAddress = 2,
        /// Opcode two bytes. Opcode, 2 least-significant bytes of address.
        OpcodeTwoBytesOpcode2LeastSignificantBytesOfAddress = 3,
        /// Opcode three bytes. Opcode, 3 least-significant bytes of address.
        OpcodeThreeBytesOpcode3LeastSignificantBytesOfAddress = 4,
        /// Opcode four bytes. Opcode, 4 bytes of address.
        OpcodeFourBytesOpcode4BytesOfAddress = 5,
        /// No opcode three bytes. No opcode, 3 least-significant bytes of address.
        NoOpcodeThreeBytesNoOpcode3LeastSignificantBytesOfAddress = 6,
        /// No opcode, 4 bytes of address.
        NoOpcode4BytesOfAddress = 7
    ],
    /// The opcode of the command (not used for some FRAMEFORM values).
    OPCODE OFFSET(24) NUMBITS(8) []
],
STAT [
    /// This bit is set when software successfully writes the Memory Command register, a
    MCINIT OFFSET(0) NUMBITS(1) [],
    /// This bit is 1 when the Command register is written. It is cleared by a hardware
    CMD OFFSET(1) NUMBITS(1) [],
    /// Write a 1 to this bit to abort a current command or memory mode. This bit is cle
    RESET OFFSET(4) NUMBITS(1) [],
    /// This bit reflects the SPIFI interrupt request. Write a 1 to this bit to clear it
    INTRQ OFFSET(5) NUMBITS(1) [],
    /// The SPIFI hardware described in this chapter returns
    VERSION OFFSET(24) NUMBITS(8) []
]
];
const SPIFI_BASE: StaticRef<SpifiRegisters> =
    unsafe { StaticRef::new(0x40003000 as *const SpifiRegisters) };
