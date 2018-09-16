
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// Clock Control Unit (CCU2)
#[repr(C)]
struct Ccu2Registers {
/// Power mode register
pm: ReadWrite<u32>,
/// CCU base clocks status register
base_stat: ReadOnly<u32, BASE_STAT::Register>,
_reserved0: [u8; 248],
/// CLK_AUDIO clock configuration register
clk_audio_cfg: ReadWrite<u32, CLK_AUDIO_CFG::Register>,
/// CLK_AUDIO clock status register
clk_audio_stat: ReadOnly<u32, CLK_AUDIO_STAT::Register>,
_reserved1: [u8; 248],
/// CLK_APB2_USART3 clock configuration register
clk_apb2_usart3_cfg: ReadWrite<u32, CLK_APB2_USART3_CFG::Register>,
/// CLK_APB2_USART3 clock status register
clk_apb2_usart3_stat: ReadOnly<u32, CLK_APB2_USART3_STAT::Register>,
_reserved2: [u8; 248],
/// CLK_APB2_USART2 clock configuration register
clk_apb2_usart2_cfg: ReadWrite<u32, CLK_APB2_USART2_CFG::Register>,
/// CLK_APB2_USART clock status register
clk_apb2_usart2_stat: ReadOnly<u32, CLK_APB2_USART2_STAT::Register>,
_reserved3: [u8; 248],
/// CLK_APB2_UART1 clock configuration register
clk_apb0_uart1_bus_cfg: ReadWrite<u32, CLK_APB0_UART1_BUS_CFG::Register>,
/// CLK_APB0_UART1 clock status register
clk_apb0_uart1_stat: ReadOnly<u32, CLK_APB0_UART1_STAT::Register>,
_reserved4: [u8; 248],
/// CLK_APB2_USART0 clock configuration register
clk_apb0_usart0_cfg: ReadWrite<u32, CLK_APB0_USART0_CFG::Register>,
/// CLK_APB0_USART0 clock status register
clk_apb0_usart0_stat: ReadOnly<u32, CLK_APB0_USART0_STAT::Register>,
_reserved5: [u8; 248],
/// CLK_APB2_SSP1 clock configuration register
clk_apb2_ssp1_cfg: ReadWrite<u32, CLK_APB2_SSP1_CFG::Register>,
/// CLK_APB2_SSP1 clock status register
clk_apb2_ssp1_stat: ReadOnly<u32, CLK_APB2_SSP1_STAT::Register>,
_reserved6: [u8; 248],
/// CLK_APB0_SSP0 clock configuration register
clk_apb0_ssp0_cfg: ReadWrite<u32, CLK_APB0_SSP0_CFG::Register>,
/// CLK_APB0_SSP0 clock status register
clk_apb0_ssp0_stat: ReadOnly<u32, CLK_APB0_SSP0_STAT::Register>,
_reserved7: [u8; 248],
/// CLK_SDIO clock configuration register
clk_sdio_cfg: ReadWrite<u32, CLK_SDIO_CFG::Register>,
/// CLK_SDIO clock status register
clk_sdio_stat: ReadOnly<u32, CLK_SDIO_STAT::Register>,
}
register_bitfields![u32,
BASE_STAT [
    /// Base clock indicator for BASE_UART3_CLK 0 = All branch clocks switched off. 1 =
    BASE_UART3_CLK OFFSET(1) NUMBITS(1) [],
    /// Base clock indicator for BASE_UART2_CLK 0 = All branch clocks switched off. 1 =
    BASE_UART2_CLK OFFSET(2) NUMBITS(1) [],
    /// Base clock indicator for BASE_UART1_CLK 0 = All branch clocks switched off. 1 =
    BASE_UART1_CLK OFFSET(3) NUMBITS(1) [],
    /// Base clock indicator for BASE_UART0_CLK 0 = All branch clocks switched off. 1 =
    BASE_UART0_CLK OFFSET(4) NUMBITS(1) [],
    /// Base clock indicator for BASE_SSP1_CLK 0 = All branch clocks switched off. 1 = A
    BASE_SSP1_CLK OFFSET(5) NUMBITS(1) [],
    /// Base clock indicator for BASE_SSP0_CLK 0 = All branch clocks switched off. 1 = A
    BASE_SSP0_CLK OFFSET(6) NUMBITS(1) []
],
CLK_AUDIO_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_APB2_USART3_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_APB2_USART2_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_APB0_UART1_BUS_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_APB0_USART0_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_APB0_SSP0_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_APB2_SSP1_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_SDIO_CFG [
    /// Run enable
    RUN OFFSET(0) NUMBITS(1) [
        /// Clock is disabled.
        ClockIsDisabled = 0,
        /// Clock is enabled.
        ClockIsEnabled = 1
    ],
    /// Auto (AHB disable mechanism) enable
    AUTO OFFSET(1) NUMBITS(1) [
        /// Auto is disabled.
        AutoIsDisabled = 0,
        /// Auto is enabled.
        AutoIsEnabled = 1
    ],
    /// Wake-up mechanism enable
    WAKEUP OFFSET(2) NUMBITS(1) [
        /// Wake-up is disabled.
        WakeUpIsDisabled = 0,
        /// Wake-up is enabled.
        WakeUpIsEnabled = 1
    ]
],
CLK_AUDIO_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_APB2_USART3_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_APB2_USART2_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_APB0_UART1_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_APB0_USART0_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_APB2_SSP1_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_APB0_SSP0_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
],
CLK_SDIO_STAT [
    /// Run enable status 0 = clock is disabled. 1 = clock is enabled.
    RUN OFFSET(0) NUMBITS(1) [],
    /// Auto (AHB disable mechanism) enable status 0 = Auto is disabled. 1 = Auto is ena
    AUTO OFFSET(1) NUMBITS(1) [],
    /// Wake-up mechanism enable status 0 = Wake-up is disabled. 1 = Wake-up is enabled.
    WAKEUP OFFSET(2) NUMBITS(1) []
]
];
const CCU2_BASE: StaticRef<Ccu2Registers> =
    unsafe { StaticRef::new(0x40052000 as *const Ccu2Registers) };
