
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// C_CAN
#[repr(C)]
struct C_Can1Registers {
/// CAN control
cntl: ReadWrite<u32, CNTL::Register>,
/// Status register
stat: ReadWrite<u32, STAT::Register>,
/// Error counter
ec: ReadOnly<u32, EC::Register>,
/// Bit timing register
bt: ReadWrite<u32, BT::Register>,
/// Interrupt register
int: ReadOnly<u32>,
/// Test register
test: ReadWrite<u32, TEST::Register>,
/// Baud rate prescaler extension register
brpe: ReadWrite<u32>,
_reserved0: [u8; 4],
/// Message interface  command request
if1_cmdreq: ReadWrite<u32, IF1_CMDREQ::Register>,
/// Message interface command mask (write direction)
if1_cmdmsk_w: ReadWrite<u32, IF1_CMDMSK_W::Register>,
/// Message interface  mask 1
if1_msk1: ReadWrite<u32>,
/// Message interface 1 mask 2
if1_msk2: ReadWrite<u32, IF1_MSK2::Register>,
/// Message interface 1 arbitration 1
if1_arb1: ReadWrite<u32>,
/// Message interface 1 arbitration 2
if1_arb2: ReadWrite<u32, IF1_ARB2::Register>,
/// Message interface 1 message control
if1_mctrl: ReadWrite<u32, IF1_MCTRL::Register>,
/// Message interface  data A1
if1_da1: ReadWrite<u32, IF1_DA1::Register>,
/// Message interface 1 data A2
if1_da2: ReadWrite<u32, IF1_DA2::Register>,
/// Message interface 1 data B1
if1_db1: ReadWrite<u32, IF1_DB1::Register>,
/// Message interface 1 data B2
if1_db2: ReadWrite<u32, IF1_DB2::Register>,
_reserved1: [u8; 52],
/// Message interface  command request
if2_cmdreq: ReadWrite<u32, IF2_CMDREQ::Register>,
/// Message interface command mask (write direction)
if2_cmdmsk_w: ReadWrite<u32, IF2_CMDMSK_W::Register>,
/// Message interface  mask 1
if2_msk1: ReadWrite<u32>,
/// Message interface 1 mask 2
if2_msk2: ReadWrite<u32, IF2_MSK2::Register>,
/// Message interface 1 arbitration 1
if2_arb1: ReadWrite<u32>,
/// Message interface 1 arbitration 2
if2_arb2: ReadWrite<u32, IF2_ARB2::Register>,
/// Message interface 1 message control
if2_mctrl: ReadWrite<u32, IF2_MCTRL::Register>,
/// Message interface  data A1
if2_da1: ReadWrite<u32, IF2_DA1::Register>,
/// Message interface 1 data A2
if2_da2: ReadWrite<u32, IF2_DA2::Register>,
/// Message interface 1 data B1
if2_db1: ReadWrite<u32, IF2_DB1::Register>,
/// Message interface 1 data B2
if2_db2: ReadWrite<u32, IF2_DB2::Register>,
_reserved2: [u8; 84],
/// Transmission request 1
txreq1: ReadOnly<u32>,
/// Transmission request 2
txreq2: ReadOnly<u32>,
_reserved3: [u8; 24],
/// New data 1
nd1: ReadOnly<u32>,
/// New data 2
nd2: ReadOnly<u32>,
_reserved4: [u8; 24],
/// Interrupt pending 1
ir1: ReadOnly<u32>,
/// Interrupt pending 2
ir2: ReadOnly<u32>,
_reserved5: [u8; 24],
/// Message valid 1
msgv1: ReadOnly<u32>,
/// Message valid 2
msgv2: ReadOnly<u32>,
_reserved6: [u8; 24],
/// CAN clock divider register
clkdiv: ReadWrite<u32>,
}
register_bitfields![u32,
CNTL [
    /// Initialization
    INIT OFFSET(0) NUMBITS(1) [
        /// Initialization is started. On reset, software needs to initialize the CAN contro
        InitializationIsStartedOnResetSoftwareNeedsToInitializeTheCANController = 1,
        /// Normal operation.
        NormalOperation = 0
    ],
    /// Module interrupt enable
    IE OFFSET(1) NUMBITS(1) [
        /// Enable CAN interrupts. The interrupt line is set to LOW and remains LOW until al
        ENABLE_CAN_INTERRUPT = 1,
        /// Disable CAN interrupts. The interrupt line is always HIGH.
        DisableCANInterruptsTheInterruptLineIsAlwaysHIGH = 0
    ],
    /// Status change interrupt enable
    SIE OFFSET(2) NUMBITS(1) [
        /// Enable status change interrupts. A status change interrupt will be generated whe
        ENABLE_STATUS_CHANGE = 1,
        /// Disable status change interrupts. No status change interrupt will be generated.
        DisableStatusChangeInterruptsNoStatusChangeInterruptWillBeGenerated = 0
    ],
    /// Error interrupt enable
    EIE OFFSET(3) NUMBITS(1) [
        /// Enable error interrupt. A change in the bits BOFF or EWARN in the CANSTAT regist
        ENABLE_ERROR_INTERRU = 1,
        /// Disable error interrupt. No error status interrupt will be generated.
        DisableErrorInterruptNoErrorStatusInterruptWillBeGenerated = 0
    ],
    /// Disable automatic retransmission
    DAR OFFSET(5) NUMBITS(1) [
        /// Automatic retransmission disabled.
        AutomaticRetransmissionDisabled = 1,
        /// Automatic retransmission of disturbed messages enabled.
        AutomaticRetransmissionOfDisturbedMessagesEnabled = 0
    ],
    /// Configuration change enable
    CCE OFFSET(6) NUMBITS(1) [
        /// The CPU has write access to the CANBT register while the INIT bit is one.
        TheCPUHasWriteAccessToTheCANBTRegisterWhileTheINITBitIsOne = 1,
        /// The CPU has no write access to the bit timing register.
        TheCPUHasNoWriteAccessToTheBitTimingRegister = 0
    ],
    /// Test mode enable
    TEST OFFSET(7) NUMBITS(1) [
        /// Test mode.
        TestMode = 1,
        /// Normal operation.
        NormalOperation = 0
    ]
],
STAT [
    /// Last error code Type of the last error to occur on the CAN bus.The LEC field hol
    LEC OFFSET(0) NUMBITS(3) [
        /// No error.
        NoError = 0,
        /// Stuff error: More than 5 equal bits in a sequence have occurred in a part of a r
        STUFF_ERROR_MORE_TH = 1,
        /// Form error: A fixed format part of a received frame has the wrong format.
        FormErrorAFixedFormatPartOfAReceivedFrameHasTheWrongFormat = 2,
        /// AckError: The message this CAN core transmitted was not acknowledged.
        AckErrorTheMessageThisCANCoreTransmittedWasNotAcknowledged = 3,
        /// Bit1Error: During the transmission of a message (with the exception of the arbit
        BIT1ERROR_DURING_TH = 4,
        /// Bit0Error: During the transmission of a message (or acknowledge bit, or active e
        BIT0ERROR_DURING_TH = 5,
        /// CRCError: The CRC checksum was incorrect in the message received.
        CRCErrorTheCRCChecksumWasIncorrectInTheMessageReceived = 6,
        /// Unused: No CAN bus event was detected (written by the CPU).
        UnusedNoCANBusEventWasDetectedWrittenByTheCPU = 7
    ],
    /// Transmitted a message successfully This bit is reset by the CPU. It is never res
    TXOK OFFSET(3) NUMBITS(1) [
        /// Since this bit was last reset by the CPU, a message has been successfully transm
        MSGTRANSFER = 1,
        /// Since this bit was reset by the CPU, no message has been successfully transmitte
        SinceThisBitWasResetByTheCPUNoMessageHasBeenSuccessfullyTransmitted = 0
    ],
    /// Received a message successfully This bit is reset by the CPU. It is never reset
    RXOK OFFSET(4) NUMBITS(1) [
        /// Since this bit was last set to zero by the CPU, a message has been successfully
        MSGTRANSFER = 1,
        /// Since this bit was last reset by the CPU, no message has been successfully trans
        SinceThisBitWasLastResetByTheCPUNoMessageHasBeenSuccessfullyTransmitted = 0
    ],
    /// Error passive
    EPASS OFFSET(5) NUMBITS(1) [
        /// The CAN controller is in the error passive state as defined in the  CAN 2.0 spec
        TheCANControllerIsInTheErrorPassiveStateAsDefinedInTheCAN20Specification = 1,
        /// The CAN controller is in the error active state.
        TheCANControllerIsInTheErrorActiveState = 0
    ],
    /// Warning status
    EWARN OFFSET(6) NUMBITS(1) [
        /// At least one of the error counters in the EML has reached the error warning limi
        AtLeastOneOfTheErrorCountersInTheEMLHasReachedTheErrorWarningLimitOf96 = 1,
        /// Both error counters are below the error warning limit of 96.
        BothErrorCountersAreBelowTheErrorWarningLimitOf96 = 0
    ],
    /// Busoff status
    BOFF OFFSET(7) NUMBITS(1) [
        /// The CAN controller is in busoff state.
        TheCANControllerIsInBusoffState = 1,
        /// The CAN module is not in busoff.
        TheCANModuleIsNotInBusoff = 0
    ]
],
EC [
    /// Transmit error counter Current value of the transmit error counter (maximum valu
    TEC_7_0 OFFSET(0) NUMBITS(8) [],
    /// Receive error counter Current value of the receive error counter (maximum value
    REC_6_0 OFFSET(8) NUMBITS(7) [],
    /// Receive error passive
    RP OFFSET(15) NUMBITS(1) [
        /// The receive counter has reached the error passive level as defined in the  CAN2.
        TheReceiveCounterHasReachedTheErrorPassiveLevelAsDefinedInTheCAN20Specification = 1,
        /// The receive counter is below the error passive level.
        TheReceiveCounterIsBelowTheErrorPassiveLevel = 0
    ]
],
BT [
    /// Baud rate prescaler The value by which the oscillator frequency is divided for g
    BRP OFFSET(0) NUMBITS(6) [],
    /// (Re)synchronization jump width Valid programmed values are 0 to 3[1].
    SJW OFFSET(6) NUMBITS(2) [],
    /// Time segment after the sample point Valid values are 0 to 7[1].
    TSEG1 OFFSET(8) NUMBITS(4) [],
    /// Time segment before the sample point Valid values are 1 to 15[1].
    TSEG2 OFFSET(12) NUMBITS(3) []
],
TEST [
    /// Basic mode
    BASIC OFFSET(2) NUMBITS(1) [
        /// IF1 registers used as TX buffer, IF2 registers used as RX buffer.
        IF1RegistersUsedAsTXBufferIF2RegistersUsedAsRXBuffer = 1,
        /// Basic mode disabled.
        BasicModeDisabled = 0
    ],
    /// Silent mode
    SILENT OFFSET(3) NUMBITS(1) [
        /// The module is in silent mode.
        TheModuleIsInSilentMode = 1,
        /// Normal operation.
        NormalOperation = 0
    ],
    /// Loop back mode
    LBACK OFFSET(4) NUMBITS(1) [
        /// Loop back mode is enabled.
        LoopBackModeIsEnabled = 1,
        /// Loop back mode is disabled.
        LoopBackModeIsDisabled = 0
    ],
    /// Control of TD pins
    TX1_0 OFFSET(5) NUMBITS(2) [
        /// Level at the TD pin is controlled by the CAN controller. This is the value at re
        LevelAtTheTDPinIsControlledByTheCANControllerThisIsTheValueAtReset = 0,
        /// The sample point can be monitored at the TD pin.
        TheSamplePointCanBeMonitoredAtTheTDPin = 1,
        /// TD pin is driven LOW/dominant.
        TDPinIsDrivenLOWDominant = 2,
        /// TD pin is driven HIGH/recessive.
        TDPinIsDrivenHIGHRecessive = 3
    ],
    /// Monitors the actual value of the RD Pin
    RX OFFSET(7) NUMBITS(1) [
        /// The CAN bus is recessive (RD = 1).
        TheCANBusIsRecessiveRD1 = 1,
        /// The CAN bus is dominant (RD = 0).
        TheCANBusIsDominantRD0 = 0
    ]
],
IF1_CMDREQ [
    /// Message number 0x01 to 0x20 = Valid message numbers The message object in the me
    MESSNUM OFFSET(0) NUMBITS(6) [],
    /// BUSY flag. Set to one by hardware when writing to this Command request register.
    BUSY OFFSET(15) NUMBITS(1) []
],
IF2_CMDREQ [
    /// Message number 0x01 to 0x20 = Valid message numbers The message object in the me
    MESSNUM OFFSET(0) NUMBITS(6) [],
    /// BUSY flag. Set to one by hardware when writing to this Command request register.
    BUSY OFFSET(15) NUMBITS(1) []
],
IF1_CMDMSK_W [
    /// Access data bytes 4-7
    DATA_B OFFSET(0) NUMBITS(1) [
        /// Transfer data bytes 4-7 to message object.
        TransferDataBytes47ToMessageObject = 1,
        /// data bytes 4-7 unchanged.
        DataBytes47Unchanged = 0
    ],
    /// Access data bytes 0-3
    DATA_A OFFSET(1) NUMBITS(1) [
        /// Transfer data bytes 0-3 to message object.
        TransferDataBytes03ToMessageObject = 1,
        /// data bytes 0-3 unchanged.
        DataBytes03Unchanged = 0
    ],
    /// Access transmission request bit
    TXRQST OFFSET(2) NUMBITS(1) [
        /// Request a transmission. Set the TXRQST bit IF1/2_MCTRL.
        RequestATransmissionSetTheTXRQSTBitIF12_MCTRL = 1,
        /// No transmission request. TXRQSRT bit unchanged in IF1/2_MCTRL. If a transmission
        NO_TRANSMISSION_REQU = 0
    ],
    /// This bit is ignored in the write direction.
    CLRINTPND OFFSET(3) NUMBITS(1) [],
    /// Access control bits
    CTRL OFFSET(4) NUMBITS(1) [
        /// Transfer control bits to message object
        TransferControlBitsToMessageObject = 1,
        /// Control bits unchanged.
        ControlBitsUnchanged = 0
    ],
    /// Access arbitration bits
    ARB OFFSET(5) NUMBITS(1) [
        /// Transfer Identifier, DIR, XTD, and MSGVAL bits to message object.
        TransferIdentifierDIRXTDAndMSGVALBitsToMessageObject = 1,
        /// Arbitration bits unchanged.
        ArbitrationBitsUnchanged = 0
    ],
    /// Access mask bits
    MASK OFFSET(6) NUMBITS(1) [
        /// Transfer Identifier MASK + MDIR + MXTD to message object.
        TransferIdentifierMASKMDIRMXTDToMessageObject = 1,
        /// Mask bits unchanged.
        MaskBitsUnchanged = 0
    ],
    /// Write transfer Transfer data from the selected message buffer registers to the m
    WR_RD OFFSET(7) NUMBITS(1) []
],
IF2_CMDMSK_W [
    /// Access data bytes 4-7
    DATA_B OFFSET(0) NUMBITS(1) [
        /// Transfer data bytes 4-7 to message object.
        TransferDataBytes47ToMessageObject = 1,
        /// data bytes 4-7 unchanged.
        DataBytes47Unchanged = 0
    ],
    /// Access data bytes 0-3
    DATA_A OFFSET(1) NUMBITS(1) [
        /// Transfer data bytes 0-3 to message object.
        TransferDataBytes03ToMessageObject = 1,
        /// data bytes 0-3 unchanged.
        DataBytes03Unchanged = 0
    ],
    /// Access transmission request bit
    TXRQST OFFSET(2) NUMBITS(1) [
        /// Request a transmission. Set the TXRQST bit IF1/2_MCTRL.
        RequestATransmissionSetTheTXRQSTBitIF12_MCTRL = 1,
        /// No transmission request. TXRQSRT bit unchanged in IF1/2_MCTRL. If a transmission
        NO_TRANSMISSION_REQU = 0
    ],
    /// This bit is ignored in the write direction.
    CLRINTPND OFFSET(3) NUMBITS(1) [],
    /// Access control bits
    CTRL OFFSET(4) NUMBITS(1) [
        /// Transfer control bits to message object
        TransferControlBitsToMessageObject = 1,
        /// Control bits unchanged.
        ControlBitsUnchanged = 0
    ],
    /// Access arbitration bits
    ARB OFFSET(5) NUMBITS(1) [
        /// Transfer Identifier, DIR, XTD, and MSGVAL bits to message object.
        TransferIdentifierDIRXTDAndMSGVALBitsToMessageObject = 1,
        /// Arbitration bits unchanged.
        ArbitrationBitsUnchanged = 0
    ],
    /// Access mask bits
    MASK OFFSET(6) NUMBITS(1) [
        /// Transfer Identifier MASK + MDIR + MXTD to message object.
        TransferIdentifierMASKMDIRMXTDToMessageObject = 1,
        /// Mask bits unchanged.
        MaskBitsUnchanged = 0
    ],
    /// Write transfer Transfer data from the selected message buffer registers to the m
    WR_RD OFFSET(7) NUMBITS(1) []
],
IF1_CMDMSK_R [
    /// Access data bytes 4-7
    DATA_B OFFSET(0) NUMBITS(1) [
        /// Transfer data bytes 4-7 to IFx message buffer register.
        TransferDataBytes47ToIFxMessageBufferRegister = 1,
        /// data bytes 4-7 unchanged.
        DataBytes47Unchanged = 0
    ],
    /// Access data bytes 0-3
    DATA_A OFFSET(1) NUMBITS(1) [
        /// Transfer data bytes 0-3 to IFx message buffer.
        TransferDataBytes03ToIFxMessageBuffer = 1,
        /// data bytes 0-3 unchanged.
        DataBytes03Unchanged = 0
    ],
    /// Access new data bit
    NEWDAT OFFSET(2) NUMBITS(1) [
        /// Clear NEWDAT bit in the message object.
        ClearNEWDATBitInTheMessageObject = 1,
        /// NEWDAT bit remains unchanged. A read access to a message object can be combined
        NEWDAT_BIT_REMAINS_U = 0
    ],
    /// Clear interrupt pending bit.
    CLRINTPND OFFSET(3) NUMBITS(1) [
        /// Clear INTPND bit in the message object.
        ClearINTPNDBitInTheMessageObject = 1,
        /// INTPND bit remains unchanged.
        INTPNDBitRemainsUnchanged = 0
    ],
    /// Access control bits
    CTRL OFFSET(4) NUMBITS(1) [
        /// Transfer control bits to IFx message buffer.
        TransferControlBitsToIFxMessageBuffer = 1,
        /// Control bits unchanged.
        ControlBitsUnchanged = 0
    ],
    /// Access arbitration bits
    ARB OFFSET(5) NUMBITS(1) [
        /// Transfer Identifier, DIR, XTD, and MSGVAL bits to IFx message buffer register.
        TransferIdentifierDIRXTDAndMSGVALBitsToIFxMessageBufferRegister = 1,
        /// Arbitration bits unchanged.
        ArbitrationBitsUnchanged = 0
    ],
    /// Access mask bits
    MASK OFFSET(6) NUMBITS(1) [
        /// Transfer Identifier MASK + MDIR + MXTD to IFx message buffer register.
        TransferIdentifierMASKMDIRMXTDToIFxMessageBufferRegister = 1,
        /// Mask bits unchanged.
        MaskBitsUnchanged = 0
    ],
    /// Read transfer Transfer data from the message object addressed by the command req
    WR_RD OFFSET(7) NUMBITS(1) []
],
IF2_CMDMSK_R [
    /// Access data bytes 4-7
    DATA_B OFFSET(0) NUMBITS(1) [
        /// Transfer data bytes 4-7 to IFx message buffer register.
        TransferDataBytes47ToIFxMessageBufferRegister = 1,
        /// data bytes 4-7 unchanged.
        DataBytes47Unchanged = 0
    ],
    /// Access data bytes 0-3
    DATA_A OFFSET(1) NUMBITS(1) [
        /// Transfer data bytes 0-3 to IFx message buffer.
        TransferDataBytes03ToIFxMessageBuffer = 1,
        /// data bytes 0-3 unchanged.
        DataBytes03Unchanged = 0
    ],
    /// Access new data bit
    NEWDAT OFFSET(2) NUMBITS(1) [
        /// Clear NEWDAT bit in the message object.
        ClearNEWDATBitInTheMessageObject = 1,
        /// NEWDAT bit remains unchanged. A read access to a message object can be combined
        NEWDAT_BIT_REMAINS_U = 0
    ],
    /// Clear interrupt pending bit.
    CLRINTPND OFFSET(3) NUMBITS(1) [
        /// Clear INTPND bit in the message object.
        ClearINTPNDBitInTheMessageObject = 1,
        /// INTPND bit remains unchanged.
        INTPNDBitRemainsUnchanged = 0
    ],
    /// Access control bits
    CTRL OFFSET(4) NUMBITS(1) [
        /// Transfer control bits to IFx message buffer.
        TransferControlBitsToIFxMessageBuffer = 1,
        /// Control bits unchanged.
        ControlBitsUnchanged = 0
    ],
    /// Access arbitration bits
    ARB OFFSET(5) NUMBITS(1) [
        /// Transfer Identifier, DIR, XTD, and MSGVAL bits to IFx message buffer register.
        TransferIdentifierDIRXTDAndMSGVALBitsToIFxMessageBufferRegister = 1,
        /// Arbitration bits unchanged.
        ArbitrationBitsUnchanged = 0
    ],
    /// Access mask bits
    MASK OFFSET(6) NUMBITS(1) [
        /// Transfer Identifier MASK + MDIR + MXTD to IFx message buffer register.
        TransferIdentifierMASKMDIRMXTDToIFxMessageBufferRegister = 1,
        /// Mask bits unchanged.
        MaskBitsUnchanged = 0
    ],
    /// Read transfer Transfer data from the message object addressed by the command req
    WR_RD OFFSET(7) NUMBITS(1) []
],
IF1_MSK2 [
    /// Identifier mask 0 = The corresponding bit in the identifier of the message can n
    MSK28_16 OFFSET(0) NUMBITS(13) [],
    /// Mask message direction
    MDIR OFFSET(14) NUMBITS(1) [
        /// The message direction bit (DIR) is used for acceptance filtering.
        TheMessageDirectionBitDIRIsUsedForAcceptanceFiltering = 1,
        /// The message direction bit (DIR) has no effect on acceptance filtering.
        TheMessageDirectionBitDIRHasNoEffectOnAcceptanceFiltering = 0
    ],
    /// Mask extend identifier
    MXTD OFFSET(15) NUMBITS(1) [
        /// The extended identifier bit (IDE) is used for acceptance filtering.
        TheExtendedIdentifierBitIDEIsUsedForAcceptanceFiltering = 1,
        /// The extended identifier bit (IDE) has no effect on acceptance filtering.
        TheExtendedIdentifierBitIDEHasNoEffectOnAcceptanceFiltering = 0
    ]
],
IF2_MSK2 [
    /// Identifier mask 0 = The corresponding bit in the identifier of the message can n
    MSK28_16 OFFSET(0) NUMBITS(13) [],
    /// Mask message direction
    MDIR OFFSET(14) NUMBITS(1) [
        /// The message direction bit (DIR) is used for acceptance filtering.
        TheMessageDirectionBitDIRIsUsedForAcceptanceFiltering = 1,
        /// The message direction bit (DIR) has no effect on acceptance filtering.
        TheMessageDirectionBitDIRHasNoEffectOnAcceptanceFiltering = 0
    ],
    /// Mask extend identifier
    MXTD OFFSET(15) NUMBITS(1) [
        /// The extended identifier bit (IDE) is used for acceptance filtering.
        TheExtendedIdentifierBitIDEIsUsedForAcceptanceFiltering = 1,
        /// The extended identifier bit (IDE) has no effect on acceptance filtering.
        TheExtendedIdentifierBitIDEHasNoEffectOnAcceptanceFiltering = 0
    ]
],
IF1_ARB2 [
    /// Message identifier  29-bit identifier (extended frame) 11-bit identifier (standa
    ID28_16 OFFSET(0) NUMBITS(13) [],
    /// Message direction
    DIR OFFSET(13) NUMBITS(1) [
        /// Direction = transmit. On TXRQST, the respective Message Object is transmitted as
        DIRECTION_EQ_TRANSMIT = 1,
        /// Direction = receive. On TXRQST, a Remote Frame with the identifier of this Messa
        DIRECTION_EQ_RECEIVE_ = 0
    ],
    /// Extend identifier
    XTD OFFSET(14) NUMBITS(1) [
        /// The 29-bit extended identifier will be used for this message object.
        The29BitExtendedIdentifierWillBeUsedForThisMessageObject = 1,
        /// The 11-bit standard identifier will be used for this message object.
        The11BitStandardIdentifierWillBeUsedForThisMessageObject = 0
    ],
    /// Message valid The CPU must reset the MSGVAL bit of all unused Messages Objects d
    MSGVAL OFFSET(15) NUMBITS(1) [
        /// The message object is configured and should be considered by the message handler
        TheMessageObjectIsConfiguredAndShouldBeConsideredByTheMessageHandler = 1,
        /// The message object is ignored by the message handler.
        TheMessageObjectIsIgnoredByTheMessageHandler = 0
    ]
],
IF2_ARB2 [
    /// Message identifier  29-bit identifier (extended frame) 11-bit identifier (standa
    ID28_16 OFFSET(0) NUMBITS(13) [],
    /// Message direction
    DIR OFFSET(13) NUMBITS(1) [
        /// Direction = transmit. On TXRQST, the respective Message Object is transmitted as
        DIRECTION_EQ_TRANSMIT = 1,
        /// Direction = receive. On TXRQST, a Remote Frame with the identifier of this Messa
        DIRECTION_EQ_RECEIVE_ = 0
    ],
    /// Extend identifier
    XTD OFFSET(14) NUMBITS(1) [
        /// The 29-bit extended identifier will be used for this message object.
        The29BitExtendedIdentifierWillBeUsedForThisMessageObject = 1,
        /// The 11-bit standard identifier will be used for this message object.
        The11BitStandardIdentifierWillBeUsedForThisMessageObject = 0
    ],
    /// Message valid The CPU must reset the MSGVAL bit of all unused Messages Objects d
    MSGVAL OFFSET(15) NUMBITS(1) [
        /// The message object is configured and should be considered by the message handler
        TheMessageObjectIsConfiguredAndShouldBeConsideredByTheMessageHandler = 1,
        /// The message object is ignored by the message handler.
        TheMessageObjectIsIgnoredByTheMessageHandler = 0
    ]
],
IF1_MCTRL [
    /// Data length code The Data Length Code of a Message Object must be defined the sa
    DLC3_0 OFFSET(0) NUMBITS(4) [],
    /// End of buffer
    EOB OFFSET(7) NUMBITS(1) [
        /// Single message object or last message object of a FIFO buffer.
        SingleMessageObjectOrLastMessageObjectOfAFIFOBuffer = 1,
        /// Message object belongs to a FIFO buffer and is not the last message object of th
        MessageObjectBelongsToAFIFOBufferAndIsNotTheLastMessageObjectOfThatFIFOBuffer = 0
    ],
    /// Transmit request
    TXRQST OFFSET(8) NUMBITS(1) [
        /// The transmission of this message object is requested and is not yet done
        TheTransmissionOfThisMessageObjectIsRequestedAndIsNotYetDone = 1,
        /// This message object is not waiting for transmission.
        ThisMessageObjectIsNotWaitingForTransmission = 0
    ],
    /// Remote enable
    RMTEN OFFSET(9) NUMBITS(1) [
        /// At the reception of a remote frame, TXRQST is set.
        AtTheReceptionOfARemoteFrameTXRQSTIsSet = 1,
        /// At the reception of a remote frame, TXRQST is left unchanged.
        AtTheReceptionOfARemoteFrameTXRQSTIsLeftUnchanged = 0
    ],
    /// Receive interrupt enable
    RXIE OFFSET(10) NUMBITS(1) [
        /// INTPND will be set after successful reception of a frame.
        INTPNDWillBeSetAfterSuccessfulReceptionOfAFrame = 1,
        /// INTPND will be left unchanged after successful reception of a frame.
        INTPNDWillBeLeftUnchangedAfterSuccessfulReceptionOfAFrame = 0
    ],
    /// Transmit interrupt enable
    TXIE OFFSET(11) NUMBITS(1) [
        /// INTPND will be set after a successful reception of a frame.
        INTPNDWillBeSetAfterASuccessfulReceptionOfAFrame = 1,
        /// The INTPND bit will be left unchanged after a successful reception of a frame.
        TheINTPNDBitWillBeLeftUnchangedAfterASuccessfulReceptionOfAFrame = 0
    ],
    /// Use acceptance mask If UMASK is set to 1, the message object's mask bits have to
    UMASK OFFSET(12) NUMBITS(1) [
        /// Use mask (MSK[28:0], MXTD, and MDIR) for acceptance filtering.
        UseMaskMSK280MXTDAndMDIRForAcceptanceFiltering = 1,
        /// Mask ignored.
        MaskIgnored = 0
    ],
    /// Interrupt pending
    INTPND OFFSET(13) NUMBITS(1) [
        /// This message object is the source of an interrupt. The Interrupt Identifier in t
        INTSOURCE = 1,
        /// This message object is not the source of an interrupt.
        ThisMessageObjectIsNotTheSourceOfAnInterrupt = 0
    ],
    /// Message lost (only valid for message objects in the direction receive).
    MSGLST OFFSET(14) NUMBITS(1) [
        /// The Message Handler stored a new message into this object when NEWDAT was still
        THE_MESSAGE_HANDLER_ = 1,
        /// No message lost since this bit was reset last by the CPU.
        NoMessageLostSinceThisBitWasResetLastByTheCPU = 0
    ],
    /// New data
    NEWDAT OFFSET(15) NUMBITS(1) [
        /// The message handler or the CPU has written new data into the data portion of thi
        TheMessageHandlerOrTheCPUHasWrittenNewDataIntoTheDataPortionOfThisMessageObject = 1,
        /// No new data has been written into the data portion of this message object by the
        NO_NEW_DATA_HAS_BEEN = 0
    ]
],
IF2_MCTRL [
    /// Data length code The Data Length Code of a Message Object must be defined the sa
    DLC3_0 OFFSET(0) NUMBITS(4) [],
    /// End of buffer
    EOB OFFSET(7) NUMBITS(1) [
        /// Single message object or last message object of a FIFO buffer.
        SingleMessageObjectOrLastMessageObjectOfAFIFOBuffer = 1,
        /// Message object belongs to a FIFO buffer and is not the last message object of th
        MessageObjectBelongsToAFIFOBufferAndIsNotTheLastMessageObjectOfThatFIFOBuffer = 0
    ],
    /// Transmit request
    TXRQST OFFSET(8) NUMBITS(1) [
        /// The transmission of this message object is requested and is not yet done
        TheTransmissionOfThisMessageObjectIsRequestedAndIsNotYetDone = 1,
        /// This message object is not waiting for transmission.
        ThisMessageObjectIsNotWaitingForTransmission = 0
    ],
    /// Remote enable
    RMTEN OFFSET(9) NUMBITS(1) [
        /// At the reception of a remote frame, TXRQST is set.
        AtTheReceptionOfARemoteFrameTXRQSTIsSet = 1,
        /// At the reception of a remote frame, TXRQST is left unchanged.
        AtTheReceptionOfARemoteFrameTXRQSTIsLeftUnchanged = 0
    ],
    /// Receive interrupt enable
    RXIE OFFSET(10) NUMBITS(1) [
        /// INTPND will be set after successful reception of a frame.
        INTPNDWillBeSetAfterSuccessfulReceptionOfAFrame = 1,
        /// INTPND will be left unchanged after successful reception of a frame.
        INTPNDWillBeLeftUnchangedAfterSuccessfulReceptionOfAFrame = 0
    ],
    /// Transmit interrupt enable
    TXIE OFFSET(11) NUMBITS(1) [
        /// INTPND will be set after a successful reception of a frame.
        INTPNDWillBeSetAfterASuccessfulReceptionOfAFrame = 1,
        /// The INTPND bit will be left unchanged after a successful reception of a frame.
        TheINTPNDBitWillBeLeftUnchangedAfterASuccessfulReceptionOfAFrame = 0
    ],
    /// Use acceptance mask If UMASK is set to 1, the message object's mask bits have to
    UMASK OFFSET(12) NUMBITS(1) [
        /// Use mask (MSK[28:0], MXTD, and MDIR) for acceptance filtering.
        UseMaskMSK280MXTDAndMDIRForAcceptanceFiltering = 1,
        /// Mask ignored.
        MaskIgnored = 0
    ],
    /// Interrupt pending
    INTPND OFFSET(13) NUMBITS(1) [
        /// This message object is the source of an interrupt. The Interrupt Identifier in t
        INTSOURCE = 1,
        /// This message object is not the source of an interrupt.
        ThisMessageObjectIsNotTheSourceOfAnInterrupt = 0
    ],
    /// Message lost (only valid for message objects in the direction receive).
    MSGLST OFFSET(14) NUMBITS(1) [
        /// The Message Handler stored a new message into this object when NEWDAT was still
        THE_MESSAGE_HANDLER_ = 1,
        /// No message lost since this bit was reset last by the CPU.
        NoMessageLostSinceThisBitWasResetLastByTheCPU = 0
    ],
    /// New data
    NEWDAT OFFSET(15) NUMBITS(1) [
        /// The message handler or the CPU has written new data into the data portion of thi
        TheMessageHandlerOrTheCPUHasWrittenNewDataIntoTheDataPortionOfThisMessageObject = 1,
        /// No new data has been written into the data portion of this message object by the
        NO_NEW_DATA_HAS_BEEN = 0
    ]
],
IF1_DA1 [
    /// Data byte 0
    DATA0 OFFSET(0) NUMBITS(8) [],
    /// Data byte 1
    DATA1 OFFSET(8) NUMBITS(8) []
],
IF2_DA1 [
    /// Data byte 0
    DATA0 OFFSET(0) NUMBITS(8) [],
    /// Data byte 1
    DATA1 OFFSET(8) NUMBITS(8) []
],
IF1_DA2 [
    /// Data byte 2
    DATA2 OFFSET(0) NUMBITS(8) [],
    /// Data byte 3
    DATA3 OFFSET(8) NUMBITS(8) []
],
IF2_DA2 [
    /// Data byte 2
    DATA2 OFFSET(0) NUMBITS(8) [],
    /// Data byte 3
    DATA3 OFFSET(8) NUMBITS(8) []
],
IF1_DB1 [
    /// Data byte 4
    DATA4 OFFSET(0) NUMBITS(8) [],
    /// Data byte 5
    DATA5 OFFSET(8) NUMBITS(8) []
],
IF2_DB1 [
    /// Data byte 4
    DATA4 OFFSET(0) NUMBITS(8) [],
    /// Data byte 5
    DATA5 OFFSET(8) NUMBITS(8) []
],
IF1_DB2 [
    /// Data byte 6
    DATA6 OFFSET(0) NUMBITS(8) [],
    /// Data byte 7
    DATA7 OFFSET(8) NUMBITS(8) []
],
IF2_DB2 [
    /// Data byte 6
    DATA6 OFFSET(0) NUMBITS(8) [],
    /// Data byte 7
    DATA7 OFFSET(8) NUMBITS(8) []
]
];
const C_CAN1_BASE: StaticRef<C_Can1Registers> =
    unsafe { StaticRef::new(0x400A4000 as *const C_Can1Registers) };
