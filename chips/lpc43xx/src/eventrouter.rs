
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly, register_bitfields};
    /// Event router
#[repr(C)]
struct EventrouterRegisters {
/// Level configuration register
hilo: ReadWrite<u32, HILO::Register>,
/// Edge configuration
edge: ReadWrite<u32, EDGE::Register>,
_reserved0: [u8; 4048],
/// Clear event enable register
clr_en: WriteOnly<u32, CLR_EN::Register>,
/// Set event enable register
set_en: WriteOnly<u32, SET_EN::Register>,
/// Event Status register
status: ReadOnly<u32, STATUS::Register>,
/// Event Enable register
enable: ReadOnly<u32, ENABLE::Register>,
/// Clear event status register
clr_stat: WriteOnly<u32, CLR_STAT::Register>,
/// Set event status register
set_stat: WriteOnly<u32, SET_STAT::Register>,
}
register_bitfields![u32,
HILO [
    /// Level detect mode for WAKEUP0 event.
    WAKEUP0_L OFFSET(0) NUMBITS(1) [
        /// Detect LOW level on the WAKEUP0 pin if bit 0 in the EDGE register is 0. Detect f
        DETECT_LOW_LEVEL = 0,
        /// Detect HIGH level on the WAKEUP0 pin if bit 0 in the EDGE register is 0. Detect
        DETECT_HIGH_LEVEL = 1
    ],
    /// Level detect mode for WAKEUP1 event. The corresponding bit in the EDGE register
    WAKEUP1_L OFFSET(1) NUMBITS(1) [
        /// Detect LOW level on the WAKEUP1 pin if bit 1 in the EDGE register is 0.
        DetectLOWLevelOnTheWAKEUP1PinIfBit1InTheEDGERegisterIs0 = 0,
        /// Detect HIGH level on the WAKEUP1 pin if bit 1 in the EDGE register is 0. Detect
        DETECT_HIGH_LEVEL = 1
    ],
    /// Level detect mode for WAKEUP2 event.
    WAKEUP2_L OFFSET(2) NUMBITS(1) [
        /// Detect LOW level on the WAKEUP2 pin if bit 2 in the EDGE register is 0. Detect f
        DETECT_LOW_LEVEL = 0,
        /// Detect HIGH level on the WAKEUP2 pin if bit 2 in the EDGE register is 0. Detect
        DETECT_HIGH_LEVEL = 1
    ],
    /// Level detect mode for WAKEUP3 event.
    WAKEUP3_L OFFSET(3) NUMBITS(1) [
        /// Detect LOW level on the WAKEUP3 pin if bit 3 in the EDGE register is 0. Detect f
        DETECT_LOW_LEVEL = 0,
        /// Detect HIGH level on the WAKEUP3 pin if bit 3 in the EDGE register is 0. Detect
        DETECT_HIGH_LEVEL = 1
    ],
    /// Level detect mode for alarm timer event.
    ATIMER_L OFFSET(4) NUMBITS(1) [
        /// Detect LOW level of the alarm timer interrupt if bit 4 in the EDGE register is 0
        DETECT_LOW_LEVEL = 0,
        /// Detect HIGH level of the alarm timer interrupt if bit 4 in the EDGE register is
        DETECT_HIGH_LEVEL = 1
    ],
    /// Level detect mode for RTC event.
    RTC_L OFFSET(5) NUMBITS(1) [
        /// Detect LOW level of the RTC interrupt if bit 5 in the EDGE register is 0. Detect
        DETECT_LOW_LEVEL = 0,
        /// Detect HIGH level of the RTC interrupt if bit 5 in the EDGE register is 0. Detec
        DETECT_HIGH_LEVEL = 1
    ],
    /// Level detect mode for BOD event.
    BOD_L OFFSET(6) NUMBITS(1) [
        /// Detect LOW level of the BOD interrupt if bit 6 in the EDGE register is 0. Detect
        DETECT_LOW_LEVEL = 0,
        /// Detect HIGH level of the BOD interrupt if bit 6 in the EDGE register is 0. Detec
        DETECT_HIGH_LEVEL = 1
    ],
    /// Level detect mode for WWDT event.
    WWDT_L OFFSET(7) NUMBITS(1) [
        /// Detect LOW level of the WWDT interrupt if bit 7 in the EDGE register is 0. Detec
        DETECT_LOW_LEVEL = 0,
        /// Detect HIGH level of the WWDT interrupt if bit 7 in the EDGE register is 0. Dete
        DETECT_HIGH_LEVEL = 1
    ],
    /// Level detect mode for Ethernet event
    ETH_L OFFSET(8) NUMBITS(1) [
        /// Detect LOW level of the Ethernet interrupt if bit 8 in the EDGE register is 0. D
        DETECT_LOW_LEVEL = 0,
        /// Detect HIGH level of the Ethernet interrupt if bit 8 in the EDGE register is 0.
        DETECT_HIGH_LEVEL = 1
    ],
    /// Level detect mode for USB0 event
    USB0_L OFFSET(9) NUMBITS(1) [
        /// Detect LOW level of the USB0 interrupt if bit 9 in the EDGE register is 0. Detec
        DETECT_LOW_LEVEL = 0,
        /// Detect HIGH level of the USB0 interrupt if bit 9 in the EDGE register is 0. Dete
        DETECT_HIGH_LEVEL = 1
    ],
    /// Level detect mode for USB1 event
    USB1_L OFFSET(10) NUMBITS(1) [
        /// Detect LOW level of the USB1 interrupt if bit 10 in the EDGE register is 0. Dete
        DETECT_LOW_LEVEL = 0,
        /// Detect HIGH level of the USB1 interrupt if bit 10 in the EDGE register is 0. Det
        DETECT_HIGH_LEVEL = 1
    ],
    /// Level detect mode for SD/MMC event
    SDMMC_L OFFSET(11) NUMBITS(1) [
        /// Detect LOW level of the SD/MMC interrupt if bit 11 in the EDGE register is 0. De
        DETECT_LOW_LEVEL = 0,
        /// Detect HIGH level of the SD/MMC interrupt if bit 11 in the EDGE register is 0. D
        DETECT_HIGH_LEVEL = 1
    ],
    /// Level detect mode for C_CAN event.
    CAN_L OFFSET(12) NUMBITS(1) [
        /// Detect LOW level of the combined C_CAN interrupt if bit 12 in the EDGE register
        DETECT_LOW_LEVEL = 0,
        /// Detect HIGH level of the combined C_CAN interrupt if bit 12 in the EDGE register
        DETECT_HIGH_LEVEL = 1
    ],
    /// Level detect mode for combined timer output 2 event.
    TIM2_L OFFSET(13) NUMBITS(1) [
        /// Detect LOW level GIMA output 25 if bit 13 in the EDGE register is 0. Detect fall
        DETECT_LOW_LEVEL = 0,
        /// Detect HIGH level GIMA output 25 if bit 13 in the EDGE register is 0. Detect ris
        DETECT_HIGH_LEVEL = 1
    ],
    /// Level detect mode for combined timer output 6 event.
    TIM6_L OFFSET(14) NUMBITS(1) [
        /// Detect LOW level of GIMA output 26 if bit 14 in the EDGE register is 0. Detect f
        DETECT_LOW_LEVEL = 0,
        /// Detect HIGH level of GIMA output 26 if bit 14 in the EDGE register is 0. Detect
        DETECT_HIGH_LEVEL = 1
    ],
    /// Level detect mode for QEI event.
    QEI_L OFFSET(15) NUMBITS(1) [
        /// Detect LOW level of the QEI interrupt if bit 15 in the EDGE register is 0. Detec
        DETECT_LOW_LEVEL = 0,
        /// Detect HIGH level of the QEI interrupt if bit 15 in the EDGE register is 0. Dete
        DETECT_HIGH_LEVEL = 1
    ],
    /// Level detect mode for combined timer output 14 event.
    TIM14_L OFFSET(16) NUMBITS(1) [
        /// Detect LOW level of GIMA output 27 if bit 16 in the EDGE register is 0. Detect f
        DETECT_LOW_LEVEL = 0,
        /// Detect HIGH level of GIMA output 27 if bit 16 in the EDGE register is 0. Detect
        DETECT_HIGH_LEVEL = 1
    ],
    /// Level detect mode for Reset
    RESET_L OFFSET(19) NUMBITS(1) [
        /// Detect LOW level if bit 17 in the EDGE register is 0. Detect falling edge if bit
        DETECT_LOW_LEVEL_IF = 0,
        /// Detect HIGH level if bit 17 in the EDGE register is 0. Detect rising edge if bit
        DETECT_HIGH_LEVEL_IF = 1
    ],
    /// Level detect mode for BOD Reset
    BODRESET_L OFFSET(20) NUMBITS(1) [
        /// Detect LOW level if bit 20 in the EDGE register is 0. Detect falling edge if bit
        DETECT_LOW_LEVEL_IF = 0,
        /// Detect HIGH level if bit 20 in the EDGE register is 0. Detect rising edge if bit
        DETECT_HIGH_LEVEL_IF = 1
    ],
    /// Level detect mode for Deep power-down Reset
    DPDRESET_L OFFSET(21) NUMBITS(1) [
        /// Detect LOW level if bit 21 in the EDGE register is 0. Detect falling edge if bit
        DETECT_LOW_LEVEL_IF = 0,
        /// Detect HIGH level if bit 21 in the EDGE register is 0. Detect rising edge if bit
        DETECT_HIGH_LEVEL_IF = 1
    ]
],
EDGE [
    /// Edge detect mode for WAKEUP0 event. The corresponding bit in the EDGE register m
    WAKEUP0_E OFFSET(0) NUMBITS(1) [
        /// Level detect.
        LevelDetect = 0,
        /// Edge detect of WAKEUP0 pin. Detect falling edge if bit 0 in the HILO register is
        EDGE_DETECT_OF_WAKEU = 1
    ],
    /// Edge/level detect mode for WAKEUP1 event. The corresponding bit in the EDGE regi
    WAKEUP1_E OFFSET(1) NUMBITS(1) [
        /// Level detect.
        LevelDetect = 0,
        /// Edge detect of WAKEUP1 pin. Detect falling edge if bit 1 in the HILO register is
        EDGE_DETECT_OF_WAKEU = 1
    ],
    /// Edge/level detect mode for WAKEUP2 event. The corresponding bit in the EDGE regi
    WAKEUP2_E OFFSET(2) NUMBITS(1) [
        /// Level detect.
        LevelDetect = 0,
        /// Edge detect of WAKEUP2 pin. Detect falling edge if bit 2 in the HILO register is
        EDGE_DETECT_OF_WAKEU = 1
    ],
    /// Edge/level detect mode for WAKEUP3 event. The corresponding bit in the EDGE regi
    WAKEUP3_E OFFSET(3) NUMBITS(1) [
        /// Level detect.
        LevelDetect = 0,
        /// Edge detect of WAKEUP3 pin. Detect falling edge if bit 30 in the HILO register i
        EDGE_DETECT_OF_WAKEU = 1
    ],
    /// Edge/level detect mode for alarm timer event. The corresponding bit in the EDGE
    ATIMER_E OFFSET(4) NUMBITS(1) [
        /// Level detect.
        LevelDetect = 0,
        /// Edge detect of the alarm timer interrupt. Detect falling edge if bit 4 in the HI
        EDGE_DETECT_OF_THE_A = 1
    ],
    /// Edge/level detect mode for RTC event. The corresponding bit in the EDGE register
    RTC_E OFFSET(5) NUMBITS(1) [
        /// Level detect.
        LevelDetect = 0,
        /// Edge detect of the RTC interrupt. Detect falling edge if bit 5 in the HILO regis
        EDGE_DETECT_OF_THE_R = 1
    ],
    /// Edge/level detect mode for BOD event. The corresponding bit in the EDGE register
    BOD_E OFFSET(6) NUMBITS(1) [
        /// Level detect.
        LevelDetect = 0,
        /// Edge detect of the BOD interrupt. Detect falling edge if bit 6 in the HILO regis
        EDGE_DETECT_OF_THE_B = 1
    ],
    /// Edge/level detect mode for WWDTD event. The corresponding bit in the EDGE regist
    WWDT_E OFFSET(7) NUMBITS(1) [
        /// Level detect.
        LevelDetect = 0,
        /// Edge detect of the WWDT interrupt. Detect falling edge if bit 7 in the HILO regi
        EDGE_DETECT_OF_THE_W = 1
    ],
    /// Edge/level detect mode for ethernet event. The corresponding bit in the EDGE reg
    ETH_E OFFSET(8) NUMBITS(1) [
        /// Level detect.
        LevelDetect = 0,
        /// Edge detect of the Ethernet interrupt. Detect falling edge if bit 8 in the HILO
        EDGE_DETECT_OF_THE_E = 1
    ],
    /// Edge/level detect mode for USB0 event. The corresponding bit in the EDGE registe
    USB0_E OFFSET(9) NUMBITS(1) [
        /// Level detect.
        LevelDetect = 0,
        /// Edge detect of the USB0 event. Detect falling edge if bit 9 in the HILO register
        EDGE_DETECT_OF_THE_U = 1
    ],
    /// Edge/level detect mode for USB1 event. The corresponding bit in the EDGE registe
    USB1_E OFFSET(10) NUMBITS(1) [
        /// Level detect.
        LevelDetect = 0,
        /// Edge detect of the USB1 interrupt. Detect falling edge if bit 10 in the HILO reg
        EDGE_DETECT_OF_THE_U = 1
    ],
    /// Edge/level detect mode for SD/MMC event.The corresponding bit in the EDGE regist
    SDMMC_E OFFSET(11) NUMBITS(1) [
        /// Level detect.
        LevelDetect = 0,
        /// Edge detect of the SD/MMC interrupt. Detect falling edge if bit 10 in the HILO r
        EDGE_DETECT_OF_THE_S = 1
    ],
    /// Edge/level detect mode for C_CAN event. The corresponding bit in the EDGE regist
    CAN_E OFFSET(12) NUMBITS(1) [
        /// Level detect.
        LevelDetect = 0,
        /// Edge detect of the combined C_CAN interrupt. Detect falling edge if bit 12 in th
        EDGE_DETECT_OF_THE_C = 1
    ],
    /// Edge/level detect mode for combined timer output 2 event. The corresponding bit
    TIM2_E OFFSET(13) NUMBITS(1) [
        /// Level detect.
        LevelDetect = 0,
        /// Edge detect of GIMA output 25. Detect falling edge if bit 13 in the HILO registe
        EDGE_DETECT_OF_GIMA = 1
    ],
    /// Edge/level detect mode for combined timer output 6 event. The corresponding bit
    TIM6_E OFFSET(14) NUMBITS(1) [
        /// Level detect.
        LevelDetect = 0,
        /// Edge detect of GIMA output 26. Detect falling edge if bit 14 in the HILO registe
        EDGE_DETECT_OF_GIMA = 1
    ],
    /// Edge/level detect mode for QEI interrupt signal. The corresponding bit in the ED
    QEI_E OFFSET(15) NUMBITS(1) [
        /// Level detect.
        LevelDetect = 0,
        /// Edge detect of QEI interrupt. Detect falling edge if bit 15 in the HILO register
        EDGE_DETECT_OF_QEI_I = 1
    ],
    /// Edge/level detect mode for combined timer output 14 event. The corresponding bit
    TIM14_E OFFSET(16) NUMBITS(1) [
        /// Level detect.
        LevelDetect = 0,
        /// Edge detect of GIMA output 27. Detect falling edge if bit 16 in the HILO registe
        EDGE_DETECT_OF_GIMA = 1
    ],
    /// Edge/level detect mode for Reset. The corresponding bit in the EDGE register mus
    RESET_E OFFSET(19) NUMBITS(1) [
        /// Level detect.
        LevelDetect = 0,
        /// Edge detect of the reset signal. Detect falling edge if bit 19 in the HILO regis
        EDGE_DETECT_OF_THE_R = 1
    ],
    /// Edge detect of the BOD reset signal. The corresponding bit in the EDGE register
    BODRESET_E OFFSET(20) NUMBITS(1) [
        /// Level detect.
        LevelDetect = 0,
        /// Edge detect of the reset signal. Detect falling edge if bit 20 in the HILO regis
        EDGE_DETECT_OF_THE_R = 1
    ],
    /// Edge detect of the deep power-down reset signal. The corresponding bit in the ED
    DPDRESET_E OFFSET(21) NUMBITS(1) [
        /// Level detect.
        LevelDetect = 0,
        /// Edge detect of the reset signal. Detect falling edge if bit 21 in the HILO regis
        EDGE_DETECT_OF_THE_R = 1
    ]
],
CLR_EN [
    /// Writing a 1 to this bit clears the event enable bit 0 in the ENABLE register.
    WAKEUP0_CLREN OFFSET(0) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the event enable bit 1 in the ENABLE register.
    WAKEUP1_CLREN OFFSET(1) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the event enable bit 2 in the ENABLE register.
    WAKEUP2_CLREN OFFSET(2) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the event enable bit 3 in the ENABLE register.
    WAKEUP3_CLREN OFFSET(3) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the event enable bit 4 in the ENABLE register.
    ATIMER_CLREN OFFSET(4) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the event enable bit 5 in the ENABLE register.
    RTC_CLREN OFFSET(5) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the event enable bit 6 in the ENABLE register.
    BOD_CLREN OFFSET(6) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the event enable bit 7 in the ENABLE register.
    WWDT_CLREN OFFSET(7) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the event enable bit 8 in the ENABLE register.
    ETH_CLREN OFFSET(8) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the event enable bit 9 in the ENABLE register.
    USB0_CLREN OFFSET(9) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the event enable bit 10 in the ENABLE register.
    USB1_CLREN OFFSET(10) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the event enable bit 11 in the ENABLE register.
    SDMMC_CLREN OFFSET(11) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the event enable bit 12 in the ENABLE register.
    CAN_CLREN OFFSET(12) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the event enable bit 13 in the ENABLE register.
    TIM2_CLREN OFFSET(13) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the event enable bit 14 in the ENABLE register.
    TIM6_CLREN OFFSET(14) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the event enable bit 15 in the ENABLE register.
    QEI_CLREN OFFSET(15) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the event enable bit 16 in the ENABLE register.
    TIM14_CLREN OFFSET(16) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the event enable bit 19 in the ENABLE register.
    RESET_CLREN OFFSET(19) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the event enable bit 20 in the ENABLE register.
    BODRESET_CLREN OFFSET(20) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the event enable bit 21 in the ENABLE register.
    DPDRESET_CLREN OFFSET(21) NUMBITS(1) []
],
SET_EN [
    /// Writing a 1 to this bit sets the event enable bit 0 in the ENABLE register.
    WAKEUP0_SETEN OFFSET(0) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the event enable bit 1 in the ENABLE register.
    WAKEUP1_SETEN OFFSET(1) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the event enable bit 2 in the ENABLE register.
    WAKEUP2_SETEN OFFSET(2) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the event enable bit 3 in the ENABLE register.
    WAKEUP3_SETEN OFFSET(3) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the event enable bit 4 in the ENABLE register.
    ATIMER_SETEN OFFSET(4) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the event enable bit 5 in the ENABLE register.
    RTC_SETEN OFFSET(5) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the event enable bit 6 in the ENABLE register.
    BOD_SETEN OFFSET(6) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the event enable bit 7 in the ENABLE register.
    WWDT_SETEN OFFSET(7) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the event enable bit 8 in the ENABLE register.
    ETH_SETEN OFFSET(8) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the event enable bit 9 in the ENABLE register.
    USB0_SETEN OFFSET(9) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the event enable bit 10 in the ENABLE register.
    USB1_SETEN OFFSET(10) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the event enable bit 11 in the ENABLE register.
    SDMMC_SETEN OFFSET(11) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the event enable bit 12 in the ENABLE register.
    CAN_SETEN OFFSET(12) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the event enable bit 13 in the ENABLE register.
    TIM2_SETEN OFFSET(13) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the event enable bit 14 in the ENABLE register.
    TIM6_SETEN OFFSET(14) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the event enable bit 15 in the ENABLE register.
    QEI_SETEN OFFSET(15) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the event enable bit 16 in the ENABLE register.
    TIM14_SETEN OFFSET(16) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the event enable bit 19 in the ENABLE register.
    RESET_SETEN OFFSET(19) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the event enable bit 20 in the ENABLE register.
    BODRESET_SETEN OFFSET(20) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the event enable bit 21 in the ENABLE register.
    DPDRESET_SETEN OFFSET(21) NUMBITS(1) []
],
STATUS [
    /// A 1 in this bit shows that the WAKEUP0 event has been raised.
    WAKEUP0_ST OFFSET(0) NUMBITS(1) [],
    /// A 1 in this bit shows that the WAKEUP1 event has been raised.
    WAKEUP1_ST OFFSET(1) NUMBITS(1) [],
    /// A 1 in this bit shows that the WAKEUP2 event has been raised.
    WAKEUP2_ST OFFSET(2) NUMBITS(1) [],
    /// A 1 in this bit shows that the WAKEUP3 event has been raised.
    WAKEUP3_ST OFFSET(3) NUMBITS(1) [],
    /// A 1 in this bit shows that the ATIMER event has been raised.
    ATIMER_ST OFFSET(4) NUMBITS(1) [],
    /// A 1 in this bit shows that the RTC event has been raised.
    RTC_ST OFFSET(5) NUMBITS(1) [],
    /// A 1 in this bit shows that the BOD event has been raised.
    BOD_ST OFFSET(6) NUMBITS(1) [],
    /// A 1 in this bit shows that the WWDT event has been raised.
    WWDT_ST OFFSET(7) NUMBITS(1) [],
    /// A 1 in this bit shows that the ETHERNET event has been raised.
    ETH_ST OFFSET(8) NUMBITS(1) [],
    /// A 1 in this bit shows that the USB0 event has been raised.
    USB0_ST OFFSET(9) NUMBITS(1) [],
    /// A 1 in this bit shows that the USB1 event has been raised.
    USB1_ST OFFSET(10) NUMBITS(1) [],
    /// A 1 in this bit indicates that the SDMMC event has been raised.
    SDMMC_ST OFFSET(11) NUMBITS(1) [],
    /// A 1 in this bit shows that the C_CAN event has been raised.
    CAN_ST OFFSET(12) NUMBITS(1) [],
    /// A 1 in this bit shows that the combined timer 2 output event has been raised.
    TIM2_ST OFFSET(13) NUMBITS(1) [],
    /// A 1 in this bit shows that the combined timer 6 output event has been raised.
    TIM6_ST OFFSET(14) NUMBITS(1) [],
    /// A 1 in this bit shows that the QEI event has been raised.
    QEI_ST OFFSET(15) NUMBITS(1) [],
    /// A 1 in this bit shows that the combined timer 14 output event has been raised.
    TIM14_ST OFFSET(16) NUMBITS(1) [],
    /// A 1 in this bit shows that the reset event has been raised.
    RESET_ST OFFSET(19) NUMBITS(1) [],
    /// A 1 in this bit indicates that the reset event has been raised.
    BODRESET_ST OFFSET(20) NUMBITS(1) [],
    /// A 1 in this bit indicates that the reset event has been raised.
    DPDRESET_ST OFFSET(21) NUMBITS(1) []
],
ENABLE [
    /// A 1 in this bit shows that the WAKEUP0 event has been enabled. This event wakes
    WAKEUP0_EN OFFSET(0) NUMBITS(1) [],
    /// A 1 in this bit shows that the WAKEUP1 event has been enabled. This event wakes
    WAKEUP1_EN OFFSET(1) NUMBITS(1) [],
    /// A 1 in this bit shows that the WAKEUP2 event has been enabled. This event wakes
    WAKEUP2_EN OFFSET(2) NUMBITS(1) [],
    /// A 1 in this bit shows that the WAKEUP3 event has been enabled. This event wakes
    WAKEUP3_EN OFFSET(3) NUMBITS(1) [],
    /// A 1 in this bit shows that the ATIMER event has been enabled. This event wakes u
    ATIMER_EN OFFSET(4) NUMBITS(1) [],
    /// A 1 in this bit shows that the RTC event has been enabled. This event wakes up t
    RTC_EN OFFSET(5) NUMBITS(1) [],
    /// A 1 in this bit shows that the BOD event has been enabled. This event wakes up t
    BOD_EN OFFSET(6) NUMBITS(1) [],
    /// A 1 in this bit shows that the WWDT event has been enabled. This event wakes up
    WWDT_EN OFFSET(7) NUMBITS(1) [],
    /// A 1 in this bit shows that the ETHERNET event has been enabled. This event wakes
    ETH_EN OFFSET(8) NUMBITS(1) [],
    /// A 1 in this bit shows that the USB0 event has been enabled. This event wakes up
    USB0_EN OFFSET(9) NUMBITS(1) [],
    /// A 1 in this bit shows that the USB1 event has been enabled. This event wakes up
    USB1_EN OFFSET(10) NUMBITS(1) [],
    /// A 1 in this bit indicates that the SDMMC event has been enabled. This event wake
    SDMMC_EN OFFSET(11) NUMBITS(1) [],
    /// A 1 in this bit shows that the CAN event has been enabled. This event wakes up t
    CAN_EN OFFSET(12) NUMBITS(1) [],
    /// A 1 in this bit shows that the TIM2 event has been enabled. This event wakes up
    TIM2_EN OFFSET(13) NUMBITS(1) [],
    /// A 1 in this bit shows that the TIM6 event has been enabled. This event wakes up
    TIM6_EN OFFSET(14) NUMBITS(1) [],
    /// A 1 in this bit shows that the QEI event has been enabled. This event wakes up t
    QEI_EN OFFSET(15) NUMBITS(1) [],
    /// A 1 in this bit shows that the TIM14 event has been enabled. This event wakes up
    TIM14_EN OFFSET(16) NUMBITS(1) [],
    /// A 1 in this bit shows that the RESET event has been enabled. This event wakes up
    RESET_EN OFFSET(19) NUMBITS(1) [],
    /// A 1 in this bit indicates that the BOD RESET event has been enabled. This event
    BODRESET_EN OFFSET(20) NUMBITS(1) [],
    /// A 1 in this bit indicates that the deep power-down RESET event has been enabled.
    DPDRESET_EN OFFSET(21) NUMBITS(1) []
],
CLR_STAT [
    /// Writing a 1 to this bit clears the STATUS event bit 0 in the STATUS register.
    WAKEUP0_CLRST OFFSET(0) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the STATUS event bit 1 in the STATUS register.
    WAKEUP1_CLRST OFFSET(1) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the STATUS event bit 2 in the STATUS register.
    WAKEUP2_CLRST OFFSET(2) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the STATUS event bit 3 in the STATUS register.
    WAKEUP3_CLRST OFFSET(3) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the STATUS event bit 4 in the STATUS register.
    ATIMER_CLRST OFFSET(4) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the STATUS event bit 5 in the STATUS register.
    RTC_CLRST OFFSET(5) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the STATUS event bit 6 in the STATUS register.
    BOD_CLRST OFFSET(6) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the STATUS event bit 7 in the STATUS register.
    WWDT_CLRST OFFSET(7) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the STATUS event bit 8 in the STATUS register.
    ETH_CLRST OFFSET(8) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the STATUS event bit 9 in the STATUS register.
    USB0_CLRST OFFSET(9) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the STATUS event bit 10 in the STATUS register.
    USB1_CLRST OFFSET(10) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the STATUS event bit 11 in the STATUS register.
    SDMMC_CLRST OFFSET(11) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the STATUS event bit 12 in the STATUS register.
    CAN_CLRST OFFSET(12) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the STATUS event bit 13 in the STATUS register.
    TIM2_CLRST OFFSET(13) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the STATUS event bit 14 in the STATUS register.
    TIM6_CLRST OFFSET(14) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the STATUS event bit 15 in the STATUS register.
    QEI_CLRST OFFSET(15) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the STATUS event bit 16 in the STATUS register.
    TIM14_CLRST OFFSET(16) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the STATUS event bit 19 in the STATUS register.
    RESET_CLRST OFFSET(19) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the STATUS event bit 20 in the STATUS register.
    BODRESET_CLRST OFFSET(20) NUMBITS(1) [],
    /// Writing a 1 to this bit clears the STATUS event bit 21 in the STATUS register.
    DPDRESET_CLRST OFFSET(21) NUMBITS(1) []
],
SET_STAT [
    /// Writing a 1 to this bit sets the STATUS event bit 0 in the STATUS register.
    WAKEUP0_SETST OFFSET(0) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the STATUS event bit 1 in the STATUS register.
    WAKEUP1_SETST OFFSET(1) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the STATUS event bit 2 in the STATUS register.
    WAKEUP2_SETST OFFSET(2) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the STATUS event bit 3 in the STATUS register.
    WAKEUP3_SETST OFFSET(3) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the STATUS event bit 4 in the STATUS register.
    ATIMER_SETST OFFSET(4) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the STATUS event bit 5 in the STATUS register.
    RTC_SETST OFFSET(5) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the STATUS event bit 6 in the STATUS register.
    BOD_SETST OFFSET(6) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the STATUS event bit 7 in the STATUS register.
    WWDT_SETST OFFSET(7) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the STATUS event bit 8 in the STATUS register.
    ETH_SETST OFFSET(8) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the STATUS event bit 9 in the STATUS register.
    USB0_SETST OFFSET(9) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the STATUS event bit 10 in the STATUS register.
    USB1_SETST OFFSET(10) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the STATUS event bit 11 in the STATUS register.
    SDMMC_SETST OFFSET(11) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the STATUS event bit 12 in the STATUS register.
    CAN_SETST OFFSET(12) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the STATUS event bit 13 in the STATUS register.
    TIM2_SETST OFFSET(13) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the STATUS event bit 14 in the STATUS register.
    TIM6_SETST OFFSET(14) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the STATUS event bit 15 in the STATUS register.
    QEI_SETST OFFSET(15) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the STATUS event bit 16 in the STATUS register.
    TIM14_SETST OFFSET(16) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the STATUS event bit 19 in the STATUS register.
    RESET_SETST OFFSET(19) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the STATUS event bit 20 in the STATUS register.
    BODRESET_SETST OFFSET(20) NUMBITS(1) [],
    /// Writing a 1 to this bit sets the STATUS event bit 21 in the STATUS register.
    DPDRESET_SETST OFFSET(21) NUMBITS(1) []
]
];
const EVENTROUTER_BASE: StaticRef<EventrouterRegisters> =
    unsafe { StaticRef::new(0x40044000 as *const EventrouterRegisters) };
