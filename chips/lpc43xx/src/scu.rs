
use kernel::common::StaticRef;
use kernel::common::registers::{ReadWrite, register_bitfields};
    /// System Control Unit (SCU) I/O configuration
/// FIOXME: edu ciaa implementation is not the same as the documentation
#[repr(C)]
pub struct ScuRegisters {
/// Pin configuration register for pins P0
pub sfsp: [[ReadWrite<u32, SFSP::Register>; 32]; 16],
pub _reserved0: [u32; 256],
/// Pin configuration register for pins CLK
pub sfsclk: [ReadWrite<u32>; 4],
pub _reserved1: [u32; 28],
/// Pin configuration register for pins USB1_DM and USB1_DP
pub sfsusb: ReadWrite<u32>,
/// Pin configuration register for I2C0-bus pins
pub sfsi2c0: ReadWrite<u32>,
/// ADC0 function select register
pub enaio: [ReadWrite<u32>; 3],
pub _reserved2: [u32; 27],
/// EMC clock delay register
pub emcdelayclk: ReadWrite<u32>,
/// TODO/FIXME: complete here. existing implementations and documentation
/// say different things
/// SD/MMC sample and drive delay register
//pub sddelay: ReadWrite<u32>,
pub _reserved3: [u32; 63],
/// Pin interrupt select register for pin interrupts 0 to 3.
pub pintsel: [ReadWrite<u32>; 2],
}

register_bitfields![u32,
SFSP [
    /// Select pin function.
    MODE OFFSET(0) NUMBITS(3) [
        /// Function 0 (default)
        Function0Default = 0,
        /// Function 1
        Function1 = 1,
        /// Function 2
        Function2 = 2,
        /// Function 3
        Function3 = 3,
        /// Function 4
        Function4 = 4,
        /// Function 5
        Function5 = 5,
        /// Function 6
        Function6 = 6,
        /// Function 7
        Function7 = 7
    ],
    /// Enable pull-down resistor at pad.
    EPD OFFSET(3) NUMBITS(1) [
        /// Disable pull-down.
        DisablePullDown = 0,
        /// Enable pull-down. Enable both pull-down resistor and pull-up resistor for repeat
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up
        DisablePullUp = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset but must b
    EZI OFFSET(6) NUMBITS(1) [
        /// Disable input buffer
        DisableInputBuffer = 0,
        /// Enable input buffer
        EnableInputBuffer = 1
    ],
    /// Input glitch filter. Disable the input glitch filter for clocking signals higher
    ZIF OFFSET(7) NUMBITS(1) [
        /// Enable input glitch filter
        EnableInputGlitchFilter = 0,
        /// Disable input glitch filter
        DisableInputGlitchFilter = 1
    ],
    /// Select drive strength.
    EHD OFFSET(8) NUMBITS(2) [
        /// Normal-drive: 4 mA drive strength
        NormalDrive4MADriveStrength = 0,
        /// Medium-drive: 8 mA drive strength
        MediumDrive8MADriveStrength = 1,
        /// High-drive: 14 mA drive strength
        HighDrive14MADriveStrength = 2,
        /// Ultra high-drive: 20 mA drive strength
        UltraHighDrive20MADriveStrength = 3
    ]
],
SFSUSB [
    /// Differential data input AIP/AIM.
    USB_AIM OFFSET(0) NUMBITS(1) [
        /// Going LOW with full speed edge rate
        GoingLOWWithFullSpeedEdgeRate = 0,
        /// Going HIGH with full speed edge rate
        GoingHIGHWithFullSpeedEdgeRate = 1
    ],
    /// Control signal for differential input or single input.
    USB_ESEA OFFSET(1) NUMBITS(1) [
        /// Reserved. Do not use.
        ReservedDoNotUse = 0,
        /// Single input. Enables USB1. Use with the on-chip full-speed PHY.
        SingleInputEnablesUSB1UseWithTheOnChipFullSpeedPHY = 1
    ],
    /// Enable pull-down connect.
    USB_EPD OFFSET(2) NUMBITS(1) [
        /// Pull-down disconnected
        PullDownDisconnected = 0,
        /// Pull-down connected
        PullDownConnected = 1
    ],
    /// Power mode.
    USB_EPWR OFFSET(4) NUMBITS(1) [
        /// Power saving mode (Suspend mode)
        PowerSavingModeSuspendMode = 0,
        /// Normal mode
        NormalMode = 1
    ],
    /// Enable the vbus_valid signal. This signal is monitored by the USB1 block. Use th
    USB_VBUS OFFSET(5) NUMBITS(1) [
        /// VBUS signal LOW or inactive
        VBUSSignalLOWOrInactive = 0,
        /// VBUS signal HIGH or active
        VBUSSignalHIGHOrActive = 1
    ]
],
SFSI2C0 [
    /// Select input glitch filter time constant for the SCL pin.
    SCL_EFP OFFSET(0) NUMBITS(1) [
        /// 50 ns glitch filter
        _50NsGlitchFilter = 0,
        /// 3 ns glitch filter
        _3NsGlitchFilter = 1
    ],
    /// Select I2C mode for the SCL pin.
    SCL_EHD OFFSET(2) NUMBITS(1) [
        /// Standard/Fast mode transmit
        StandardFastModeTransmit = 0,
        /// Fast-mode Plus transmit
        FastModePlusTransmit = 1
    ],
    /// Enable the input receiver for the SCL pin. Always write a 1 to this bit when usi
    SCL_EZI OFFSET(3) NUMBITS(1) [
        /// Disabled
        Disabled = 0,
        /// Enabled
        Enabled = 1
    ],
    /// Enable or disable input glitch filter for the SCL pin. The filter time constant
    SCL_ZIF OFFSET(7) NUMBITS(1) [
        /// Enable input filter
        EnableInputFilter = 0,
        /// Disable input filter
        DisableInputFilter = 1
    ],
    /// Select input glitch filter time constant for the SDA pin.
    SDA_EFP OFFSET(8) NUMBITS(1) [
        /// 50 ns glitch filter
        _50NsGlitchFilter = 0,
        /// 3 ns glitch filter
        _3NsGlitchFilter = 1
    ],
    /// Select I2C mode for the SDA pin.
    SDA_EHD OFFSET(10) NUMBITS(1) [
        /// Standard/Fast mode transmit
        StandardFastModeTransmit = 0,
        /// Fast-mode Plus transmit
        FastModePlusTransmit = 1
    ],
    /// Enable the input receiver for the SDA pin. Always write a 1 to this bit when usi
    SDA_EZI OFFSET(11) NUMBITS(1) [
        /// Disabled
        Disabled = 0,
        /// Enabled
        Enabled = 1
    ],
    /// Enable or disable input glitch filter for the SDA pin. The filter time constant
    SDA_ZIF OFFSET(15) NUMBITS(1) [
        /// Enable input filter
        EnableInputFilter = 0,
        /// Disable input filter
        DisableInputFilter = 1
    ]
],
ENAIO0 [
    /// Select ADC0_0
    ADC0_0 OFFSET(0) NUMBITS(1) [
        /// Digital function selected on pin P4_3.
        DigitalFunctionSelectedOnPinP4_3 = 0,
        /// Analog function ADC0_0 selected on pin P4_3
        AnalogFunctionADC0_0SelectedOnPinP4_3 = 1
    ],
    /// Select ADC0_1
    ADC0_1 OFFSET(1) NUMBITS(1) [
        /// Digital function selected on pin P4_1.
        DigitalFunctionSelectedOnPinP4_1 = 0,
        /// Analog function ADC0_1 selected on pin P4_1.
        AnalogFunctionADC0_1SelectedOnPinP4_1 = 1
    ],
    /// Select ADC0_2
    ADC0_2 OFFSET(2) NUMBITS(1) [
        /// Digital function selected on pin PF_8.
        DigitalFunctionSelectedOnPinPF_8 = 0,
        /// Analog function ADC0_2 selected on pin PF_8.
        AnalogFunctionADC0_2SelectedOnPinPF_8 = 1
    ],
    /// Select ADC0_3
    ADC0_3 OFFSET(3) NUMBITS(1) [
        /// Digital function selected on pin P7_5.
        DigitalFunctionSelectedOnPinP7_5 = 0,
        /// Analog function ADC0_3 selected on pin P7_5.
        AnalogFunctionADC0_3SelectedOnPinP7_5 = 1
    ],
    /// Select ADC0_4
    ADC0_4 OFFSET(4) NUMBITS(1) [
        /// Digital function selected on pin P7_4.
        DigitalFunctionSelectedOnPinP7_4 = 0,
        /// Analog function ADC0_4 selected on pin P7_4.
        AnalogFunctionADC0_4SelectedOnPinP7_4 = 1
    ],
    /// Select ADC0_5
    ADC0_5 OFFSET(5) NUMBITS(1) [
        /// Digital function selected on pin PF_10.
        DigitalFunctionSelectedOnPinPF_10 = 0,
        /// Analog function ADC0_5 selected on pin PF_10.
        AnalogFunctionADC0_5SelectedOnPinPF_10 = 1
    ],
    /// Select ADC0_6
    ADC0_6 OFFSET(6) NUMBITS(1) [
        /// Digital function selected on pin PB_6.
        DigitalFunctionSelectedOnPinPB_6 = 0,
        /// Analog function ADC0_6 selected on pin PB_6.
        AnalogFunctionADC0_6SelectedOnPinPB_6 = 1
    ]
],
ENAIO1 [
    /// Select ADC1_0
    ADC1_0 OFFSET(0) NUMBITS(1) [
        /// Digital function selected on pin PC_3.
        DigitalFunctionSelectedOnPinPC_3 = 0,
        /// Analog function ADC1_0 selected on pin PC_3.
        AnalogFunctionADC1_0SelectedOnPinPC_3 = 1
    ],
    /// Select ADC1_1
    ADC1_1 OFFSET(1) NUMBITS(1) [
        /// Digital function selected on pin PC_0.
        DigitalFunctionSelectedOnPinPC_0 = 0,
        /// Analog function ADC1_1 selected on pin PC_0.
        AnalogFunctionADC1_1SelectedOnPinPC_0 = 1
    ],
    /// Select ADC1_2
    ADC1_2 OFFSET(2) NUMBITS(1) [
        /// Digital function selected on pin PF_9.
        DigitalFunctionSelectedOnPinPF_9 = 0,
        /// Analog function ADC1_2 selected on pin PF_9.
        AnalogFunctionADC1_2SelectedOnPinPF_9 = 1
    ],
    /// Select ADC1_3
    ADC1_3 OFFSET(3) NUMBITS(1) [
        /// Digital function selected on pin PF_6.
        DigitalFunctionSelectedOnPinPF_6 = 0,
        /// Analog function ADC1_3 selected on pin PF_6.
        AnalogFunctionADC1_3SelectedOnPinPF_6 = 1
    ],
    /// Select ADC1_4
    ADC1_4 OFFSET(4) NUMBITS(1) [
        /// Digital function selected on pin PF_5.
        DigitalFunctionSelectedOnPinPF_5 = 0,
        /// Analog function ADC1_4 selected on pin PF_5.
        AnalogFunctionADC1_4SelectedOnPinPF_5 = 1
    ],
    /// Select ADC1_5
    ADC1_5 OFFSET(5) NUMBITS(1) [
        /// Digital function selected on pin PF_11.
        DigitalFunctionSelectedOnPinPF_11 = 0,
        /// Analog function ADC1_5 selected on pin PF_11.
        AnalogFunctionADC1_5SelectedOnPinPF_11 = 1
    ],
    /// Select ADC1_6
    ADC1_6 OFFSET(6) NUMBITS(1) [
        /// Digital function selected on pin P7_7.
        DigitalFunctionSelectedOnPinP7_7 = 0,
        /// Analog function ADC1_6 selected on pin P7_7.
        AnalogFunctionADC1_6SelectedOnPinP7_7 = 1
    ],
    /// Select ADC1_7.
    ADC1_7 OFFSET(7) NUMBITS(1) [
        /// Digital function selected on pin PF_7.
        DigitalFunctionSelectedOnPinPF_7 = 0,
        /// Analog function ADC1_7 selected on pin PF_7.
        AnalogFunctionADC1_7SelectedOnPinPF_7 = 1
    ]
],
ENAIO2 [
    /// Select DAC
    DAC OFFSET(0) NUMBITS(1) [
        /// Digital function selected on pin P4_4.
        DigitalFunctionSelectedOnPinP4_4 = 0,
        /// Analog function DAC selected on pin P4_4.
        AnalogFunctionDACSelectedOnPinP4_4 = 1
    ],
    /// Select band gap output. To measure the band gap, disable the pull-up on pin PF_7
    BG OFFSET(4) NUMBITS(1) [
        /// Digital function selected on pin PF_7.
        DigitalFunctionSelectedOnPinPF_7 = 0,
        /// Band gap output selected for pin PF_7.
        BandGapOutputSelectedForPinPF_7 = 1
    ]
],
SDDELAY [
    /// SD/MMC sample delay. The delay value is SAMPLE_DELAY x 0.5 ns.
    SAMPLE_DELAY OFFSET(0) NUMBITS(4) [],
    /// SD/MMC drive delay. The delay value is DRV_DELAY x 0.5 ns. The values DRV_DELAY
    DRV_DELAY OFFSET(8) NUMBITS(4) []
],
PINTSEL0 [
    /// Pint interrupt 0: Select the pin number within the GPIO port selected by the POR
    INTPIN0 OFFSET(0) NUMBITS(5) [],
    /// Pin interrupt 0: Select the port for the pin number to be selected in the INTPIN
    PORTSEL0 OFFSET(5) NUMBITS(3) [
        /// GPIO Port 0
        GPIOPort0 = 0,
        /// GPIO Port 1
        GPIOPort1 = 1,
        /// GPIO Port 2
        GPIOPort2 = 2,
        /// GPIO Port 3
        GPIOPort3 = 3,
        /// GPIO Port 4
        GPIOPort4 = 4,
        /// GPIO Port 5
        GPIOPort5 = 5,
        /// GPIO Port 6
        GPIOPort6 = 6,
        /// GPIO Port 7
        GPIOPort7 = 7
    ],
    /// Pint interrupt 1: Select the pin number within the GPIO port selected by the POR
    INTPIN1 OFFSET(8) NUMBITS(5) [],
    /// Pin interrupt 1: Select the port for the pin number to be selected in the INTPIN
    PORTSEL1 OFFSET(13) NUMBITS(3) [
        /// GPIO Port 0
        GPIOPort0 = 0,
        /// GPIO Port 1
        GPIOPort1 = 1,
        /// GPIO Port 2
        GPIOPort2 = 2,
        /// GPIO Port 3
        GPIOPort3 = 3,
        /// GPIO Port 4
        GPIOPort4 = 4,
        /// GPIO Port 5
        GPIOPort5 = 5,
        /// GPIO Port 6
        GPIOPort6 = 6,
        /// GPIO Port 7
        GPIOPort7 = 7
    ],
    /// Pint interrupt 2: Select the pin number within the GPIO port selected by the POR
    INTPIN2 OFFSET(16) NUMBITS(5) [],
    /// Pin interrupt 2: Select the port for the pin number to be selected in the INTPIN
    PORTSEL2 OFFSET(21) NUMBITS(3) [
        /// GPIO Port 0
        GPIOPort0 = 0,
        /// GPIO Port 1
        GPIOPort1 = 1,
        /// GPIO Port 2
        GPIOPort2 = 2,
        /// GPIO Port 3
        GPIOPort3 = 3,
        /// GPIO Port 4
        GPIOPort4 = 4,
        /// GPIO Port 5
        GPIOPort5 = 5,
        /// GPIO Port 6
        GPIOPort6 = 6,
        /// GPIO Port 7
        GPIOPort7 = 7
    ],
    /// Pint interrupt 3: Select the pin number within the GPIO port selected by the POR
    INTPIN3 OFFSET(24) NUMBITS(5) [],
    /// Pin interrupt 3: Select the port for the pin number to be selected in the INTPIN
    PORTSEL3 OFFSET(29) NUMBITS(3) [
        /// GPIO Port 0
        GPIOPort0 = 0,
        /// GPIO Port 1
        GPIOPort1 = 1,
        /// GPIO Port 2
        GPIOPort2 = 2,
        /// GPIO Port 3
        GPIOPort3 = 3,
        /// GPIO Port 4
        GPIOPort4 = 4,
        /// GPIO Port 5
        GPIOPort5 = 5,
        /// GPIO Port 6
        GPIOPort6 = 6,
        /// GPIO Port 7
        GPIOPort7 = 7
    ]
],
PINTSEL1 [
    /// Pint interrupt 4: Select the pin number within the GPIO port selected by the POR
    INTPIN4 OFFSET(0) NUMBITS(5) [],
    /// Pin interrupt 4: Select the port for the pin number to be selected in the INTPIN
    PORTSEL4 OFFSET(5) NUMBITS(3) [
        /// GPIO Port 0
        GPIOPort0 = 0,
        /// GPIO Port 1
        GPIOPort1 = 1,
        /// GPIO Port 2
        GPIOPort2 = 2,
        /// GPIO Port 3
        GPIOPort3 = 3,
        /// GPIO Port 4
        GPIOPort4 = 4,
        /// GPIO Port 5
        GPIOPort5 = 5,
        /// GPIO Port 6
        GPIOPort6 = 6,
        /// GPIO Port 7
        GPIOPort7 = 7
    ],
    /// Pint interrupt 5: Select the pin number within the GPIO port selected by the POR
    INTPIN5 OFFSET(8) NUMBITS(5) [],
    /// Pin interrupt 5: Select the port for the pin number to be selected in the INTPIN
    PORTSEL5 OFFSET(13) NUMBITS(3) [
        /// GPIO Port 0
        GPIOPort0 = 0,
        /// GPIO Port 1
        GPIOPort1 = 1,
        /// GPIO Port 2
        GPIOPort2 = 2,
        /// GPIO Port 3
        GPIOPort3 = 3,
        /// GPIO Port 4
        GPIOPort4 = 4,
        /// GPIO Port 5
        GPIOPort5 = 5,
        /// GPIO Port 6
        GPIOPort6 = 6,
        /// GPIO Port 7
        GPIOPort7 = 7
    ],
    /// Pint interrupt 6: Select the pin number within the GPIO port selected by the POR
    INTPIN6 OFFSET(16) NUMBITS(5) [],
    /// Pin interrupt 6: Select the port for the pin number to be selected in the INTPIN
    PORTSEL6 OFFSET(21) NUMBITS(3) [
        /// GPIO Port 0
        GPIOPort0 = 0,
        /// GPIO Port 1
        GPIOPort1 = 1,
        /// GPIO Port 2
        GPIOPort2 = 2,
        /// GPIO Port 3
        GPIOPort3 = 3,
        /// GPIO Port 4
        GPIOPort4 = 4,
        /// GPIO Port 5
        GPIOPort5 = 5,
        /// GPIO Port 6
        GPIOPort6 = 6,
        /// GPIO Port 7
        GPIOPort7 = 7
    ],
    /// Pint interrupt 7: Select the pin number within the GPIO port selected by the POR
    INTPIN7 OFFSET(24) NUMBITS(5) [],
    /// Pin interrupt 7: Select the port for the pin number to be selected in the INTPIN
    PORTSEL7 OFFSET(29) NUMBITS(3) [
        /// GPIO Port 0
        GPIOPort0 = 0,
        /// GPIO Port 1
        GPIOPort1 = 1,
        /// GPIO Port 2
        GPIOPort2 = 2,
        /// GPIO Port 3
        GPIOPort3 = 3,
        /// GPIO Port 4
        GPIOPort4 = 4,
        /// GPIO Port 5
        GPIOPort5 = 5,
        /// GPIO Port 6
        GPIOPort6 = 6,
        /// GPIO Port 7
        GPIOPort7 = 7
    ]
]
];

pub const SCU_BASE: StaticRef<ScuRegisters> =
    unsafe { StaticRef::new(0x40086000 as *const ScuRegisters) };
