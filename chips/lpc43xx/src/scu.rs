
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// System Control Unit (SCU) I/O configuration
#[repr(C)]
struct ScuRegisters {
/// Pin configuration register for pins P0
sfsp0_0: ReadWrite<u32, SFSP0_0::Register>,
/// Pin configuration register for pins P0
sfsp0_1: ReadWrite<u32, SFSP0_1::Register>,
_reserved0: [u8; 120],
/// Pin configuration register for pins P1
sfsp1_0: ReadWrite<u32, SFSP1_0::Register>,
/// Pin configuration register for pins P1
sfsp1_1: ReadWrite<u32, SFSP1_1::Register>,
/// Pin configuration register for pins P1
sfsp1_2: ReadWrite<u32, SFSP1_2::Register>,
/// Pin configuration register for pins P1
sfsp1_3: ReadWrite<u32, SFSP1_3::Register>,
/// Pin configuration register for pins P1
sfsp1_4: ReadWrite<u32, SFSP1_4::Register>,
/// Pin configuration register for pins P1
sfsp1_5: ReadWrite<u32, SFSP1_5::Register>,
/// Pin configuration register for pins P1
sfsp1_6: ReadWrite<u32, SFSP1_6::Register>,
/// Pin configuration register for pins P1
sfsp1_7: ReadWrite<u32, SFSP1_7::Register>,
/// Pin configuration register for pins P1
sfsp1_8: ReadWrite<u32, SFSP1_8::Register>,
/// Pin configuration register for pins P1
sfsp1_9: ReadWrite<u32, SFSP1_9::Register>,
/// Pin configuration register for pins P1
sfsp1_10: ReadWrite<u32, SFSP1_10::Register>,
/// Pin configuration register for pins P1
sfsp1_11: ReadWrite<u32, SFSP1_11::Register>,
/// Pin configuration register for pins P1
sfsp1_12: ReadWrite<u32, SFSP1_12::Register>,
/// Pin configuration register for pins P1
sfsp1_13: ReadWrite<u32, SFSP1_13::Register>,
/// Pin configuration register for pins P1
sfsp1_14: ReadWrite<u32, SFSP1_14::Register>,
/// Pin configuration register for pins P1
sfsp1_15: ReadWrite<u32, SFSP1_15::Register>,
/// Pin configuration register for pins P1
sfsp1_16: ReadWrite<u32, SFSP1_16::Register>,
/// Pin configuration register for pins P1_17
sfsp1_17: ReadWrite<u32, SFSP1_17::Register>,
/// Pin configuration register for pins P1
sfsp1_18: ReadWrite<u32, SFSP1_18::Register>,
/// Pin configuration register for pins P1
sfsp1_19: ReadWrite<u32, SFSP1_19::Register>,
/// Pin configuration register for pins P1
sfsp1_20: ReadWrite<u32, SFSP1_20::Register>,
_reserved1: [u8; 44],
/// Pin configuration register for pins P2
sfsp2_0: ReadWrite<u32, SFSP2_0::Register>,
/// Pin configuration register for pins P2
sfsp2_1: ReadWrite<u32, SFSP2_1::Register>,
/// Pin configuration register for pins P2
sfsp2_2: ReadWrite<u32, SFSP2_2::Register>,
/// Pin configuration register for pins P2
sfsp2_3: ReadWrite<u32, SFSP2_3::Register>,
/// Pin configuration register for pins P2
sfsp2_4: ReadWrite<u32, SFSP2_4::Register>,
/// Pin configuration register for pins P2
sfsp2_5: ReadWrite<u32, SFSP2_5::Register>,
/// Pin configuration register for pins P2
sfsp2_6: ReadWrite<u32, SFSP2_6::Register>,
/// Pin configuration register for pins P2
sfsp2_7: ReadWrite<u32, SFSP2_7::Register>,
/// Pin configuration register for pins P2
sfsp2_8: ReadWrite<u32, SFSP2_8::Register>,
/// Pin configuration register for pins P2
sfsp2_9: ReadWrite<u32, SFSP2_9::Register>,
/// Pin configuration register for pins P2
sfsp2_10: ReadWrite<u32, SFSP2_10::Register>,
/// Pin configuration register for pins P2
sfsp2_11: ReadWrite<u32, SFSP2_11::Register>,
/// Pin configuration register for pins P2
sfsp2_12: ReadWrite<u32, SFSP2_12::Register>,
_reserved2: [u8; 76],
/// Pin configuration register for pins P3
sfsp3_0: ReadWrite<u32, SFSP3_0::Register>,
/// Pin configuration register for pins P3
sfsp3_1: ReadWrite<u32, SFSP3_1::Register>,
/// Pin configuration register for pins P3
sfsp3_2: ReadWrite<u32, SFSP3_2::Register>,
/// Pin configuration register for pins P3
sfsp3_3: ReadWrite<u32, SFSP3_3::Register>,
/// Pin configuration register for pins P3
sfsp3_4: ReadWrite<u32, SFSP3_4::Register>,
/// Pin configuration register for pins P3
sfsp3_5: ReadWrite<u32, SFSP3_5::Register>,
/// Pin configuration register for pins P3
sfsp3_6: ReadWrite<u32, SFSP3_6::Register>,
/// Pin configuration register for pins P3
sfsp3_7: ReadWrite<u32, SFSP3_7::Register>,
/// Pin configuration register for pins P3
sfsp3_8: ReadWrite<u32, SFSP3_8::Register>,
_reserved3: [u8; 92],
/// Pin configuration register for pins P4
sfsp4_0: ReadWrite<u32, SFSP4_0::Register>,
/// Pin configuration register for pins P4
sfsp4_1: ReadWrite<u32, SFSP4_1::Register>,
/// Pin configuration register for pins P4
sfsp4_2: ReadWrite<u32, SFSP4_2::Register>,
/// Pin configuration register for pins P4
sfsp4_3: ReadWrite<u32, SFSP4_3::Register>,
/// Pin configuration register for pins P4
sfsp4_4: ReadWrite<u32, SFSP4_4::Register>,
/// Pin configuration register for pins P4
sfsp4_5: ReadWrite<u32, SFSP4_5::Register>,
/// Pin configuration register for pins P4
sfsp4_6: ReadWrite<u32, SFSP4_6::Register>,
/// Pin configuration register for pins P4
sfsp4_7: ReadWrite<u32, SFSP4_7::Register>,
/// Pin configuration register for pins P4
sfsp4_8: ReadWrite<u32, SFSP4_8::Register>,
/// Pin configuration register for pins P4
sfsp4_9: ReadWrite<u32, SFSP4_9::Register>,
/// Pin configuration register for pins P4
sfsp4_10: ReadWrite<u32, SFSP4_10::Register>,
_reserved4: [u8; 84],
/// Pin configuration register for pins P5
sfsp5_0: ReadWrite<u32, SFSP5_0::Register>,
/// Pin configuration register for pins P5
sfsp5_1: ReadWrite<u32, SFSP5_1::Register>,
/// Pin configuration register for pins P5
sfsp5_2: ReadWrite<u32, SFSP5_2::Register>,
/// Pin configuration register for pins P5
sfsp5_3: ReadWrite<u32, SFSP5_3::Register>,
/// Pin configuration register for pins P5
sfsp5_4: ReadWrite<u32, SFSP5_4::Register>,
/// Pin configuration register for pins P5
sfsp5_5: ReadWrite<u32, SFSP5_5::Register>,
/// Pin configuration register for pins P5
sfsp5_6: ReadWrite<u32, SFSP5_6::Register>,
/// Pin configuration register for pins P5
sfsp5_7: ReadWrite<u32, SFSP5_7::Register>,
_reserved5: [u8; 96],
/// Pin configuration register for pins P6
sfsp6_0: ReadWrite<u32, SFSP6_0::Register>,
/// Pin configuration register for pins P6
sfsp6_1: ReadWrite<u32, SFSP6_1::Register>,
/// Pin configuration register for pins P6
sfsp6_2: ReadWrite<u32, SFSP6_2::Register>,
/// Pin configuration register for pins P6
sfsp6_3: ReadWrite<u32, SFSP6_3::Register>,
/// Pin configuration register for pins P6
sfsp6_4: ReadWrite<u32, SFSP6_4::Register>,
/// Pin configuration register for pins P6
sfsp6_5: ReadWrite<u32, SFSP6_5::Register>,
/// Pin configuration register for pins P6
sfsp6_6: ReadWrite<u32, SFSP6_6::Register>,
/// Pin configuration register for pins P6
sfsp6_7: ReadWrite<u32, SFSP6_7::Register>,
/// Pin configuration register for pins P6
sfsp6_8: ReadWrite<u32, SFSP6_8::Register>,
/// Pin configuration register for pins P6
sfsp6_9: ReadWrite<u32, SFSP6_9::Register>,
/// Pin configuration register for pins P6
sfsp6_10: ReadWrite<u32, SFSP6_10::Register>,
/// Pin configuration register for pins P6
sfsp6_11: ReadWrite<u32, SFSP6_11::Register>,
/// Pin configuration register for pins P6
sfsp6_12: ReadWrite<u32, SFSP6_12::Register>,
_reserved6: [u8; 76],
/// Pin configuration register for pins P7
sfsp7_0: ReadWrite<u32, SFSP7_0::Register>,
/// Pin configuration register for pins P7
sfsp7_1: ReadWrite<u32, SFSP7_1::Register>,
/// Pin configuration register for pins P7
sfsp7_2: ReadWrite<u32, SFSP7_2::Register>,
/// Pin configuration register for pins P7
sfsp7_3: ReadWrite<u32, SFSP7_3::Register>,
/// Pin configuration register for pins P7
sfsp7_4: ReadWrite<u32, SFSP7_4::Register>,
/// Pin configuration register for pins P7
sfsp7_5: ReadWrite<u32, SFSP7_5::Register>,
/// Pin configuration register for pins P7
sfsp7_6: ReadWrite<u32, SFSP7_6::Register>,
/// Pin configuration register for pins P7
sfsp7_7: ReadWrite<u32, SFSP7_7::Register>,
_reserved7: [u8; 96],
/// Pin configuration register for pins P8
sfsp8_0: ReadWrite<u32, SFSP8_0::Register>,
/// Pin configuration register for pins P8
sfsp8_1: ReadWrite<u32, SFSP8_1::Register>,
/// Pin configuration register for pins P8
sfsp8_2: ReadWrite<u32, SFSP8_2::Register>,
/// Pin configuration register for pins P8
sfsp8_3: ReadWrite<u32, SFSP8_3::Register>,
/// Pin configuration register for pins P8
sfsp8_4: ReadWrite<u32, SFSP8_4::Register>,
/// Pin configuration register for pins P8
sfsp8_5: ReadWrite<u32, SFSP8_5::Register>,
/// Pin configuration register for pins P8
sfsp8_6: ReadWrite<u32, SFSP8_6::Register>,
/// Pin configuration register for pins P8
sfsp8_7: ReadWrite<u32, SFSP8_7::Register>,
/// Pin configuration register for pins P8
sfsp8_8: ReadWrite<u32, SFSP8_8::Register>,
_reserved8: [u8; 92],
/// Pin configuration register for pins P9
sfsp9_0: ReadWrite<u32, SFSP9_0::Register>,
/// Pin configuration register for pins P9
sfsp9_1: ReadWrite<u32, SFSP9_1::Register>,
/// Pin configuration register for pins P9
sfsp9_2: ReadWrite<u32, SFSP9_2::Register>,
/// Pin configuration register for pins P9
sfsp9_3: ReadWrite<u32, SFSP9_3::Register>,
/// Pin configuration register for pins P9
sfsp9_4: ReadWrite<u32, SFSP9_4::Register>,
/// Pin configuration register for pins P9
sfsp9_5: ReadWrite<u32, SFSP9_5::Register>,
/// Pin configuration register for pins P9
sfsp9_6: ReadWrite<u32, SFSP9_6::Register>,
_reserved9: [u8; 100],
/// Pin configuration register for pins PA
sfspa_0: ReadWrite<u32, SFSPA_0::Register>,
/// Pin configuration register for pins PA
sfspa_1: ReadWrite<u32, SFSPA_1::Register>,
/// Pin configuration register for pins PA
sfspa_2: ReadWrite<u32, SFSPA_2::Register>,
/// Pin configuration register for pins PA
sfspa_3: ReadWrite<u32, SFSPA_3::Register>,
/// Pin configuration register for pins PA
sfspa_4: ReadWrite<u32, SFSPA_4::Register>,
_reserved10: [u8; 108],
/// Pin configuration register for pins PB
sfspb_0: ReadWrite<u32, SFSPB_0::Register>,
/// Pin configuration register for pins PB
sfspb_1: ReadWrite<u32, SFSPB_1::Register>,
/// Pin configuration register for pins PB
sfspb_2: ReadWrite<u32, SFSPB_2::Register>,
/// Pin configuration register for pins PB
sfspb_3: ReadWrite<u32, SFSPB_3::Register>,
/// Pin configuration register for pins PB
sfspb_4: ReadWrite<u32, SFSPB_4::Register>,
/// Pin configuration register for pins PB
sfspb_5: ReadWrite<u32, SFSPB_5::Register>,
/// Pin configuration register for pins PB
sfspb_6: ReadWrite<u32, SFSPB_6::Register>,
_reserved11: [u8; 100],
/// Pin configuration register for pins PC
sfspc_0: ReadWrite<u32, SFSPC_0::Register>,
/// Pin configuration register for pins PC
sfspc_1: ReadWrite<u32, SFSPC_1::Register>,
/// Pin configuration register for pins PC
sfspc_2: ReadWrite<u32, SFSPC_2::Register>,
/// Pin configuration register for pins PC
sfspc_3: ReadWrite<u32, SFSPC_3::Register>,
/// Pin configuration register for pins PC
sfspc_4: ReadWrite<u32, SFSPC_4::Register>,
/// Pin configuration register for pins PC
sfspc_5: ReadWrite<u32, SFSPC_5::Register>,
/// Pin configuration register for pins PC
sfspc_6: ReadWrite<u32, SFSPC_6::Register>,
/// Pin configuration register for pins PC
sfspc_7: ReadWrite<u32, SFSPC_7::Register>,
/// Pin configuration register for pins PC
sfspc_8: ReadWrite<u32, SFSPC_8::Register>,
/// Pin configuration register for pins PC
sfspc_9: ReadWrite<u32, SFSPC_9::Register>,
/// Pin configuration register for pins PC
sfspc_10: ReadWrite<u32, SFSPC_10::Register>,
/// Pin configuration register for pins PC
sfspc_11: ReadWrite<u32, SFSPC_11::Register>,
/// Pin configuration register for pins PC
sfspc_12: ReadWrite<u32, SFSPC_12::Register>,
/// Pin configuration register for pins PC
sfspc_13: ReadWrite<u32, SFSPC_13::Register>,
/// Pin configuration register for pins PC
sfspc_14: ReadWrite<u32, SFSPC_14::Register>,
_reserved12: [u8; 68],
/// Pin configuration register for pins PD
sfspd_0: ReadWrite<u32, SFSPD_0::Register>,
/// Pin configuration register for pins PD
sfspd_1: ReadWrite<u32, SFSPD_1::Register>,
/// Pin configuration register for pins PD
sfspd_2: ReadWrite<u32, SFSPD_2::Register>,
/// Pin configuration register for pins PD
sfspd_3: ReadWrite<u32, SFSPD_3::Register>,
/// Pin configuration register for pins PD
sfspd_4: ReadWrite<u32, SFSPD_4::Register>,
/// Pin configuration register for pins PD
sfspd_5: ReadWrite<u32, SFSPD_5::Register>,
/// Pin configuration register for pins PD
sfspd_6: ReadWrite<u32, SFSPD_6::Register>,
/// Pin configuration register for pins PD
sfspd_7: ReadWrite<u32, SFSPD_7::Register>,
/// Pin configuration register for pins PD
sfspd_8: ReadWrite<u32, SFSPD_8::Register>,
/// Pin configuration register for pins PD
sfspd_9: ReadWrite<u32, SFSPD_9::Register>,
/// Pin configuration register for pins PD
sfspd_10: ReadWrite<u32, SFSPD_10::Register>,
/// Pin configuration register for pins PD
sfspd_11: ReadWrite<u32, SFSPD_11::Register>,
/// Pin configuration register for pins PD
sfspd_12: ReadWrite<u32, SFSPD_12::Register>,
/// Pin configuration register for pins PD
sfspd_13: ReadWrite<u32, SFSPD_13::Register>,
/// Pin configuration register for pins PD
sfspd_14: ReadWrite<u32, SFSPD_14::Register>,
/// Pin configuration register for pins PD
sfspd_15: ReadWrite<u32, SFSPD_15::Register>,
/// Pin configuration register for pins PD
sfspd_16: ReadWrite<u32, SFSPD_16::Register>,
_reserved13: [u8; 60],
/// Pin configuration register for pins PE
sfspe_0: ReadWrite<u32, SFSPE_0::Register>,
/// Pin configuration register for pins PE
sfspe_1: ReadWrite<u32, SFSPE_1::Register>,
/// Pin configuration register for pins PE
sfspe_2: ReadWrite<u32, SFSPE_2::Register>,
/// Pin configuration register for pins PE
sfspe_3: ReadWrite<u32, SFSPE_3::Register>,
/// Pin configuration register for pins PE
sfspe_4: ReadWrite<u32, SFSPE_4::Register>,
/// Pin configuration register for pins PE
sfspe_5: ReadWrite<u32, SFSPE_5::Register>,
/// Pin configuration register for pins PE
sfspe_6: ReadWrite<u32, SFSPE_6::Register>,
/// Pin configuration register for pins PE
sfspe_7: ReadWrite<u32, SFSPE_7::Register>,
/// Pin configuration register for pins PE
sfspe_8: ReadWrite<u32, SFSPE_8::Register>,
/// Pin configuration register for pins PE
sfspe_9: ReadWrite<u32, SFSPE_9::Register>,
/// Pin configuration register for pins PE
sfspe_10: ReadWrite<u32, SFSPE_10::Register>,
/// Pin configuration register for pins PE
sfspe_11: ReadWrite<u32, SFSPE_11::Register>,
/// Pin configuration register for pins PE
sfspe_12: ReadWrite<u32, SFSPE_12::Register>,
/// Pin configuration register for pins PE
sfspe_13: ReadWrite<u32, SFSPE_13::Register>,
/// Pin configuration register for pins PE
sfspe_14: ReadWrite<u32, SFSPE_14::Register>,
/// Pin configuration register for pins PE
sfspe_15: ReadWrite<u32, SFSPE_15::Register>,
_reserved14: [u8; 64],
/// Pin configuration register for pins PF
sfspf_0: ReadWrite<u32, SFSPF_0::Register>,
/// Pin configuration register for pins PF
sfspf_1: ReadWrite<u32, SFSPF_1::Register>,
/// Pin configuration register for pins PF
sfspf_2: ReadWrite<u32, SFSPF_2::Register>,
/// Pin configuration register for pins PF
sfspf_3: ReadWrite<u32, SFSPF_3::Register>,
/// Pin configuration register for pins PF
sfspf_4: ReadWrite<u32, SFSPF_4::Register>,
/// Pin configuration register for pins PF
sfspf_5: ReadWrite<u32, SFSPF_5::Register>,
/// Pin configuration register for pins PF
sfspf_6: ReadWrite<u32, SFSPF_6::Register>,
/// Pin configuration register for pins PF
sfspf_7: ReadWrite<u32, SFSPF_7::Register>,
/// Pin configuration register for pins PF
sfspf_8: ReadWrite<u32, SFSPF_8::Register>,
/// Pin configuration register for pins PF
sfspf_9: ReadWrite<u32, SFSPF_9::Register>,
/// Pin configuration register for pins PF
sfspf_10: ReadWrite<u32, SFSPF_10::Register>,
/// Pin configuration register for pins PF
sfspf_11: ReadWrite<u32, SFSPF_11::Register>,
_reserved15: [u8; 1104],
/// Pin configuration register for pins CLK
sfsclk_0: ReadWrite<u32, SFSCLK_0::Register>,
/// Pin configuration register for pins CLK
sfsclk_1: ReadWrite<u32, SFSCLK_1::Register>,
/// Pin configuration register for pins CLK
sfsclk_2: ReadWrite<u32, SFSCLK_2::Register>,
/// Pin configuration register for pins CLK
sfsclk_3: ReadWrite<u32, SFSCLK_3::Register>,
_reserved16: [u8; 112],
/// Pin configuration register for pins USB1_DM and USB1_DP
sfsusb: ReadWrite<u32, SFSUSB::Register>,
/// Pin configuration register for I2C0-bus pins
sfsi2c0: ReadWrite<u32, SFSI2C0::Register>,
/// ADC0 function select register
enaio0: ReadWrite<u32, ENAIO0::Register>,
/// ADC1 function select register
enaio1: ReadWrite<u32, ENAIO1::Register>,
/// Analog function select register
enaio2: ReadWrite<u32, ENAIO2::Register>,
_reserved17: [u8; 108],
/// EMC clock delay register
emcdelayclk: ReadWrite<u32>,
_reserved18: [u8; 124],
/// SD/MMC sample and drive delay register
sddelay: ReadWrite<u32, SDDELAY::Register>,
_reserved19: [u8; 124],
/// Pin interrupt select register for pin interrupts 0 to 3.
pintsel0: ReadWrite<u32, PINTSEL0::Register>,
/// Pin interrupt select register for pin interrupts 4 to 7.
pintsel1: ReadWrite<u32, PINTSEL1::Register>,
}
register_bitfields![u32,
SFSP1_17 [
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
SFSP3_3 [
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
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Slew rate
    EHS OFFSET(5) NUMBITS(1) [
        /// Fast (low noise with fast speed)
        FastLowNoiseWithFastSpeed = 0,
        /// High-speed (medium noise with high speed)
        HighSpeedMediumNoiseWithHighSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
    EZI OFFSET(6) NUMBITS(1) [
        /// Disable input buffer
        DisableInputBuffer = 0,
        /// Enable input buffer
        EnableInputBuffer = 1
    ],
    /// Input glitch filter. Disable the input glitch filter for clocking signals higher
    ZIF OFFSET(7) NUMBITS(1) [
        /// Enable input filter
        EnableInputFilter = 0,
        /// Disable input filter
        DisableInputFilter = 1
    ]
],
SFSPA_0 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPA_4 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
],
SFSP0_0 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP0_1 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP1_0 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP1_1 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP1_2 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP1_3 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP1_4 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP1_5 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP1_6 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP1_7 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP1_8 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP1_9 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP1_10 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP1_11 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP1_12 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP1_13 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP1_14 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP1_15 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP1_16 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP1_18 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP1_19 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP1_20 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP2_0 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP2_1 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP2_2 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP2_3 [
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
SFSP2_4 [
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
SFSP2_5 [
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
SFSP2_6 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP2_7 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP2_8 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP2_9 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP2_10 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP2_11 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP2_12 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP3_0 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP3_1 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP3_2 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP3_4 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP3_5 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP3_6 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP3_7 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP3_8 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP4_0 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP4_1 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP4_2 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP4_3 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP4_4 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP4_5 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP4_6 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP4_7 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP4_8 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP4_9 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP4_10 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP5_0 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP5_1 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP5_2 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP5_3 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP5_4 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP5_5 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP5_6 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP5_7 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP6_0 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP6_1 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP6_2 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP6_3 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP6_4 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP6_5 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP6_6 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP6_7 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP6_8 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP6_9 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP6_10 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP6_11 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP6_12 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP7_0 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP7_1 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP7_2 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP7_3 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP7_4 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP7_5 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP7_6 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP7_7 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP8_0 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP8_1 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP8_2 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP8_3 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP8_4 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP8_5 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP8_6 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP8_7 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP8_8 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSP9_0 [
    /// Select pin function
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
    /// Enable pull-down resistor at pad
    EPD OFFSET(3) NUMBITS(1) [
        /// Disable pull-down.
        DisablePullDown = 0,
        /// Enable pull-down.
        EnablePullDown = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up
        EnablePullUp = 0,
        /// Disable pull-up
        DisablePullUp = 1
    ],
    /// Slew rate
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow
        Slow = 0,
        /// Fast
        Fast = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset but must b
    EZI OFFSET(6) NUMBITS(1) [
        /// Disable input buffer
        DisableInputBuffer = 0,
        /// Enable input buffer
        EnableInputBuffer = 1
    ],
    /// Select drive strength
    EHD OFFSET(8) NUMBITS(2) [
        /// Standard drive: 4 mA drive strength
        StandardDrive4MADriveStrength = 0,
        /// Medium drive: 8 mA drive strength
        MediumDrive8MADriveStrength = 1,
        /// High drive: 14 mA drive strength
        HighDrive14MADriveStrength = 2,
        /// Ultra-high drive: 20 mA drive strength
        UltraHighDrive20MADriveStrength = 3
    ]
],
SFSP9_1 [
    /// Select pin function
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
    /// Enable pull-down resistor at pad
    EPD OFFSET(3) NUMBITS(1) [
        /// Disable pull-down.
        DisablePullDown = 0,
        /// Enable pull-down.
        EnablePullDown = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up
        EnablePullUp = 0,
        /// Disable pull-up
        DisablePullUp = 1
    ],
    /// Slew rate
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow
        Slow = 0,
        /// Fast
        Fast = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset but must b
    EZI OFFSET(6) NUMBITS(1) [
        /// Disable input buffer
        DisableInputBuffer = 0,
        /// Enable input buffer
        EnableInputBuffer = 1
    ],
    /// Select drive strength
    EHD OFFSET(8) NUMBITS(2) [
        /// Standard drive: 4 mA drive strength
        StandardDrive4MADriveStrength = 0,
        /// Medium drive: 8 mA drive strength
        MediumDrive8MADriveStrength = 1,
        /// High drive: 14 mA drive strength
        HighDrive14MADriveStrength = 2,
        /// Ultra-high drive: 20 mA drive strength
        UltraHighDrive20MADriveStrength = 3
    ]
],
SFSP9_2 [
    /// Select pin function
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
    /// Enable pull-down resistor at pad
    EPD OFFSET(3) NUMBITS(1) [
        /// Disable pull-down.
        DisablePullDown = 0,
        /// Enable pull-down.
        EnablePullDown = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up
        EnablePullUp = 0,
        /// Disable pull-up
        DisablePullUp = 1
    ],
    /// Slew rate
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow
        Slow = 0,
        /// Fast
        Fast = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset but must b
    EZI OFFSET(6) NUMBITS(1) [
        /// Disable input buffer
        DisableInputBuffer = 0,
        /// Enable input buffer
        EnableInputBuffer = 1
    ],
    /// Select drive strength
    EHD OFFSET(8) NUMBITS(2) [
        /// Standard drive: 4 mA drive strength
        StandardDrive4MADriveStrength = 0,
        /// Medium drive: 8 mA drive strength
        MediumDrive8MADriveStrength = 1,
        /// High drive: 14 mA drive strength
        HighDrive14MADriveStrength = 2,
        /// Ultra-high drive: 20 mA drive strength
        UltraHighDrive20MADriveStrength = 3
    ]
],
SFSP9_3 [
    /// Select pin function
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
    /// Enable pull-down resistor at pad
    EPD OFFSET(3) NUMBITS(1) [
        /// Disable pull-down.
        DisablePullDown = 0,
        /// Enable pull-down.
        EnablePullDown = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up
        EnablePullUp = 0,
        /// Disable pull-up
        DisablePullUp = 1
    ],
    /// Slew rate
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow
        Slow = 0,
        /// Fast
        Fast = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset but must b
    EZI OFFSET(6) NUMBITS(1) [
        /// Disable input buffer
        DisableInputBuffer = 0,
        /// Enable input buffer
        EnableInputBuffer = 1
    ],
    /// Select drive strength
    EHD OFFSET(8) NUMBITS(2) [
        /// Standard drive: 4 mA drive strength
        StandardDrive4MADriveStrength = 0,
        /// Medium drive: 8 mA drive strength
        MediumDrive8MADriveStrength = 1,
        /// High drive: 14 mA drive strength
        HighDrive14MADriveStrength = 2,
        /// Ultra-high drive: 20 mA drive strength
        UltraHighDrive20MADriveStrength = 3
    ]
],
SFSP9_4 [
    /// Select pin function
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
    /// Enable pull-down resistor at pad
    EPD OFFSET(3) NUMBITS(1) [
        /// Disable pull-down.
        DisablePullDown = 0,
        /// Enable pull-down.
        EnablePullDown = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up
        EnablePullUp = 0,
        /// Disable pull-up
        DisablePullUp = 1
    ],
    /// Slew rate
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow
        Slow = 0,
        /// Fast
        Fast = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset but must b
    EZI OFFSET(6) NUMBITS(1) [
        /// Disable input buffer
        DisableInputBuffer = 0,
        /// Enable input buffer
        EnableInputBuffer = 1
    ],
    /// Select drive strength
    EHD OFFSET(8) NUMBITS(2) [
        /// Standard drive: 4 mA drive strength
        StandardDrive4MADriveStrength = 0,
        /// Medium drive: 8 mA drive strength
        MediumDrive8MADriveStrength = 1,
        /// High drive: 14 mA drive strength
        HighDrive14MADriveStrength = 2,
        /// Ultra-high drive: 20 mA drive strength
        UltraHighDrive20MADriveStrength = 3
    ]
],
SFSP9_5 [
    /// Select pin function
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
    /// Enable pull-down resistor at pad
    EPD OFFSET(3) NUMBITS(1) [
        /// Disable pull-down.
        DisablePullDown = 0,
        /// Enable pull-down.
        EnablePullDown = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up
        EnablePullUp = 0,
        /// Disable pull-up
        DisablePullUp = 1
    ],
    /// Slew rate
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow
        Slow = 0,
        /// Fast
        Fast = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset but must b
    EZI OFFSET(6) NUMBITS(1) [
        /// Disable input buffer
        DisableInputBuffer = 0,
        /// Enable input buffer
        EnableInputBuffer = 1
    ],
    /// Select drive strength
    EHD OFFSET(8) NUMBITS(2) [
        /// Standard drive: 4 mA drive strength
        StandardDrive4MADriveStrength = 0,
        /// Medium drive: 8 mA drive strength
        MediumDrive8MADriveStrength = 1,
        /// High drive: 14 mA drive strength
        HighDrive14MADriveStrength = 2,
        /// Ultra-high drive: 20 mA drive strength
        UltraHighDrive20MADriveStrength = 3
    ]
],
SFSP9_6 [
    /// Select pin function
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
    /// Enable pull-down resistor at pad
    EPD OFFSET(3) NUMBITS(1) [
        /// Disable pull-down.
        DisablePullDown = 0,
        /// Enable pull-down.
        EnablePullDown = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up
        EnablePullUp = 0,
        /// Disable pull-up
        DisablePullUp = 1
    ],
    /// Slew rate
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow
        Slow = 0,
        /// Fast
        Fast = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset but must b
    EZI OFFSET(6) NUMBITS(1) [
        /// Disable input buffer
        DisableInputBuffer = 0,
        /// Enable input buffer
        EnableInputBuffer = 1
    ],
    /// Select drive strength
    EHD OFFSET(8) NUMBITS(2) [
        /// Standard drive: 4 mA drive strength
        StandardDrive4MADriveStrength = 0,
        /// Medium drive: 8 mA drive strength
        MediumDrive8MADriveStrength = 1,
        /// High drive: 14 mA drive strength
        HighDrive14MADriveStrength = 2,
        /// Ultra-high drive: 20 mA drive strength
        UltraHighDrive20MADriveStrength = 3
    ]
],
SFSPA_1 [
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
SFSPA_2 [
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
SFSPA_3 [
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
SFSPB_0 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPB_1 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPB_2 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPB_3 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPB_4 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPB_5 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPB_6 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPC_0 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPC_1 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPC_2 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPC_3 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPC_4 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPC_5 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPC_6 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPC_7 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPC_8 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPC_9 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPC_10 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPC_11 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPC_12 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPC_13 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPC_14 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPD_0 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPD_1 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPD_2 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPD_3 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPD_4 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPD_5 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPD_6 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPD_7 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPD_8 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPD_9 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPD_10 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPD_11 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPD_12 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPD_13 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPD_14 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPD_15 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPD_16 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPE_0 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPE_1 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPE_2 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPE_3 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPE_4 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPE_5 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPE_6 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPE_7 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPE_8 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPE_9 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPE_10 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPE_11 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPE_12 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPE_13 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPE_14 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPE_15 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPF_0 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPF_1 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPF_2 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPF_3 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPF_4 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPF_5 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPF_6 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPF_7 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPF_8 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPF_9 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPF_10 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSPF_11 [
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
        /// Enable pull-down.Enable both pull-down resistor and pull-up resistor for repeate
        EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 1
    ],
    /// Disable pull-up resistor at pad. By default, the pull-up resistor is enabled at
    EPUN OFFSET(4) NUMBITS(1) [
        /// Enable pull-up. Enable both pull-down resistor and pull-up resistor for repeater
        EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode = 0,
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Select Slew rate.
    EHS OFFSET(5) NUMBITS(1) [
        /// Slow (low noise with medium speed)
        SlowLowNoiseWithMediumSpeed = 0,
        /// Fast (medium noise with fast speed)
        FastMediumNoiseWithFastSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
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
    ]
],
SFSCLK_0 [
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
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Slew rate
    EHS OFFSET(5) NUMBITS(1) [
        /// Fast (low noise with fast speed)
        FastLowNoiseWithFastSpeed = 0,
        /// High-speed (medium noise with high speed)
        HighSpeedMediumNoiseWithHighSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
    EZI OFFSET(6) NUMBITS(1) [
        /// Disable input buffer
        DisableInputBuffer = 0,
        /// Enable input buffer
        EnableInputBuffer = 1
    ],
    /// Input glitch filter. Disable the input glitch filter for clocking signals higher
    ZIF OFFSET(7) NUMBITS(1) [
        /// Enable input filter
        EnableInputFilter = 0,
        /// Disable input filter
        DisableInputFilter = 1
    ]
],
SFSCLK_1 [
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
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Slew rate
    EHS OFFSET(5) NUMBITS(1) [
        /// Fast (low noise with fast speed)
        FastLowNoiseWithFastSpeed = 0,
        /// High-speed (medium noise with high speed)
        HighSpeedMediumNoiseWithHighSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
    EZI OFFSET(6) NUMBITS(1) [
        /// Disable input buffer
        DisableInputBuffer = 0,
        /// Enable input buffer
        EnableInputBuffer = 1
    ],
    /// Input glitch filter. Disable the input glitch filter for clocking signals higher
    ZIF OFFSET(7) NUMBITS(1) [
        /// Enable input filter
        EnableInputFilter = 0,
        /// Disable input filter
        DisableInputFilter = 1
    ]
],
SFSCLK_2 [
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
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Slew rate
    EHS OFFSET(5) NUMBITS(1) [
        /// Fast (low noise with fast speed)
        FastLowNoiseWithFastSpeed = 0,
        /// High-speed (medium noise with high speed)
        HighSpeedMediumNoiseWithHighSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
    EZI OFFSET(6) NUMBITS(1) [
        /// Disable input buffer
        DisableInputBuffer = 0,
        /// Enable input buffer
        EnableInputBuffer = 1
    ],
    /// Input glitch filter. Disable the input glitch filter for clocking signals higher
    ZIF OFFSET(7) NUMBITS(1) [
        /// Enable input filter
        EnableInputFilter = 0,
        /// Disable input filter
        DisableInputFilter = 1
    ]
],
SFSCLK_3 [
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
        /// Disable pull-up.
        DisablePullUp = 1
    ],
    /// Slew rate
    EHS OFFSET(5) NUMBITS(1) [
        /// Fast (low noise with fast speed)
        FastLowNoiseWithFastSpeed = 0,
        /// High-speed (medium noise with high speed)
        HighSpeedMediumNoiseWithHighSpeed = 1
    ],
    /// Input buffer enable. The input buffer is disabled by default at reset and must b
    EZI OFFSET(6) NUMBITS(1) [
        /// Disable input buffer
        DisableInputBuffer = 0,
        /// Enable input buffer
        EnableInputBuffer = 1
    ],
    /// Input glitch filter. Disable the input glitch filter for clocking signals higher
    ZIF OFFSET(7) NUMBITS(1) [
        /// Enable input filter
        EnableInputFilter = 0,
        /// Disable input filter
        DisableInputFilter = 1
    ]
]
];
const SCU_BASE: StaticRef<ScuRegisters> =
    unsafe { StaticRef::new(0x40086000 as *const ScuRegisters) };
