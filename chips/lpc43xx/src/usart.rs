use kernel::common::cells::{MapCell, OptionalCell};
use kernel::common::registers::{register_bitfields, ReadOnly, ReadWrite};
use kernel::common::StaticRef;
use kernel::ReturnCode;
use crate::{ccu1, nvic, scu};


/// USART0_2_3
#[repr(C)]
struct UsartRegisters {
    /// Receiver Buffer Register. Contains the next received character to be read (DLAB
    /// Also:
    ///  * Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider (DLAB = 1).
    ///  * Transmit Holding Register. The next character to be transmitted is written here (DLAB = 0).
    rbr: ReadWrite<u32>,
    /// Divisor Latch MSB (DLM). Most significant byte of the baud rate divisor value. The ful
    /// Also Interrupt Enable Register IER. Contains individual interrupt enable bits for the 7 potential UART interrupts (DLAB = 0).
    ier: ReadWrite<u32, IER::Register>,
    /// Interrupt ID Register. Identifies which interrupt(s) are pending. (ReadOnly)
    /// Also FIFO Control Register. Controls UART FIFO usage and modes. So we changed from ReadOnly to ReadWrite
    fcr: ReadWrite<u32, FCR::Register>,
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
    _reserved2: [u8; 16], //TER1, etc
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
    ter: ReadWrite<u32>, //AKA TER2
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
        RX_CLEAR = 1
    ],
    /// TX FIFO Reset.
    TXFIFORES OFFSET(2) NUMBITS(1) [
        /// No effect. No impact on either of USART FIFOs.
        NoEffectNoImpactOnEitherOfUSARTFIFOs = 0,
        /// Clear. Writing a logic 1 to FCR[2] will clear all bytes in USART TX FIFO, reset
        TX_CLEAR = 1
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
//const USART0_BASE: StaticRef<UsartRegisters> =
//    unsafe { StaticRef::new(0x40081000 as *const UsartRegisters) };

const USART2_BASE: StaticRef<UsartRegisters> =
    unsafe { StaticRef::new(0x400C1000 as *const UsartRegisters) };

//const USART3_BASE: StaticRef<UsartRegisters> =
//    unsafe { StaticRef::new(0x400C2000 as *const UsartRegisters) };

/// Stores an ongoing TX transaction. Similar to cc26x2
struct Transaction {
    /// The buffer containing the bytes to transmit as it should be returned to
    /// the client
    buffer: &'static mut [u8],
    /// The total amount to transmit
    length: usize,
    /// The index of the byte currently being sent
    index: usize,
}

pub struct Usart<'a> {
    registers: StaticRef<UsartRegisters>,
    tx_client: OptionalCell<&'a dyn kernel::hil::uart::TransmitClient>,
    rx_client: OptionalCell<&'a dyn kernel::hil::uart::ReceiveClient>,
    tx: MapCell<Transaction>,
    rx: MapCell<Transaction>,
}

impl<'a> Usart<'a> {
    const fn new(base_addr: StaticRef<UsartRegisters>) -> Usart<'a> {
        Usart {
            registers: base_addr,
            tx_client: OptionalCell::empty(),
            rx_client: OptionalCell::empty(),

            tx: MapCell::empty(),
            rx: MapCell::empty(),
        }
    }
    fn disable_tx_interrupts(&self) {
        // disable interrupts
        self.registers.ier.modify(
            IER::THREIE::DisableDisableTheTHREInterrupt
            );

    }

    fn enable_tx_interrupts(&self) {
        // set only interrupts used
        self.registers.ier.modify(
            IER::THREIE::EnableEnableTheTHREInterrupt
            );
    }
    fn enable_rx_interrupts(&self) {
        // set only interrupts used
        self.registers.ier.modify(
            IER::RBRIE::EnableEnableTheRDAInterrupt 
            + IER::RXIE::EnableEnableTheRXLineStatusInterrupts
            );
    }
    #[no_mangle]
    #[inline(never)]
    pub fn handle_interrupt(&self) {
        // Hardware RX FIFO is not empty
        while self.is_rx_fifo_not_empty() {
            debug!("rx fifo was not empty... I don't know why");
            // buffer read request was made
            if self.rx.is_some() {
                self.rx.take().map(|mut rx| {
                    // read in a byte
                    if rx.index < rx.length {
                        let byte = self.get_byte();
                        rx.buffer[rx.index] = byte;
                        rx.index += 1;
                    }

                    if rx.index == rx.length {
                        self.rx_client.map(move |client| {
                            client.received_buffer(
                                rx.buffer,
                                rx.index,
                                ReturnCode::SUCCESS,
                                kernel::hil::uart::Error::None,
                            );
                        });
                    } else {
                        self.rx.put(rx);
                    }
                });
            }
            // no current read request
            else {
                // read bytes into the void to avoid hardware RX buffer overflow
                self.get_byte();
            }
        }
        // You NEED to read IIR, or the interrupt DOES NOT get cleared
        let iir = self.registers.fcr.get();
        if iir & 3 == 2 { // Pending interrupt with THREIE flag
            self.tx.take().map(|mut tx| {
                // send out the buffer if available, IRQ when TX FIFO empty will bring us back
                if self.is_tx_fifo_available() && tx.index < tx.length {
                        self.put_byte(tx.buffer[tx.index]);
                        tx.index += 1;
                }
                // request is done
                if tx.index >= tx.length {
                    self.tx_client.map(move |client| {
                        client.transmitted_buffer(tx.buffer, tx.length, ReturnCode::SUCCESS);
                    });
                } else {
                    // keep TX buffer as there is more left in request
                    self.tx.put(tx);
                }
            });
        }
    }
    pub fn disable_tx(&self) {
        self.registers.ter.set(0);
    }
    pub fn enable_tx(&self) {
        self.registers.ter.set(1);
    }
    pub fn init(&self) {
        // This assumes you already called ccu1.uart2_init()
        /* Enable FIFOs by default, reset them */
        self.registers
            .fcr
            .write(FCR::FIFOEN::ENABLED
             + FCR::RXFIFORES::RX_CLEAR
             + FCR::TXFIFORES::TX_CLEAR
             + FCR::RXTRIGLVL::Level3TriggerLevel314CharactersOr0x0E);
        /* Disable Tx */
        self.disable_tx();
        /* Disable interrupts */
        self.registers.ier.set(0);
        /* Set LCR to default state */
        self.registers.lcr.set(0);
        /* Set ACR to default state */
        self.registers.acr.set(0);
        /* Set RS485 control to default state */
        self.registers.rs485ctrl.set(0);
        /* Set RS485 delay timer to default state */
        self.registers.rs485dly.set(0);
        /* Set RS485 addr match to default state */
        self.registers.rs485adrmatch.set(0);
        /* No need to clear MCR. Only in UART1, and it's NOT supported in the EDU-CIAA*/
        /* Default 8N1, with DLAB disabled */
        self.registers.lcr.write(
            LCR::WLS::_8BitCharacterLength
                + LCR::SBS::_1StopBit
                + LCR::PE::DisableParityGenerationAndChecking,
        );
        /* Disable fractional divider */
        self.registers.fdr.set(0x10)
    }
    /* I just wrote this to avoid making public some fields */
    pub fn init_lcr(&self) {
        self.registers.lcr.modify(
            LCR::WLS::_8BitCharacterLength
                + LCR::SBS::_1StopBit
                + LCR::PE::DisableParityGenerationAndChecking,
        );
    }
    
    pub fn is_tx_fifo_available(&self) -> bool {
        self.registers.lsr.is_set(LSR::THRE)
    }
    
    pub fn is_rx_fifo_not_empty(&self) -> bool {
        self.registers.lsr.is_set(LSR::RDR)
    }
    /* Determines and sets best dividers to get a target baud rate */
    pub fn set_baud_fdr(&self, baud: u32) -> u32 {
        let (mut sdiv, mut sm, mut sd): (u32, u32, u32) = (0, 1, 0);
        let pclk: u32;
        let mut odiff: u32 = 0xFFFFFFFF; /* old best diff */
        /* Get base clock for the corresponding UART */
        pclk = ccu1::get_uart2_rate();
        /* Loop through all possible fractional divider values */
        for m in 1..16 {
            if odiff == 0 {
                break;
            }
            for d in 0..m {
                let (mut diff, mut div): (u32, u32);
                let dval: u64 = ((u64::from(pclk) << 28) * m) / (u64::from(baud) * (m + d));
                /* Lower 32-bit of dval has diff */
                diff = dval as u32;
                /* Upper 32-bit of dval has div */
                div = (dval >> 32) as u32;
                /* Closer to next div */
                if diff >= 0x80000000 {
                    diff = !diff + 1;
                    div += 1;
                }
                /* Check if new value is worse than old or out of range */
                if odiff < diff || div == 0 || (div >> 16) != 0 || (div < 3 && d != 0) {
                    continue;
                }
                /* Store the new better values */
                sdiv = div;
                sd = d as u32;
                sm = m as u32;
                odiff = diff;

                /* On perfect match, break loop */
                if diff == 0 {
                    break;
                }
            }
        }
        /* Return 0 if a vaild divisor is not possible */
        if sdiv == 0 {
            return 0;
        }
        /* Update UART registers */
        self.registers
            .lcr
            .modify(LCR::DLAB::EnabledEnableAccessToDivisorLatches);
        self.registers.rbr.set(sdiv & 0xff);
        self.registers.ier.set((sdiv >> 8) & 0xff);
        self.registers
            .lcr
            .modify(LCR::DLAB::DisabledDisableAccessToDivisorLatches);
        /* Set best fractional divider */
        self.registers
            .fdr
            .write(FDR::MULVAL.val(sm) + FDR::DIVADDVAL.val(sd));
        /* Return actual baud rate */
        let result = (pclk >> 4) * sm / (sdiv * (sm + sd));
        result //115210
    }
    pub fn put_byte(&self, some_byte: u8) {
        self.registers.rbr.set(u32::from(some_byte))
    }
    pub fn get_byte(&self) -> u8 {
        (self.registers.rbr.get() & 0xff) as u8
    }
}

// ################# TOCKOS required code below ##################

impl<'a> kernel::hil::uart::Configure for Usart<'a> {
    #[inline(never)]
    #[no_mangle]
    fn configure(&self, params: kernel::hil::uart::Parameters) -> ReturnCode {
        // These could easily be implemented, but are currently ignored, so
        // throw an error.
        if params.stop_bits != kernel::hil::uart::StopBits::One {
            return ReturnCode::ENOSUPPORT;
        }
        if params.parity != kernel::hil::uart::Parity::None {
            return ReturnCode::ENOSUPPORT;
        }
        if params.hw_flow_control != false {
            return ReturnCode::ENOSUPPORT;
        }
        //TODO: implement other UART in scu and ccu1
        scu::init_uart2_pinfunc();
        ccu1::uart2_init();
        self.init();
        self.set_baud_fdr(params.baud_rate);
        self.init_lcr();
        self.enable_tx();
        
        // Enable interrupts
        self.enable_rx_interrupts();
        unsafe {
            let n = cortexm4::nvic::Nvic::new(nvic::USART2);
            n.clear_pending();
            n.enable();
        }
        ReturnCode::SUCCESS
    }
}

impl<'a> kernel::hil::uart::Transmit<'a> for Usart<'a> {
    /// Set the transmit client, which will be called when transmissions
    /// complete.
    fn set_transmit_client(&self, client: &'a dyn kernel::hil::uart::TransmitClient) {
        self.tx_client.set(client);
    }
    /// Transmit a buffer of data. On completion, `transmitted_buffer`
    /// in the `TransmitClient` will be called.  If the `ReturnCode`
    /// of `transmit`'s return tuple is SUCCESS, the `Option` will be
    /// `None` and the struct will issue a `transmitted_buffer`
    /// callback in the future. If the value of the `ReturnCode` is
    /// not SUCCESS, then the `tx_buffer` argument is returned in the
    /// `Option`. Other valid `ReturnCode` values are:
    ///  - EOFF: The underlying hardware is not available, perhaps because
    ///          because it has not been initialized or in the case of a shared
    ///          hardware USART controller because it is set up for SPI.
    ///  - EBUSY: the UART is already transmitting and has not made a
    ///           transmission callback yet.
    ///  - ESIZE : `tx_len` is larger than the passed slice.
    ///  - FAIL: some other error.
    ///
    /// Each byte in `tx_buffer` is a UART transfer word of 8 or fewer
    /// bits.  The word width is determined by the UART configuration,
    /// truncating any more significant bits. E.g., 0x18f transmitted in
    /// 8N1 will be sent as 0x8f and in 7N1 will be sent as 0x0f. Clients
    /// that need to transfer 9-bit words should use `transmit_word`.
    ///
    /// Calling `transmit_buffer` while there is an outstanding
    /// `transmit_buffer` or `transmit_word` operation will return EBUSY.
    fn transmit_buffer(
        &self,
        tx_buffer: &'static mut [u8],
        len: usize,
    ) -> (ReturnCode, Option<&'static mut [u8]>) {
        self.disable_tx_interrupts();
        let result;
        // if there is a weird input, don't try to do any transfers
        if len == 0 || len > tx_buffer.len() {
            result = (ReturnCode::ESIZE, Some(tx_buffer));
        } else if self.tx.is_some() {
            result = (ReturnCode::EBUSY, Some(tx_buffer));
        } else {
            // we will send one byte, causing EOT interrupt
            if self.is_tx_fifo_available() {
                self.put_byte(tx_buffer[0]);
            } else {
                //I don't like returns in the middle of the code...
                // but we need to tell we are busy if the fifo is not available
                return (ReturnCode::EBUSY, Some(tx_buffer));
            }
            
            // Transaction will be continued in interrupt bottom half
            self.tx.put(Transaction {
                buffer: tx_buffer,
                length: len,
                index: 1,
            });
            result = (ReturnCode::SUCCESS, None);
        }
        self.enable_tx_interrupts();
        result
    }
    /// Transmit a single word of data asynchronously. The word length is
    /// determined by the UART configuration: it can be 6, 7, 8, or 9 bits long.
    /// If the `ReturnCode` is SUCCESS, on completion,
    /// `transmitted_word` will be called on the `TransmitClient`.
    /// Other valid `ReturnCode` values are:
    ///  - EOFF: The underlying hardware is not available, perhaps because
    ///          because it has not been initialized or in the case of a shared
    ///          hardware USART controller because it is set up for SPI.
    ///  - EBUSY: the UART is already transmitting and has not made a
    ///           transmission callback yet.
    ///  - FAIL: not supported, or some other error.
    /// If the `ReturnCode` is not SUCCESS, no callback will be made.
    /// Calling `transmit_word` while there is an outstanding
    /// `transmit_buffer` or `transmit_word` operation will return
    /// EBUSY.
    fn transmit_word(&self, _word: u32) -> ReturnCode {
        ReturnCode::FAIL
    }

    /// Abort an outstanding call to `transmit_word` or `transmit_buffer`.
    /// The return code indicates whether the call has fully terminated or
    /// there will be a callback. Cancelled calls to `transmit_buffer` MUST
    /// always make a callback, to return the passed buffer back to the caller.
    ///
    /// If abort_transmit returns SUCCESS, there will be no future
    /// callback and the client may retransmit immediately. If
    /// abort_transmit returns any other `ReturnCode` there will be a
    /// callback. This means that if there is no outstanding call to
    /// `transmit_word` or `transmit_buffer` then a call to
    /// `abort_transmit` returns SUCCESS. If there was a `transmit`
    /// outstanding and is cancelled successfully then `EBUSY` will
    /// be returned and a there will be a callback with a `ReturnCode`
    /// of `ECANCEL`.  If there was a reception outstanding, which is
    /// not cancelled successfully, then `FAIL` will be returned and
    /// there will be a later callback.
    ///
    /// Returns SUCCESS or
    ///  - FAIL if the outstanding call to either transmit operation could
    ///    not be synchronously cancelled. A callback will be made on the
    ///    client indicating whether the call was successfully cancelled.
    fn transmit_abort(&self) -> ReturnCode {
        //TODO: not implemented
        ReturnCode::FAIL
    }
}
impl<'a> kernel::hil::uart::Receive<'a> for Usart<'a> {
    fn set_receive_client(&self, client: &'a dyn kernel::hil::uart::ReceiveClient) {
        self.rx_client.set(client);
    }

    fn receive_buffer(
        &self,
        rx_buffer: &'static mut [u8],
        len: usize,
    ) -> (ReturnCode, Option<&'static mut [u8]>) {
        if len == 0 || len > rx_buffer.len() {
            (ReturnCode::ESIZE, Some(rx_buffer))
        } else if self.rx.is_some() {
            (ReturnCode::EBUSY, Some(rx_buffer))
        } else {
            self.rx.put(Transaction {
                buffer: rx_buffer,
                length: len,
                index: 0,
            });

            (ReturnCode::SUCCESS, None)
        }
    }
    fn receive_word(&self) -> ReturnCode {
        ReturnCode::FAIL
    }

    fn receive_abort(&self) -> ReturnCode {
        //self.abort_rx(ReturnCode::ECANCEL, hil::uart::Error::Aborted);
        //ReturnCode::EBUSY
        ReturnCode::FAIL
    }
}
impl<'a> kernel::hil::uart::Uart<'a> for Usart<'a> {}
// USART hardware peripherals on lpc4337
pub static mut USART2: Usart = Usart::new(USART2_BASE);
