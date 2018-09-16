
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// USART0_2_3
#[repr(C)]
struct UsartRegisters {
/// Receiver Buffer Register. Contains the next received character to be read (DLAB
rbr: ReadOnly<u32>,
/// Divisor Latch MSB. Most significant byte of the baud rate divisor value. The ful
dlm: ReadWrite<u32>,
/// Interrupt ID Register. Identifies which interrupt(s) are pending.
iir: ReadOnly<u32, IIR::Register>,
/// Line Control Register. Contains controls for frame formatting and break generati
lcr: ReadWrite<u32, LCR::Register>,
_reserved0: [u8; 4],
/// Line Status Register. Contains flags for transmit and receive status, including
lsr: ReadOnly<u32, LSR::Register>,
_reserved1: [u8; 4],
/// Scratch Pad Register. Eight-bit temporary storage for software.
scr: ReadWrite<u32>,
/// Auto-baud Control Register. Contains controls for the auto-baud feature.
acr: ReadWrite<u32, ACR::Register>,
/// IrDA control register (USART3 only)
icr: ReadWrite<u32, ICR::Register>,
/// Fractional Divider Register. Generates a clock input for the baud rate divider.
fdr: ReadWrite<u32, FDR::Register>,
/// Oversampling Register. Controls the degree of oversampling during each bit time.
osr: ReadWrite<u32, OSR::Register>,
_reserved2: [u8; 16],
/// Half-duplex enable Register
hden: ReadWrite<u32>,
_reserved3: [u8; 4],
/// Smart card interface control register
scictrl: ReadWrite<u32, SCICTRL::Register>,
/// RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485
rs485ctrl: ReadWrite<u32, RS485CTRL::Register>,
/// RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-48
rs485adrmatch: ReadWrite<u32>,
/// RS-485/EIA-485 direction control delay.
rs485dly: ReadWrite<u32>,
/// Synchronous mode control register.
syncctrl: ReadWrite<u32, SYNCCTRL::Register>,
/// Transmit Enable Register. Turns off USART transmitter for use with software flow
ter: ReadWrite<u32>,
}
register_bitfields![u32,
IER [
    /// RBR Interrupt Enable. Enables the Receive Data Available interrupt for USART. It
    RBRIE OFFSET(0) NUMBITS(1) [
        /// Disable. Disable the RDA interrupt.
        DisableDisableTheRDAInterrupt = 0,
        /// Enable. Enable the RDA interrupt.
        EnableEnableTheRDAInterrupt = 1
    ],
    /// THRE Interrupt Enable. Enables the THRE interrupt for USART. The status of this
    THREIE OFFSET(1) NUMBITS(1) [
        /// Disable. Disable the THRE interrupt.
        DisableDisableTheTHREInterrupt = 0,
        /// Enable. Enable the THRE interrupt.
        EnableEnableTheTHREInterrupt = 1
    ],
    /// RX Line Interrupt Enable. Enables the USART RX line status interrupts. The statu
    RXIE OFFSET(2) NUMBITS(1) [
        /// Disable. Disable the RX line status interrupts.
        DisableDisableTheRXLineStatusInterrupts = 0,
        /// Enable. Enable the RX line status interrupts.
        EnableEnableTheRXLineStatusInterrupts = 1
    ],
    /// Enables the end of auto-baud interrupt.
    ABEOINTEN OFFSET(8) NUMBITS(1) [
        /// Disable. Disable end of auto-baud Interrupt.
        DisableDisableEndOfAutoBaudInterrupt = 0,
        /// Enable. Enable end of auto-baud Interrupt.
        EnableEnableEndOfAutoBaudInterrupt = 1
    ],
    /// Enables the auto-baud time-out interrupt.
    ABTOINTEN OFFSET(9) NUMBITS(1) [
        /// Disable. Disable auto-baud time-out Interrupt.
        DisableDisableAutoBaudTimeOutInterrupt = 0,
        /// Enable. Enable auto-baud time-out Interrupt.
        EnableEnableAutoBaudTimeOutInterrupt = 1
    ]
],
IIR [
    /// Interrupt status.  Note that IIR[0] is active low. The pending interrupt can be
    INTSTATUS OFFSET(0) NUMBITS(1) [
        /// Interrupt pending. At least one interrupt is pending.
        InterruptPendingAtLeastOneInterruptIsPending = 0,
        /// Not pending. No interrupt is pending.
        NotPendingNoInterruptIsPending = 1
    ],
    /// Interrupt identification.  IER[3:1] identifies an interrupt corresponding to the
    INTID OFFSET(1) NUMBITS(3) [
        /// RLS. Priority 1 (highest). (Highest) Receive Line Status (RLS).
        RLSPriority1HighestHighestReceiveLineStatusRLS = 3,
        /// RDA. Priority 2 - Receive Data Available (RDA).
        RDAPriority2ReceiveDataAvailableRDA = 2,
        /// CTI. Priority 2 - Character Time-out Indicator (CTI).
        CTIPriority2CharacterTimeOutIndicatorCTI = 6,
        /// THRE. Priority 3 - THRE Interrupt.
        THREPriority3THREInterrupt = 1,
        /// Reserved. Priority 4 (lowest) - Reserved.
        ReservedPriority4LowestReserved = 0
    ],
    /// Copies of FCR[0].
    FIFOENABLE OFFSET(6) NUMBITS(2) [],
    /// End of auto-baud interrupt.  True if auto-baud has finished successfully and int
    ABEOINT OFFSET(8) NUMBITS(1) [],
    /// Auto-baud time-out interrupt.  True if auto-baud has timed out and interrupt is
    ABTOINT OFFSET(9) NUMBITS(1) []
],
FCR [
    /// FIFO Enable.
    FIFOEN OFFSET(0) NUMBITS(1) [
        /// Disabled. USART FIFOs are disabled. Must not be used in the application.
        DisabledUSARTFIFOsAreDisabledMustNotBeUsedInTheApplication = 0,
        /// Enabled. Active high enable for both USART Rx and TX FIFOs and FCR[7:1] access.
        ENABLED = 1
    ],
    /// RX FIFO Reset.
    RXFIFORES OFFSET(1) NUMBITS(1) [
        /// No effect. No impact on either of USART FIFOs.
        NoEffectNoImpactOnEitherOfUSARTFIFOs = 0,
        /// Clear. Writing a logic 1 to FCR[1] will clear all bytes in USART Rx FIFO, reset
        CLEAR = 1
    ],
    /// TX FIFO Reset.
    TXFIFORES OFFSET(2) NUMBITS(1) [
        /// No effect. No impact on either of USART FIFOs.
        NoEffectNoImpactOnEitherOfUSARTFIFOs = 0,
        /// Clear. Writing a logic 1 to FCR[2] will clear all bytes in USART TX FIFO, reset
        CLEAR = 1
    ],
    /// DMA Mode Select. When the FIFO enable bit (bit 0 of this register) is set, this
    DMAMODE OFFSET(3) NUMBITS(1) [],
    /// RX Trigger Level. These two bits determine how many receiver USART FIFO characte
    RXTRIGLVL OFFSET(6) NUMBITS(2) [
        /// Level 0. Trigger level 0 (1 character or 0x01).
        Level0TriggerLevel01CharacterOr0x01 = 0,
        /// Level 1. Trigger level 1 (4 characters or 0x04).
        Level1TriggerLevel14CharactersOr0x04 = 1,
        /// Level 2. Trigger level 2 (8 characters or 0x08).
        Level2TriggerLevel28CharactersOr0x08 = 2,
        /// Level 3. Trigger level 3 (14 characters or 0x0E).
        Level3TriggerLevel314CharactersOr0x0E = 3
    ]
],
LCR [
    /// Word Length Select.
    WLS OFFSET(0) NUMBITS(2) [
        /// 5-bit character length.
        _5BitCharacterLength = 0,
        /// 6-bit character length.
        _6BitCharacterLength = 1,
        /// 7-bit character length.
        _7BitCharacterLength = 2,
        /// 8-bit character length.
        _8BitCharacterLength = 3
    ],
    /// Stop Bit Select.
    SBS OFFSET(2) NUMBITS(1) [
        /// 1 stop bit.
        _1StopBit = 0,
        /// 2 stop bits (1.5 if LCR[1:0]=00).
        _2StopBits15IfLCR1000 = 1
    ],
    /// Parity Enable
    PE OFFSET(3) NUMBITS(1) [
        /// Disable parity generation and checking.
        DisableParityGenerationAndChecking = 0,
        /// Enable parity generation and checking.
        EnableParityGenerationAndChecking = 1
    ],
    /// Parity Select.
    PS OFFSET(4) NUMBITS(2) [
        /// Odd parity. Number of 1s in the transmitted character and the attached parity bi
        OddParityNumberOf1sInTheTransmittedCharacterAndTheAttachedParityBitWillBeOdd = 0,
        /// Even Parity. Number of 1s in the transmitted character and the attached parity b
        EvenParityNumberOf1sInTheTransmittedCharacterAndTheAttachedParityBitWillBeEven = 1,
        /// Force HIGH. Forced 1 stick parity.
        ForceHIGHForced1StickParity = 2,
        /// Force LOW. Forced 0 stick parity.
        ForceLOWForced0StickParity = 3
    ],
    /// Break Control.
    BC OFFSET(6) NUMBITS(1) [
        /// Disabled. Disable break transmission.
        DisabledDisableBreakTransmission = 0,
        /// Enabled. Enable break transmission. Output pin USART TXD is forced to logic 0 wh
        ENABLED = 1
    ],
    /// Divisor Latch Access Bit.
    DLAB OFFSET(7) NUMBITS(1) [
        /// Disabled. Disable access to Divisor Latches.
        DisabledDisableAccessToDivisorLatches = 0,
        /// Enabled. Enable access to Divisor Latches.
        EnabledEnableAccessToDivisorLatches = 1
    ]
],
LSR [
    /// Receiver Data Ready. LSR[0] is set when the RBR holds an unread character and is
    RDR OFFSET(0) NUMBITS(1) [
        /// Empty. RBR is empty.
        EmptyRBRIsEmpty = 0,
        /// Data. RBR contains valid data.
        DataRBRContainsValidData = 1
    ],
    /// Overrun Error. The overrun error condition is set as soon as it occurs. A LSR re
    OE OFFSET(1) NUMBITS(1) [
        /// Inactive. Overrun error status is inactive.
        InactiveOverrunErrorStatusIsInactive = 0,
        /// Active. Overrun error status is active.
        ActiveOverrunErrorStatusIsActive = 1
    ],
    /// Parity Error. When the parity bit of a received character is in the wrong state,
    PE OFFSET(2) NUMBITS(1) [
        /// Inactive. Parity error status is inactive.
        InactiveParityErrorStatusIsInactive = 0,
        /// Active. Parity error status is active.
        ActiveParityErrorStatusIsActive = 1
    ],
    /// Framing Error. When the stop bit of a received character is a logic 0, a framing
    FE OFFSET(3) NUMBITS(1) [
        /// Inactive. Framing error status is inactive.
        InactiveFramingErrorStatusIsInactive = 0,
        /// Active. Framing error status is active.
        ActiveFramingErrorStatusIsActive = 1
    ],
    /// Break Interrupt. When RXD1 is held in the spacing state (all zeros) for one full
    BI OFFSET(4) NUMBITS(1) [
        /// Inactive. Break interrupt status is inactive.
        InactiveBreakInterruptStatusIsInactive = 0,
        /// Active. Break interrupt status is active.
        ActiveBreakInterruptStatusIsActive = 1
    ],
    /// Transmitter Holding Register Empty. THRE is set immediately upon detection of an
    THRE OFFSET(5) NUMBITS(1) [
        /// Not empty. THR contains valid data.
        NotEmptyTHRContainsValidData = 0,
        /// Empty. THR is empty.
        EmptyTHRIsEmpty = 1
    ],
    /// Transmitter Empty. TEMT is set when both THR and TSR are empty; TEMT is cleared
    TEMT OFFSET(6) NUMBITS(1) [
        /// Not empty. THR and/or the TSR contains valid data.
        NotEmptyTHRAndOrTheTSRContainsValidData = 0,
        /// Empty. THR and the TSR are empty.
        EmptyTHRAndTheTSRAreEmpty = 1
    ],
    /// Error in RX FIFO. LSR[7] is set when a character with a RX error such as framing
    RXFE OFFSET(7) NUMBITS(1) [
        /// No error. RBR contains no USART RX errors or FCR[0]=0.
        NoErrorRBRContainsNoUSARTRXErrorsOrFCR00 = 0,
        /// Error. USART RBR contains at least one USART RX error.
        ErrorUSARTRBRContainsAtLeastOneUSARTRXError = 1
    ],
    /// Error in transmitted character.   A NACK response is given by the receiver in Sm
    TXERR OFFSET(8) NUMBITS(1) [
        /// No error. No error (normal default condition).
        NoErrorNoErrorNormalDefaultCondition = 0,
        /// NACK. A NACK response is received during Smart card T=0 operation.
        NACKANACKResponseIsReceivedDuringSmartCardT0Operation = 1
    ]
],
ACR [
    /// Start bit. This bit is automatically cleared after auto-baud completion.
    START OFFSET(0) NUMBITS(1) [
        /// Stop. Auto-baud stop (auto-baud is not running).
        StopAutoBaudStopAutoBaudIsNotRunning = 0,
        /// Start. Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is au
        START = 1
    ],
    /// Auto-baud mode select bit.
    MODE OFFSET(1) NUMBITS(1) [
        /// Mode 0.
        Mode0 = 0,
        /// Mode 1.
        Mode1 = 1
    ],
    /// Restart bit.
    AUTORESTART OFFSET(2) NUMBITS(1) [
        /// No restart.
        NoRestart = 0,
        /// Restart. Restart in case of time-out (counter restarts at next USART Rx falling
        RestartRestartInCaseOfTimeOutCounterRestartsAtNextUSARTRxFallingEdge = 1
    ],
    /// End of auto-baud interrupt clear bit (write-only).
    ABEOINTCLR OFFSET(8) NUMBITS(1) [
        /// No effect. Writing a 0 has no impact.
        NoEffectWritingA0HasNoImpact = 0,
        /// Clear. Writing a 1 will clear the corresponding interrupt in the IIR.
        ClearWritingA1WillClearTheCorrespondingInterruptInTheIIR = 1
    ],
    /// Auto-baud time-out interrupt clear bit (write-only).
    ABTOINTCLR OFFSET(9) NUMBITS(1) [
        /// No effect. Writing a 0 has no impact.
        NoEffectWritingA0HasNoImpact = 0,
        /// Clear. Writing a 1 will clear the corresponding interrupt in the IIR.
        ClearWritingA1WillClearTheCorrespondingInterruptInTheIIR = 1
    ]
],
ICR [
    /// IrDA mode enable.
    IRDAEN OFFSET(0) NUMBITS(1) [
        /// Disabled. IrDA mode on USART3 is disabled, USART3 acts as a standard USART.
        DisabledIrDAModeOnUSART3IsDisabledUSART3ActsAsAStandardUSART = 0,
        /// Enabled. IrDA mode on USART3 is enabled.
        EnabledIrDAModeOnUSART3IsEnabled = 1
    ],
    /// Serial input direction.
    IRDAINV OFFSET(1) NUMBITS(1) [
        /// Not inverted. The serial input is not inverted.
        NotInvertedTheSerialInputIsNotInverted = 0,
        /// Inverted. The serial input is inverted. This has no effect on the serial output.
        InvertedTheSerialInputIsInvertedThisHasNoEffectOnTheSerialOutput = 1
    ],
    /// IrDA fixed pulse width mode.
    FIXPULSEEN OFFSET(2) NUMBITS(1) [
        /// Disabled. IrDA fixed pulse width mode disabled.
        DisabledIrDAFixedPulseWidthModeDisabled = 0,
        /// Enabled. IrDA fixed pulse width mode enabled.
        EnabledIrDAFixedPulseWidthModeEnabled = 1
    ],
    /// Configures the pulse when FixPulseEn = 1. See Table 885 for details.
    PULSEDIV OFFSET(3) NUMBITS(3) []
],
FDR [
    /// Baud rate generation pre-scaler divisor value.  If this field is 0, fractional b
    DIVADDVAL OFFSET(0) NUMBITS(4) [],
    /// Baud rate pre-scaler multiplier value.  This field must be greater or equal 1 fo
    MULVAL OFFSET(4) NUMBITS(4) []
],
OSR [
    /// Fractional part of the oversampling ratio, in units of 1/8th of an input clock p
    OSFRAC OFFSET(1) NUMBITS(3) [],
    /// Integer part of the oversampling ratio, minus 1. The reset values equate to the
    OSINT OFFSET(4) NUMBITS(4) [],
    /// In Smart Card mode, these bits act as a more-significant extension of the OSint
    FDINT OFFSET(8) NUMBITS(7) []
],
SCICTRL [
    /// Smart Card Interface Enable.
    SCIEN OFFSET(0) NUMBITS(1) [
        /// Disabled. Smart card interface disabled.
        DisabledSmartCardInterfaceDisabled = 0,
        /// Enabled. synchronous half duplex smart card interface is enabled.
        EnabledSynchronousHalfDuplexSmartCardInterfaceIsEnabled = 1
    ],
    /// NACK response disable. Only applicable in T=0.
    NACKDIS OFFSET(1) NUMBITS(1) [
        /// Enabled. A NACK response is enabled.
        EnabledANACKResponseIsEnabled = 0,
        /// Disabled. A NACK response is inhibited.
        DisabledANACKResponseIsInhibited = 1
    ],
    /// Protocol selection as defined in the ISO7816-3 standard.
    PROTSEL OFFSET(2) NUMBITS(1) [
        /// T = 0
        T0 = 0,
        /// T = 1
        T1 = 1
    ],
    /// Maximum number of retransmissions in case of a negative acknowledge (protocol T=
    TXRETRY OFFSET(5) NUMBITS(3) [],
    /// Extra guard time. No extra guard time (0x0) results in a standard guard time as
    GUARDTIME OFFSET(8) NUMBITS(8) []
],
RS485CTRL [
    /// NMM enable.
    NMMEN OFFSET(0) NUMBITS(1) [
        /// Disabled. RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled.
        DisabledRS485EIA485NormalMultidropModeNMMIsDisabled = 0,
        /// Enabled. RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an
        ENABLED = 1
    ],
    /// Receiver enable.
    RXDIS OFFSET(1) NUMBITS(1) [
        /// Enabled. The receiver is enabled.
        EnabledTheReceiverIsEnabled = 0,
        /// Disabled.The receiver is disabled.
        DisabledTheReceiverIsDisabled = 1
    ],
    /// AAD enable
    AADEN OFFSET(2) NUMBITS(1) [
        /// Disabled. Auto Address Detect (AAD) is disabled.
        DisabledAutoAddressDetectAADIsDisabled = 0,
        /// Enabled. Auto Address Detect (AAD) is enabled.
        EnabledAutoAddressDetectAADIsEnabled = 1
    ],
    /// Direction control for DIR pin.
    DCTRL OFFSET(4) NUMBITS(1) [
        /// Disabled. Disable Auto Direction Control.
        DisabledDisableAutoDirectionControl = 0,
        /// Enabled. Enable Auto Direction Control.
        EnabledEnableAutoDirectionControl = 1
    ],
    /// Direction control pin polarity. This bit reverses the polarity of the direction
    OINV OFFSET(5) NUMBITS(1) [
        /// Low. The direction control pin will be driven to logic 0 when the transmitter ha
        LOW = 0,
        /// High. The direction control pin will be driven to logic 1 when the transmitter h
        HIGH = 1
    ]
],
SYNCCTRL [
    /// Enables synchronous mode.
    SYNC OFFSET(0) NUMBITS(1) [
        /// Disabled.
        Disabled = 0,
        /// Enabled.
        Enabled = 1
    ],
    /// Clock source select.
    CSRC OFFSET(1) NUMBITS(1) [
        /// Slave mode. Synchronous slave mode (SCLK in)
        SlaveModeSynchronousSlaveModeSCLKIn = 0,
        /// Master mode. Synchronous master mode (SCLK out)
        MasterModeSynchronousMasterModeSCLKOut = 1
    ],
    /// Edge sampling.
    FES OFFSET(2) NUMBITS(1) [
        /// Rising. RxD is sampled on the rising edge of SCLK.
        RisingRxDIsSampledOnTheRisingEdgeOfSCLK = 0,
        /// Falling. RxD is sampled on the falling edge of SCLK.
        FallingRxDIsSampledOnTheFallingEdgeOfSCLK = 1
    ],
    /// Transmit synchronization bypass in synchronous slave mode.
    TSBYPASS OFFSET(3) NUMBITS(1) [
        /// Synchronized. The input clock is synchronized prior to being used in clock edge
        SYNCHRONIZED = 0,
        /// Not synchronized. The input clock is not synchronized prior to being used in clo
        NOT_SYNCHRONIZED = 1
    ],
    /// Continuous master clock enable (used only when CSRC is 1)
    CSCEN OFFSET(4) NUMBITS(1) [
        /// On character. SCLK cycles only when characters are being sent on TxD.
        OnCharacterSCLKCyclesOnlyWhenCharactersAreBeingSentOnTxD = 0,
        /// Continuously. SCLK runs continuously (characters can be received on RxD independ
        CONTINUOUSLY = 1
    ],
    /// Start/stop bits
    SSSDIS OFFSET(5) NUMBITS(1) [
        /// Send. Send start and stop bits as in other modes.
        SendSendStartAndStopBitsAsInOtherModes = 0,
        /// Do not send. Do not send start/stop bits.
        DoNotSendDoNotSendStartStopBits = 1
    ],
    /// Continuous clock clear
    CCCLR OFFSET(6) NUMBITS(1) [
        /// Software. CSCEN is under software control.
        SoftwareCSCENIsUnderSoftwareControl = 0,
        /// Hardware. Hardware clears CSCEN after each character is received.
        HardwareHardwareClearsCSCENAfterEachCharacterIsReceived = 1
    ]
]
];
const USART0_BASE: StaticRef<UsartRegisters> =
    unsafe { StaticRef::new(0x40081000 as *const UsartRegisters) };


const USART2_BASE: StaticRef<UsartRegisters> =
    unsafe { StaticRef::new(0x400C1000 as *const UsartRegisters) };


const USART3_BASE: StaticRef<UsartRegisters> =
    unsafe { StaticRef::new(0x400C2000 as *const UsartRegisters) };
