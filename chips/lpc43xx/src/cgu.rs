use cortexm4::support;
use kernel::common::StaticRef;
use kernel::common::registers::{ReadOnly, ReadWrite, register_bitfields, Field, FieldValue, RegisterLongName};
use creg::is_creg6_rmii_mode;

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
pll1_stat: ReadOnly<u32, PLL1_STAT::Register>,
/// PLL1 control register
pll1_ctrl: ReadWrite<u32, PLL1_CTRL::Register>,
/// Integer divider A control register
idiva_ctrl: ReadWrite<u32, IDIVA_CTRL::Register>,
/// Integer divider B control register
idivb_ctrl: ReadWrite<u32, IDIVBCD_CTRL::Register>,
/// Integer divider C control register
idivc_ctrl: ReadWrite<u32, IDIVBCD_CTRL::Register>,
/// Integer divider D control register
idivd_ctrl: ReadWrite<u32, IDIVBCD_CTRL::Register>,
/// Integer divider E control register
idive_ctrl: ReadWrite<u32, IDIVE_CTRL::Register>,
/// Output stage 0 control register for base clock BASE_SAFE_CLK
base_safe_clk: ReadOnly<u32, BASE_CLK::Register>,
/// Output stage 1 control register for base clock BASE_USB0_CLK
base_usb0_clk: ReadWrite<u32, BASE_CLK::Register>,
/// Output stage 2 control register for base clock BASE_PERIPH_CLK
base_periph_clk: ReadWrite<u32, BASE_CLK::Register>,
/// Output stage 3 control register for base clock BASE_USB1_CLK
base_usb1_clk: ReadWrite<u32, BASE_CLK::Register>,
/// Output stage BASE_M4_CLK control register
base_m4_clk: ReadWrite<u32, BASE_CLK::Register>,
/// Output stage BASE_SPIFI_CLK control register
base_spifi_clk: ReadWrite<u32, BASE_CLK::Register>,
/// Output stage BASE_SPI_CLK control register
base_spi_clk: ReadWrite<u32, BASE_CLK::Register>,
/// Output stage BASE_PHY_RX_CLK control register
base_phy_rx_clk: ReadWrite<u32, BASE_CLK::Register>,
/// Output stage BASE_PHY_TX_CLK control register
base_phy_tx_clk: ReadWrite<u32, BASE_CLK::Register>,
/// Output stage BASE_APB1_CLK control register
base_apb1_clk: ReadWrite<u32, BASE_CLK::Register>,
/// Output stage BASE_APB3_CLK control register
base_apb3_clk: ReadWrite<u32, BASE_CLK::Register>,
/// Output stage BASE_LCD_CLK control register
base_lcd_clk: ReadWrite<u32, BASE_CLK::Register>,
_reserved1: [u8; 4], //TODO: ADCHS clock here?
/// Output stage BASE_SDIO_CLK control register
base_sdio_clk: ReadWrite<u32, BASE_CLK::Register>,
/// Output stage BASE_SSP0_CLK control register
base_ssp0_clk: ReadWrite<u32, BASE_CLK::Register>,
/// Output stage BASE_SSP1_CLK control register
base_ssp1_clk: ReadWrite<u32, BASE_CLK::Register>,
/// Output stage BASE_UART0_CLK control register
base_uart0_clk: ReadWrite<u32, BASE_CLK::Register>,
/// Output stage BASE_UART1_CLK control register
base_uart1_clk: ReadWrite<u32, BASE_CLK::Register>,
/// Output stage BASE_UART2_CLK control register
base_uart2_clk: ReadWrite<u32, BASE_CLK::Register>,
/// Output stage BASE_UART3_CLK control register
base_uart3_clk: ReadWrite<u32, BASE_CLK::Register>,
/// Output stage 20 control register for base clock BASE_OUT_CLK
base_out_clk: ReadWrite<u32, BASE_CLK::Register>,
_reserved2: [u8; 16],
/// Output stage 25 control register for base clock BASE_AUDIO_CLK
base_audio_clk: ReadWrite<u32, BASE_CLK::Register>,
/// Output stage 25 control register for base clock BASE_CGU_OUT0_CLK
base_cgu_out0_clk: ReadWrite<u32, BASE_CLK::Register>,
/// Output stage 25 control register for base clock BASE_CGU_OUT1_CLK
base_cgu_out1_clk: ReadWrite<u32, BASE_CLK::Register>,
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
PLL1_STAT [
    /// PLL0 lock indicator
    LOCK OFFSET(0) NUMBITS(1) [
        /// PLL1 unlocked
        Unlocked = 0,
        /// PLL1 locked
        Locked = 1
    ]
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
IDIVBCD_CTRL [
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
BASE_CLK [
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
        PLL0USBDefault = 7,
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
]
];


/// CGU dividers
/// CGU dividers provide an extra clock state where a specific clock can be
/// divided before being routed to a peripheral group. A divider accepts an
/// input clock and then divides it. To use the divided clock for a base clock
/// group, use the divider as the input clock for the base clock (for example,
/// use CLKIN_IDIVB, where CLKIN_MAINPLL might be the input into the divider).
#[repr(C)]
enum CHIP_CGU_IDIV {
    /* CGU clock divider A */
    ClkIdivA,     
    /* CGU clock divider B */
    ClkIdivB,
    /* CGU clock divider A */
    ClkIdivC,
    /* CGU clock divider D */
    ClkIdivD,
    /* CGU clock divider E */
    ClkIdivE,
}

/// Maximum clock frequency: 204Mhz
pub const MAX_CLOCK_FREQ : u32 = 204000000;

/// Internal oscillator frequency
const CGU_IRC_FREQ : u32 = 12000000;

const CRYSTAL_32K_FREQ_IN : u32 = 32 * 1024;

/// Min CCO frequency of main PLL
const PLL_MIN_CCO_FREQ : u32 = 156000000;

/// Max CCO frequency of main PLL
const PLL_MAX_CCO_FREQ : u32 = 320000000;


/// System configuration variables used by chip driver for LPC43xx

const OSCRATEIN : u32 = 12000000;
const EXTRATEIN : u32 = 0;

const CGU_BASE: StaticRef<CguRegisters> =
    unsafe { StaticRef::new(0x40050000 as *const CguRegisters) };

/// Helper function to initialize all system clocks to a safe value
#[no_mangle]
#[inline(never)]
pub fn board_setup_clocking(clkin: FieldValue<u32, BASE_CLK::Register>, core_freq: u32, set_initial_clocks: bool){
    let mut delay : u32 = 5500;
    let mut direct : bool = false;

    //PartEq is not implemented for FieldValue. I wonder why
    if is_field_value_set::<BASE_CLK::Register>(clkin, BASE_CLK::CLK_SEL::CrystalOscillator) {
        enable_crystal();
    }

    // Enable M4 clock
    CGU_BASE.base_m4_clk.modify(clkin + BASE_CLK::PD::EnabledOutputStageEnabledDefault + BASE_CLK::AUTOBLOCK::EnabledAutoblockingEnabled);
   
    // Shutdown main PLL
    CGU_BASE.pll1_ctrl.modify(PLL1_CTRL::PD::PLL1PoweredDown);
    
    let mut ppll = calculate_main_pll_value(clkin, core_freq);
    
    if core_freq > 110000000 {
        if is_field_value_set::<PLL1_CTRL::Register>(ppll, PLL1_CTRL::DIRECT::Disabled)
            || !is_field_value_set::<PLL1_CTRL::Register>(ppll, PLL1_CTRL::PSEL::_1) {
              /* Calculate the PLL Parameters */
              let lpll = calculate_main_pll_value(clkin, 110000000);
              setup_main_pll(lpll);
              /* Wait for the PLL to lock */
              while !is_main_pll_locked() {support::nop()}
              CGU_BASE.base_m4_clk.modify(BASE_CLK::CLK_SEL::PLL1 + BASE_CLK::PD::EnabledOutputStageEnabledDefault + BASE_CLK::AUTOBLOCK::EnabledAutoblockingEnabled);
              for _ in 0..delay {support::nop()}
              delay = 5500;
         } else {
              direct = true;
              ppll = field_value_set(ppll, PLL1_CTRL::DIRECT::Disabled); //lets enable it later
         }
    }

    /* Setup and start the PLL */
    setup_main_pll(ppll);

    /* Wait for the PLL to lock */
    while !is_main_pll_locked(){support::nop() }

    /* Set core clock base as PLL1 */
    CGU_BASE.base_m4_clk.modify(BASE_CLK::CLK_SEL::PLL1 + BASE_CLK::PD::EnabledOutputStageEnabledDefault + BASE_CLK::AUTOBLOCK::EnabledAutoblockingEnabled);

    for _ in 0..delay {support::nop()} /* Wait for approx 50 uSec */
    if direct {
         delay = 5500;
         ppll = field_value_set(ppll, PLL1_CTRL::DIRECT::Enabled);
         setup_main_pll(ppll); /* Set DIRECT to operate at full frequency */
         for _ in 0..delay {support::nop()} /* Wait for approx 50 uSec */
    }
    
    if set_initial_clocks {
        /* Setup system base clocks and initial states. This won't enable and
           disable individual clocks, but sets up the base clock sources for
           each individual peripheral clock. */
        //Is SAFE_CLK really readOnly? I've seen it being set in other implementations.
        //CGU_BASE.base_safe_clk.modify(BASE_CLK::CLK_SEL::IRCDefault + BASE_CLK::PD::EnabledOutputStageEnabledDefault + BASE_CLK::AUTOBLOCK::EnabledAutoblockingEnabled);
        CGU_BASE.base_apb1_clk.modify(BASE_CLK::CLK_SEL::PLL1 + BASE_CLK::PD::EnabledOutputStageEnabledDefault + BASE_CLK::AUTOBLOCK::EnabledAutoblockingEnabled);
        CGU_BASE.base_apb3_clk.modify(BASE_CLK::CLK_SEL::PLL1 + BASE_CLK::PD::EnabledOutputStageEnabledDefault + BASE_CLK::AUTOBLOCK::EnabledAutoblockingEnabled);
        CGU_BASE.base_usb0_clk.modify(BASE_CLK::CLK_SEL::PLL0USBDefault + BASE_CLK::PD::PowerDown + BASE_CLK::AUTOBLOCK::EnabledAutoblockingEnabled);
        CGU_BASE.base_periph_clk.modify(BASE_CLK::CLK_SEL::PLL1 + BASE_CLK::PD::EnabledOutputStageEnabledDefault + BASE_CLK::AUTOBLOCK::EnabledAutoblockingEnabled);
        CGU_BASE.base_spi_clk.modify(BASE_CLK::CLK_SEL::PLL1 + BASE_CLK::PD::EnabledOutputStageEnabledDefault + BASE_CLK::AUTOBLOCK::EnabledAutoblockingEnabled);
        CGU_BASE.base_sdio_clk.modify(BASE_CLK::CLK_SEL::PLL1 + BASE_CLK::PD::EnabledOutputStageEnabledDefault + BASE_CLK::AUTOBLOCK::EnabledAutoblockingEnabled);
        CGU_BASE.base_ssp0_clk.modify(BASE_CLK::CLK_SEL::PLL1 + BASE_CLK::PD::EnabledOutputStageEnabledDefault + BASE_CLK::AUTOBLOCK::EnabledAutoblockingEnabled);
        CGU_BASE.base_ssp1_clk.modify(BASE_CLK::CLK_SEL::PLL1 + BASE_CLK::PD::EnabledOutputStageEnabledDefault + BASE_CLK::AUTOBLOCK::EnabledAutoblockingEnabled);
        CGU_BASE.base_uart0_clk.modify(BASE_CLK::CLK_SEL::PLL1 + BASE_CLK::PD::EnabledOutputStageEnabledDefault + BASE_CLK::AUTOBLOCK::EnabledAutoblockingEnabled);
        CGU_BASE.base_uart1_clk.modify(BASE_CLK::CLK_SEL::PLL1 + BASE_CLK::PD::EnabledOutputStageEnabledDefault + BASE_CLK::AUTOBLOCK::EnabledAutoblockingEnabled);
        CGU_BASE.base_uart2_clk.modify(BASE_CLK::CLK_SEL::PLL1 + BASE_CLK::PD::EnabledOutputStageEnabledDefault + BASE_CLK::AUTOBLOCK::EnabledAutoblockingEnabled);
        CGU_BASE.base_uart3_clk.modify(BASE_CLK::CLK_SEL::PLL1 + BASE_CLK::PD::EnabledOutputStageEnabledDefault + BASE_CLK::AUTOBLOCK::EnabledAutoblockingEnabled);
        //CGU_BASE.base_out_clk.modify(BASE_CLK::PD::PowerDown);
        //CGU_BASE.base_audio_clk.modify(BASE_CLK::PD::PowerDown);
        //CGU_BASE.base_cgu_out0_clk.modify(BASE_CLK::PD::PowerDown);
        //CGU_BASE.base_cgu_out1_clk.modify(BASE_CLK::PD::PowerDown);
        CGU_BASE.base_phy_rx_clk.modify(BASE_CLK::CLK_SEL::ENET_TX_CLK + BASE_CLK::PD::EnabledOutputStageEnabledDefault + BASE_CLK::AUTOBLOCK::EnabledAutoblockingEnabled);
        CGU_BASE.base_phy_tx_clk.modify(BASE_CLK::CLK_SEL::ENET_TX_CLK + BASE_CLK::PD::EnabledOutputStageEnabledDefault + BASE_CLK::AUTOBLOCK::EnabledAutoblockingEnabled);
        CGU_BASE.base_usb0_clk.modify(BASE_CLK::CLK_SEL::IDIVD + BASE_CLK::PD::PowerDown + BASE_CLK::AUTOBLOCK::EnabledAutoblockingEnabled);
        //TODO: ADHS??? it's defined as reserved1
    }
}

#[no_mangle]
#[inline(never)]
fn setup_main_pll(config: FieldValue<u32, PLL1_CTRL::Register>){
   /* power up main PLL */
   //CGU_BASE.pll1_ctrl.modify(config)
   CGU_BASE.pll1_ctrl.set(u32::from(config)) //overwrite the entire register. Don't modify it
}

#[no_mangle]
#[inline(never)]
fn is_main_pll_locked() -> bool {
   CGU_BASE.pll1_stat.is_set(PLL1_STAT::LOCK)
}

#[no_mangle]
#[inline(never)]
fn enable_crystal() {
    let xtal_local_cpy = CGU_BASE.xtal_osc_ctrl.extract();
    let mut xtal_fields = XTAL_OSC_CTRL::ENABLE::Enable + XTAL_OSC_CTRL::BYPASS::CrystalOperationWithCrystalConnectedDefault;
    
    if OSCRATEIN >= 20000000 {
        xtal_fields = xtal_fields + XTAL_OSC_CTRL::HF::HIGH;  /* Set high frequency mode */
    }
    
    /* Clear bypass mode before anything else */
    if CGU_BASE.xtal_osc_ctrl.is_set(XTAL_OSC_CTRL::BYPASS){
        CGU_BASE.xtal_osc_ctrl.modify(XTAL_OSC_CTRL::BYPASS::CrystalOperationWithCrystalConnectedDefault);
    }
    
    /* Enable crystal oscillator */
    CGU_BASE.xtal_osc_ctrl.modify_no_read(xtal_local_cpy, xtal_fields);
        
    //delay a little bit (at least 250uS)
     for _ in 0..27500 {
            support::nop();
     }
}

#[no_mangle]
#[inline(never)]
fn calculate_main_pll_value(srcin: FieldValue<u32, BASE_CLK::Register>, freq: u32) -> FieldValue<u32, PLL1_CTRL::Register>{
    let input_freq = get_clock_input_hz(srcin);
    if input_freq > MAX_CLOCK_FREQ || input_freq < (PLL_MIN_CCO_FREQ / 16) || input_freq == 0 {
       panic!("Unable to calculate main pll value!!!");
    }
    
    let pll1_clkin = PLL1_CTRL::CLK_SEL.val(get_raw_value_from_field_value(BASE_CLK::CLK_SEL, srcin)); //convert from one register type to the other
    let mut msel : u32 = freq / input_freq;
    let mut nsel : u32 = 0;
    let mut psel : u32 = 0;
    let mut config = pll1_clkin + PLL1_CTRL::DIRECT::Enabled + PLL1_CTRL::PSEL.val(psel) + PLL1_CTRL::NSEL.val(nsel) + PLL1_CTRL::MSEL.val(msel);
    
    if freq < PLL_MIN_CCO_FREQ || (freq / input_freq) * input_freq != freq {
        config = pll_get_frac(freq, input_freq); //recalculate the config. step over previous config, not updating
        if is_field_value_set::<PLL1_CTRL::Register>(config, PLL1_CTRL::NSEL::_1) {
            panic!("Unable to calculate main pll value! nsel = 0");
        }
        nsel = get_raw_value_from_field_value(PLL1_CTRL::NSEL, config) - 1;
        msel = get_raw_value_from_field_value(PLL1_CTRL::MSEL, config);
        psel = get_raw_value_from_field_value(PLL1_CTRL::PSEL, config);
    }
    
    if get_raw_value_from_field_value(PLL1_CTRL::MSEL, config) == 0 {
        panic!("Unable to calculate main pll value! msel = 0");
    }

    if get_raw_value_from_field_value(PLL1_CTRL::PSEL, config) != 0 {
        psel = psel - 1;
    }

    msel = msel - 1;
    config = pll1_clkin + PLL1_CTRL::DIRECT::Enabled + PLL1_CTRL::PSEL.val(psel) + PLL1_CTRL::NSEL.val(nsel) + PLL1_CTRL::MSEL.val(msel);
    config
}

#[no_mangle]
#[inline(never)]
pub fn get_clock_input_hz(clkin: FieldValue<u32, BASE_CLK::Register>) -> u32 {
    let mut ret_val: u32 = 0;
    // We can't use match here because the registers don't have Eq and PartialEq implemented
    if is_field_value_set(clkin,BASE_CLK::CLK_SEL::_32KHzOscillator){
        ret_val = CRYSTAL_32K_FREQ_IN;
    } else if is_field_value_set(clkin, BASE_CLK::CLK_SEL::IRCDefault){
        ret_val = CGU_IRC_FREQ;
    } else if is_field_value_set(clkin, BASE_CLK::CLK_SEL::ENET_RX_CLK){
        if !is_creg6_rmii_mode() {
            /* MII mode requires 25MHz clock */
            ret_val = 25000000;
        }
    } else if is_field_value_set(clkin, BASE_CLK::CLK_SEL::ENET_TX_CLK){
        if is_creg6_rmii_mode() {
            /* RMII uses 50 MHz */
            ret_val = 50000000;
        } else {
            /* MII mode requires 25MHz clock */
            ret_val = 25000000;
        }
    } else if is_field_value_set(clkin, BASE_CLK::CLK_SEL::GP_CLKIN){
        ret_val = EXTRATEIN;
    } else if is_field_value_set(clkin, BASE_CLK::CLK_SEL::CrystalOscillator){
        ret_val = OSCRATEIN;
    } else if is_field_value_set(clkin, BASE_CLK::CLK_SEL::PLL0USBDefault){
        // TODO: implement this. It should be stored somewhere after we setup the USB PLL, but who will be the owner?
        //maybe create a different function for PLL0
    } else if is_field_value_set(clkin, BASE_CLK::CLK_SEL::PLL0AUDIO){
        // TODO: implement this. It should be stored somewhere after we setup the USB PLL, but who will be the owner?
    } else if is_field_value_set(clkin, BASE_CLK::CLK_SEL::PLL1){
        ret_val = get_main_pll_hz();
    } else if is_field_value_set(clkin, BASE_CLK::CLK_SEL::IDIVA){
        ret_val = get_div_rate(CHIP_CGU_IDIV::ClkIdivA);
    } else if is_field_value_set(clkin, BASE_CLK::CLK_SEL::IDIVB){
        ret_val = get_div_rate(CHIP_CGU_IDIV::ClkIdivB);
    } else if is_field_value_set(clkin, BASE_CLK::CLK_SEL::IDIVC){
        ret_val = get_div_rate(CHIP_CGU_IDIV::ClkIdivC);
    } else if is_field_value_set(clkin, BASE_CLK::CLK_SEL::IDIVD){
        ret_val = get_div_rate(CHIP_CGU_IDIV::ClkIdivD);
    } else if is_field_value_set(clkin, BASE_CLK::CLK_SEL::IDIVE){
        ret_val = get_div_rate(CHIP_CGU_IDIV::ClkIdivE);
    }
    ret_val
}

#[no_mangle]
#[inline(never)]
fn get_main_pll_hz() -> u32 {
    let freq : u32 = get_clock_input_hz(BASE_CLK::CLK_SEL.val(CGU_BASE.pll1_ctrl.read(PLL1_CTRL::CLK_SEL)));
    let msel : u32;
    let nsel : u32;
    let psel : u32;
    let direct : u32;
    let fbsel : u32;
    let m : u32;
    let n : u32;
    let p : u32;
    let ptab = [1, 2, 4, 8];
    let ret_val;
    msel = CGU_BASE.pll1_ctrl.read(PLL1_CTRL::MSEL);
    nsel = CGU_BASE.pll1_ctrl.read(PLL1_CTRL::NSEL);
    psel = CGU_BASE.pll1_ctrl.read(PLL1_CTRL::PSEL);
    direct = CGU_BASE.pll1_ctrl.read(PLL1_CTRL::DIRECT);
    fbsel = CGU_BASE.pll1_ctrl.read(PLL1_CTRL::FBSEL);
    
    m = msel + 1;
    n = nsel + 1;
    p = ptab[psel as usize];
    if direct != 0 || fbsel != 0 {
        ret_val = m * (freq / n);
    } else {
        ret_val = (m / (2 * p)) * (freq / n);
    }
    ret_val
}

#[no_mangle]
#[inline(never)]
fn get_div_rate(divider: CHIP_CGU_IDIV) -> u32 {
    match divider {
            CHIP_CGU_IDIV::ClkIdivA => {
                let divider_source = CGU_BASE.idiva_ctrl.read(IDIVA_CTRL::CLK_SEL);
                let divider_source_hz = get_clock_input_hz(BASE_CLK::CLK_SEL.val(divider_source));
                let div = CGU_BASE.idiva_ctrl.read(IDIVA_CTRL::IDIV);
                return divider_source_hz / (div + 1);
            },
            CHIP_CGU_IDIV::ClkIdivB => {
                let divider_source = CGU_BASE.idivb_ctrl.read(IDIVBCD_CTRL::CLK_SEL);
                let divider_source_hz = get_clock_input_hz(BASE_CLK::CLK_SEL.val(divider_source));
                let div = CGU_BASE.idivb_ctrl.read(IDIVBCD_CTRL::IDIV);
                return divider_source_hz / (div + 1);
            },
            CHIP_CGU_IDIV::ClkIdivC => {
                let divider_source = CGU_BASE.idivc_ctrl.read(IDIVBCD_CTRL::CLK_SEL);
                let divider_source_hz = get_clock_input_hz(BASE_CLK::CLK_SEL.val(divider_source));
                let div = CGU_BASE.idivc_ctrl.read(IDIVBCD_CTRL::IDIV);
                return divider_source_hz / (div + 1);
            },
            CHIP_CGU_IDIV::ClkIdivD => {
                let divider_source = CGU_BASE.idivd_ctrl.read(IDIVBCD_CTRL::CLK_SEL);
                let divider_source_hz = get_clock_input_hz(BASE_CLK::CLK_SEL.val(divider_source));
                let div = CGU_BASE.idivd_ctrl.read(IDIVBCD_CTRL::IDIV);
                return divider_source_hz / (div + 1);
            },
            CHIP_CGU_IDIV::ClkIdivE => {
                let divider_source = CGU_BASE.idive_ctrl.read(IDIVE_CTRL::CLK_SEL);
                let divider_source_hz = get_clock_input_hz(BASE_CLK::CLK_SEL.val(divider_source));
                let div = CGU_BASE.idive_ctrl.read(IDIVE_CTRL::IDIV);
                return divider_source_hz / (div + 1);
            }
    }

}

#[no_mangle]
#[inline(never)]
fn pll_get_frac(freq: u32, current_input_freq : u32) -> FieldValue<u32, PLL1_CTRL::Register> {
    let diff_0 : u32;
    let diff_1 : u32;
    let diff_2 : u32;

    /* Try direct mode */
    let pll_config_0 = PLL1_CTRL::DIRECT::Enabled;
    let (pll_freq_0, pll_config_0) = pll_calc_divs(freq, pll_config_0, current_input_freq);
    if pll_freq_0 == freq {
        return pll_config_0;
    }
    diff_0 = abs_sub(freq, pll_freq_0);
    

   /* Try non-Integer mode */
   let pll_config_2 = PLL1_CTRL::FBSEL::PLL_OUT;
   let (pll_freq_2, pll_config_2) = pll_calc_divs(freq, pll_config_2, current_input_freq);
   if pll_freq_2 == freq {
       return pll_config_2;
   }
   diff_2 = abs_sub(freq, pll_freq_2);
   
   /* Try integer mode. FIXME. Twice the same? I didn't understand what they tried to do here.*/
   let pll_config_1 = PLL1_CTRL::FBSEL::PLL_OUT;
   let (pll_freq_1, pll_config_1) = pll_calc_divs(freq, pll_config_1, current_input_freq);
   if pll_freq_1 == freq {
       return pll_config_1;
   }
   diff_1 = abs_sub(freq, pll_freq_1);

   /* Find the min of 3 and return */
   if diff_0 <= diff_1 {
       if diff_0 <= diff_2 {
           return pll_config_0;
       } else {
           return pll_config_2;
       }
   } else {
       if diff_1 <= diff_2 {
           return pll_config_1;
       } else {
           return pll_config_2;
       }
   }
}

#[no_mangle]
#[inline(never)]
fn pll_calc_divs(freq: u32, config: FieldValue<u32, PLL1_CTRL::Register>, current_input_freq : u32) -> (u32, FieldValue<u32, PLL1_CTRL::Register>)
{

    let mut prev : u32 = freq;
    let mut calculated_freq : u32 = freq;
    let mut new_config : FieldValue<u32, PLL1_CTRL::Register> = config;
    /* When direct mode is set FBSEL should be a don't care */
    if is_field_value_set(config, PLL1_CTRL::DIRECT::Enabled) {
        new_config = field_value_set::<PLL1_CTRL::Register>(new_config, PLL1_CTRL::FBSEL::CCOOutCCOOutputIsUsedAsFeedbackDividerInputClock);
    }
    for n in 1..5 {
        for p in 0..4 {
            for m in 1..257 {
                let fcco : u32;
                let fout : u32;
                if is_field_value_set(new_config, PLL1_CTRL::FBSEL::CCOOutCCOOutputIsUsedAsFeedbackDividerInputClock) {
                    fcco = ((m << (p + 1)) * current_input_freq) / n;
                } else {
                    fcco = (m * current_input_freq) / n;
                }
                if fcco < PLL_MIN_CCO_FREQ {continue;}
                if fcco > PLL_MAX_CCO_FREQ {break;}
                if is_field_value_set(new_config, PLL1_CTRL::DIRECT::Enabled) {
                    fout = fcco;
                } else {
                    fout = fcco >> (p + 1);
                }
                //no std lib, and no std abs function...
                let diff = abs_sub(freq, fout);
                if diff < prev {
                    let nsel = PLL1_CTRL::NSEL.val(n);
                    let psel = PLL1_CTRL::PSEL.val(p + 1);
                    let msel = PLL1_CTRL::MSEL.val(m);
                    calculated_freq = fout;
                    new_config = field_value_set::<PLL1_CTRL::Register>(new_config, nsel + psel + msel);
                    //ppll->fcco = fcco; not used
                    prev = diff;
                }
            }
        }
    }
    (calculated_freq, new_config)
}


#[no_mangle]
#[inline(never)]
pub fn get_uart2_base_clk() -> FieldValue<u32,BASE_CLK::CLK_SEL> {
    CGU_BASE.base_uart2_clk.read(BASE_CLK::CLK_SEL)
}

/// We don't have std::num::abs so we implement this substraction manually
#[no_mangle]
#[inline(never)]
fn abs_sub (a : u32, b : u32) -> u32 {
    if a > b {
        return a - b;
    } else {
        return b - a;
    }
}


/// Replace bits in existing FieldValue by the bits and values from the second FieldValue.
/// This is different than adding, since adding only performs an OR operation.
/// This is also different than .modify, since the modify function for 
/// FielValue types actually replaces the entire value with a new u32 but you want to edit a few bits.
/// 
/// Note: RegisterLongName was made public just for this. I was using IntLike to also
/// use a generic for the type, but the new method started giving me issues so I skipped the problem.
fn field_value_set<R>(old: FieldValue<u32, R>, new : FieldValue<u32, R>) -> FieldValue<u32, R> where R: RegisterLongName {
    let nmask = new.mask;
    let omask = old.mask;
    FieldValue::<u32, R>::new(omask | nmask, 0, (u32::from(old) & !nmask) | u32::from(new))
}

/// Take two FieldValues. Check that the second value is set on the first one
/// This is useful for checking that some FieldValue bits are equal to another defined one
/// ie: is_field_value_set::<BASE_CLK::Register>(clkin, BASE_CLK::CLK_SEL::CrystalOscillator)) {
fn is_field_value_set<R>(current_field_value: FieldValue<u32, R>, expected_bits : FieldValue<u32, R>) -> bool where R: RegisterLongName {
    current_field_value.value & expected_bits.mask == expected_bits.value
}

/// Extract a single value (a single field) from a composed field value
fn get_raw_value_from_field_value<R>(field: Field<u32, R>, composed_value : FieldValue<u32, R>) -> u32 where R: RegisterLongName {
    // For the Field, the mask is unshifted, so...
    (composed_value.value >> field.shift) & field.mask
}
