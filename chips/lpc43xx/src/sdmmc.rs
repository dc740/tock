
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly, register_bitfields};
    /// SD/MMC
#[repr(C)]
struct SdmmcRegisters {
/// Control Register
ctrl: ReadWrite<u32, CTRL::Register>,
/// Power Enable Register
pwren: ReadWrite<u32>,
/// Clock Divider Register
clkdiv: ReadWrite<u32, CLKDIV::Register>,
/// SD Clock Source Register
clksrc: ReadWrite<u32>,
/// Clock Enable Register
clkena: ReadWrite<u32, CLKENA::Register>,
/// Time-out Register
tmout: ReadWrite<u32, TMOUT::Register>,
/// Card Type Register
ctype: ReadWrite<u32, CTYPE::Register>,
/// Block Size Register
blksiz: ReadWrite<u32>,
/// Byte Count Register
bytcnt: ReadWrite<u32>,
/// Interrupt Mask Register
intmask: ReadWrite<u32, INTMASK::Register>,
/// Command Argument Register
cmdarg: ReadWrite<u32>,
/// Command Register
cmd: ReadWrite<u32, CMD::Register>,
/// Response Register 0
resp0: ReadOnly<u32>,
/// Response Register 1
resp1: ReadOnly<u32>,
/// Response Register 2
resp2: ReadOnly<u32>,
/// Response Register 3
resp3: ReadOnly<u32>,
/// Masked Interrupt Status Register
mintsts: ReadOnly<u32, MINTSTS::Register>,
/// Raw Interrupt Status Register
rintsts: ReadWrite<u32, RINTSTS::Register>,
/// Status Register
status: ReadOnly<u32, STATUS::Register>,
/// FIFO Threshold Watermark Register
fifoth: ReadWrite<u32, FIFOTH::Register>,
/// Card Detect Register
cdetect: ReadOnly<u32>,
/// Write Protect Register
wrtprt: ReadOnly<u32>,
_reserved0: [u8; 4],
/// Transferred CIU Card Byte Count Register
tcbcnt: ReadOnly<u32>,
/// Transferred Host to BIU-FIFO Byte Count Register
tbbcnt: ReadOnly<u32>,
/// Debounce Count Register
debnce: ReadWrite<u32>,
_reserved1: [u8; 16],
/// Hardware Reset
rst_n: ReadWrite<u32>,
_reserved2: [u8; 4],
/// Bus Mode Register
bmod: ReadWrite<u32, BMOD::Register>,
/// Poll Demand Register
pldmnd: WriteOnly<u32>,
/// Descriptor List Base Address Register
dbaddr: ReadWrite<u32>,
/// Internal DMAC Status Register
idsts: ReadWrite<u32, IDSTS::Register>,
/// Internal DMAC Interrupt Enable Register
idinten: ReadWrite<u32, IDINTEN::Register>,
/// Current Host Descriptor Address Register
dscaddr: ReadOnly<u32>,
/// Current Buffer Descriptor Address Register
bufaddr: ReadOnly<u32>,
}
register_bitfields![u32,
CTRL [
    /// Controller reset. To reset controller, software should set bit to 1. This bit is
    CONTROLLER_RESET OFFSET(0) NUMBITS(1) [
        /// No change.
        NoChange = 0,
        /// Reset. Reset SD/MMC controller
        ResetResetSDMMCController = 1
    ],
    /// Fifo reset. To reset FIFO, software should set bit to 1. This bit is auto-cleare
    FIFO_RESET OFFSET(1) NUMBITS(1) [
        /// No change.
        NoChange = 0,
        /// Reset. Reset to data FIFO To reset FIFO pointers
        ResetResetToDataFIFOToResetFIFOPointers = 1
    ],
    /// Dma reset. To reset DMA interface, software should set bit to 1. This bit is aut
    DMA_RESET OFFSET(2) NUMBITS(1) [
        /// No change.
        NoChange = 0,
        /// Reset. Reset internal DMA interface control logic
        ResetResetInternalDMAInterfaceControlLogic = 1
    ],
    /// Global interrupt enable/disable bit. The int port is 1 only when this bit is 1 a
    INT_ENABLE OFFSET(4) NUMBITS(1) [
        /// Disable interrupts
        DisableInterrupts = 0,
        /// Enable interrupts
        EnableInterrupts = 1
    ],
    /// Read/wait. For sending read-wait to SDIO cards.
    READ_WAIT OFFSET(6) NUMBITS(1) [
        /// Clear read wait
        ClearReadWait = 0,
        /// Assert read wait
        AssertReadWait = 1
    ],
    /// Send irq response. This bit automatically clears once response is sent. To wait
    SEND_IRQ_RESPONSE OFFSET(7) NUMBITS(1) [
        /// No change
        NoChange = 0,
        /// Send auto IRQ response
        SendAutoIRQResponse = 1
    ],
    /// Abort read data. Used in SDIO card suspend sequence.
    ABORT_READ_DATA OFFSET(8) NUMBITS(1) [
        /// No change
        NoChange = 0,
        /// Abort. After suspend command is issued during read-transfer, software polls card
        ABORT = 1
    ],
    /// Send ccsd. When set, the SD/MMC controller sends CCSD to the CE-ATA device. Soft
    SEND_CCSD OFFSET(9) NUMBITS(1) [
        /// Clear bit if the SD/MMC controller does not reset the bit.
        ClearBitIfTheSDMMCControllerDoesNotResetTheBit = 0,
        /// Send Command Completion Signal Disable (CCSD) to CE-ATA device
        SendCommandCompletionSignalDisableCCSDToCEATADevice = 1
    ],
    /// Send auto stop ccsd. NOTE: Always set send_auto_stop_ccsd and send_ccsd bits tog
    SEND_AUTO_STOP OFFSET(10) NUMBITS(1) [
        /// Clear this bit if the SD/MMC controller does not reset the bit.
        ClearThisBitIfTheSDMMCControllerDoesNotResetTheBit = 0,
        /// Send internally generated STOP after sending CCSD to CE-ATA device.
        SendInternallyGeneratedSTOPAfterSendingCCSDToCEATADevice = 1
    ],
    /// CEATA device interrupt status. Software should appropriately write to this bit a
    CEATA_DEVICE_INTERRUPT_STATUS OFFSET(11) NUMBITS(1) [
        /// Disabled. Interrupts not enabled in CE-ATA device (nIEN = 1 in ATA control regis
        DisabledInterruptsNotEnabledInCEATADeviceNIEN1InATAControlRegister = 0,
        /// Enabled. Interrupts are enabled in CE-ATA device (nIEN = 0 in ATA control regist
        EnabledInterruptsAreEnabledInCEATADeviceNIEN0InATAControlRegister = 1
    ],
    /// Controls the state of the SD_VOLT0 pin. SD/MMC card voltage control is not imple
    CARD_VOLTAGE_A0 OFFSET(16) NUMBITS(1) [],
    /// Controls the state of the SD_VOLT1 pin. SD/MMC card voltage control is not imple
    CARD_VOLTAGE_A1 OFFSET(17) NUMBITS(1) [],
    /// Controls the state of the SD_VOLT2 pin. SD/MMC card voltage control is not imple
    CARD_VOLTAGE_A2 OFFSET(18) NUMBITS(1) [],
    /// SD/MMC DMA use.
    USE_INTERNAL_DMAC OFFSET(25) NUMBITS(1) [
        /// Host. The host performs data transfers through the slave interface
        HostTheHostPerformsDataTransfersThroughTheSlaveInterface = 0,
        /// DMA. Internal DMA used for data transfer
        DMAInternalDMAUsedForDataTransfer = 1
    ]
],
CLKDIV [
    /// Clock divider-0 value. Clock division is 2*n. For example, value of 0 means divi
    CLK_DIVIDER0 OFFSET(0) NUMBITS(8) [],
    /// Clock divider-1 value. Clock division is 2*n. For example, value of 0 means divi
    CLK_DIVIDER1 OFFSET(8) NUMBITS(8) [],
    /// Clock divider-2 value. Clock division is 2*n. For example, value of 0 means divi
    CLK_DIVIDER2 OFFSET(16) NUMBITS(8) [],
    /// Clock divider-3 value. Clock division is 2*n. For example, value of 0 means divi
    CLK_DIVIDER3 OFFSET(24) NUMBITS(8) []
],
CLKENA [
    /// Clock-enable control for SD card clock. One MMC card clock supported.  0 - Clock
    CCLK_ENABLE OFFSET(0) NUMBITS(1) [],
    /// Low-power control for SD card clock. One MMC card clock supported.  0 - Non-low-
    CCLK_LOW_POWER OFFSET(16) NUMBITS(1) []
],
TMOUT [
    /// Response time-out value. Value is in number of card output clocks - cclk_out.
    RESPONSE_TIMEOUT OFFSET(0) NUMBITS(8) [],
    /// Value for card Data Read time-out; same value also used for Data Starvation by H
    DATA_TIMEOUT OFFSET(8) NUMBITS(24) []
],
CTYPE [
    /// Indicates if card is 1-bit or 4-bit:  0 - 1-bit mode  1 - 4-bit mode  1 and 4-bi
    CARD_WIDTH0 OFFSET(0) NUMBITS(1) [],
    /// Indicates if card is 8-bit:  0 - Non 8-bit mode  1 - 8-bit mode.
    CARD_WIDTH1 OFFSET(16) NUMBITS(1) []
],
INTMASK [
    /// Card detect. Bits used to mask unwanted interrupts. Value of 0 masks interrupt;
    CDET OFFSET(0) NUMBITS(1) [],
    /// Response error. Bits used to mask unwanted interrupts. Value of 0 masks interrup
    RE OFFSET(1) NUMBITS(1) [],
    /// Command done. Bits used to mask unwanted interrupts. Value of 0 masks interrupt;
    CDONE OFFSET(2) NUMBITS(1) [],
    /// Data transfer over. Bits used to mask unwanted interrupts. Value of 0 masks inte
    DTO OFFSET(3) NUMBITS(1) [],
    /// Transmit FIFO data request. Bits used to mask unwanted interrupts. Value of 0 ma
    TXDR OFFSET(4) NUMBITS(1) [],
    /// Receive FIFO data request. Bits used to mask unwanted interrupts. Value of 0 mas
    RXDR OFFSET(5) NUMBITS(1) [],
    /// Response CRC error. Bits used to mask unwanted interrupts. Value of 0 masks inte
    RCRC OFFSET(6) NUMBITS(1) [],
    /// Data CRC error. Bits used to mask unwanted interrupts. Value of 0 masks interrup
    DCRC OFFSET(7) NUMBITS(1) [],
    /// Response time-out. Bits used to mask unwanted interrupts. Value of 0 masks inter
    RTO OFFSET(8) NUMBITS(1) [],
    /// Data read time-out. Bits used to mask unwanted interrupts. Value of 0 masks inte
    DRTO OFFSET(9) NUMBITS(1) [],
    /// Data starvation-by-host time-out (HTO) /Volt_switch_int. Bits used to mask unwan
    HTO OFFSET(10) NUMBITS(1) [],
    /// FIFO underrun/overrun error. Bits used to mask unwanted interrupts. Value of 0 m
    FRUN OFFSET(11) NUMBITS(1) [],
    /// Hardware locked write error. Bits used to mask unwanted interrupts. Value of 0 m
    HLE OFFSET(12) NUMBITS(1) [],
    /// Start-bit error. Bits used to mask unwanted interrupts. Value of 0 masks interru
    SBE OFFSET(13) NUMBITS(1) [],
    /// Auto command done. Bits used to mask unwanted interrupts. Value of 0 masks inter
    ACD OFFSET(14) NUMBITS(1) [],
    /// End-bit error (read)/Write no CRC. Bits used to mask unwanted interrupts. Value
    EBE OFFSET(15) NUMBITS(1) [],
    /// Mask SDIO interrupt. When masked, SDIO interrupt detection for card is disabled.
    SDIO_INT_MASK OFFSET(16) NUMBITS(1) []
],
CMD [
    /// Command index
    CMD_INDEX OFFSET(0) NUMBITS(6) [],
    /// Response expect
    RESPONSE_EXPECT OFFSET(6) NUMBITS(1) [
        /// None. No response expected from card
        NoneNoResponseExpectedFromCard = 0,
        /// Expected. Response expected from card
        ExpectedResponseExpectedFromCard = 1
    ],
    /// Response length
    RESPONSE_LENGTH OFFSET(7) NUMBITS(1) [
        /// Short. Short response expected from card
        ShortShortResponseExpectedFromCard = 0,
        /// Long. Long response expected from card
        LongLongResponseExpectedFromCard = 1
    ],
    /// Check response crc. Some of command responses do not return valid CRC bits. Soft
    CHECK_RESPONSE_CRC OFFSET(8) NUMBITS(1) [
        /// Do not check response CRC
        DoNotCheckResponseCRC = 0,
        /// Check response CRC
        CheckResponseCRC = 1
    ],
    /// Data expected
    DATA_EXPECTED OFFSET(9) NUMBITS(1) [
        /// None. No data transfer expected (read/write)
        NoneNoDataTransferExpectedReadWrite = 0,
        /// Data. Data transfer expected (read/write)
        DataDataTransferExpectedReadWrite = 1
    ],
    /// read/write. Don't care if no data expected from card.
    READ_WRITE OFFSET(10) NUMBITS(1) [
        /// Read from card
        ReadFromCard = 0,
        /// Write to card
        WriteToCard = 1
    ],
    /// Transfer mode. Don't care if no data expected.
    TRANSFER_MODE OFFSET(11) NUMBITS(1) [
        /// Block data transfer command
        BlockDataTransferCommand = 0,
        /// Stream data transfer command
        StreamDataTransferCommand = 1
    ],
    /// Send auto stop. When set, the SD/MMC interface sends stop command to SD_MMC_CEAT
    SEND_AUTO_STOP OFFSET(12) NUMBITS(1) [
        /// No stop command sent at end of data transfer
        NoStopCommandSentAtEndOfDataTransfer = 0,
        /// Send stop command at end of data transfer
        SendStopCommandAtEndOfDataTransfer = 1
    ],
    /// Wait prvdata complete. The wait_prvdata_complete = 0 option typically used to qu
    WAIT_PRVDATA_COMPLETE OFFSET(13) NUMBITS(1) [
        /// Send. Send command at once, even if previous data transfer has not completed.
        SendSendCommandAtOnceEvenIfPreviousDataTransferHasNotCompleted = 0,
        /// Wait. Wait for previous data transfer completion before sending command.
        WaitWaitForPreviousDataTransferCompletionBeforeSendingCommand = 1
    ],
    /// Stop abort command. When open-ended or predefined data transfer is in progress,
    STOP_ABORT_CMD OFFSET(14) NUMBITS(1) [
        /// Disabled. Neither stop nor abort command to stop current data transfer in progre
        DISABLED = 0,
        /// Enabled. Stop or abort command intended to stop current data transfer in progres
        EnabledStopOrAbortCommandIntendedToStopCurrentDataTransferInProgress = 1
    ],
    /// Send initialization. After power on, 80 clocks must be sent to card for initiali
    SEND_INITIALIZATION OFFSET(15) NUMBITS(1) [
        /// No. Do not send initialization sequence (80 clocks of 1) before sending this com
        NoDoNotSendInitializationSequence80ClocksOf1BeforeSendingThisCommand = 0,
        /// Send. Send initialization sequence before sending this command.
        SendSendInitializationSequenceBeforeSendingThisCommand = 1
    ],
    /// Update clock registers only. Following register values transferred into card clo
    UPDATE_CLOCK_REGISTERS_ONLY OFFSET(21) NUMBITS(1) [
        /// Normal. Normal command sequence
        NormalNormalCommandSequence = 0,
        /// No. Do not send commands, just update clock register value into card clock domai
        NoDoNotSendCommandsJustUpdateClockRegisterValueIntoCardClockDomain = 1
    ],
    /// Read ceata device. Software should set this bit to indicate that CE-ATA device i
    READ_CEATA_DEVICE OFFSET(22) NUMBITS(1) [
        /// No read. Host is not performing read access (RW_REG or RW_BLK) towards CE-ATA de
        NoReadHostIsNotPerformingReadAccessRW_REGOrRW_BLKTowardsCEATADevice = 0,
        /// Read. Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device.
        ReadHostIsPerformingReadAccessRW_REGOrRW_BLKTowardsCEATADevice = 1
    ],
    /// CCS expected. If the command expects Command Completion Signal (CCS) from the CE
    CCS_EXPECTED OFFSET(23) NUMBITS(1) [
        /// Disabled. Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control r
        DISABLED = 0,
        /// Enabled. Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command
        ENABLED = 1
    ],
    /// Enable Boot - this bit should be set only for mandatory boot mode. When Software
    ENABLE_BOOT OFFSET(24) NUMBITS(1) [],
    /// Expect Boot Acknowledge. When Software sets this bit along with enable_boot, CIU
    EXPECT_BOOT_ACK OFFSET(25) NUMBITS(1) [],
    /// Disable Boot. When software sets this bit along with start_cmd, CIU terminates t
    DISABLE_BOOT OFFSET(26) NUMBITS(1) [],
    /// Boot Mode
    BOOT_MODE OFFSET(27) NUMBITS(1) [
        /// Mandatory Boot operation
        MandatoryBootOperation = 0,
        /// Alternate Boot operation
        AlternateBootOperation = 1
    ],
    /// Voltage switch bit
    VOLT_SWITCH OFFSET(28) NUMBITS(1) [
        /// Disabled. No voltage switching
        DisabledNoVoltageSwitching = 0,
        /// Enabled. Voltage switching enabled; must be set for CMD11 only
        EnabledVoltageSwitchingEnabledMustBeSetForCMD11Only = 1
    ],
    /// Start command. Once command is taken by CIU, this bit is cleared. When bit is se
    START_CMD OFFSET(31) NUMBITS(1) []
],
MINTSTS [
    /// Card detect. Interrupt enabled only if corresponding bit in interrupt mask regis
    CDET OFFSET(0) NUMBITS(1) [],
    /// Response error. Interrupt enabled only if corresponding bit in interrupt mask re
    RE OFFSET(1) NUMBITS(1) [],
    /// Command done. Interrupt enabled only if corresponding bit in interrupt mask regi
    CDONE OFFSET(2) NUMBITS(1) [],
    /// Data transfer over. Interrupt enabled only if corresponding bit in interrupt mas
    DTO OFFSET(3) NUMBITS(1) [],
    /// Transmit FIFO data request. Interrupt enabled only if corresponding bit in inter
    TXDR OFFSET(4) NUMBITS(1) [],
    /// Receive FIFO data request. Interrupt enabled only if corresponding bit in interr
    RXDR OFFSET(5) NUMBITS(1) [],
    /// Response CRC error. Interrupt enabled only if corresponding bit in interrupt mas
    RCRC OFFSET(6) NUMBITS(1) [],
    /// Data CRC error. Interrupt enabled only if corresponding bit in interrupt mask re
    DCRC OFFSET(7) NUMBITS(1) [],
    /// Response time-out. Interrupt enabled only if corresponding bit in interrupt mask
    RTO OFFSET(8) NUMBITS(1) [],
    /// Data read time-out. Interrupt enabled only if corresponding bit in interrupt mas
    DRTO OFFSET(9) NUMBITS(1) [],
    /// Data starvation-by-host time-out (HTO). Interrupt enabled only if corresponding
    HTO OFFSET(10) NUMBITS(1) [],
    /// FIFO underrun/overrun error. Interrupt enabled only if corresponding bit in inte
    FRUN OFFSET(11) NUMBITS(1) [],
    /// Hardware locked write error. Interrupt enabled only if corresponding bit in inte
    HLE OFFSET(12) NUMBITS(1) [],
    /// Start-bit error. Interrupt enabled only if corresponding bit in interrupt mask r
    SBE OFFSET(13) NUMBITS(1) [],
    /// Auto command done. Interrupt enabled only if corresponding bit in interrupt mask
    ACD OFFSET(14) NUMBITS(1) [],
    /// End-bit error (read)/write no CRC. Interrupt enabled only if corresponding bit i
    EBE OFFSET(15) NUMBITS(1) [],
    /// Interrupt from SDIO card. SDIO interrupt for card enabled only if corresponding
    SDIO_INTERRUPT OFFSET(16) NUMBITS(1) []
],
RINTSTS [
    /// Card detect. Writes to bits clear status bit. Value of 1 clears status bit, and
    CDET OFFSET(0) NUMBITS(1) [],
    /// Response error. Writes to bits clear status bit. Value of 1 clears status bit, a
    RE OFFSET(1) NUMBITS(1) [],
    /// Command done. Writes to bits clear status bit. Value of 1 clears status bit, and
    CDONE OFFSET(2) NUMBITS(1) [],
    /// Data transfer over. Writes to bits clear status bit. Value of 1 clears status bi
    DTO OFFSET(3) NUMBITS(1) [],
    /// Transmit FIFO data request. Writes to bits clear status bit. Value of 1 clears s
    TXDR OFFSET(4) NUMBITS(1) [],
    /// Receive FIFO data request. Writes to bits clear status bit. Value of 1 clears st
    RXDR OFFSET(5) NUMBITS(1) [],
    /// Response CRC error. Writes to bits clear status bit. Value of 1 clears status bi
    RCRC OFFSET(6) NUMBITS(1) [],
    /// Data CRC error. Writes to bits clear status bit. Value of 1 clears status bit, a
    DCRC OFFSET(7) NUMBITS(1) [],
    /// Response time-out (RTO)/Boot Ack Received (BAR). Writes to bits clear status bit
    RTO_BAR OFFSET(8) NUMBITS(1) [],
    /// Data read time-out (DRTO)/Boot Data Start (BDS). Writes to bits clear status bit
    DRTO_BDS OFFSET(9) NUMBITS(1) [],
    /// Data starvation-by-host time-out (HTO). Writes to bits clear status bit. Value o
    HTO OFFSET(10) NUMBITS(1) [],
    /// FIFO underrun/overrun error. Writes to bits clear status bit. Value of 1 clears
    FRUN OFFSET(11) NUMBITS(1) [],
    /// Hardware locked write error. Writes to bits clear status bit. Value of 1 clears
    HLE OFFSET(12) NUMBITS(1) [],
    /// Start-bit error. Writes to bits clear status bit. Value of 1 clears status bit,
    SBE OFFSET(13) NUMBITS(1) [],
    /// Auto command done. Writes to bits clear status bit. Value of 1 clears status bit
    ACD OFFSET(14) NUMBITS(1) [],
    /// End-bit error (read)/write no CRC. Writes to bits clear status bit. Value of 1 c
    EBE OFFSET(15) NUMBITS(1) [],
    /// Interrupt from SDIO card. Writes to these bits clear them. Value of 1 clears bit
    SDIO_INTERRUPT OFFSET(16) NUMBITS(1) []
],
STATUS [
    /// FIFO reached Receive watermark level; not qualified with data transfer.
    FIFO_RX_WATERMARK OFFSET(0) NUMBITS(1) [],
    /// FIFO reached Transmit watermark level; not qualified with data transfer.
    FIFO_TX_WATERMARK OFFSET(1) NUMBITS(1) [],
    /// FIFO is empty status
    FIFO_EMPTY OFFSET(2) NUMBITS(1) [],
    /// FIFO is full status
    FIFO_FULL OFFSET(3) NUMBITS(1) [],
    /// Command FSM states:  0 - Idle  1 - Send init sequence  2 - Tx cmd start bit  3 -
    CMDFSMSTATES OFFSET(4) NUMBITS(4) [],
    /// Raw selected card_data[3]; checks whether card is present  0 - card not present
    DATA_3_STATUS OFFSET(8) NUMBITS(1) [],
    /// Inverted version of raw selected card_data[0]  0 - card data not busy  1 - card
    DATA_BUSY OFFSET(9) NUMBITS(1) [],
    /// Data transmit or receive state-machine is busy
    DATA_STATE_MC_BUSY OFFSET(10) NUMBITS(1) [],
    /// Index of previous response, including any auto-stop sent by core.
    RESPONSE_INDEX OFFSET(11) NUMBITS(6) [],
    /// FIFO count - Number of filled locations in FIFO
    FIFO_COUNT OFFSET(17) NUMBITS(13) [],
    /// DMA acknowledge signal state
    DMA_ACK OFFSET(30) NUMBITS(1) [],
    /// DMA request signal state
    DMA_REQ OFFSET(31) NUMBITS(1) []
],
FIFOTH [
    /// FIFO threshold watermark level when transmitting data to card. When FIFO data co
    TX_WMARK OFFSET(0) NUMBITS(12) [],
    /// FIFO threshold watermark level when receiving data to card. When FIFO data count
    RX_WMARK OFFSET(16) NUMBITS(12) [],
    /// Burst size of multiple transaction; should be programmed same as DW-DMA controll
    DMA_MTS OFFSET(28) NUMBITS(3) [
        /// 1 transfer
        _1Transfer = 0,
        /// 4 transfers
        _4Transfers = 1,
        /// 8 transfers
        _8Transfers = 2,
        /// 16 transfers
        _16Transfers = 3,
        /// 32 transfers
        _32Transfers = 4,
        /// 64 transfers
        _64Transfers = 5,
        /// 128 transfers
        _128Transfers = 6,
        /// 256 transfers
        _256Transfers = 7
    ]
],
BMOD [
    /// Software Reset. When set, the DMA Controller resets all its internal registers.
    SWR OFFSET(0) NUMBITS(1) [],
    /// Fixed Burst. Controls whether the AHB Master interface performs fixed burst tran
    FB OFFSET(1) NUMBITS(1) [],
    /// Descriptor Skip Length. Specifies the number of HWord/Word/Dword to skip between
    DSL OFFSET(2) NUMBITS(5) [],
    /// SD/MMC DMA Enable. When set, the SD/MMC DMA is enabled. DE is read/write.
    DE OFFSET(7) NUMBITS(1) [],
    /// Programmable Burst Length. These bits indicate the maximum number of beats to be
    PBL OFFSET(8) NUMBITS(3) [
        /// 1 transfer
        _1Transfer = 0,
        /// 4 transfers
        _4Transfers = 1,
        /// 8 transfers
        _8Transfers = 2,
        /// 16 transfers
        _16Transfers = 3,
        /// 32 transfers
        _32Transfers = 4,
        /// 64 transfers
        _64Transfers = 5,
        /// 128 transfers
        _128Transfers = 6,
        /// 256 transfers
        _256Transfers = 7
    ]
],
IDSTS [
    /// Transmit Interrupt. Indicates that data transmission is finished for a descripto
    TI OFFSET(0) NUMBITS(1) [],
    /// Receive Interrupt. Indicates the completion of data reception for a descriptor.
    RI OFFSET(1) NUMBITS(1) [],
    /// Fatal Bus Error Interrupt. Indicates that a Bus Error occurred (IDSTS[12:10]). W
    FBE OFFSET(2) NUMBITS(1) [],
    /// Descriptor Unavailable Interrupt. This bit is set when the descriptor is unavail
    DU OFFSET(4) NUMBITS(1) [],
    /// Card Error Summary. Indicates the status of the transaction to/from the card; al
    CES OFFSET(5) NUMBITS(1) [],
    /// Normal Interrupt Summary. Logical OR of the following: IDSTS[0] - Transmit Inter
    NIS OFFSET(8) NUMBITS(1) [],
    /// Abnormal Interrupt Summary. Logical OR of the following: IDSTS[2] - Fatal Bus In
    AIS OFFSET(9) NUMBITS(1) [],
    /// Error Bits. Indicates the type of error that caused a Bus Error. Valid only with
    EB OFFSET(10) NUMBITS(3) [],
    /// DMAC state machine present state.  0 - DMA_IDLE  1 - DMA_SUSPEND  2 - DESC_RD  3
    FSM OFFSET(13) NUMBITS(4) []
],
IDINTEN [
    /// Transmit Interrupt Enable. When set with Normal Interrupt Summary Enable, Transm
    TI OFFSET(0) NUMBITS(1) [],
    /// Receive Interrupt Enable. When set with Normal Interrupt Summary Enable, Receive
    RI OFFSET(1) NUMBITS(1) [],
    /// Fatal Bus Error Enable. When set with Abnormal Interrupt Summary Enable, the Fat
    FBE OFFSET(2) NUMBITS(1) [],
    /// Descriptor Unavailable Interrupt. When set along with Abnormal Interrupt Summary
    DU OFFSET(4) NUMBITS(1) [],
    /// Card Error summary Interrupt Enable. When set, it enables the Card Interrupt sum
    CES OFFSET(5) NUMBITS(1) [],
    /// Normal Interrupt Summary Enable. When set, a normal interrupt is enabled. When r
    NIS OFFSET(8) NUMBITS(1) [],
    /// Abnormal Interrupt Summary Enable. When set, an abnormal interrupt is enabled. T
    AIS OFFSET(9) NUMBITS(1) []
]
];
const SDMMC_BASE: StaticRef<SdmmcRegisters> =
    unsafe { StaticRef::new(0x40004000 as *const SdmmcRegisters) };
