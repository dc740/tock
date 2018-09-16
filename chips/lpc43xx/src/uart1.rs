
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// UART1
#[repr(C)]
struct Uart1Registers {
/// Receiver Buffer Register. Contains the next received character to be read. (DLAB
rbr: ReadOnly<u32>,
/// Divisor Latch MSB. Most significant byte of the baud rate divisor value. The ful
dlm: ReadWrite<u32>,
/// Interrupt ID Register. Identifies which interrupt(s) are pending.
iir: ReadOnly<u32, IIR::Register>,
/// Line Control Register. Contains controls for frame formatting and break generati
lcr: ReadWrite<u32, LCR::Register>,
/// Modem Control Register. Contains controls for flow control handshaking and loopb
mcr: ReadWrite<u32, MCR::Register>,
/// Line Status Register. Contains flags for transmit and receive status, including
lsr: ReadOnly<u32, LSR::Register>,
/// Modem Status Register. Contains handshake signal status flags.
msr: ReadOnly<u32, MSR::Register>,
/// Scratch Pad Register. 8-bit temporary storage for software.
scr: ReadWrite<u32>,
/// Auto-baud Control Register. Contains controls for the auto-baud feature.
acr: ReadWrite<u32, ACR::Register>,
_reserved0: [u8; 4],
/// Fractional Divider Register. Generates a clock input for the baud rate divider.
fdr: ReadWrite<u32, FDR::Register>,
_reserved1: [u8; 32],
/// RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485
rs485ctrl: ReadWrite<u32, RS485CTRL::Register>,
/// RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-48
rs485adrmatch: ReadWrite<u32>,
/// RS-485/EIA-485 direction control delay.
rs485dly: ReadWrite<u32>,
_reserved2: [u8; 4],
/// Transmit Enable Register. Turns off UART transmitter for use with software flow
ter: ReadWrite<u32>,
}
register_bitfields![u32,
IER [
    /// RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART1. It
    RBRIE OFFSET(0) NUMBITS(1) [
        /// Disable. Disable the RDA interrupts.
        DisableDisableTheRDAInterrupts = 0,
        /// Enable. Enable the RDA interrupts.
        EnableEnableTheRDAInterrupts = 1
    ],
    /// THRE Interrupt Enable. Enables the THRE interrupt for UART1. The status of this
    THREIE OFFSET(1) NUMBITS(1) [
        /// Disable. Disable the THRE interrupts.
        DisableDisableTheTHREInterrupts = 0,
        /// Enable. Enable the THRE interrupts.
        EnableEnableTheTHREInterrupts = 1
    ],
    /// RX Line Interrupt Enable. Enables the UART1 RX line status interrupts. The statu
    RXIE OFFSET(2) NUMBITS(1) [
        /// Disable. Disable the RX line status interrupts.
        DisableDisableTheRXLineStatusInterrupts = 0,
        /// Enable. Enable the RX line status interrupts.
        EnableEnableTheRXLineStatusInterrupts = 1
    ],
    /// Modem Status Interrupt Enable. Enables the modem interrupt. The status of this i
    MSIE OFFSET(3) NUMBITS(1) [
        /// Disable. Disable the modem interrupt.
        DisableDisableTheModemInterrupt = 0,
        /// Enable. Enable the modem interrupt.
        EnableEnableTheModemInterrupt = 1
    ],
    /// CTS Interrupt Enable. If auto-cts mode is enabled this bit enables/disables the
    CTSIE OFFSET(7) NUMBITS(1) [
        /// Disable. Disable the CTS interrupt.
        DisableDisableTheCTSInterrupt = 0,
        /// Enable. Enable the CTS interrupt.
        EnableEnableTheCTSInterrupt = 1
    ],
    /// Enables the end of auto-baud interrupt.
    ABEOIE OFFSET(8) NUMBITS(1) [
        /// Disable. Disable end of auto-baud Interrupt.
        DisableDisableEndOfAutoBaudInterrupt = 0,
        /// Enable. Enable end of auto-baud Interrupt.
        EnableEnableEndOfAutoBaudInterrupt = 1
    ],
    /// Enables the auto-baud time-out interrupt.
    ABTOIE OFFSET(9) NUMBITS(1) [
        /// Disable. Disable auto-baud time-out Interrupt.
        DisableDisableAutoBaudTimeOutInterrupt = 0,
        /// Enable. Enable auto-baud time-out Interrupt.
        EnableEnableAutoBaudTimeOutInterrupt = 1
    ]
],
IIR [
    /// Interrupt status. Note that IIR[0] is active low. The pending interrupt can be d
    INTSTATUS OFFSET(0) NUMBITS(1) [
        /// Interrupt pending. At least one interrupt is pending.
        InterruptPendingAtLeastOneInterruptIsPending = 0,
        /// Not pending. No interrupt is pending.
        NotPendingNoInterruptIsPending = 1
    ],
    /// Interrupt identification. IER[3:1] identifies an interrupt corresponding to the
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
    /// End of auto-baud interrupt. True if auto-baud has finished successfully and inte
    ABEOINT OFFSET(8) NUMBITS(1) [],
    /// Auto-baud time-out interrupt. True if auto-baud has timed out and interrupt is e
    ABTOINT OFFSET(9) NUMBITS(1) []
],
FCR [
    /// FIFO enable.
    FIFOEN OFFSET(0) NUMBITS(1) [
        /// Disabled. Must not be used in the application.
        DisabledMustNotBeUsedInTheApplication = 0,
        /// Enabled. Active high enable for both UART1 Rx and TX FIFOs and FCR[7:1] access.
        ENABLED = 1
    ],
    /// RX FIFO Reset.
    RXFIFORES OFFSET(1) NUMBITS(1) [
        /// No effect. No impact on either of UART1 FIFOs.
        NoEffectNoImpactOnEitherOfUART1FIFOs = 0,
        /// Clear. Writing a logic 1 to FCR[1] will clear all bytes in UART1 Rx FIFO, reset
        CLEAR = 1
    ],
    /// TX FIFO Reset.
    TXFIFORES OFFSET(2) NUMBITS(1) [
        /// No effect. No impact on either of UART1 FIFOs.
        NoEffectNoImpactOnEitherOfUART1FIFOs = 0,
        /// Clear. Writing a logic 1 to FCR[2] will clear all bytes in UART1 TX FIFO, reset
        CLEAR = 1
    ],
    /// DMA Mode Select. When the FIFO enable bit (bit 0 of this register) is set, this
    DMAMODE OFFSET(3) NUMBITS(1) [],
    /// RX Trigger Level. These two bits determine how many receiver UART1 FIFO characte
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
        /// 2 stop bits. (1.5 if LCR[1:0]=00).
        _2StopBits15IfLCR1000 = 1
    ],
    /// Parity Enable.
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
        /// Enabled. Enable break transmission. Output pin UART1 TXD is forced to logic 0 wh
        ENABLED = 1
    ],
    /// Divisor Latch Access Bit (DLAB)
    DLAB OFFSET(7) NUMBITS(1) [
        /// Disabled. Disable access to Divisor Latches.
        DisabledDisableAccessToDivisorLatches = 0,
        /// Enabled. Enable access to Divisor Latches.
        EnabledEnableAccessToDivisorLatches = 1
    ]
],
MCR [
    /// DTR Control.  Source for modem output pin, DTR. This bit reads as 0 when modem l
    DTRCTRL OFFSET(0) NUMBITS(1) [],
    /// RTS Control.  Source for modem output pin RTS. This bit reads as 0 when modem lo
    RTSCTRL OFFSET(1) NUMBITS(1) [],
    /// Loopback Mode Select.  The modem loopback mode provides a mechanism to perform d
    LMS OFFSET(4) NUMBITS(1) [
        /// Disabled. Disable modem loopback mode.
        DisabledDisableModemLoopbackMode = 0,
        /// Enabled. Enable modem loopback mode.
        EnabledEnableModemLoopbackMode = 1
    ],
    /// RTS enable.
    RTSEN OFFSET(6) NUMBITS(1) [
        /// Disabled. Disable auto-rts flow control.
        DisabledDisableAutoRtsFlowControl = 0,
        /// Enabled. Enable auto-rts flow control.
        EnabledEnableAutoRtsFlowControl = 1
    ],
    /// CTS enable.
    CTSEN OFFSET(7) NUMBITS(1) [
        /// Disabled. Disable auto-cts flow control.
        DisabledDisableAutoCtsFlowControl = 0,
        /// Enabled. Enable auto-cts flow control.
        EnabledEnableAutoCtsFlowControl = 1
    ]
],
LSR [
    /// Receiver Data Ready.  LSR[0] is set when the RBR holds an unread character and i
    RDR OFFSET(0) NUMBITS(1) [
        /// Empty. The UART1 receiver FIFO is empty.
        EmptyTheUART1ReceiverFIFOIsEmpty = 0,
        /// Data. The UART1 receiver FIFO is not empty.
        DataTheUART1ReceiverFIFOIsNotEmpty = 1
    ],
    /// Overrun Error. The overrun error condition is set as soon as it occurs. An LSR r
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
    /// Break Interrupt.  When RXD1 is held in the spacing state (all zeroes) for one fu
    BI OFFSET(4) NUMBITS(1) [
        /// Break interrupt status is inactive.
        BreakInterruptStatusIsInactive = 0,
        /// Break interrupt status is active.
        BreakInterruptStatusIsActive = 1
    ],
    /// Transmitter Holding Register Empty.  THRE is set immediately upon detection of a
    THRE OFFSET(5) NUMBITS(1) [
        /// Not empty. THR contains valid data.
        NotEmptyTHRContainsValidData = 0,
        /// Empty. THR is empty.
        EmptyTHRIsEmpty = 1
    ],
    /// Transmitter Empty.  TEMT is set when both THR and TSR are empty; TEMT is cleared
    TEMT OFFSET(6) NUMBITS(1) [
        /// Not empty. THR and/or the TSR contains valid data.
        NotEmptyTHRAndOrTheTSRContainsValidData = 0,
        /// Empty. THR and the TSR are empty.
        EmptyTHRAndTheTSRAreEmpty = 1
    ],
    /// Error in RX FIFO. LSR[7] is set when a character with a RX error such as framing
    RXFE OFFSET(7) NUMBITS(1) [
        /// No error. RBR contains no UART1 RX errors or FCR[0]=0.
        NoErrorRBRContainsNoUART1RXErrorsOrFCR00 = 0,
        /// Error. UART1 RBR contains at least one UART1 RX error.
        ErrorUART1RBRContainsAtLeastOneUART1RXError = 1
    ]
],
MSR [
    /// Delta CTS. Set upon state change of input CTS. Cleared on an MSR read.
    DCTS OFFSET(0) NUMBITS(1) [
        /// No change. No change detected on modem input, CTS.
        NoChangeNoChangeDetectedOnModemInputCTS = 0,
        /// State change. State change detected on modem input, CTS.
        StateChangeStateChangeDetectedOnModemInputCTS = 1
    ],
    /// Delta DSR. Set upon state change of input DSR. Cleared on an MSR read.
    DDSR OFFSET(1) NUMBITS(1) [
        /// No change. No change detected on modem input, DSR.
        NoChangeNoChangeDetectedOnModemInputDSR = 0,
        /// State change. State change detected on modem input, DSR.
        StateChangeStateChangeDetectedOnModemInputDSR = 1
    ],
    /// Trailing Edge RI. Set upon low to high transition of input RI. Cleared on an MSR
    TERI OFFSET(2) NUMBITS(1) [
        /// No change. No change detected on modem input, RI.
        NoChangeNoChangeDetectedOnModemInputRI = 0,
        /// Rising. Low-to-high transition detected on RI.
        RisingLowToHighTransitionDetectedOnRI = 1
    ],
    /// Delta DCD. Set upon state change of input DCD. Cleared on an MSR read.
    DDCD OFFSET(3) NUMBITS(1) [
        /// No change. No change detected on modem input, DCD.
        NoChangeNoChangeDetectedOnModemInputDCD = 0,
        /// State change. State change detected on modem input, DCD.
        StateChangeStateChangeDetectedOnModemInputDCD = 1
    ],
    /// Clear To Send State. Complement of input signal CTS. This bit is connected to MC
    CTS OFFSET(4) NUMBITS(1) [],
    /// Data Set Ready State. Complement of input signal DSR. This bit is connected to M
    DSR OFFSET(5) NUMBITS(1) [],
    /// Ring Indicator State. Complement of input RI. This bit is connected to MCR[2] in
    RI OFFSET(6) NUMBITS(1) [],
    /// Data Carrier Detect State. Complement of input DCD. This bit is connected to MCR
    DCD OFFSET(7) NUMBITS(1) []
],
ACR [
    /// Auto-baud start bit. This bit is automatically cleared after auto-baud completio
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
    /// Auto-baud restart bit.
    AUTORESTART OFFSET(2) NUMBITS(1) [
        /// No restart
        NoRestart = 0,
        /// Restart. Restart in case of time-out (counter restarts at next UART1 Rx falling
        RestartRestartInCaseOfTimeOutCounterRestartsAtNextUART1RxFallingEdge = 1
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
FDR [
    /// Baud-rate generation pre-scaler divisor value. If this field is 0, fractional ba
    DIVADDVAL OFFSET(0) NUMBITS(4) [],
    /// Baud-rate pre-scaler multiplier value. This field must be greater or equal 1 for
    MULVAL OFFSET(4) NUMBITS(4) []
],
RS485CTRL [
    /// Multidrop mode select.
    NMMEN OFFSET(0) NUMBITS(1) [
        /// Disabled. RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled.
        DisabledRS485EIA485NormalMultidropModeNMMIsDisabled = 0,
        /// Enabled. RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an
        ENABLED = 1
    ],
    /// Receive enable.
    RXDIS OFFSET(1) NUMBITS(1) [
        /// Enabled. The receiver is enabled.
        EnabledTheReceiverIsEnabled = 0,
        /// Disabled.The receiver is disabled.
        DisabledTheReceiverIsDisabled = 1
    ],
    /// Auto Address Detect enable.
    AADEN OFFSET(2) NUMBITS(1) [
        /// Disabled. Auto Address Detect (AAD) is disabled.
        DisabledAutoAddressDetectAADIsDisabled = 0,
        /// Enabled. Auto Address Detect (AAD) is enabled.
        EnabledAutoAddressDetectAADIsEnabled = 1
    ],
    /// Direction control.
    SEL OFFSET(3) NUMBITS(1) [
        /// RTS. If direction control is enabled (bit DCTRL = 1), pin RTS is used for direct
        RTSIfDirectionControlIsEnabledBitDCTRL1PinRTSIsUsedForDirectionControl = 0,
        /// DTR. If direction control is enabled (bit DCTRL = 1), pin DTR is used for direct
        DTRIfDirectionControlIsEnabledBitDCTRL1PinDTRIsUsedForDirectionControl = 1
    ],
    /// Direction control enable.
    DCTRL OFFSET(4) NUMBITS(1) [
        /// Disabled. Disable Auto Direction Control.
        DisabledDisableAutoDirectionControl = 0,
        /// Enabled. Enable Auto Direction Control.
        EnabledEnableAutoDirectionControl = 1
    ],
    /// Polarity. This bit reverses the polarity of the direction control signal on the
    OINV OFFSET(5) NUMBITS(1) [
        /// Low. The direction control pin will be driven to logic 0 when the transmitter ha
        LOW = 0,
        /// High. The direction control pin will be driven to logic 1 when the transmitter h
        HIGH = 1
    ]
]
];
const UART1_BASE: StaticRef<Uart1Registers> =
    unsafe { StaticRef::new(0x40082000 as *const Uart1Registers) };
