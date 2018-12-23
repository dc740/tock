
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly, register_bitfields};
    /// Clock Generation Unit (CGU)
#[repr(C)]
struct CguRegisters {
_reserved0: [u8; 20],
/// Frequency monitor register
freq_mon: ReadWrite<u32, FREQ_MON::Register>,
/// Crystal oscillator control register
xtal_osc_ctrl: ReadWrite<u32, XTAL_OSC_CTRL::Register>,
/// PLL0USB status register
pll0usb_stat: ReadOnly<u32, PLL0USB_STAT::Register>,
/// PLL0USB control register
pll0usb_ctrl: ReadWrite<u32, PLL0USB_CTRL::Register>,
/// PLL0USB M-divider register
pll0usb_mdiv: ReadWrite<u32, PLL0USB_MDIV::Register>,
/// PLL0USB N/P-divider register
pll0usb_np_div: ReadWrite<u32, PLL0USB_NP_DIV::Register>,
/// PLL0AUDIO status register
pll0audio_stat: ReadOnly<u32, PLL0AUDIO_STAT::Register>,
/// PLL0AUDIO control register
pll0audio_ctrl: ReadWrite<u32, PLL0AUDIO_CTRL::Register>,
/// PLL0AUDIO M-divider register
pll0audio_mdiv: ReadWrite<u32>,
/// PLL0AUDIO N/P-divider register
pll0audio_np_div: ReadWrite<u32, PLL0AUDIO_NP_DIV::Register>,
/// PLL0AUDIO fractional divider register
pll0audio_frac: ReadWrite<u32>,
/// PLL1 status register
pll1_stat: ReadOnly<u32>,
/// PLL1 control register
pll1_ctrl: ReadWrite<u32, PLL1_CTRL::Register>,
/// Integer divider A control register
idiva_ctrl: ReadWrite<u32, IDIVA_CTRL::Register>,
/// Integer divider B control register
idivb_ctrl: ReadWrite<u32, IDIVB_CTRL::Register>,
/// Integer divider C control register
idivc_ctrl: ReadWrite<u32, IDIVC_CTRL::Register>,
/// Integer divider D control register
idivd_ctrl: ReadWrite<u32, IDIVD_CTRL::Register>,
/// Integer divider E control register
idive_ctrl: ReadWrite<u32, IDIVE_CTRL::Register>,
/// Output stage 0 control register for base clock BASE_SAFE_CLK
base_safe_clk: ReadOnly<u32, BASE_SAFE_CLK::Register>,
/// Output stage 1 control register for base clock BASE_USB0_CLK
base_usb0_clk: ReadWrite<u32, BASE_USB0_CLK::Register>,
/// Output stage 2 control register for base clock BASE_PERIPH_CLK
base_periph_clk: ReadWrite<u32, BASE_PERIPH_CLK::Register>,
/// Output stage 3 control register for base clock BASE_USB1_CLK
base_usb1_clk: ReadWrite<u32, BASE_USB1_CLK::Register>,
/// Output stage BASE_M4_CLK control register
base_m4_clk: ReadWrite<u32, BASE_M4_CLK::Register>,
/// Output stage BASE_SPIFI_CLK control register
base_spifi_clk: ReadWrite<u32, BASE_SPIFI_CLK::Register>,
/// Output stage BASE_SPI_CLK control register
base_spi_clk: ReadWrite<u32, BASE_SPI_CLK::Register>,
/// Output stage BASE_PHY_RX_CLK control register
base_phy_rx_clk: ReadWrite<u32, BASE_PHY_RX_CLK::Register>,
/// Output stage BASE_PHY_TX_CLK control register
base_phy_tx_clk: ReadWrite<u32, BASE_PHY_TX_CLK::Register>,
/// Output stage BASE_APB1_CLK control register
base_apb1_clk: ReadWrite<u32, BASE_APB1_CLK::Register>,
/// Output stage BASE_APB3_CLK control register
base_apb3_clk: ReadWrite<u32, BASE_APB3_CLK::Register>,
/// Output stage BASE_LCD_CLK control register
base_lcd_clk: ReadWrite<u32, BASE_LCD_CLK::Register>,
_reserved1: [u8; 4],
/// Output stage BASE_SDIO_CLK control register
base_sdio_clk: ReadWrite<u32, BASE_SDIO_CLK::Register>,
/// Output stage BASE_SSP0_CLK control register
base_ssp0_clk: ReadWrite<u32, BASE_SSP0_CLK::Register>,
/// Output stage BASE_SSP1_CLK control register
base_ssp1_clk: ReadWrite<u32, BASE_SSP1_CLK::Register>,
/// Output stage BASE_UART0_CLK control register
base_uart0_clk: ReadWrite<u32, BASE_UART0_CLK::Register>,
/// Output stage BASE_UART1_CLK control register
base_uart1_clk: ReadWrite<u32, BASE_UART1_CLK::Register>,
/// Output stage BASE_UART2_CLK control register
base_uart2_clk: ReadWrite<u32, BASE_UART2_CLK::Register>,
/// Output stage BASE_UART3_CLK control register
base_uart3_clk: ReadWrite<u32, BASE_UART3_CLK::Register>,
/// Output stage 20 control register for base clock BASE_OUT_CLK
base_out_clk: ReadWrite<u32, BASE_OUT_CLK::Register>,
_reserved2: [u8; 16],
/// Output stage 25 control register for base clock BASE_AUDIO_CLK
base_audio_clk: ReadWrite<u32, BASE_AUDIO_CLK::Register>,
/// Output stage 25 control register for base clock BASE_CGU_OUT0_CLK
base_cgu_out0_clk: ReadWrite<u32, BASE_CGU_OUT0_CLK::Register>,
/// Output stage 25 control register for base clock BASE_CGU_OUT1_CLK
base_cgu_out1_clk: ReadWrite<u32, BASE_CGU_OUT1_CLK::Register>,
}
register_bitfields![u32,
FREQ_MON [
    /// 9-bit reference clock-counter value
    RCNT OFFSET(0) NUMBITS(9) [],
    /// 14-bit selected clock-counter value
    FCNT OFFSET(9) NUMBITS(14) [],
    /// Measure frequency
    MEAS OFFSET(23) NUMBITS(1) [
        /// RCNT and FCNT disabled
        RCNTAndFCNTDisabled = 0,
        /// Frequency counters started
        FrequencyCountersStarted = 1
    ],
    /// Clock-source selection for the clock to be measured. All other values are reserv
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator (default)
        _32KHzOscillatorDefault = 0,
        /// IRC
        IRC = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Reserved
        Reserved = 5,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0USB
        PLL0USB = 7,
        /// PLL0AUDIO
        PLL0AUDIO = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
XTAL_OSC_CTRL [
    /// Oscillator-pad enable. Do not change the BYPASS and ENABLE bits in one write-act
    ENABLE OFFSET(0) NUMBITS(1) [
        /// Enable
        Enable = 0,
        /// Power-down (default)
        PowerDownDefault = 1
    ],
    /// Configure crystal operation or external-clock input pin XTAL1. Do not change the
    BYPASS OFFSET(1) NUMBITS(1) [
        /// Crystal. Operation with crystal connected (default).
        CrystalOperationWithCrystalConnectedDefault = 0,
        /// Bypass mode. Use this mode when an external clock source is used instead of a cr
        BypassModeUseThisModeWhenAnExternalClockSourceIsUsedInsteadOfACrystal = 1
    ],
    /// Select frequency range
    HF OFFSET(2) NUMBITS(1) [
        /// Low. Oscillator low-frequency mode (crystal or external clock source 1 to 20 MHz
        LOW = 0,
        /// High. Oscillator high-frequency mode; crystal or external clock source 15 to 25
        HIGH = 1
    ]
],
PLL0USB_STAT [
    /// PLL0 lock indicator
    LOCK OFFSET(0) NUMBITS(1) [],
    /// PLL0 free running indicator
    FR OFFSET(1) NUMBITS(1) []
],
PLL0USB_CTRL [
    /// PLL0 power down
    PD OFFSET(0) NUMBITS(1) [
        /// PLL0 enabled
        PLL0Enabled = 0,
        /// PLL0 powered down
        PLL0PoweredDown = 1
    ],
    /// Input clock bypass control
    BYPASS OFFSET(1) NUMBITS(1) [
        /// CCO clock sent to post-dividers. Use this in normal operation.
        CCOClockSentToPostDividersUseThisInNormalOperation = 0,
        /// PLL0 input clock sent to post-dividers (default).
        PLL0InputClockSentToPostDividersDefault = 1
    ],
    /// PLL0 direct input
    DIRECTI OFFSET(2) NUMBITS(1) [],
    /// PLL0 direct output
    DIRECTO OFFSET(3) NUMBITS(1) [],
    /// PLL0 clock enable
    CLKEN OFFSET(4) NUMBITS(1) [],
    /// Free running mode
    FRM OFFSET(6) NUMBITS(1) [],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
PLL0USB_MDIV [
    /// Decoded M-divider coefficient value. Select values for the M-divider between 1 a
    MDEC OFFSET(0) NUMBITS(17) [],
    /// Bandwidth select P value
    SELP OFFSET(17) NUMBITS(5) [],
    /// Bandwidth select I value
    SELI OFFSET(22) NUMBITS(6) [],
    /// Bandwidth select R value; SELR = 0.
    SELR OFFSET(28) NUMBITS(4) []
],
PLL0USB_NP_DIV [
    /// Decoded P-divider coefficient value
    PDEC OFFSET(0) NUMBITS(7) [],
    /// Decoded N-divider coefficient value
    NDEC OFFSET(12) NUMBITS(10) []
],
PLL0AUDIO_STAT [
    /// PLL0 lock indicator
    LOCK OFFSET(0) NUMBITS(1) [],
    /// PLL0 free running indicator
    FR OFFSET(1) NUMBITS(1) []
],
PLL0AUDIO_CTRL [
    /// PLL0 power down
    PD OFFSET(0) NUMBITS(1) [
        /// PLL0 enabled
        PLL0Enabled = 0,
        /// PLL0 powered down
        PLL0PoweredDown = 1
    ],
    /// Input clock bypass control
    BYPASS OFFSET(1) NUMBITS(1) [
        /// CCO clock sent to post-dividers. Use this in normal operation.
        CCOClockSentToPostDividersUseThisInNormalOperation = 0,
        /// PLL0 input clock sent to post-dividers (default).
        PLL0InputClockSentToPostDividersDefault = 1
    ],
    /// PLL0 direct input
    DIRECTI OFFSET(2) NUMBITS(1) [],
    /// PLL0 direct output
    DIRECTO OFFSET(3) NUMBITS(1) [],
    /// PLL0 clock enable
    CLKEN OFFSET(4) NUMBITS(1) [],
    /// Free running mode
    FRM OFFSET(6) NUMBITS(1) [],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Fractional PLL word write request. Set this bit to 1 if the fractional divider i
    PLLFRACT_REQ OFFSET(12) NUMBITS(1) [],
    /// Select fractional divider.
    SEL_EXT OFFSET(13) NUMBITS(1) [
        /// FRAC Enabled. Enable fractional divider.
        FRACEnabledEnableFractionalDivider = 0,
        /// MDEC enabled. Fractional divider not used.
        MDECEnabledFractionalDividerNotUsed = 1
    ],
    /// Sigma-Delta modulator power-down
    MOD_PD OFFSET(14) NUMBITS(1) [
        /// Enabled. Sigma-Delta modulator enabled
        EnabledSigmaDeltaModulatorEnabled = 0,
        /// Disabled. Sigma-Delta modulator powered down
        DisabledSigmaDeltaModulatorPoweredDown = 1
    ],
    /// Clock source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
PLL0AUDIO_NP_DIV [
    /// Decoded P-divider coefficient value
    PDEC OFFSET(0) NUMBITS(7) [],
    /// Decoded N-divider coefficient value
    NDEC OFFSET(12) NUMBITS(10) []
],
PLL1_CTRL [
    /// PLL1 power down
    PD OFFSET(0) NUMBITS(1) [
        /// PLL1 enabled
        PLL1Enabled = 0,
        /// PLL1 powered down
        PLL1PoweredDown = 1
    ],
    /// Input clock bypass control
    BYPASS OFFSET(1) NUMBITS(1) [
        /// Normal. CCO clock sent to post-dividers. Use for normal operation.
        NormalCCOClockSentToPostDividersUseForNormalOperation = 0,
        /// Input clock. PLL1 input clock sent to post-dividers (default).
        InputClockPLL1InputClockSentToPostDividersDefault = 1
    ],
    /// PLL feedback select.
    FBSEL OFFSET(6) NUMBITS(1) [
        /// CCO out. CCO output is used as feedback divider input clock.
        CCOOutCCOOutputIsUsedAsFeedbackDividerInputClock = 0,
        /// PLL out. PLL output clock (clkout) is used as feedback divider input clock. Use
        PLL_OUT = 1
    ],
    /// PLL direct CCO output
    DIRECT OFFSET(7) NUMBITS(1) [
        /// Disabled
        Disabled = 0,
        /// Enabled
        Enabled = 1
    ],
    /// Post-divider division ratio P. The value applied is 2xP.
    PSEL OFFSET(8) NUMBITS(2) [
        /// 1
        _1 = 0,
        /// 2 (default)
        _2Default = 1,
        /// 4
        _4 = 2,
        /// 8
        _8 = 3
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Pre-divider division ratio N
    NSEL OFFSET(12) NUMBITS(2) [
        /// 1
        _1 = 0,
        /// 2
        _2 = 1,
        /// 3 (default)
        _3Default = 2,
        /// 4
        _4 = 3
    ],
    /// Feedback-divider division ratio (M) 00000000 = 1 00000001 = 2  ... 11111111 = 25
    MSEL OFFSET(16) NUMBITS(8) [],
    /// Clock-source selection.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Reserved
        Reserved = 5,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0USB
        PLL0USB = 7,
        /// PLL0AUDIO
        PLL0AUDIO = 8,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
IDIVA_CTRL [
    /// Integer divider A power down
    PD OFFSET(0) NUMBITS(1) [
        /// Enabled. IDIVA enabled (default)
        EnabledIDIVAEnabledDefault = 0,
        /// Power-down
        PowerDown = 1
    ],
    /// Integer divider A divider values  (1/(IDIV + 1))
    IDIV OFFSET(2) NUMBITS(2) [
        /// 1 (default)
        _1Default = 0,
        /// 2
        _2 = 1,
        /// 3
        _3 = 2,
        /// 4
        _4 = 3
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0USB
        PLL0USB = 7,
        /// PLL0AUDIO
        PLL0AUDIO = 8,
        /// PLL1
        PLL1 = 9
    ]
],
IDIVB_CTRL [
    /// Integer divider power down
    PD OFFSET(0) NUMBITS(1) [
        /// Enabled. IDIV enabled (default)
        EnabledIDIVEnabledDefault = 0,
        /// Power-down
        PowerDown = 1
    ],
    /// Integer divider B, C, D divider values (1/(IDIV + 1)) 0000 = 1 (default) 0001 =
    IDIV OFFSET(2) NUMBITS(4) [],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock-source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0AUDIO
        PLL0AUDIO = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12
    ]
],
IDIVC_CTRL [
    /// Integer divider power down
    PD OFFSET(0) NUMBITS(1) [
        /// Enabled. IDIV enabled (default)
        EnabledIDIVEnabledDefault = 0,
        /// Power-down
        PowerDown = 1
    ],
    /// Integer divider B, C, D divider values (1/(IDIV + 1)) 0000 = 1 (default) 0001 =
    IDIV OFFSET(2) NUMBITS(4) [],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock-source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0AUDIO
        PLL0AUDIO = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12
    ]
],
IDIVD_CTRL [
    /// Integer divider power down
    PD OFFSET(0) NUMBITS(1) [
        /// Enabled. IDIV enabled (default)
        EnabledIDIVEnabledDefault = 0,
        /// Power-down
        PowerDown = 1
    ],
    /// Integer divider B, C, D divider values (1/(IDIV + 1)) 0000 = 1 (default) 0001 =
    IDIV OFFSET(2) NUMBITS(4) [],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock-source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0AUDIO
        PLL0AUDIO = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12
    ]
],
IDIVE_CTRL [
    /// Integer divider power down
    PD OFFSET(0) NUMBITS(1) [
        /// Enabled. IDIV enabled (default)
        EnabledIDIVEnabledDefault = 0,
        /// Power-down
        PowerDown = 1
    ],
    /// Integer divider E divider values  (1/(IDIV + 1)) 00000000 = 1 (default) 00000001
    IDIV OFFSET(2) NUMBITS(8) [],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock-source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0AUDIO
        PLL0AUDIO = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12
    ]
],
BASE_SAFE_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Enabled. Output stage enabled (default)
        EnabledOutputStageEnabledDefault = 0,
        /// Power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// IRC (default)
        IRCDefault = 1
    ]
],
BASE_USB0_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Enabled. Output stage enabled (default)
        EnabledOutputStageEnabledDefault = 0,
        /// Power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock-source selection.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// PLL0USB (default)
        PLL0USBDefault = 7
    ]
],
BASE_PERIPH_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Enabled. Output stage enabled (default)
        EnabledOutputStageEnabledDefault = 0,
        /// Power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0AUDIO
        PLL0AUDIO = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
BASE_USB1_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Enabled. Output stage enabled (default)
        EnabledOutputStageEnabledDefault = 0,
        /// Power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0USB
        PLL0USB = 7,
        /// PLL0AUDIO
        PLL0AUDIO = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
BASE_M4_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Output stage enabled (default)
        OutputStageEnabledDefault = 0,
        /// power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0 (for audio)
        PLL0ForAudio = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
BASE_SPIFI_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Output stage enabled (default)
        OutputStageEnabledDefault = 0,
        /// power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0 (for audio)
        PLL0ForAudio = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
BASE_SPI_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Output stage enabled (default)
        OutputStageEnabledDefault = 0,
        /// power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0 (for audio)
        PLL0ForAudio = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
BASE_PHY_RX_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Output stage enabled (default)
        OutputStageEnabledDefault = 0,
        /// power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0 (for audio)
        PLL0ForAudio = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
BASE_PHY_TX_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Output stage enabled (default)
        OutputStageEnabledDefault = 0,
        /// power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0 (for audio)
        PLL0ForAudio = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
BASE_APB1_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Output stage enabled (default)
        OutputStageEnabledDefault = 0,
        /// power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0 (for audio)
        PLL0ForAudio = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
BASE_APB3_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Output stage enabled (default)
        OutputStageEnabledDefault = 0,
        /// power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0 (for audio)
        PLL0ForAudio = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
BASE_LCD_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Output stage enabled (default)
        OutputStageEnabledDefault = 0,
        /// power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0 (for audio)
        PLL0ForAudio = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
BASE_SDIO_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Output stage enabled (default)
        OutputStageEnabledDefault = 0,
        /// power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0 (for audio)
        PLL0ForAudio = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
BASE_SSP0_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Output stage enabled (default)
        OutputStageEnabledDefault = 0,
        /// power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0 (for audio)
        PLL0ForAudio = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
BASE_SSP1_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Output stage enabled (default)
        OutputStageEnabledDefault = 0,
        /// power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0 (for audio)
        PLL0ForAudio = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
BASE_UART0_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Output stage enabled (default)
        OutputStageEnabledDefault = 0,
        /// power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0 (for audio)
        PLL0ForAudio = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
BASE_UART1_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Output stage enabled (default)
        OutputStageEnabledDefault = 0,
        /// power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0 (for audio)
        PLL0ForAudio = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
BASE_UART2_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Output stage enabled (default)
        OutputStageEnabledDefault = 0,
        /// power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0 (for audio)
        PLL0ForAudio = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
BASE_UART3_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Output stage enabled (default)
        OutputStageEnabledDefault = 0,
        /// power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Disabled. Autoblocking disabled
        DisabledAutoblockingDisabled = 0,
        /// Enabled. Autoblocking enabled
        EnabledAutoblockingEnabled = 1
    ],
    /// Clock source selection. All other values are reserved.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0 (for audio)
        PLL0ForAudio = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
BASE_OUT_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Output stage enabled (default)
        OutputStageEnabledDefault = 0,
        /// power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Autoblocking disabled
        AutoblockingDisabled = 0,
        /// Autoblocking enabled
        AutoblockingEnabled = 1
    ],
    /// Clock-source selection.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Reserved
        Reserved = 5,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0 (for USB)
        PLL0ForUSB = 7,
        /// PLL0 (for audio)
        PLL0ForAudio = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
BASE_AUDIO_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Output stage enabled (default)
        OutputStageEnabledDefault = 0,
        /// power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Autoblocking disabled
        AutoblockingDisabled = 0,
        /// Autoblocking enabled
        AutoblockingEnabled = 1
    ],
    /// Clock-source selection.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Reserved
        Reserved = 5,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0 (for audio)
        PLL0ForAudio = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
BASE_CGU_OUT0_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Output stage enabled (default)
        OutputStageEnabledDefault = 0,
        /// power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Autoblocking disabled
        AutoblockingDisabled = 0,
        /// Autoblocking enabled
        AutoblockingEnabled = 1
    ],
    /// Clock-source selection.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Reserved
        Reserved = 5,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0 (for audio)
        PLL0ForAudio = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
],
BASE_CGU_OUT1_CLK [
    /// Output stage power down
    PD OFFSET(0) NUMBITS(1) [
        /// Output stage enabled (default)
        OutputStageEnabledDefault = 0,
        /// power-down
        PowerDown = 1
    ],
    /// Block clock automatically during frequency change
    AUTOBLOCK OFFSET(11) NUMBITS(1) [
        /// Autoblocking disabled
        AutoblockingDisabled = 0,
        /// Autoblocking enabled
        AutoblockingEnabled = 1
    ],
    /// Clock-source selection.
    CLK_SEL OFFSET(24) NUMBITS(5) [
        /// 32 kHz oscillator
        _32KHzOscillator = 0,
        /// IRC (default)
        IRCDefault = 1,
        /// ENET_RX_CLK
        ENET_RX_CLK = 2,
        /// ENET_TX_CLK
        ENET_TX_CLK = 3,
        /// GP_CLKIN
        GP_CLKIN = 4,
        /// Reserved
        Reserved = 5,
        /// Crystal oscillator
        CrystalOscillator = 6,
        /// PLL0 (for audio)
        PLL0ForAudio = 8,
        /// PLL1
        PLL1 = 9,
        /// IDIVA
        IDIVA = 12,
        /// IDIVB
        IDIVB = 13,
        /// IDIVC
        IDIVC = 14,
        /// IDIVD
        IDIVD = 15,
        /// IDIVE
        IDIVE = 16
    ]
]
];
const CGU_BASE: StaticRef<CguRegisters> =
    unsafe { StaticRef::new(0x40050000 as *const CguRegisters) };
