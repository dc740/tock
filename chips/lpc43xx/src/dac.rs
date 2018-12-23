
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly, register_bitfields};
    /// Digital-to-Analog Converter (DAC)
#[repr(C)]
struct DacRegisters {
/// DAC register. Holds the conversion data.
cr: ReadWrite<u32, CR::Register>,
/// DAC control register.
ctrl: ReadWrite<u32, CTRL::Register>,
/// DAC counter value register.
cntval: ReadWrite<u32>,
}
register_bitfields![u32,
CR [
    /// After the selected settling time after this field is written with a new VALUE, t
    VALUE OFFSET(6) NUMBITS(10) [],
    /// Settling time
    BIAS OFFSET(16) NUMBITS(1) [
        /// The settling time of the DAC is 1 micros max, and the maximum current is 700 mic
        TheSettlingTimeOfTheDACIs1MicrosMaxAndTheMaximumCurrentIs700MicroA = 0,
        /// The settling time of the DAC is 2.5 micros and the maximum current is 350 microA
        TheSettlingTimeOfTheDACIs25MicrosAndTheMaximumCurrentIs350MicroA = 1
    ]
],
CTRL [
    /// DMA request
    INT_DMA_REQ OFFSET(0) NUMBITS(1) [
        /// This bit is cleared on any write to the DACR register.
        ThisBitIsClearedOnAnyWriteToTheDACRRegister = 0,
        /// This bit is set by hardware when the timer times out.
        ThisBitIsSetByHardwareWhenTheTimerTimesOut = 1
    ],
    /// DMA double-buffering
    DBLBUF_ENA OFFSET(1) NUMBITS(1) [
        /// DACR double-buffering is disabled.
        DACRDoubleBufferingIsDisabled = 0,
        /// When this bit and the CNT_ENA bit are both set, the double-buffering feature in
        ENABLED = 1
    ],
    /// DMA time-out
    CNT_ENA OFFSET(2) NUMBITS(1) [
        /// Time-out counter operation is disabled.
        TimeOutCounterOperationIsDisabled = 0,
        /// Time-out counter operation is enabled.
        TimeOutCounterOperationIsEnabled = 1
    ],
    /// DMA enable
    DMA_ENA OFFSET(3) NUMBITS(1) [
        /// DMA access is disabled.
        DMAAccessIsDisabled = 0,
        /// DMA Burst Request Input 15 is enabled for the DAC (see Table 136).
        DMABurstRequestInput15IsEnabledForTheDACSeeTable136 = 1
    ]
]
];
const DAC_BASE: StaticRef<DacRegisters> =
    unsafe { StaticRef::new(0x400E1000 as *const DacRegisters) };
