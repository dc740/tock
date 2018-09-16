
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// USB1 Host/Device controller
#[repr(C)]
struct Usb1Registers {
_reserved0: [u8; 256],
/// Capability register length
caplength: ReadOnly<u32, CAPLENGTH::Register>,
/// Host controller structural parameters
hcsparams: ReadOnly<u32, HCSPARAMS::Register>,
/// Host controller capability parameters
hccparams: ReadOnly<u32, HCCPARAMS::Register>,
_reserved1: [u8; 20],
/// Device interface version number
dciversion: ReadOnly<u32>,
_reserved2: [u8; 28],
/// USB command (device mode)
usbcmd_d: ReadWrite<u32, USBCMD_D::Register>,
/// USB status (device mode)
usbsts_d: ReadWrite<u32, USBSTS_D::Register>,
/// USB interrupt enable (device mode)
usbintr_d: ReadWrite<u32, USBINTR_D::Register>,
/// USB frame index (device mode)
frindex_d: ReadOnly<u32, FRINDEX_D::Register>,
_reserved3: [u8; 4],
/// USB device address
deviceaddr: ReadWrite<u32, DEVICEADDR::Register>,
/// Address of endpoint list in memory (device mode)
endpointlistaddr: ReadWrite<u32>,
/// Asynchronous buffer status for embedded TT (host mode)
ttctrl: ReadWrite<u32>,
/// Programmable burst size
burstsize: ReadWrite<u32, BURSTSIZE::Register>,
/// Host transmit pre-buffer packet tuning (host mode)
txfilltuning: ReadWrite<u32, TXFILLTUNING::Register>,
_reserved4: [u8; 8],
/// ULPI viewport
ulpiviewport: ReadWrite<u32, ULPIVIEWPORT::Register>,
/// Length of virtual frame
binterval: ReadWrite<u32>,
/// Endpoint NAK (device mode)
endptnak: ReadWrite<u32, ENDPTNAK::Register>,
/// Endpoint NAK Enable (device mode)
endptnaken: ReadWrite<u32, ENDPTNAKEN::Register>,
_reserved5: [u8; 4],
/// Port 1 status/control (device mode)
portsc1_d: ReadWrite<u32, PORTSC1_D::Register>,
_reserved6: [u8; 32],
/// USB mode (device mode)
usbmode_d: ReadWrite<u32, USBMODE_D::Register>,
/// Endpoint setup status
endptsetupstat: ReadWrite<u32, ENDPTSETUPSTAT::Register>,
/// Endpoint initialization
endptprime: ReadWrite<u32, ENDPTPRIME::Register>,
/// Endpoint de-initialization
endptflush: ReadWrite<u32, ENDPTFLUSH::Register>,
/// Endpoint status
endptstat: ReadOnly<u32, ENDPTSTAT::Register>,
/// Endpoint complete
endptcomplete: ReadWrite<u32, ENDPTCOMPLETE::Register>,
/// Endpoint control 0
endptctrl0: ReadWrite<u32, ENDPTCTRL0::Register>,
/// Endpoint control
endptctrl1: ReadWrite<u32, ENDPTCTRL1::Register>,
/// Endpoint control
endptctrl2: ReadWrite<u32, ENDPTCTRL2::Register>,
/// Endpoint control
endptctrl3: ReadWrite<u32, ENDPTCTRL3::Register>,
}
register_bitfields![u32,
CAPLENGTH [
    /// Indicates offset to add to the register base address at the beginning of the Ope
    CAPLENGTH OFFSET(0) NUMBITS(8) [],
    /// BCD encoding of the EHCI revision number supported by this host controller.
    HCIVERSION OFFSET(8) NUMBITS(16) []
],
HCSPARAMS [
    /// Number of downstream ports. This field specifies the number of physical downstre
    N_PORTS OFFSET(0) NUMBITS(4) [],
    /// Port Power Control. This field indicates whether the host controller implementat
    PPC OFFSET(4) NUMBITS(1) [],
    /// Number of Ports per Companion Controller. This field indicates the number of por
    N_PCC OFFSET(8) NUMBITS(4) [],
    /// Number of Companion Controller. This field indicates the number of companion con
    N_CC OFFSET(12) NUMBITS(4) [],
    /// Port indicators. This bit indicates whether the ports support port indicator con
    PI OFFSET(16) NUMBITS(1) [],
    /// Number of Ports per Transaction Translator. This field indicates the number of p
    N_PTT OFFSET(20) NUMBITS(4) [],
    /// Number of Transaction Translators. This field indicates the number of embedded t
    N_TT OFFSET(24) NUMBITS(4) []
],
HCCPARAMS [
    /// 64-bit Addressing Capability. If zero, no 64-bit addressing capability is suppor
    ADC OFFSET(0) NUMBITS(1) [],
    /// Programmable Frame List Flag. If set to one, then the system software can specif
    PFL OFFSET(1) NUMBITS(1) [],
    /// Asynchronous Schedule Park Capability. If this bit is set to a one, then the hos
    ASP OFFSET(2) NUMBITS(1) [],
    /// Isochronous Scheduling Threshold. This field indicates, relative to the current
    IST OFFSET(4) NUMBITS(4) [],
    /// EHCI Extended Capabilities Pointer. This optional field indicates the existence
    EECP OFFSET(8) NUMBITS(8) []
],
USBCMD_D [
    /// Run/Stop
    RS OFFSET(0) NUMBITS(1) [
        /// Writing a 0 to this bit will cause a detach event.
        WritingA0ToThisBitWillCauseADetachEvent = 0,
        /// Writing a one to this bit will cause the device controller to enable a pull-up o
        ATACH = 1
    ],
    /// Controller reset. Software uses this bit to reset the controller. This bit is se
    RST OFFSET(1) NUMBITS(1) [
        /// Set to 0 by hardware when the reset process is complete.
        SetTo0ByHardwareWhenTheResetProcessIsComplete = 0,
        /// When software writes a one to this bit, the Device Controller resets its interna
        RESET = 1
    ],
    /// Setup trip wire  During handling a setup packet, this bit is used as a semaphore
    SUTW OFFSET(13) NUMBITS(1) [],
    /// Add dTD trip wire This bit is used as a semaphore to ensure the to proper additi
    ATDTW OFFSET(14) NUMBITS(1) [],
    /// Not used in device mode.
    FS2 OFFSET(15) NUMBITS(1) [],
    /// Interrupt threshold control. The system software uses this field to set the maxi
    ITC OFFSET(16) NUMBITS(8) []
],
USBCMD_H [
    /// Run/Stop
    RS OFFSET(0) NUMBITS(1) [
        /// When this bit is set to 0, the Host Controller completes the current transaction
        HALT = 0,
        /// When set to a 1, the Host Controller proceeds with the execution of the schedule
        PROCEED = 1
    ],
    /// Controller reset.  Software uses this bit to reset the controller. This bit is s
    RST OFFSET(1) NUMBITS(1) [
        /// This bit is set to zero by hardware when the reset process is complete.
        ThisBitIsSetToZeroByHardwareWhenTheResetProcessIsComplete = 0,
        /// When software writes a one to this bit, the Host Controller resets its internal
        RESET = 1
    ],
    /// Bit 0 of the Frame List Size bits. See Table 281. This field specifies the size
    FS0 OFFSET(2) NUMBITS(1) [],
    /// Bit 1 of the Frame List Size bits. See Table 281
    FS1 OFFSET(3) NUMBITS(1) [],
    /// This bit controls whether the host controller skips processing the periodic sche
    PSE OFFSET(4) NUMBITS(1) [
        /// Do not process the periodic schedule.
        DoNotProcessThePeriodicSchedule = 0,
        /// Use the PERIODICLISTBASE register to access the periodic schedule.
        UseThePERIODICLISTBASERegisterToAccessThePeriodicSchedule = 1
    ],
    /// This bit controls whether the host controller skips processing the asynchronous
    ASE OFFSET(5) NUMBITS(1) [
        /// Do not process the asynchronous schedule.
        DoNotProcessTheAsynchronousSchedule = 0,
        /// Use the ASYNCLISTADDR to access the asynchronous schedule.
        UseTheASYNCLISTADDRToAccessTheAsynchronousSchedule = 1
    ],
    /// This bit is used as a doorbell by software to tell the host controller to issue
    IAA OFFSET(6) NUMBITS(1) [
        /// The host controller sets this bit to zero after it has set the Interrupt on Sync
        ST = 0,
        /// Software must write a 1 to this bit to ring the doorbell. When the host controll
        DOORBELL = 1
    ],
    /// Asynchronous schedule park mode. Contains a count of the number of successive tr
    ASP1_0 OFFSET(8) NUMBITS(2) [],
    /// Asynchronous Schedule Park Mode Enable
    ASPE OFFSET(11) NUMBITS(1) [
        /// Park mode is disabled.
        ParkModeIsDisabled = 0,
        /// Park mode is enabled.
        ParkModeIsEnabled = 1
    ],
    /// Bit 2 of the Frame List Size bits. See Table 281.
    FS2 OFFSET(15) NUMBITS(1) [],
    /// Interrupt threshold control. The system software uses this field to set the maxi
    ITC OFFSET(16) NUMBITS(8) []
],
USBSTS_D [
    /// USB interrupt
    UI OFFSET(0) NUMBITS(1) [
        /// This bit is cleared by software writing a one to it.
        ThisBitIsClearedBySoftwareWritingAOneToIt = 0,
        /// This bit is set by the Host/Device Controller when the cause of an interrupt is
        CLEAR = 1
    ],
    /// USB error interrupt
    UEI OFFSET(1) NUMBITS(1) [
        /// This bit is cleared by software writing a one to it.
        ThisBitIsClearedBySoftwareWritingAOneToIt = 0,
        /// When completion of a USB transaction results in an error condition, this bit is
        CLEAR = 1
    ],
    /// Port change detect.
    PCI OFFSET(2) NUMBITS(1) [
        /// This bit is cleared by software writing a one to it.
        ThisBitIsClearedBySoftwareWritingAOneToIt = 0,
        /// The Device Controller sets this bit to a one when the port controller enters the
        CLEAR = 1
    ],
    /// USB reset received
    URI OFFSET(6) NUMBITS(1) [
        /// This bit is cleared by software writing a one to it.
        ThisBitIsClearedBySoftwareWritingAOneToIt = 0,
        /// When the device controller detects a USB Reset and enters the default state, thi
        CLEAR = 1
    ],
    /// SOF received
    SRI OFFSET(7) NUMBITS(1) [
        /// This bit is cleared by software writing a one to it.
        ThisBitIsClearedBySoftwareWritingAOneToIt = 0,
        /// When the device controller detects a Start Of (micro) Frame, this bit will be se
        CLEAR = 1
    ],
    /// DCSuspend
    SLI OFFSET(8) NUMBITS(1) [
        /// The device controller clears the bit upon exiting from a suspend state. This bit
        ST = 0,
        /// When a device controller enters a suspend state from an active state, this bit w
        WhenADeviceControllerEntersASuspendStateFromAnActiveStateThisBitWillBeSetToAOne = 1
    ],
    /// NAK interrupt bit
    NAKI OFFSET(16) NUMBITS(1) [
        /// This bit is automatically cleared by hardware when the all the enabled TX/RX End
        ENDPCLEAR = 0,
        /// It is set by hardware when for a particular endpoint both the TX/RX Endpoint NAK
        SET = 1
    ]
],
USBSTS_H [
    /// USB interrupt (USBINT)
    UI OFFSET(0) NUMBITS(1) [
        /// This bit is cleared by software writing a one to it.
        ThisBitIsClearedBySoftwareWritingAOneToIt = 0,
        /// This bit is set by the Host/Device Controller when the cause of an interrupt is
        CLEAR = 1
    ],
    /// USB error interrupt (USBERRINT)
    UEI OFFSET(1) NUMBITS(1) [
        /// This bit is cleared by software writing a one to it.
        ThisBitIsClearedBySoftwareWritingAOneToIt = 0,
        /// When completion of a USB transaction results in an error condition, this bit is
        CLEAR = 1
    ],
    /// Port change detect.
    PCI OFFSET(2) NUMBITS(1) [
        /// This bit is cleared by software writing a one to it.
        ThisBitIsClearedBySoftwareWritingAOneToIt = 0,
        /// The Host Controller sets this bit to a one when on any port a Connect Status occ
        CLEAR = 1
    ],
    /// Frame list roll-over
    FRI OFFSET(3) NUMBITS(1) [
        /// This bit is cleared by software writing a one to it.
        ThisBitIsClearedBySoftwareWritingAOneToIt = 0,
        /// The Host Controller sets this bit to a one when the Frame List Index rolls over
        CLEAR = 1
    ],
    /// Interrupt on async advance
    AAI OFFSET(5) NUMBITS(1) [
        /// This bit is cleared by software writing a one to it.
        ThisBitIsClearedBySoftwareWritingAOneToIt = 0,
        /// System software can force the host controller to issue an interrupt the next tim
        CLEAR = 1
    ],
    /// SOF received
    SRI OFFSET(7) NUMBITS(1) [
        /// This bit is cleared by software writing a one to it.
        ThisBitIsClearedBySoftwareWritingAOneToIt = 0,
        /// In host mode, this bit will be set every 125 ms and can be used by host controll
        CLEAR = 1
    ],
    /// Not used by the Host controller.
    SLI OFFSET(8) NUMBITS(1) [],
    /// HCHalted
    HCH OFFSET(12) NUMBITS(1) [
        /// The RS bit in USBCMD is set to zero. Set by the host controller.
        TheRSBitInUSBCMDIsSetToZeroSetByTheHostController = 0,
        /// The Host Controller sets this bit to one after it has stopped executing because
        STOP = 1
    ],
    /// Reclamation
    RCL OFFSET(13) NUMBITS(1) [
        /// No empty asynchronous schedule detected.
        NoEmptyAsynchronousScheduleDetected = 0,
        /// An empty asynchronous schedule is detected. Set by the host controller.
        AnEmptyAsynchronousScheduleIsDetectedSetByTheHostController = 1
    ],
    /// Periodic schedule status This bit reports the current real status of the Periodi
    PS OFFSET(14) NUMBITS(1) [
        /// The periodic schedule status is disabled.
        ThePeriodicScheduleStatusIsDisabled = 0,
        /// The periodic schedule status is enabled.
        ThePeriodicScheduleStatusIsEnabled = 1
    ],
    /// Asynchronous schedule status This bit reports the current real status of the Asy
    AS OFFSET(15) NUMBITS(1) [
        /// Asynchronous schedule status is disabled.
        AsynchronousScheduleStatusIsDisabled = 0,
        /// Asynchronous schedule status is enabled.
        AsynchronousScheduleStatusIsEnabled = 1
    ],
    /// USB host asynchronous interrupt (USBHSTASYNCINT)
    UAI OFFSET(18) NUMBITS(1) [
        /// This bit is cleared by software writing a one to it.
        ThisBitIsClearedBySoftwareWritingAOneToIt = 0,
        /// This bit is set by the Host Controller when the cause of an interrupt is a compl
        CLEAR = 1
    ],
    /// USB host periodic interrupt (USBHSTPERINT)
    UPI OFFSET(19) NUMBITS(1) [
        /// This bit is cleared by software writing a one to it.
        ThisBitIsClearedBySoftwareWritingAOneToIt = 0,
        /// This bit is set by the Host Controller when the cause of an interrupt is a compl
        CLEAR = 1
    ]
],
USBINTR_D [
    /// USB interrupt enable When this bit is one, and the USBINT bit in the USBSTS regi
    UE OFFSET(0) NUMBITS(1) [],
    /// USB error interrupt enable When this bit is a one, and the USBERRINT bit in the
    UEE OFFSET(1) NUMBITS(1) [],
    /// Port change detect enable When this bit is a one, and the Port Change Detect bit
    PCE OFFSET(2) NUMBITS(1) [],
    /// USB reset enable When this bit is a one, and the USB Reset Received bit in the U
    URE OFFSET(6) NUMBITS(1) [],
    /// SOF received enable When this bit is a one, and the SOF Received bit in the USBS
    SRE OFFSET(7) NUMBITS(1) [],
    /// Sleep enable When this bit is a one, and the DCSuspend bit in the USBSTS registe
    SLE OFFSET(8) NUMBITS(1) [],
    /// NAK interrupt enable This bit is set by software if it wants to enable the hardw
    NAKE OFFSET(16) NUMBITS(1) [],
    /// Not used by the Device controller.
    UAIE OFFSET(18) NUMBITS(1) [],
    /// Not used by the Device controller.
    UPIA OFFSET(19) NUMBITS(1) []
],
USBINTR_H [
    /// USB interrupt enable When this bit is one, and the USBINT bit in the USBSTS regi
    UE OFFSET(0) NUMBITS(1) [],
    /// USB error interrupt enable When this bit is a one, and the USBERRINT bit in the
    UEE OFFSET(1) NUMBITS(1) [],
    /// Port change detect enable When this bit is a one, and the Port Change Detect bit
    PCE OFFSET(2) NUMBITS(1) [],
    /// Frame list rollover enable When this bit is a one, and the Frame List Rollover b
    FRE OFFSET(3) NUMBITS(1) [],
    /// Interrupt on asynchronous advance enable When this bit is a one, and the Interru
    AAE OFFSET(5) NUMBITS(1) [],
    /// If this bit is one and the SRI bit in the USBSTS register is one, the host contr
    SRE OFFSET(7) NUMBITS(1) [],
    /// USB host asynchronous interrupt enable When this bit is a one, and the USBHSTASY
    UAIE OFFSET(18) NUMBITS(1) [],
    /// USB host periodic interrupt enable When this bit is a one, and the USBHSTPERINT
    UPIA OFFSET(19) NUMBITS(1) []
],
FRINDEX_D [
    /// Current micro frame number
    FRINDEX2_0 OFFSET(0) NUMBITS(3) [],
    /// Current frame number of the last frame transmitted
    FRINDEX13_3 OFFSET(3) NUMBITS(11) []
],
FRINDEX_H [
    /// Current micro frame number
    FRINDEX2_0 OFFSET(0) NUMBITS(3) [],
    /// Frame list current index for 1024 elements.
    FRINDEX12_3 OFFSET(3) NUMBITS(10) []
],
DEVICEADDR [
    /// Device address advance
    USBADRA OFFSET(24) NUMBITS(1) [
        /// Any write to USBADR are instantaneous.
        AnyWriteToUSBADRAreInstantaneous = 0,
        /// When the user writes a one to this bit at the same time or before USBADR is writ
        HOLD = 1
    ],
    /// USB device address
    USBADR OFFSET(25) NUMBITS(7) []
],
BURSTSIZE [
    /// Programmable RX burst length This register represents the maximum length of a bu
    RXPBURST OFFSET(0) NUMBITS(8) [],
    /// Programmable TX burst length This register represents the maximum length of a bu
    TXPBURST OFFSET(8) NUMBITS(8) []
],
TXFILLTUNING [
    /// FIFO burst threshold This register controls the number of data bursts that are p
    TXSCHOH OFFSET(0) NUMBITS(8) [],
    /// Scheduler health counter This register increments when the host controller fails
    TXSCHEATLTH OFFSET(8) NUMBITS(5) [],
    /// Scheduler overhead This register adds an additional fixed offset to the schedule
    TXFIFOTHRES OFFSET(16) NUMBITS(6) []
],
ULPIVIEWPORT [
    /// When a write operation is commanded, the  data to be sent is written to this fie
    ULPIDATWR OFFSET(0) NUMBITS(8) [],
    /// After a read operation completes, the result is  placed in this field.
    ULPIDATRD OFFSET(8) NUMBITS(8) [],
    /// When a read or write operation is  commanded, the address of the operation is wr
    ULPIADDR OFFSET(16) NUMBITS(8) [],
    /// For the wakeup or read/write operation to  be executed, this value must be writt
    ULPIPORT OFFSET(24) NUMBITS(3) [],
    /// ULPI sync state. This bit represents the state of the ULPI  interface.
    ULPISS OFFSET(27) NUMBITS(1) [
        /// In another state  (ie. carkit, serial, low power)
        InAnotherStateIeCarkitSerialLowPower = 0,
        /// Normal Sync. State.
        NormalSyncState = 1
    ],
    /// ULPI Read/Write control. This bit selects between running a read or write operat
    ULPIRW OFFSET(29) NUMBITS(1) [
        /// Read
        Read = 0,
        /// Write
        Write = 1
    ],
    /// ULPI Read/Write Run. Writing the 1 to this bit will begin the  read/write operat
    ULPIRUN OFFSET(30) NUMBITS(1) [],
    /// ULPI Wake-up.  Writing the 1 to this bit will begin the wakeup  operation. The b
    ULPIWU OFFSET(31) NUMBITS(1) []
],
ENDPTNAK [
    /// Rx endpoint NAK Each RX endpoint has one bit in this field. The bit is set when
    EPRN0 OFFSET(0) NUMBITS(1) [],
    /// Rx endpoint NAK Each RX endpoint has one bit in this field. The bit is set when
    EPRN1 OFFSET(1) NUMBITS(1) [],
    /// Rx endpoint NAK Each RX endpoint has one bit in this field. The bit is set when
    EPRN2 OFFSET(2) NUMBITS(1) [],
    /// Rx endpoint NAK Each RX endpoint has one bit in this field. The bit is set when
    EPRN3 OFFSET(3) NUMBITS(1) [],
    /// Tx endpoint NAK Each TX endpoint has one bit in this field. The bit is set when
    EPTN16 OFFSET(16) NUMBITS(1) [],
    /// Tx endpoint NAK Each TX endpoint has one bit in this field. The bit is set when
    EPTN17 OFFSET(17) NUMBITS(1) [],
    /// Tx endpoint NAK Each TX endpoint has one bit in this field. The bit is set when
    EPTN18 OFFSET(18) NUMBITS(1) [],
    /// Tx endpoint NAK Each TX endpoint has one bit in this field. The bit is set when
    EPTN19 OFFSET(19) NUMBITS(1) []
],
ENDPTNAKEN [
    /// Rx endpoint NAK enable Each bit enables the corresponding RX NAK bit. If this bi
    EPRNE0 OFFSET(0) NUMBITS(1) [],
    /// Rx endpoint NAK enable Each bit enables the corresponding RX NAK bit. If this bi
    EPRNE1 OFFSET(1) NUMBITS(1) [],
    /// Rx endpoint NAK enable Each bit enables the corresponding RX NAK bit. If this bi
    EPRNE2 OFFSET(2) NUMBITS(1) [],
    /// Rx endpoint NAK enable Each bit enables the corresponding RX NAK bit. If this bi
    EPRNE3 OFFSET(3) NUMBITS(1) [],
    /// Tx endpoint NAK Each bit enables the corresponding TX NAK bit. If this bit is se
    EPTNE16 OFFSET(16) NUMBITS(1) [],
    /// Tx endpoint NAK Each bit enables the corresponding TX NAK bit. If this bit is se
    EPTNE17 OFFSET(17) NUMBITS(1) [],
    /// Tx endpoint NAK Each bit enables the corresponding TX NAK bit. If this bit is se
    EPTNE18 OFFSET(18) NUMBITS(1) [],
    /// Tx endpoint NAK Each bit enables the corresponding TX NAK bit. If this bit is se
    EPTNE19 OFFSET(19) NUMBITS(1) []
],
PORTSC1_D [
    /// Current connect status
    CCS OFFSET(0) NUMBITS(1) [
        /// Device not attached A zero indicates that the device did not attach successfully
        DEVICE_NOT_ATTACHED_ = 0,
        /// Device attached.  A one indicates that the device successfully attached and is o
        DEVICE_ATTACHED__A_ = 1
    ],
    /// Not used in device mode
    CSC OFFSET(1) NUMBITS(1) [],
    /// Port enable. This bit is always 1. The device port is always enabled.
    PE OFFSET(2) NUMBITS(1) [],
    /// Port enable/disable change This bit is always 0. The device port is always enabl
    PEC OFFSET(3) NUMBITS(1) [],
    /// Force port resume After the device has been in Suspend State for 5 ms or more, s
    FPR OFFSET(6) NUMBITS(1) [
        /// No resume (K-state) detected/driven on port.
        NoResumeKStateDetectedDrivenOnPort = 0,
        /// Resume detected/driven on port.
        ResumeDetectedDrivenOnPort = 1
    ],
    /// Suspend In device mode, this is a read-only status bit .
    SUSP OFFSET(7) NUMBITS(1) [
        /// Port not in suspend state
        PortNotInSuspendState = 0,
        /// Port in suspend state
        PortInSuspendState = 1
    ],
    /// Port reset In device mode, this is a read-only status bit. A device reset from t
    PR OFFSET(8) NUMBITS(1) [
        /// Port is not in the reset state.
        PortIsNotInTheResetState = 0,
        /// Port is in the reset state.
        PortIsInTheResetState = 1
    ],
    /// High-speed status This bit is redundant with bits 27:26 (PSPD) in this register.
    HSP OFFSET(9) NUMBITS(1) [
        /// Host/device connected to the port is not in High-speed mode.
        HostDeviceConnectedToThePortIsNotInHighSpeedMode = 0,
        /// Host/device connected to the port is in High-speed mode.
        HostDeviceConnectedToThePortIsInHighSpeedMode = 1
    ],
    /// Not used in device mode.
    LS OFFSET(10) NUMBITS(2) [],
    /// Not used in device mode.
    PP OFFSET(12) NUMBITS(1) [],
    /// Port indicator control Writing to this field effects the value of the USB1_IND1:
    PIC1_0 OFFSET(14) NUMBITS(2) [
        /// Port indicators are off.
        PortIndicatorsAreOff = 0,
        /// amber
        Amber = 1,
        /// green
        Green = 2,
        /// undefined
        Undefined = 3
    ],
    /// Port test control Any value other than 0000 indicates that the port is operating
    PTC3_0 OFFSET(16) NUMBITS(4) [
        /// TEST_MODE_DISABLE
        TEST_MODE_DISABLE = 0,
        /// J_STATE
        J_STATE = 1,
        /// K_STATE
        K_STATE = 2,
        /// SE0 (host)/NAK (device)
        SE0HostNAKDevice = 3,
        /// Packet
        Packet = 4,
        /// FORCE_ENABLE_HS
        FORCE_ENABLE_HS = 5,
        /// FORCE_ENABLE_FS
        FORCE_ENABLE_FS = 6
    ],
    /// PHY low power suspend - clock disable (PLPSCD) In device mode, The PHY can be pu
    PHCD OFFSET(23) NUMBITS(1) [
        /// Writing a 0 enables the PHY clock. Reading a 0 indicates the status of the PHY c
        WritingA0EnablesThePHYClockReadingA0IndicatesTheStatusOfThePHYClockEnabled = 0,
        /// Writing a 1 disables the PHY clock. Reading a 1 indicates the status of the PHY
        WritingA1DisablesThePHYClockReadingA1IndicatesTheStatusOfThePHYClockDisabled = 1
    ],
    /// Port force full speed connect
    PFSC OFFSET(24) NUMBITS(1) [
        /// Port connects at any speed.
        PortConnectsAtAnySpeed = 0,
        /// Writing this bit to a 1 will force the port to only connect at full speed. It di
        FULLSPEED = 1
    ],
    /// Port speed This register field indicates the speed at which the port is operatin
    PSPD OFFSET(26) NUMBITS(2) [
        /// Full-speed
        FullSpeed = 1,
        /// invalid in device mode
        InvalidInDeviceMode = 2,
        /// High-speed
        HighSpeed = 3
    ],
    /// Parallel transceiver select. All other values are reserved.
    PTS OFFSET(30) NUMBITS(2) [
        /// ULPI
        ULPI = 2,
        /// Serial/ 1.1 PHY (Full-speed only)
        Serial11PHYFullSpeedOnly = 3
    ]
],
PORTSC1_H [
    /// Current connect status This value reflects the current state of the port and may
    CCS OFFSET(0) NUMBITS(1) [
        /// No device is present.
        NoDeviceIsPresent = 0,
        /// Device is present on the port.
        DeviceIsPresentOnThePort = 1
    ],
    /// Connect status change Indicates a change has occurred in the port's Current Conn
    CSC OFFSET(1) NUMBITS(1) [
        /// No change in current status.
        NoChangeInCurrentStatus = 0,
        /// Change in current status.
        ChangeInCurrentStatus = 1
    ],
    /// Port enable. Ports can only be enabled by the host controller as a part of the r
    PE OFFSET(2) NUMBITS(1) [
        /// Port disabled.
        PortDisabled = 0,
        /// Port enabled.
        PortEnabled = 1
    ],
    /// Port disable/enable change For the root hub, this bit gets set to a one only whe
    PEC OFFSET(3) NUMBITS(1) [
        /// No change.
        NoChange = 0,
        /// Port enabled/disabled status has changed.
        PortEnabledDisabledStatusHasChanged = 1
    ],
    /// Over-current active This bit will automatically transition from 1 to 0 when the
    OCA OFFSET(4) NUMBITS(1) [
        /// The port does not have an over-current condition.
        ThePortDoesNotHaveAnOverCurrentCondition = 0,
        /// The port has currently an over-current condition.
        ThePortHasCurrentlyAnOverCurrentCondition = 1
    ],
    /// Over-current change This bit gets set to one when there is a change to Over-curr
    OCC OFFSET(5) NUMBITS(1) [],
    /// Force port resume Software sets this bit to one to drive resume signaling. The H
    FPR OFFSET(6) NUMBITS(1) [
        /// No resume (K-state) detected/driven on port.
        NoResumeKStateDetectedDrivenOnPort = 0,
        /// Resume detected/driven on port.
        ResumeDetectedDrivenOnPort = 1
    ],
    /// Suspend Together with the PE (Port enabled bit), this bit describes the port sta
    SUSP OFFSET(7) NUMBITS(1) [
        /// Port not in suspend state
        PortNotInSuspendState = 0,
        /// Port in suspend state When in suspend state, downstream propagation of data is b
        PORT_IN_SUSPEND_STAT = 1
    ],
    /// Port reset When software writes a one to this bit the bus-reset sequence as defi
    PR OFFSET(8) NUMBITS(1) [
        /// Port is not in the reset state.
        PortIsNotInTheResetState = 0,
        /// Port is in the reset state.
        PortIsInTheResetState = 1
    ],
    /// High-speed status
    HSP OFFSET(9) NUMBITS(1) [
        /// Host/device connected to the port is not in High-speed mode.
        HostDeviceConnectedToThePortIsNotInHighSpeedMode = 0,
        /// Host/device connected to the port is in High-speed mode.
        HostDeviceConnectedToThePortIsInHighSpeedMode = 1
    ],
    /// Line status These bits reflect the current logical levels of the USB_DP and USB_
    LS OFFSET(10) NUMBITS(2) [
        /// SE0 (USB_DP and USB_DM LOW)
        SE0USB_DPAndUSB_DMLOW = 0,
        /// J-state (USB_DP HIGH and USB_DM LOW)
        JStateUSB_DPHIGHAndUSB_DMLOW = 1,
        /// K-state (USB_DP LOW and USB_DM HIGH)
        KStateUSB_DPLOWAndUSB_DMHIGH = 2,
        /// Undefined
        Undefined = 3
    ],
    /// Port power control Host controller requires port power control switches. This bi
    PP OFFSET(12) NUMBITS(1) [
        /// Port power off.
        PortPowerOff = 0,
        /// Port power on.
        PortPowerOn = 1
    ],
    /// Port indicator control Writing to this field controls the value of the pins USB1
    PIC1_0 OFFSET(14) NUMBITS(2) [
        /// Port indicators are off.
        PortIndicatorsAreOff = 0,
        /// Amber
        Amber = 1,
        /// Green
        Green = 2,
        /// Undefined
        Undefined = 3
    ],
    /// Port test control Any value other than 0000 indicates that the port is operating
    PTC3_0 OFFSET(16) NUMBITS(4) [
        /// TEST_MODE_DISABLE
        TEST_MODE_DISABLE = 0,
        /// J_STATE
        J_STATE = 1,
        /// K_STATE
        K_STATE = 2,
        /// SE0 (host)/NAK (device)
        SE0HostNAKDevice = 3,
        /// Packet
        Packet = 4,
        /// FORCE_ENABLE_HS
        FORCE_ENABLE_HS = 5,
        /// FORCE_ENABLE_FS
        FORCE_ENABLE_FS = 6,
        /// FORCE_ENABLE_LS
        FORCE_ENABLE_LS = 7
    ],
    /// Wake on connect enable (WKCNNT_E) This bit is 0 if PP (Port Power bit) is 0
    WKCN OFFSET(20) NUMBITS(1) [
        /// Disables the port to wake up on device connects.
        DisablesThePortToWakeUpOnDeviceConnects = 0,
        /// Writing this bit to a one enables the port to be sensitive to device connects as
        WritingThisBitToAOneEnablesThePortToBeSensitiveToDeviceConnectsAsWakeUpEvents = 1
    ],
    /// Wake on disconnect enable (WKDSCNNT_E) This bit is 0 if PP (Port Power bit) is 0
    WKDC OFFSET(21) NUMBITS(1) [
        /// Disables the port to wake up on device disconnects.
        DisablesThePortToWakeUpOnDeviceDisconnects = 0,
        /// Writing this bit to a one enables the port to be sensitive to device disconnects
        ENABLE_DEVICE_CON = 1
    ],
    /// Wake on over-current enable (WKOC_E)
    WKOC OFFSET(22) NUMBITS(1) [
        /// Disables the port to wake up on over-current events.
        DisablesThePortToWakeUpOnOverCurrentEvents = 0,
        /// Writing a one to this bit enabled the port to be sensitive to over-current condi
        ENABLE_OVERCURRENT = 1
    ],
    /// PHY low power suspend - clock disable (PLPSCD) In host mode, the PHY can be put
    PHCD OFFSET(23) NUMBITS(1) [
        /// Writing a 0 enables the PHY clock. Reading a 0 indicates the status of the PHY c
        WritingA0EnablesThePHYClockReadingA0IndicatesTheStatusOfThePHYClockEnabled = 0,
        /// Writing a 1 disables the PHY clock. Reading a 1 indicates the status of the PHY
        WritingA1DisablesThePHYClockReadingA1IndicatesTheStatusOfThePHYClockDisabled = 1
    ],
    /// Port force full speed connect
    PFSC OFFSET(24) NUMBITS(1) [
        /// Port connects at any speed.
        PortConnectsAtAnySpeed = 0,
        /// Writing this bit to a 1 will force the port to only connect at Full Speed. It di
        FULLSPEED = 1
    ],
    /// Port speed This register field indicates the speed at which the port is operatin
    PSPD OFFSET(26) NUMBITS(2) [
        /// Full-speed
        FullSpeed = 0,
        /// Low-speed
        LowSpeed = 1,
        /// High-speed
        HighSpeed = 2
    ],
    /// Parallel transceiver select. All other values are reserved.
    PTS OFFSET(30) NUMBITS(2) [
        /// ULPI
        ULPI = 2,
        /// Serial/ 1.1 PHY (Full-speed only)
        Serial11PHYFullSpeedOnly = 3
    ]
],
USBMODE_D [
    /// Controller mode The controller defaults to an idle state and needs to be initial
    CM1_0 OFFSET(0) NUMBITS(2) [
        /// Idle
        Idle = 0,
        /// Reserved
        Reserved = 1,
        /// Device controller
        DeviceController = 2,
        /// Host controller
        HostController = 3
    ],
    /// Endian select This bit can change the byte ordering of the transfer buffers to m
    ES OFFSET(2) NUMBITS(1) [
        /// Little endian: first byte referenced in least significant byte of 32-bit word.
        LittleEndianFirstByteReferencedInLeastSignificantByteOf32BitWord = 0,
        /// Big endian: first byte referenced in most significant byte of 32-bit word.
        BigEndianFirstByteReferencedInMostSignificantByteOf32BitWord = 1
    ],
    /// Setup Lockout mode In device mode, this bit controls behavior of the setup lock
    SLOM OFFSET(3) NUMBITS(1) [
        /// Setup Lockouts on
        SetupLockoutsOn = 0,
        /// Setup Lockouts Off (DCD requires the use of Setup Buffer Tripwire in USBCMD)
        SetupLockoutsOffDCDRequiresTheUseOfSetupBufferTripwireInUSBCMD = 1
    ],
    /// Stream disable mode  The use of this feature substantially limits the overall US
    SDIS OFFSET(4) NUMBITS(1) [
        /// Not disabled
        NotDisabled = 0,
        /// Disabled. Setting this bit to one disables double priming on both RX and TX for
        DISABLED_SETTING_TH = 1
    ]
],
USBMODE_H [
    /// Controller mode The controller defaults to an idle state and needs to be initial
    CM1_0 OFFSET(0) NUMBITS(2) [
        /// Idle
        Idle = 0,
        /// Reserved
        Reserved = 1,
        /// Device controller
        DeviceController = 2,
        /// Host controller
        HostController = 3
    ],
    /// Endian select This bit can change the byte ordering of the transfer buffers. The
    ES OFFSET(2) NUMBITS(1) [
        /// Little endian: first byte referenced in least significant byte of 32-bit word.
        LittleEndianFirstByteReferencedInLeastSignificantByteOf32BitWord = 0,
        /// Big endian: first byte referenced in most significant byte of 32-bit word.
        BigEndianFirstByteReferencedInMostSignificantByteOf32BitWord = 1
    ],
    /// Stream disable mode  The use of this feature substantially limits the overall US
    SDIS OFFSET(4) NUMBITS(1) [
        /// Not disabled
        NotDisabled = 0,
        /// Disabled. Setting to a 1 ensures that overruns/underruns of the latency FIFO are
        DISABLED_SETTING_TO = 1
    ],
    /// VBUS power select
    VBPS OFFSET(5) NUMBITS(1) [
        /// vbus_pwr_select is set LOW.
        Vbus_pwr_selectIsSetLOW = 0,
        /// vbus_pwr_select is set HIGH
        Vbus_pwr_selectIsSetHIGH = 1
    ]
],
ENDPTSETUPSTAT [
    /// Setup endpoint status for logical endpoints. For every setup transaction that is
    ENDPTSETUPSTAT0 OFFSET(0) NUMBITS(1) [],
    /// Setup endpoint status for logical endpoints. For every setup transaction that is
    ENDPTSETUPSTAT1 OFFSET(1) NUMBITS(1) [],
    /// Setup endpoint status for logical endpoints. For every setup transaction that is
    ENDPTSETUPSTAT2 OFFSET(2) NUMBITS(1) [],
    /// Setup endpoint status for logical endpoints. For every setup transaction that is
    ENDPTSETUPSTAT3 OFFSET(3) NUMBITS(1) []
],
ENDPTPRIME [
    /// Prime endpoint receive buffer for physical OUT endpoints. For each OUT endpoint,
    PERB0 OFFSET(0) NUMBITS(1) [],
    /// Prime endpoint receive buffer for physical OUT endpoints. For each OUT endpoint,
    PERB1 OFFSET(1) NUMBITS(1) [],
    /// Prime endpoint receive buffer for physical OUT endpoints. For each OUT endpoint,
    PERB2 OFFSET(2) NUMBITS(1) [],
    /// Prime endpoint receive buffer for physical OUT endpoints. For each OUT endpoint,
    PERB3 OFFSET(3) NUMBITS(1) [],
    /// Prime endpoint transmit buffer for physical IN endpoints. For each IN endpoint a
    PETB0 OFFSET(16) NUMBITS(1) [],
    /// Prime endpoint transmit buffer for physical IN endpoints. For each IN endpoint a
    PETB1 OFFSET(17) NUMBITS(1) [],
    /// Prime endpoint transmit buffer for physical IN endpoints. For each IN endpoint a
    PETB2 OFFSET(18) NUMBITS(1) [],
    /// Prime endpoint transmit buffer for physical IN endpoints. For each IN endpoint a
    PETB3 OFFSET(19) NUMBITS(1) []
],
ENDPTFLUSH [
    /// Flush endpoint receive buffer for physical OUT endpoints. Writing a one to a bit
    FERB0 OFFSET(0) NUMBITS(1) [],
    /// Flush endpoint receive buffer for physical OUT endpoints. Writing a one to a bit
    FERB1 OFFSET(1) NUMBITS(1) [],
    /// Flush endpoint receive buffer for physical OUT endpoints. Writing a one to a bit
    FERB2 OFFSET(2) NUMBITS(1) [],
    /// Flush endpoint receive buffer for physical OUT endpoints. Writing a one to a bit
    FERB3 OFFSET(3) NUMBITS(1) [],
    /// Flush endpoint transmit buffer for physical IN endpoints. Writing a one to a bit
    FETB0 OFFSET(16) NUMBITS(1) [],
    /// Flush endpoint transmit buffer for physical IN endpoints. Writing a one to a bit
    FETB1 OFFSET(17) NUMBITS(1) [],
    /// Flush endpoint transmit buffer for physical IN endpoints. Writing a one to a bit
    FETB2 OFFSET(18) NUMBITS(1) [],
    /// Flush endpoint transmit buffer for physical IN endpoints. Writing a one to a bit
    FETB3 OFFSET(19) NUMBITS(1) []
],
ENDPTSTAT [
    /// Endpoint receive buffer ready for physical OUT endpoints. This bit is set to 1 b
    ERBR0 OFFSET(0) NUMBITS(1) [],
    /// Endpoint receive buffer ready for physical OUT endpoints. This bit is set to 1 b
    ERBR1 OFFSET(1) NUMBITS(1) [],
    /// Endpoint receive buffer ready for physical OUT endpoints. This bit is set to 1 b
    ERBR2 OFFSET(2) NUMBITS(1) [],
    /// Endpoint receive buffer ready for physical OUT endpoints. This bit is set to 1 b
    ERBR3 OFFSET(3) NUMBITS(1) [],
    /// Endpoint transmit buffer ready for physical IN endpoints 3 to 0. This bit is set
    ETBR0 OFFSET(16) NUMBITS(1) [],
    /// Endpoint transmit buffer ready for physical IN endpoints 3 to 0. This bit is set
    ETBR1 OFFSET(17) NUMBITS(1) [],
    /// Endpoint transmit buffer ready for physical IN endpoints 3 to 0. This bit is set
    ETBR2 OFFSET(18) NUMBITS(1) [],
    /// Endpoint transmit buffer ready for physical IN endpoints 3 to 0. This bit is set
    ETBR3 OFFSET(19) NUMBITS(1) []
],
ENDPTCOMPLETE [
    /// Endpoint receive complete event for physical OUT endpoints. This bit is set to 1
    ERCE0 OFFSET(0) NUMBITS(1) [],
    /// Endpoint receive complete event for physical OUT endpoints. This bit is set to 1
    ERCE1 OFFSET(1) NUMBITS(1) [],
    /// Endpoint receive complete event for physical OUT endpoints. This bit is set to 1
    ERCE2 OFFSET(2) NUMBITS(1) [],
    /// Endpoint receive complete event for physical OUT endpoints. This bit is set to 1
    ERCE3 OFFSET(3) NUMBITS(1) [],
    /// Endpoint transmit complete event for physical IN endpoints. This bit is set to 1
    ETCE0 OFFSET(16) NUMBITS(1) [],
    /// Endpoint transmit complete event for physical IN endpoints. This bit is set to 1
    ETCE1 OFFSET(17) NUMBITS(1) [],
    /// Endpoint transmit complete event for physical IN endpoints. This bit is set to 1
    ETCE2 OFFSET(18) NUMBITS(1) [],
    /// Endpoint transmit complete event for physical IN endpoints. This bit is set to 1
    ETCE3 OFFSET(19) NUMBITS(1) []
],
ENDPTCTRL0 [
    /// Rx endpoint stall
    RXS OFFSET(0) NUMBITS(1) [
        /// Endpoint ok.
        EndpointOk = 0,
        /// Endpoint stalled Software can write a one to this bit to force the endpoint to r
        ENDPOINT_STALLED_SOF = 1
    ],
    /// Endpoint type Endpoint 0 is always a control endpoint.
    RXT OFFSET(2) NUMBITS(2) [],
    /// Rx endpoint enable Endpoint enabled. Control endpoint 0 is always enabled. This
    RXE OFFSET(7) NUMBITS(1) [],
    /// Tx endpoint stall
    TXS OFFSET(16) NUMBITS(1) [
        /// Endpoint ok.
        EndpointOk = 0,
        /// Endpoint stalled Software can write a one to this bit to force the endpoint to r
        ENDPOINT_STALLED_SOF = 1
    ],
    /// Endpoint type Endpoint 0 is always a control endpoint.
    TXT OFFSET(18) NUMBITS(2) [],
    /// Tx endpoint enable Endpoint enabled. Control endpoint 0 is always enabled. This
    TXE OFFSET(23) NUMBITS(1) []
],
ENDPTCTRL1 [
    /// Rx endpoint stall
    RXS OFFSET(0) NUMBITS(1) [
        /// Endpoint ok. This bit will be cleared automatically upon receipt of a SETUP requ
        ENDPOINT_OK_THIS_BI = 0,
        /// Endpoint stalled Software can write a one to this bit to force the endpoint to r
        ENDPOINT_STALLED_SOF = 1
    ],
    /// Endpoint type
    RXT OFFSET(2) NUMBITS(2) [
        /// Control
        Control = 0,
        /// Isochronous
        Isochronous = 1,
        /// Bulk
        Bulk = 2,
        /// Reserved
        Reserved = 3
    ],
    /// Rx data toggle inhibit This bit is only used for test and should always be writt
    RXI OFFSET(5) NUMBITS(1) [
        /// Disabled
        Disabled = 0,
        /// Enabled
        Enabled = 1
    ],
    /// Rx data toggle reset Write 1 to reset the PID sequence. Whenever a configuration
    RXR OFFSET(6) NUMBITS(1) [],
    /// Rx endpoint enable An endpoint should be enabled only after it has been configur
    RXE OFFSET(7) NUMBITS(1) [
        /// Endpoint disabled.
        EndpointDisabled = 0,
        /// Endpoint enabled.
        EndpointEnabled = 1
    ],
    /// Tx endpoint stall
    TXS OFFSET(16) NUMBITS(1) [
        /// Endpoint ok. This bit will be cleared automatically upon receipt of a SETUP requ
        ENDPOINT_OK_THIS_BI = 0,
        /// Endpoint stalled Software can write a one to this bit to force the endpoint to r
        ENDPOINT_STALLED_SOF = 1
    ],
    /// Tx endpoint type
    TXT OFFSET(18) NUMBITS(2) [
        /// Control
        Control = 0,
        /// Isochronous
        Isochronous = 1,
        /// Bulk
        Bulk = 2,
        /// Interrupt
        Interrupt = 3
    ],
    /// Tx data toggle inhibit This bit is only used for test and should always be writt
    TXI OFFSET(21) NUMBITS(1) [
        /// Enabled
        Enabled = 0,
        /// Disabled
        Disabled = 1
    ],
    /// Tx data toggle reset Write 1 to reset the PID sequence. Whenever a configuration
    TXR OFFSET(22) NUMBITS(1) [],
    /// Tx endpoint enable An endpoint should be enabled only after it has been configur
    TXE OFFSET(23) NUMBITS(1) [
        /// Endpoint disabled.
        EndpointDisabled = 0,
        /// Endpoint enabled.
        EndpointEnabled = 1
    ]
],
ENDPTCTRL2 [
    /// Rx endpoint stall
    RXS OFFSET(0) NUMBITS(1) [
        /// Endpoint ok. This bit will be cleared automatically upon receipt of a SETUP requ
        ENDPOINT_OK_THIS_BI = 0,
        /// Endpoint stalled Software can write a one to this bit to force the endpoint to r
        ENDPOINT_STALLED_SOF = 1
    ],
    /// Endpoint type
    RXT OFFSET(2) NUMBITS(2) [
        /// Control
        Control = 0,
        /// Isochronous
        Isochronous = 1,
        /// Bulk
        Bulk = 2,
        /// Reserved
        Reserved = 3
    ],
    /// Rx data toggle inhibit This bit is only used for test and should always be writt
    RXI OFFSET(5) NUMBITS(1) [
        /// Disabled
        Disabled = 0,
        /// Enabled
        Enabled = 1
    ],
    /// Rx data toggle reset Write 1 to reset the PID sequence. Whenever a configuration
    RXR OFFSET(6) NUMBITS(1) [],
    /// Rx endpoint enable An endpoint should be enabled only after it has been configur
    RXE OFFSET(7) NUMBITS(1) [
        /// Endpoint disabled.
        EndpointDisabled = 0,
        /// Endpoint enabled.
        EndpointEnabled = 1
    ],
    /// Tx endpoint stall
    TXS OFFSET(16) NUMBITS(1) [
        /// Endpoint ok. This bit will be cleared automatically upon receipt of a SETUP requ
        ENDPOINT_OK_THIS_BI = 0,
        /// Endpoint stalled Software can write a one to this bit to force the endpoint to r
        ENDPOINT_STALLED_SOF = 1
    ],
    /// Tx endpoint type
    TXT OFFSET(18) NUMBITS(2) [
        /// Control
        Control = 0,
        /// Isochronous
        Isochronous = 1,
        /// Bulk
        Bulk = 2,
        /// Interrupt
        Interrupt = 3
    ],
    /// Tx data toggle inhibit This bit is only used for test and should always be writt
    TXI OFFSET(21) NUMBITS(1) [
        /// Enabled
        Enabled = 0,
        /// Disabled
        Disabled = 1
    ],
    /// Tx data toggle reset Write 1 to reset the PID sequence. Whenever a configuration
    TXR OFFSET(22) NUMBITS(1) [],
    /// Tx endpoint enable An endpoint should be enabled only after it has been configur
    TXE OFFSET(23) NUMBITS(1) [
        /// Endpoint disabled.
        EndpointDisabled = 0,
        /// Endpoint enabled.
        EndpointEnabled = 1
    ]
],
ENDPTCTRL3 [
    /// Rx endpoint stall
    RXS OFFSET(0) NUMBITS(1) [
        /// Endpoint ok. This bit will be cleared automatically upon receipt of a SETUP requ
        ENDPOINT_OK_THIS_BI = 0,
        /// Endpoint stalled Software can write a one to this bit to force the endpoint to r
        ENDPOINT_STALLED_SOF = 1
    ],
    /// Endpoint type
    RXT OFFSET(2) NUMBITS(2) [
        /// Control
        Control = 0,
        /// Isochronous
        Isochronous = 1,
        /// Bulk
        Bulk = 2,
        /// Reserved
        Reserved = 3
    ],
    /// Rx data toggle inhibit This bit is only used for test and should always be writt
    RXI OFFSET(5) NUMBITS(1) [
        /// Disabled
        Disabled = 0,
        /// Enabled
        Enabled = 1
    ],
    /// Rx data toggle reset Write 1 to reset the PID sequence. Whenever a configuration
    RXR OFFSET(6) NUMBITS(1) [],
    /// Rx endpoint enable An endpoint should be enabled only after it has been configur
    RXE OFFSET(7) NUMBITS(1) [
        /// Endpoint disabled.
        EndpointDisabled = 0,
        /// Endpoint enabled.
        EndpointEnabled = 1
    ],
    /// Tx endpoint stall
    TXS OFFSET(16) NUMBITS(1) [
        /// Endpoint ok. This bit will be cleared automatically upon receipt of a SETUP requ
        ENDPOINT_OK_THIS_BI = 0,
        /// Endpoint stalled Software can write a one to this bit to force the endpoint to r
        ENDPOINT_STALLED_SOF = 1
    ],
    /// Tx endpoint type
    TXT OFFSET(18) NUMBITS(2) [
        /// Control
        Control = 0,
        /// Isochronous
        Isochronous = 1,
        /// Bulk
        Bulk = 2,
        /// Interrupt
        Interrupt = 3
    ],
    /// Tx data toggle inhibit This bit is only used for test and should always be writt
    TXI OFFSET(21) NUMBITS(1) [
        /// Enabled
        Enabled = 0,
        /// Disabled
        Disabled = 1
    ],
    /// Tx data toggle reset Write 1 to reset the PID sequence. Whenever a configuration
    TXR OFFSET(22) NUMBITS(1) [],
    /// Tx endpoint enable An endpoint should be enabled only after it has been configur
    TXE OFFSET(23) NUMBITS(1) [
        /// Endpoint disabled.
        EndpointDisabled = 0,
        /// Endpoint enabled.
        EndpointEnabled = 1
    ]
]
];
const USB1_BASE: StaticRef<Usb1Registers> =
    unsafe { StaticRef::new(0x40007000 as *const Usb1Registers) };
