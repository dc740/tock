
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly, register_bitfields};
    /// Global Input Multiplexer Array (GIMA)
#[repr(C)]
struct GimaRegisters {
/// Timer 0 CAP0_0 capture input multiplexer (GIMA output 0)
cap0_0_in: ReadWrite<u32, CAP0_0_IN::Register>,
/// Timer 0 CAP0_1 capture input multiplexer (GIMA output 1)
cap0_1_in: ReadWrite<u32, CAP0_1_IN::Register>,
/// Timer 0 CAP0_2 capture input multiplexer (GIMA output 2)
cap0_2_in: ReadWrite<u32, CAP0_2_IN::Register>,
/// Timer 0 CAP0_3 capture input multiplexer (GIMA output 3)
cap0_3_in: ReadWrite<u32, CAP0_3_IN::Register>,
/// Timer 1 CAP1_0 capture input multiplexer (GIMA output 4)
cap1_0_in: ReadWrite<u32, CAP1_0_IN::Register>,
/// Timer 1 CAP1_1 capture input multiplexer (GIMA output 5)
cap1_1_in: ReadWrite<u32, CAP1_1_IN::Register>,
/// Timer 1 CAP1_2 capture input multiplexer (GIMA output 6)
cap1_2_in: ReadWrite<u32, CAP1_2_IN::Register>,
/// Timer 1 CAP1_3 capture input multiplexer (GIMA output 7)
cap1_3_in: ReadWrite<u32, CAP1_3_IN::Register>,
/// Timer 2 CAP2_0 capture input multiplexer (GIMA output 8)
cap2_0_in: ReadWrite<u32, CAP2_0_IN::Register>,
/// Timer 2 CAP2_1 capture input multiplexer (GIMA output 9)
cap2_1_in: ReadWrite<u32, CAP2_1_IN::Register>,
/// Timer 2 CAP2_2 capture input multiplexer (GIMA output 10)
cap2_2_in: ReadWrite<u32, CAP2_2_IN::Register>,
/// Timer 2 CAP2_3 capture input multiplexer (GIMA output 11)
cap2_3_in: ReadWrite<u32, CAP2_3_IN::Register>,
/// Timer 3 CAP3_0 capture input multiplexer (GIMA output 12)
cap3_0_in: ReadWrite<u32, CAP3_0_IN::Register>,
/// Timer 3 CAP3_1 capture input multiplexer (GIMA output 13)
cap3_1_in: ReadWrite<u32, CAP3_1_IN::Register>,
/// Timer 3 CAP3_2 capture input multiplexer (GIMA output 14)
cap3_2_in: ReadWrite<u32, CAP3_2_IN::Register>,
/// Timer 3 CAP3_3 capture input multiplexer (GIMA output 15)
cap3_3_in: ReadWrite<u32, CAP3_3_IN::Register>,
/// SCT CTIN_0 capture input multiplexer (GIMA output 16)
ctin_0_in: ReadWrite<u32, CTIN_0_IN::Register>,
/// SCT CTIN_1 capture input multiplexer (GIMA output 17)
ctin_1_in: ReadWrite<u32, CTIN_1_IN::Register>,
/// SCT CTIN_2 capture input multiplexer (GIMA output 18)
ctin_2_in: ReadWrite<u32, CTIN_2_IN::Register>,
/// SCT CTIN_3 capture input multiplexer (GIMA output 19)
ctin_3_in: ReadWrite<u32, CTIN_3_IN::Register>,
/// SCT CTIN_4 capture input multiplexer (GIMA output 20)
ctin_4_in: ReadWrite<u32, CTIN_4_IN::Register>,
/// SCT CTIN_5 capture input multiplexer (GIMA output 21)
ctin_5_in: ReadWrite<u32, CTIN_5_IN::Register>,
/// SCT CTIN_6 capture input multiplexer (GIMA output 22)
ctin_6_in: ReadWrite<u32, CTIN_6_IN::Register>,
/// SCT CTIN_7 capture input multiplexer (GIMA output 23)
ctin_7_in: ReadWrite<u32, CTIN_7_IN::Register>,
/// ADCHS trigger input multiplexer (GIMA output 24)
adchs_trigger_in: ReadWrite<u32, ADCHS_TRIGGER_IN::Register>,
/// Event router   input 13 multiplexer (GIMA output 25)
eventrouter_13_in: ReadWrite<u32, EVENTROUTER_13_IN::Register>,
/// Event router   input 14 multiplexer (GIMA output 26)
eventrouter_14_in: ReadWrite<u32, EVENTROUTER_14_IN::Register>,
/// Event router   input 16 multiplexer (GIMA output 27)
eventrouter_16_in: ReadWrite<u32, EVENTROUTER_16_IN::Register>,
/// ADC start0   input   multiplexer (GIMA output 28)
adcstart0_in: ReadWrite<u32, ADCSTART0_IN::Register>,
/// ADC start1 input   multiplexer (GIMA output 29)
adcstart1_in: ReadWrite<u32, ADCSTART1_IN::Register>,
}
register_bitfields![u32,
CAP0_0_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x3 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTIN_0
        CTIN_0 = 0,
        /// SGPIO3
        SGPIO3 = 1,
        /// T0_CAP0
        T0_CAP0 = 2
    ]
],
CAP0_1_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x3 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTIN_1
        CTIN_1 = 0,
        /// USART2 TX active
        USART2TXActive = 1,
        /// T0_CAP1
        T0_CAP1 = 2
    ]
],
CAP0_2_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x3 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTIN_2
        CTIN_2 = 0,
        /// SGPIO3_DIV
        SGPIO3_DIV = 1,
        /// T0_CAP2
        T0_CAP2 = 2
    ]
],
CAP0_3_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x3 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTOUT_15 or T3_MAT3
        CTOUT_15OrT3_MAT3 = 0,
        /// T0_CAP3
        T0_CAP3 = 1,
        /// T3_MAT3
        T3_MAT3 = 2
    ]
],
CAP1_0_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x3 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTIN_0
        CTIN_0 = 0,
        /// SGPIO12
        SGPIO12 = 1,
        /// T1_CAP0
        T1_CAP0 = 2
    ]
],
CAP1_1_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x3 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTIN_3
        CTIN_3 = 0,
        /// USART0 TX active
        USART0TXActive = 1,
        /// T1_CAP1
        T1_CAP1 = 2
    ]
],
CAP1_2_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x3 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTIN_4
        CTIN_4 = 0,
        /// USART0 RX active
        USART0RXActive = 1,
        /// T1_CAP2
        T1_CAP2 = 2
    ]
],
CAP1_3_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x3 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTOUT_3 or T0_MAT3
        CTOUT_3OrT0_MAT3 = 0,
        /// T1_CAP3
        T1_CAP3 = 1,
        /// T0_MAT3
        T0_MAT3 = 2
    ]
],
CAP2_0_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x4 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTIN_0
        CTIN_0 = 0,
        /// SGPIO12_DIV
        SGPIO12_DIV = 1,
        /// T2_CAP0
        T2_CAP0 = 2
    ]
],
CAP2_1_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x4 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTIN_1
        CTIN_1 = 0,
        /// USART2 TX active
        USART2TXActive = 1,
        /// - I2S1_RX_MWS
        I2S1_RX_MWS = 2,
        /// T2_CAP1
        T2_CAP1 = 3
    ]
],
CAP2_2_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x4 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTIN_5
        CTIN_5 = 0,
        /// USART2 RX active
        USART2RXActive = 1,
        /// - I2S1_TX_MWS
        I2S1_TX_MWS = 2,
        /// T2_CAP2
        T2_CAP2 = 3
    ]
],
CAP2_3_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x3 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTOUT_7 or T1_MAT3
        CTOUT_7OrT1_MAT3 = 0,
        /// T2_CAP3
        T2_CAP3 = 1,
        /// T1_MAT3
        T1_MAT3 = 2
    ]
],
CAP3_0_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x3 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTIN_0
        CTIN_0 = 0,
        /// I2S0_RX_MWS
        I2S0_RX_MWS = 1,
        /// T3_CAP0
        T3_CAP0 = 2
    ]
],
CAP3_1_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x4 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTIN_6
        CTIN_6 = 0,
        /// USART3 TX active
        USART3TXActive = 1,
        /// TBD - I2S0_TX_MWS
        TBDI2S0_TX_MWS = 2,
        /// T3_CAP1
        T3_CAP1 = 3
    ]
],
CAP3_2_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x4 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTIN_7
        CTIN_7 = 0,
        /// USART3 RX active
        USART3RXActive = 1,
        /// SOF0 (Start-Of-Frame USB0)
        SOF0StartOfFrameUSB0 = 2,
        /// T3_CAP2
        T3_CAP2 = 3
    ]
],
CAP3_3_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x4 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTOUT11 or T2_MAT3
        CTOUT11OrT2_MAT3 = 0,
        /// SOF1 (Start-Of-Frame USB1)
        SOF1StartOfFrameUSB1 = 1,
        /// T3_CAP3
        T3_CAP3 = 2,
        /// T2_MAT3
        T2_MAT3 = 3
    ]
],
CTIN_0_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x3 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTIN_0
        CTIN_0 = 0,
        /// SGPIO3
        SGPIO3 = 1,
        /// SGPIO3_DIV
        SGPIO3_DIV = 2
    ]
],
CTIN_1_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x3 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTIN_1
        CTIN_1 = 0,
        /// USART2 TX active
        USART2TXActive = 1,
        /// SGPIO12
        SGPIO12 = 2
    ]
],
CTIN_2_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x3 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTIN_2
        CTIN_2 = 0,
        /// SGPIO12
        SGPIO12 = 1,
        /// SGPIO12_DIV
        SGPIO12_DIV = 2
    ]
],
CTIN_3_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x4 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTIN_3
        CTIN_3 = 0,
        /// USART0 TX active
        USART0TXActive = 1,
        /// Reserved
        Reserved = 2
    ]
],
CTIN_4_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x4 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTIN_4
        CTIN_4 = 0,
        /// USART0 RX active
        USART0RXActive = 1,
        /// - I2S1_RX_MWS1
        I2S1_RX_MWS1 = 2,
        /// - I2S1_TX_MWS1
        I2S1_TX_MWS1 = 3
    ]
],
CTIN_5_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x3 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTIN_5
        CTIN_5 = 0,
        /// USART2 RX active
        USART2RXActive = 1,
        /// SGPIO12_DIV
        SGPIO12_DIV = 2
    ]
],
CTIN_6_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x4 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTIN_6
        CTIN_6 = 0,
        /// USART3 TX active
        USART3TXActive = 1,
        /// I2S0_RX_MWS
        I2S0_RX_MWS = 2,
        /// I2S0_TX_MWS
        I2S0_TX_MWS = 3
    ]
],
CTIN_7_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x4 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTIN_7
        CTIN_7 = 0,
        /// USART3 RX active
        USART3RXActive = 1,
        /// SOF0 (Start-Of-Frame USB0)
        SOF0StartOfFrameUSB0 = 2,
        /// SOF1 (Start-Of-Frame USB1)
        SOF1StartOfFrameUSB1 = 3
    ]
],
ADCHS_TRIGGER_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0xA to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// GPIO6[28]
        GPIO628 = 0,
        /// GPIO5[3]
        GPIO53 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO12
        SGPIO12 = 3,
        /// Reserved
        Reserved = 4,
        /// MCOB2
        MCOB2 = 5,
        /// CTOUT_0 or T0_MAT0
        CTOUT_0OrT0_MAT0 = 6,
        /// CTOUT_8 or T2_MAT0
        CTOUT_8OrT2_MAT0 = 7,
        /// T0_MAT0
        T0_MAT0 = 8,
        /// T2_MAT0
        T2_MAT0 = 9
    ]
],
EVENTROUTER_13_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x3 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTOUT_2 or T0_MAT2
        CTOUT_2OrT0_MAT2 = 0,
        /// SGPIO3
        SGPIO3 = 1,
        /// T0_MAT2
        T0_MAT2 = 2
    ]
],
EVENTROUTER_14_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x3 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTOUT_6 or T1_MAT2
        CTOUT_6OrT1_MAT2 = 0,
        /// SGPIO12
        SGPIO12 = 1,
        /// T1_MAT2
        T1_MAT2 = 2
    ]
],
EVENTROUTER_16_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x2 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTOUT_14 or T3_MAT2
        CTOUT_14OrT3_MAT2 = 0,
        /// T3_MAT2
        T3_MAT2 = 1
    ]
],
ADCSTART0_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x2 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTOUT_15 or T3_MAT3
        CTOUT_15OrT3_MAT3 = 0,
        /// T0_MAT0
        T0_MAT0 = 1
    ]
],
ADCSTART1_IN [
    /// Invert input
    INV OFFSET(0) NUMBITS(1) [
        /// Not inverted.
        NotInverted = 0,
        /// Input inverted.
        InputInverted = 1
    ],
    /// Enable rising edge detection
    EDGE OFFSET(1) NUMBITS(1) [
        /// No edge detection.
        NoEdgeDetection = 0,
        /// Rising edge detection enabled.
        RisingEdgeDetectionEnabled = 1
    ],
    /// Enable   synchronization
    SYNCH OFFSET(2) NUMBITS(1) [
        /// Disable   synchronization.
        DisableSynchronization = 0,
        /// Enable   synchronization.
        EnableSynchronization = 1
    ],
    /// Enable single pulse generation.
    PULSE OFFSET(3) NUMBITS(1) [
        /// Disable single pulse generation.
        DisableSinglePulseGeneration = 0,
        /// Enable single pulse generation.
        EnableSinglePulseGeneration = 1
    ],
    /// Select input. Values 0x2 to 0xF are reserved.
    SELECT OFFSET(4) NUMBITS(4) [
        /// CTOUT_8 or T2_MAT0
        CTOUT_8OrT2_MAT0 = 0,
        /// T2_MAT0
        T2_MAT0 = 1
    ]
]
];
const GIMA_BASE: StaticRef<GimaRegisters> =
    unsafe { StaticRef::new(0x400C7000 as *const GimaRegisters) };
