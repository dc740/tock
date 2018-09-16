
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// Serial GPIO (SGPIO)
#[repr(C)]
struct SgpioRegisters {
/// Pin multiplexer configuration registers.
out_mux_cfg_0: ReadWrite<u32, OUT_MUX_CFG[0]::Register>,
/// Pin multiplexer configuration registers.
out_mux_cfg_1: ReadWrite<u32, OUT_MUX_CFG[1]::Register>,
/// Pin multiplexer configuration registers.
out_mux_cfg_2: ReadWrite<u32, OUT_MUX_CFG[2]::Register>,
/// Pin multiplexer configuration registers.
out_mux_cfg_3: ReadWrite<u32, OUT_MUX_CFG[3]::Register>,
/// Pin multiplexer configuration registers.
out_mux_cfg_4: ReadWrite<u32, OUT_MUX_CFG[4]::Register>,
/// Pin multiplexer configuration registers.
out_mux_cfg_5: ReadWrite<u32, OUT_MUX_CFG[5]::Register>,
/// Pin multiplexer configuration registers.
out_mux_cfg_6: ReadWrite<u32, OUT_MUX_CFG[6]::Register>,
/// Pin multiplexer configuration registers.
out_mux_cfg_7: ReadWrite<u32, OUT_MUX_CFG[7]::Register>,
/// Pin multiplexer configuration registers.
out_mux_cfg_8: ReadWrite<u32, OUT_MUX_CFG[8]::Register>,
/// Pin multiplexer configuration registers.
out_mux_cfg_9: ReadWrite<u32, OUT_MUX_CFG[9]::Register>,
/// Pin multiplexer configuration registers.
out_mux_cfg_10: ReadWrite<u32, OUT_MUX_CFG[10]::Register>,
/// Pin multiplexer configuration registers.
out_mux_cfg_11: ReadWrite<u32, OUT_MUX_CFG[11]::Register>,
/// Pin multiplexer configuration registers.
out_mux_cfg_12: ReadWrite<u32, OUT_MUX_CFG[12]::Register>,
/// Pin multiplexer configuration registers.
out_mux_cfg_13: ReadWrite<u32, OUT_MUX_CFG[13]::Register>,
/// Pin multiplexer configuration registers.
out_mux_cfg_14: ReadWrite<u32, OUT_MUX_CFG[14]::Register>,
/// Pin multiplexer configuration registers.
out_mux_cfg_15: ReadWrite<u32, OUT_MUX_CFG[15]::Register>,
/// SGPIO multiplexer configuration registers.
sgpio_mux_cfg_0: ReadWrite<u32, SGPIO_MUX_CFG[0]::Register>,
/// SGPIO multiplexer configuration registers.
sgpio_mux_cfg_1: ReadWrite<u32, SGPIO_MUX_CFG[1]::Register>,
/// SGPIO multiplexer configuration registers.
sgpio_mux_cfg_2: ReadWrite<u32, SGPIO_MUX_CFG[2]::Register>,
/// SGPIO multiplexer configuration registers.
sgpio_mux_cfg_3: ReadWrite<u32, SGPIO_MUX_CFG[3]::Register>,
/// SGPIO multiplexer configuration registers.
sgpio_mux_cfg_4: ReadWrite<u32, SGPIO_MUX_CFG[4]::Register>,
/// SGPIO multiplexer configuration registers.
sgpio_mux_cfg_5: ReadWrite<u32, SGPIO_MUX_CFG[5]::Register>,
/// SGPIO multiplexer configuration registers.
sgpio_mux_cfg_6: ReadWrite<u32, SGPIO_MUX_CFG[6]::Register>,
/// SGPIO multiplexer configuration registers.
sgpio_mux_cfg_7: ReadWrite<u32, SGPIO_MUX_CFG[7]::Register>,
/// SGPIO multiplexer configuration registers.
sgpio_mux_cfg_8: ReadWrite<u32, SGPIO_MUX_CFG[8]::Register>,
/// SGPIO multiplexer configuration registers.
sgpio_mux_cfg_9: ReadWrite<u32, SGPIO_MUX_CFG[9]::Register>,
/// SGPIO multiplexer configuration registers.
sgpio_mux_cfg_10: ReadWrite<u32, SGPIO_MUX_CFG[10]::Register>,
/// SGPIO multiplexer configuration registers.
sgpio_mux_cfg_11: ReadWrite<u32, SGPIO_MUX_CFG[11]::Register>,
/// SGPIO multiplexer configuration registers.
sgpio_mux_cfg_12: ReadWrite<u32, SGPIO_MUX_CFG[12]::Register>,
/// SGPIO multiplexer configuration registers.
sgpio_mux_cfg_13: ReadWrite<u32, SGPIO_MUX_CFG[13]::Register>,
/// SGPIO multiplexer configuration registers.
sgpio_mux_cfg_14: ReadWrite<u32, SGPIO_MUX_CFG[14]::Register>,
/// SGPIO multiplexer configuration registers.
sgpio_mux_cfg_15: ReadWrite<u32, SGPIO_MUX_CFG[15]::Register>,
/// Slice multiplexer configuration registers.
slice_mux_cfg_0: ReadWrite<u32, SLICE_MUX_CFG[0]::Register>,
/// Slice multiplexer configuration registers.
slice_mux_cfg_1: ReadWrite<u32, SLICE_MUX_CFG[1]::Register>,
/// Slice multiplexer configuration registers.
slice_mux_cfg_2: ReadWrite<u32, SLICE_MUX_CFG[2]::Register>,
/// Slice multiplexer configuration registers.
slice_mux_cfg_3: ReadWrite<u32, SLICE_MUX_CFG[3]::Register>,
/// Slice multiplexer configuration registers.
slice_mux_cfg_4: ReadWrite<u32, SLICE_MUX_CFG[4]::Register>,
/// Slice multiplexer configuration registers.
slice_mux_cfg_5: ReadWrite<u32, SLICE_MUX_CFG[5]::Register>,
/// Slice multiplexer configuration registers.
slice_mux_cfg_6: ReadWrite<u32, SLICE_MUX_CFG[6]::Register>,
/// Slice multiplexer configuration registers.
slice_mux_cfg_7: ReadWrite<u32, SLICE_MUX_CFG[7]::Register>,
/// Slice multiplexer configuration registers.
slice_mux_cfg_8: ReadWrite<u32, SLICE_MUX_CFG[8]::Register>,
/// Slice multiplexer configuration registers.
slice_mux_cfg_9: ReadWrite<u32, SLICE_MUX_CFG[9]::Register>,
/// Slice multiplexer configuration registers.
slice_mux_cfg_10: ReadWrite<u32, SLICE_MUX_CFG[10]::Register>,
/// Slice multiplexer configuration registers.
slice_mux_cfg_11: ReadWrite<u32, SLICE_MUX_CFG[11]::Register>,
/// Slice multiplexer configuration registers.
slice_mux_cfg_12: ReadWrite<u32, SLICE_MUX_CFG[12]::Register>,
/// Slice multiplexer configuration registers.
slice_mux_cfg_13: ReadWrite<u32, SLICE_MUX_CFG[13]::Register>,
/// Slice multiplexer configuration registers.
slice_mux_cfg_14: ReadWrite<u32, SLICE_MUX_CFG[14]::Register>,
/// Slice multiplexer configuration registers.
slice_mux_cfg_15: ReadWrite<u32, SLICE_MUX_CFG[15]::Register>,
/// Slice data registers. Each time COUNT0 reaches 0x0 the register shifts loading b
reg_0: ReadWrite<u32>,
/// Slice data registers. Each time COUNT0 reaches 0x0 the register shifts loading b
reg_1: ReadWrite<u32>,
/// Slice data registers. Each time COUNT0 reaches 0x0 the register shifts loading b
reg_2: ReadWrite<u32>,
/// Slice data registers. Each time COUNT0 reaches 0x0 the register shifts loading b
reg_3: ReadWrite<u32>,
/// Slice data registers. Each time COUNT0 reaches 0x0 the register shifts loading b
reg_4: ReadWrite<u32>,
/// Slice data registers. Each time COUNT0 reaches 0x0 the register shifts loading b
reg_5: ReadWrite<u32>,
/// Slice data registers. Each time COUNT0 reaches 0x0 the register shifts loading b
reg_6: ReadWrite<u32>,
/// Slice data registers. Each time COUNT0 reaches 0x0 the register shifts loading b
reg_7: ReadWrite<u32>,
/// Slice data registers. Each time COUNT0 reaches 0x0 the register shifts loading b
reg_8: ReadWrite<u32>,
/// Slice data registers. Each time COUNT0 reaches 0x0 the register shifts loading b
reg_9: ReadWrite<u32>,
/// Slice data registers. Each time COUNT0 reaches 0x0 the register shifts loading b
reg_10: ReadWrite<u32>,
/// Slice data registers. Each time COUNT0 reaches 0x0 the register shifts loading b
reg_11: ReadWrite<u32>,
/// Slice data registers. Each time COUNT0 reaches 0x0 the register shifts loading b
reg_12: ReadWrite<u32>,
/// Slice data registers. Each time COUNT0 reaches 0x0 the register shifts loading b
reg_13: ReadWrite<u32>,
/// Slice data registers. Each time COUNT0 reaches 0x0 the register shifts loading b
reg_14: ReadWrite<u32>,
/// Slice data registers. Each time COUNT0 reaches 0x0 the register shifts loading b
reg_15: ReadWrite<u32>,
/// Slice data shadow registers. Each time POS reaches 0x0 the contents of REG_SS is
reg_ss_0: ReadWrite<u32>,
/// Slice data shadow registers. Each time POS reaches 0x0 the contents of REG_SS is
reg_ss_1: ReadWrite<u32>,
/// Slice data shadow registers. Each time POS reaches 0x0 the contents of REG_SS is
reg_ss_2: ReadWrite<u32>,
/// Slice data shadow registers. Each time POS reaches 0x0 the contents of REG_SS is
reg_ss_3: ReadWrite<u32>,
/// Slice data shadow registers. Each time POS reaches 0x0 the contents of REG_SS is
reg_ss_4: ReadWrite<u32>,
/// Slice data shadow registers. Each time POS reaches 0x0 the contents of REG_SS is
reg_ss_5: ReadWrite<u32>,
/// Slice data shadow registers. Each time POS reaches 0x0 the contents of REG_SS is
reg_ss_6: ReadWrite<u32>,
/// Slice data shadow registers. Each time POS reaches 0x0 the contents of REG_SS is
reg_ss_7: ReadWrite<u32>,
/// Slice data shadow registers. Each time POS reaches 0x0 the contents of REG_SS is
reg_ss_8: ReadWrite<u32>,
/// Slice data shadow registers. Each time POS reaches 0x0 the contents of REG_SS is
reg_ss_9: ReadWrite<u32>,
/// Slice data shadow registers. Each time POS reaches 0x0 the contents of REG_SS is
reg_ss_10: ReadWrite<u32>,
/// Slice data shadow registers. Each time POS reaches 0x0 the contents of REG_SS is
reg_ss_11: ReadWrite<u32>,
/// Slice data shadow registers. Each time POS reaches 0x0 the contents of REG_SS is
reg_ss_12: ReadWrite<u32>,
/// Slice data shadow registers. Each time POS reaches 0x0 the contents of REG_SS is
reg_ss_13: ReadWrite<u32>,
/// Slice data shadow registers. Each time POS reaches 0x0 the contents of REG_SS is
reg_ss_14: ReadWrite<u32>,
/// Slice data shadow registers. Each time POS reaches 0x0 the contents of REG_SS is
reg_ss_15: ReadWrite<u32>,
/// Reload value of COUNT0, loaded when COUNT0 reaches 0x0
preset_0: ReadWrite<u32>,
/// Reload value of COUNT0, loaded when COUNT0 reaches 0x0
preset_1: ReadWrite<u32>,
/// Reload value of COUNT0, loaded when COUNT0 reaches 0x0
preset_2: ReadWrite<u32>,
/// Reload value of COUNT0, loaded when COUNT0 reaches 0x0
preset_3: ReadWrite<u32>,
/// Reload value of COUNT0, loaded when COUNT0 reaches 0x0
preset_4: ReadWrite<u32>,
/// Reload value of COUNT0, loaded when COUNT0 reaches 0x0
preset_5: ReadWrite<u32>,
/// Reload value of COUNT0, loaded when COUNT0 reaches 0x0
preset_6: ReadWrite<u32>,
/// Reload value of COUNT0, loaded when COUNT0 reaches 0x0
preset_7: ReadWrite<u32>,
/// Reload value of COUNT0, loaded when COUNT0 reaches 0x0
preset_8: ReadWrite<u32>,
/// Reload value of COUNT0, loaded when COUNT0 reaches 0x0
preset_9: ReadWrite<u32>,
/// Reload value of COUNT0, loaded when COUNT0 reaches 0x0
preset_10: ReadWrite<u32>,
/// Reload value of COUNT0, loaded when COUNT0 reaches 0x0
preset_11: ReadWrite<u32>,
/// Reload value of COUNT0, loaded when COUNT0 reaches 0x0
preset_12: ReadWrite<u32>,
/// Reload value of COUNT0, loaded when COUNT0 reaches 0x0
preset_13: ReadWrite<u32>,
/// Reload value of COUNT0, loaded when COUNT0 reaches 0x0
preset_14: ReadWrite<u32>,
/// Reload value of COUNT0, loaded when COUNT0 reaches 0x0
preset_15: ReadWrite<u32>,
/// Down counter, counts down each clock cycle.
count_0: ReadWrite<u32>,
/// Down counter, counts down each clock cycle.
count_1: ReadWrite<u32>,
/// Down counter, counts down each clock cycle.
count_2: ReadWrite<u32>,
/// Down counter, counts down each clock cycle.
count_3: ReadWrite<u32>,
/// Down counter, counts down each clock cycle.
count_4: ReadWrite<u32>,
/// Down counter, counts down each clock cycle.
count_5: ReadWrite<u32>,
/// Down counter, counts down each clock cycle.
count_6: ReadWrite<u32>,
/// Down counter, counts down each clock cycle.
count_7: ReadWrite<u32>,
/// Down counter, counts down each clock cycle.
count_8: ReadWrite<u32>,
/// Down counter, counts down each clock cycle.
count_9: ReadWrite<u32>,
/// Down counter, counts down each clock cycle.
count_10: ReadWrite<u32>,
/// Down counter, counts down each clock cycle.
count_11: ReadWrite<u32>,
/// Down counter, counts down each clock cycle.
count_12: ReadWrite<u32>,
/// Down counter, counts down each clock cycle.
count_13: ReadWrite<u32>,
/// Down counter, counts down each clock cycle.
count_14: ReadWrite<u32>,
/// Down counter, counts down each clock cycle.
count_15: ReadWrite<u32>,
/// Each time COUNT0 reaches 0x0 POS counts down.
pos_0: ReadWrite<u32, POS[0]::Register>,
/// Each time COUNT0 reaches 0x0 POS counts down.
pos_1: ReadWrite<u32, POS[1]::Register>,
/// Each time COUNT0 reaches 0x0 POS counts down.
pos_2: ReadWrite<u32, POS[2]::Register>,
/// Each time COUNT0 reaches 0x0 POS counts down.
pos_3: ReadWrite<u32, POS[3]::Register>,
/// Each time COUNT0 reaches 0x0 POS counts down.
pos_4: ReadWrite<u32, POS[4]::Register>,
/// Each time COUNT0 reaches 0x0 POS counts down.
pos_5: ReadWrite<u32, POS[5]::Register>,
/// Each time COUNT0 reaches 0x0 POS counts down.
pos_6: ReadWrite<u32, POS[6]::Register>,
/// Each time COUNT0 reaches 0x0 POS counts down.
pos_7: ReadWrite<u32, POS[7]::Register>,
/// Each time COUNT0 reaches 0x0 POS counts down.
pos_8: ReadWrite<u32, POS[8]::Register>,
/// Each time COUNT0 reaches 0x0 POS counts down.
pos_9: ReadWrite<u32, POS[9]::Register>,
/// Each time COUNT0 reaches 0x0 POS counts down.
pos_10: ReadWrite<u32, POS[10]::Register>,
/// Each time COUNT0 reaches 0x0 POS counts down.
pos_11: ReadWrite<u32, POS[11]::Register>,
/// Each time COUNT0 reaches 0x0 POS counts down.
pos_12: ReadWrite<u32, POS[12]::Register>,
/// Each time COUNT0 reaches 0x0 POS counts down.
pos_13: ReadWrite<u32, POS[13]::Register>,
/// Each time COUNT0 reaches 0x0 POS counts down.
pos_14: ReadWrite<u32, POS[14]::Register>,
/// Each time COUNT0 reaches 0x0 POS counts down.
pos_15: ReadWrite<u32, POS[15]::Register>,
/// Mask for pattern match function of slice A
mask_a: ReadWrite<u32>,
/// Mask for pattern match function of slice H
mask_h: ReadWrite<u32>,
/// Mask for pattern match function of slice I
mask_i: ReadWrite<u32>,
/// Mask for pattern match function of slice P
mask_p: ReadWrite<u32>,
/// GPIO input status register
gpio_inreg: ReadOnly<u32>,
/// GPIO output control register
gpio_outreg: ReadWrite<u32>,
/// GPIO OE control register
gpio_oenreg: ReadWrite<u32>,
/// Enables the slice COUNT counter
ctrl_enable: ReadWrite<u32>,
/// Disables the slice POS counter
ctrl_disable: ReadWrite<u32>,
_reserved0: [u8; 3292],
/// Shift clock interrupt clear mask
clr_en_0: WriteOnly<u32>,
/// Shift clock interrupt set mask
set_en_0: WriteOnly<u32>,
/// Shift clock interrupt enable
enable_0: ReadOnly<u32>,
/// Shift clock interrupt status
status_0: ReadOnly<u32>,
/// Shift clock interrupt clear status
clr_status_0: WriteOnly<u32>,
/// Shift clock interrupt set status
set_status_0: WriteOnly<u32>,
_reserved1: [u8; 8],
/// Exchange clock interrupt clear mask
clr_en_1: WriteOnly<u32>,
/// Exchange clock interrupt set mask
set_en_1: WriteOnly<u32>,
/// Exchange clock interrupt enable
enable_1: ReadOnly<u32>,
/// Exchange clock interrupt status
status_1: ReadOnly<u32>,
/// Exchange clock interrupt clear status
clr_status_1: WriteOnly<u32>,
/// Exchange clock interrupt set status
set_status_1: WriteOnly<u32>,
_reserved2: [u8; 8],
/// Pattern match interrupt clear mask
clr_en_2: WriteOnly<u32>,
/// Pattern match interrupt set mask
set_en_2: WriteOnly<u32>,
/// Pattern match interrupt enable
enable_2: ReadOnly<u32>,
/// Pattern match interrupt status
status_2: ReadOnly<u32>,
/// Pattern match interrupt clear status
clr_status_2: WriteOnly<u32>,
/// Pattern match interrupt set status
set_status_2: WriteOnly<u32>,
_reserved3: [u8; 8],
/// Input interrupt clear mask
clr_en_3: WriteOnly<u32>,
/// Input bit match interrupt set mask
set_en_3: WriteOnly<u32>,
/// Input bit match interrupt enable
enable_3: ReadOnly<u32>,
/// Input bit match interrupt status
status_3: ReadOnly<u32>,
/// Input bit match interrupt clear status
clr_status_3: WriteOnly<u32>,
/// Input bit match interrupt set status
set_status_3: WriteOnly<u32>,
}
register_bitfields![u32,
OUT_MUX_CFG[0] [
    /// Output control of output SGPIOn. All other values are reserved.
    P_OUT_CFG OFFSET(0) NUMBITS(4) [
        /// dout_doutm1 (1-bit mode)
        Dout_doutm11BitMode = 0,
        /// dout_doutm2a (2-bit mode 2a)
        Dout_doutm2a2BitMode2a = 1,
        /// dout_doutm2b (2-bit mode 2b)
        Dout_doutm2b2BitMode2b = 2,
        /// dout_doutm2c (2-bit mode 2c)
        Dout_doutm2c2BitMode2c = 3,
        /// gpio_out (level set by GPIO_OUTREG)
        Gpio_outLevelSetByGPIO_OUTREG = 4,
        /// dout_doutm4a (4-bit mode 4a)
        Dout_doutm4a4BitMode4a = 5,
        /// dout_doutm4b (4-bit mode 4b)
        Dout_doutm4b4BitMode4b = 6,
        /// dout_doutm4c (4-bit mode 4c)
        Dout_doutm4c4BitMode4c = 7,
        /// clk_out
        Clk_out = 8,
        /// dout_doutm8a (8-bit mode 8a)
        Dout_doutm8a8BitMode8a = 9,
        /// dout_doutm8b (8-bit mode 8b)
        Dout_doutm8b8BitMode8b = 10,
        /// dout_doutm8c (8-bit mode 8c)
        Dout_doutm8c8BitMode8c = 11
    ],
    /// Output enable source. All other values are reserved.
    P_OE_CFG OFFSET(4) NUMBITS(3) [
        /// gpio_oe (state set by GPIO_OEREG)
        Gpio_oeStateSetByGPIO_OEREG = 0,
        /// dout_oem1 (1-bit mode)
        Dout_oem11BitMode = 4,
        /// dout_oem2 (2-bit mode)
        Dout_oem22BitMode = 5,
        /// dout_oem4 (4-bit mode)
        Dout_oem44BitMode = 6,
        /// dout_oem8 (8-bit mode)
        Dout_oem88BitMode = 7
    ]
],
OUT_MUX_CFG[1] [
    /// Output control of output SGPIOn. All other values are reserved.
    P_OUT_CFG OFFSET(0) NUMBITS(4) [
        /// dout_doutm1 (1-bit mode)
        Dout_doutm11BitMode = 0,
        /// dout_doutm2a (2-bit mode 2a)
        Dout_doutm2a2BitMode2a = 1,
        /// dout_doutm2b (2-bit mode 2b)
        Dout_doutm2b2BitMode2b = 2,
        /// dout_doutm2c (2-bit mode 2c)
        Dout_doutm2c2BitMode2c = 3,
        /// gpio_out (level set by GPIO_OUTREG)
        Gpio_outLevelSetByGPIO_OUTREG = 4,
        /// dout_doutm4a (4-bit mode 4a)
        Dout_doutm4a4BitMode4a = 5,
        /// dout_doutm4b (4-bit mode 4b)
        Dout_doutm4b4BitMode4b = 6,
        /// dout_doutm4c (4-bit mode 4c)
        Dout_doutm4c4BitMode4c = 7,
        /// clk_out
        Clk_out = 8,
        /// dout_doutm8a (8-bit mode 8a)
        Dout_doutm8a8BitMode8a = 9,
        /// dout_doutm8b (8-bit mode 8b)
        Dout_doutm8b8BitMode8b = 10,
        /// dout_doutm8c (8-bit mode 8c)
        Dout_doutm8c8BitMode8c = 11
    ],
    /// Output enable source. All other values are reserved.
    P_OE_CFG OFFSET(4) NUMBITS(3) [
        /// gpio_oe (state set by GPIO_OEREG)
        Gpio_oeStateSetByGPIO_OEREG = 0,
        /// dout_oem1 (1-bit mode)
        Dout_oem11BitMode = 4,
        /// dout_oem2 (2-bit mode)
        Dout_oem22BitMode = 5,
        /// dout_oem4 (4-bit mode)
        Dout_oem44BitMode = 6,
        /// dout_oem8 (8-bit mode)
        Dout_oem88BitMode = 7
    ]
],
OUT_MUX_CFG[2] [
    /// Output control of output SGPIOn. All other values are reserved.
    P_OUT_CFG OFFSET(0) NUMBITS(4) [
        /// dout_doutm1 (1-bit mode)
        Dout_doutm11BitMode = 0,
        /// dout_doutm2a (2-bit mode 2a)
        Dout_doutm2a2BitMode2a = 1,
        /// dout_doutm2b (2-bit mode 2b)
        Dout_doutm2b2BitMode2b = 2,
        /// dout_doutm2c (2-bit mode 2c)
        Dout_doutm2c2BitMode2c = 3,
        /// gpio_out (level set by GPIO_OUTREG)
        Gpio_outLevelSetByGPIO_OUTREG = 4,
        /// dout_doutm4a (4-bit mode 4a)
        Dout_doutm4a4BitMode4a = 5,
        /// dout_doutm4b (4-bit mode 4b)
        Dout_doutm4b4BitMode4b = 6,
        /// dout_doutm4c (4-bit mode 4c)
        Dout_doutm4c4BitMode4c = 7,
        /// clk_out
        Clk_out = 8,
        /// dout_doutm8a (8-bit mode 8a)
        Dout_doutm8a8BitMode8a = 9,
        /// dout_doutm8b (8-bit mode 8b)
        Dout_doutm8b8BitMode8b = 10,
        /// dout_doutm8c (8-bit mode 8c)
        Dout_doutm8c8BitMode8c = 11
    ],
    /// Output enable source. All other values are reserved.
    P_OE_CFG OFFSET(4) NUMBITS(3) [
        /// gpio_oe (state set by GPIO_OEREG)
        Gpio_oeStateSetByGPIO_OEREG = 0,
        /// dout_oem1 (1-bit mode)
        Dout_oem11BitMode = 4,
        /// dout_oem2 (2-bit mode)
        Dout_oem22BitMode = 5,
        /// dout_oem4 (4-bit mode)
        Dout_oem44BitMode = 6,
        /// dout_oem8 (8-bit mode)
        Dout_oem88BitMode = 7
    ]
],
OUT_MUX_CFG[3] [
    /// Output control of output SGPIOn. All other values are reserved.
    P_OUT_CFG OFFSET(0) NUMBITS(4) [
        /// dout_doutm1 (1-bit mode)
        Dout_doutm11BitMode = 0,
        /// dout_doutm2a (2-bit mode 2a)
        Dout_doutm2a2BitMode2a = 1,
        /// dout_doutm2b (2-bit mode 2b)
        Dout_doutm2b2BitMode2b = 2,
        /// dout_doutm2c (2-bit mode 2c)
        Dout_doutm2c2BitMode2c = 3,
        /// gpio_out (level set by GPIO_OUTREG)
        Gpio_outLevelSetByGPIO_OUTREG = 4,
        /// dout_doutm4a (4-bit mode 4a)
        Dout_doutm4a4BitMode4a = 5,
        /// dout_doutm4b (4-bit mode 4b)
        Dout_doutm4b4BitMode4b = 6,
        /// dout_doutm4c (4-bit mode 4c)
        Dout_doutm4c4BitMode4c = 7,
        /// clk_out
        Clk_out = 8,
        /// dout_doutm8a (8-bit mode 8a)
        Dout_doutm8a8BitMode8a = 9,
        /// dout_doutm8b (8-bit mode 8b)
        Dout_doutm8b8BitMode8b = 10,
        /// dout_doutm8c (8-bit mode 8c)
        Dout_doutm8c8BitMode8c = 11
    ],
    /// Output enable source. All other values are reserved.
    P_OE_CFG OFFSET(4) NUMBITS(3) [
        /// gpio_oe (state set by GPIO_OEREG)
        Gpio_oeStateSetByGPIO_OEREG = 0,
        /// dout_oem1 (1-bit mode)
        Dout_oem11BitMode = 4,
        /// dout_oem2 (2-bit mode)
        Dout_oem22BitMode = 5,
        /// dout_oem4 (4-bit mode)
        Dout_oem44BitMode = 6,
        /// dout_oem8 (8-bit mode)
        Dout_oem88BitMode = 7
    ]
],
OUT_MUX_CFG[4] [
    /// Output control of output SGPIOn. All other values are reserved.
    P_OUT_CFG OFFSET(0) NUMBITS(4) [
        /// dout_doutm1 (1-bit mode)
        Dout_doutm11BitMode = 0,
        /// dout_doutm2a (2-bit mode 2a)
        Dout_doutm2a2BitMode2a = 1,
        /// dout_doutm2b (2-bit mode 2b)
        Dout_doutm2b2BitMode2b = 2,
        /// dout_doutm2c (2-bit mode 2c)
        Dout_doutm2c2BitMode2c = 3,
        /// gpio_out (level set by GPIO_OUTREG)
        Gpio_outLevelSetByGPIO_OUTREG = 4,
        /// dout_doutm4a (4-bit mode 4a)
        Dout_doutm4a4BitMode4a = 5,
        /// dout_doutm4b (4-bit mode 4b)
        Dout_doutm4b4BitMode4b = 6,
        /// dout_doutm4c (4-bit mode 4c)
        Dout_doutm4c4BitMode4c = 7,
        /// clk_out
        Clk_out = 8,
        /// dout_doutm8a (8-bit mode 8a)
        Dout_doutm8a8BitMode8a = 9,
        /// dout_doutm8b (8-bit mode 8b)
        Dout_doutm8b8BitMode8b = 10,
        /// dout_doutm8c (8-bit mode 8c)
        Dout_doutm8c8BitMode8c = 11
    ],
    /// Output enable source. All other values are reserved.
    P_OE_CFG OFFSET(4) NUMBITS(3) [
        /// gpio_oe (state set by GPIO_OEREG)
        Gpio_oeStateSetByGPIO_OEREG = 0,
        /// dout_oem1 (1-bit mode)
        Dout_oem11BitMode = 4,
        /// dout_oem2 (2-bit mode)
        Dout_oem22BitMode = 5,
        /// dout_oem4 (4-bit mode)
        Dout_oem44BitMode = 6,
        /// dout_oem8 (8-bit mode)
        Dout_oem88BitMode = 7
    ]
],
OUT_MUX_CFG[5] [
    /// Output control of output SGPIOn. All other values are reserved.
    P_OUT_CFG OFFSET(0) NUMBITS(4) [
        /// dout_doutm1 (1-bit mode)
        Dout_doutm11BitMode = 0,
        /// dout_doutm2a (2-bit mode 2a)
        Dout_doutm2a2BitMode2a = 1,
        /// dout_doutm2b (2-bit mode 2b)
        Dout_doutm2b2BitMode2b = 2,
        /// dout_doutm2c (2-bit mode 2c)
        Dout_doutm2c2BitMode2c = 3,
        /// gpio_out (level set by GPIO_OUTREG)
        Gpio_outLevelSetByGPIO_OUTREG = 4,
        /// dout_doutm4a (4-bit mode 4a)
        Dout_doutm4a4BitMode4a = 5,
        /// dout_doutm4b (4-bit mode 4b)
        Dout_doutm4b4BitMode4b = 6,
        /// dout_doutm4c (4-bit mode 4c)
        Dout_doutm4c4BitMode4c = 7,
        /// clk_out
        Clk_out = 8,
        /// dout_doutm8a (8-bit mode 8a)
        Dout_doutm8a8BitMode8a = 9,
        /// dout_doutm8b (8-bit mode 8b)
        Dout_doutm8b8BitMode8b = 10,
        /// dout_doutm8c (8-bit mode 8c)
        Dout_doutm8c8BitMode8c = 11
    ],
    /// Output enable source. All other values are reserved.
    P_OE_CFG OFFSET(4) NUMBITS(3) [
        /// gpio_oe (state set by GPIO_OEREG)
        Gpio_oeStateSetByGPIO_OEREG = 0,
        /// dout_oem1 (1-bit mode)
        Dout_oem11BitMode = 4,
        /// dout_oem2 (2-bit mode)
        Dout_oem22BitMode = 5,
        /// dout_oem4 (4-bit mode)
        Dout_oem44BitMode = 6,
        /// dout_oem8 (8-bit mode)
        Dout_oem88BitMode = 7
    ]
],
OUT_MUX_CFG[6] [
    /// Output control of output SGPIOn. All other values are reserved.
    P_OUT_CFG OFFSET(0) NUMBITS(4) [
        /// dout_doutm1 (1-bit mode)
        Dout_doutm11BitMode = 0,
        /// dout_doutm2a (2-bit mode 2a)
        Dout_doutm2a2BitMode2a = 1,
        /// dout_doutm2b (2-bit mode 2b)
        Dout_doutm2b2BitMode2b = 2,
        /// dout_doutm2c (2-bit mode 2c)
        Dout_doutm2c2BitMode2c = 3,
        /// gpio_out (level set by GPIO_OUTREG)
        Gpio_outLevelSetByGPIO_OUTREG = 4,
        /// dout_doutm4a (4-bit mode 4a)
        Dout_doutm4a4BitMode4a = 5,
        /// dout_doutm4b (4-bit mode 4b)
        Dout_doutm4b4BitMode4b = 6,
        /// dout_doutm4c (4-bit mode 4c)
        Dout_doutm4c4BitMode4c = 7,
        /// clk_out
        Clk_out = 8,
        /// dout_doutm8a (8-bit mode 8a)
        Dout_doutm8a8BitMode8a = 9,
        /// dout_doutm8b (8-bit mode 8b)
        Dout_doutm8b8BitMode8b = 10,
        /// dout_doutm8c (8-bit mode 8c)
        Dout_doutm8c8BitMode8c = 11
    ],
    /// Output enable source. All other values are reserved.
    P_OE_CFG OFFSET(4) NUMBITS(3) [
        /// gpio_oe (state set by GPIO_OEREG)
        Gpio_oeStateSetByGPIO_OEREG = 0,
        /// dout_oem1 (1-bit mode)
        Dout_oem11BitMode = 4,
        /// dout_oem2 (2-bit mode)
        Dout_oem22BitMode = 5,
        /// dout_oem4 (4-bit mode)
        Dout_oem44BitMode = 6,
        /// dout_oem8 (8-bit mode)
        Dout_oem88BitMode = 7
    ]
],
OUT_MUX_CFG[7] [
    /// Output control of output SGPIOn. All other values are reserved.
    P_OUT_CFG OFFSET(0) NUMBITS(4) [
        /// dout_doutm1 (1-bit mode)
        Dout_doutm11BitMode = 0,
        /// dout_doutm2a (2-bit mode 2a)
        Dout_doutm2a2BitMode2a = 1,
        /// dout_doutm2b (2-bit mode 2b)
        Dout_doutm2b2BitMode2b = 2,
        /// dout_doutm2c (2-bit mode 2c)
        Dout_doutm2c2BitMode2c = 3,
        /// gpio_out (level set by GPIO_OUTREG)
        Gpio_outLevelSetByGPIO_OUTREG = 4,
        /// dout_doutm4a (4-bit mode 4a)
        Dout_doutm4a4BitMode4a = 5,
        /// dout_doutm4b (4-bit mode 4b)
        Dout_doutm4b4BitMode4b = 6,
        /// dout_doutm4c (4-bit mode 4c)
        Dout_doutm4c4BitMode4c = 7,
        /// clk_out
        Clk_out = 8,
        /// dout_doutm8a (8-bit mode 8a)
        Dout_doutm8a8BitMode8a = 9,
        /// dout_doutm8b (8-bit mode 8b)
        Dout_doutm8b8BitMode8b = 10,
        /// dout_doutm8c (8-bit mode 8c)
        Dout_doutm8c8BitMode8c = 11
    ],
    /// Output enable source. All other values are reserved.
    P_OE_CFG OFFSET(4) NUMBITS(3) [
        /// gpio_oe (state set by GPIO_OEREG)
        Gpio_oeStateSetByGPIO_OEREG = 0,
        /// dout_oem1 (1-bit mode)
        Dout_oem11BitMode = 4,
        /// dout_oem2 (2-bit mode)
        Dout_oem22BitMode = 5,
        /// dout_oem4 (4-bit mode)
        Dout_oem44BitMode = 6,
        /// dout_oem8 (8-bit mode)
        Dout_oem88BitMode = 7
    ]
],
OUT_MUX_CFG[8] [
    /// Output control of output SGPIOn. All other values are reserved.
    P_OUT_CFG OFFSET(0) NUMBITS(4) [
        /// dout_doutm1 (1-bit mode)
        Dout_doutm11BitMode = 0,
        /// dout_doutm2a (2-bit mode 2a)
        Dout_doutm2a2BitMode2a = 1,
        /// dout_doutm2b (2-bit mode 2b)
        Dout_doutm2b2BitMode2b = 2,
        /// dout_doutm2c (2-bit mode 2c)
        Dout_doutm2c2BitMode2c = 3,
        /// gpio_out (level set by GPIO_OUTREG)
        Gpio_outLevelSetByGPIO_OUTREG = 4,
        /// dout_doutm4a (4-bit mode 4a)
        Dout_doutm4a4BitMode4a = 5,
        /// dout_doutm4b (4-bit mode 4b)
        Dout_doutm4b4BitMode4b = 6,
        /// dout_doutm4c (4-bit mode 4c)
        Dout_doutm4c4BitMode4c = 7,
        /// clk_out
        Clk_out = 8,
        /// dout_doutm8a (8-bit mode 8a)
        Dout_doutm8a8BitMode8a = 9,
        /// dout_doutm8b (8-bit mode 8b)
        Dout_doutm8b8BitMode8b = 10,
        /// dout_doutm8c (8-bit mode 8c)
        Dout_doutm8c8BitMode8c = 11
    ],
    /// Output enable source. All other values are reserved.
    P_OE_CFG OFFSET(4) NUMBITS(3) [
        /// gpio_oe (state set by GPIO_OEREG)
        Gpio_oeStateSetByGPIO_OEREG = 0,
        /// dout_oem1 (1-bit mode)
        Dout_oem11BitMode = 4,
        /// dout_oem2 (2-bit mode)
        Dout_oem22BitMode = 5,
        /// dout_oem4 (4-bit mode)
        Dout_oem44BitMode = 6,
        /// dout_oem8 (8-bit mode)
        Dout_oem88BitMode = 7
    ]
],
OUT_MUX_CFG[9] [
    /// Output control of output SGPIOn. All other values are reserved.
    P_OUT_CFG OFFSET(0) NUMBITS(4) [
        /// dout_doutm1 (1-bit mode)
        Dout_doutm11BitMode = 0,
        /// dout_doutm2a (2-bit mode 2a)
        Dout_doutm2a2BitMode2a = 1,
        /// dout_doutm2b (2-bit mode 2b)
        Dout_doutm2b2BitMode2b = 2,
        /// dout_doutm2c (2-bit mode 2c)
        Dout_doutm2c2BitMode2c = 3,
        /// gpio_out (level set by GPIO_OUTREG)
        Gpio_outLevelSetByGPIO_OUTREG = 4,
        /// dout_doutm4a (4-bit mode 4a)
        Dout_doutm4a4BitMode4a = 5,
        /// dout_doutm4b (4-bit mode 4b)
        Dout_doutm4b4BitMode4b = 6,
        /// dout_doutm4c (4-bit mode 4c)
        Dout_doutm4c4BitMode4c = 7,
        /// clk_out
        Clk_out = 8,
        /// dout_doutm8a (8-bit mode 8a)
        Dout_doutm8a8BitMode8a = 9,
        /// dout_doutm8b (8-bit mode 8b)
        Dout_doutm8b8BitMode8b = 10,
        /// dout_doutm8c (8-bit mode 8c)
        Dout_doutm8c8BitMode8c = 11
    ],
    /// Output enable source. All other values are reserved.
    P_OE_CFG OFFSET(4) NUMBITS(3) [
        /// gpio_oe (state set by GPIO_OEREG)
        Gpio_oeStateSetByGPIO_OEREG = 0,
        /// dout_oem1 (1-bit mode)
        Dout_oem11BitMode = 4,
        /// dout_oem2 (2-bit mode)
        Dout_oem22BitMode = 5,
        /// dout_oem4 (4-bit mode)
        Dout_oem44BitMode = 6,
        /// dout_oem8 (8-bit mode)
        Dout_oem88BitMode = 7
    ]
],
OUT_MUX_CFG[10] [
    /// Output control of output SGPIOn. All other values are reserved.
    P_OUT_CFG OFFSET(0) NUMBITS(4) [
        /// dout_doutm1 (1-bit mode)
        Dout_doutm11BitMode = 0,
        /// dout_doutm2a (2-bit mode 2a)
        Dout_doutm2a2BitMode2a = 1,
        /// dout_doutm2b (2-bit mode 2b)
        Dout_doutm2b2BitMode2b = 2,
        /// dout_doutm2c (2-bit mode 2c)
        Dout_doutm2c2BitMode2c = 3,
        /// gpio_out (level set by GPIO_OUTREG)
        Gpio_outLevelSetByGPIO_OUTREG = 4,
        /// dout_doutm4a (4-bit mode 4a)
        Dout_doutm4a4BitMode4a = 5,
        /// dout_doutm4b (4-bit mode 4b)
        Dout_doutm4b4BitMode4b = 6,
        /// dout_doutm4c (4-bit mode 4c)
        Dout_doutm4c4BitMode4c = 7,
        /// clk_out
        Clk_out = 8,
        /// dout_doutm8a (8-bit mode 8a)
        Dout_doutm8a8BitMode8a = 9,
        /// dout_doutm8b (8-bit mode 8b)
        Dout_doutm8b8BitMode8b = 10,
        /// dout_doutm8c (8-bit mode 8c)
        Dout_doutm8c8BitMode8c = 11
    ],
    /// Output enable source. All other values are reserved.
    P_OE_CFG OFFSET(4) NUMBITS(3) [
        /// gpio_oe (state set by GPIO_OEREG)
        Gpio_oeStateSetByGPIO_OEREG = 0,
        /// dout_oem1 (1-bit mode)
        Dout_oem11BitMode = 4,
        /// dout_oem2 (2-bit mode)
        Dout_oem22BitMode = 5,
        /// dout_oem4 (4-bit mode)
        Dout_oem44BitMode = 6,
        /// dout_oem8 (8-bit mode)
        Dout_oem88BitMode = 7
    ]
],
OUT_MUX_CFG[11] [
    /// Output control of output SGPIOn. All other values are reserved.
    P_OUT_CFG OFFSET(0) NUMBITS(4) [
        /// dout_doutm1 (1-bit mode)
        Dout_doutm11BitMode = 0,
        /// dout_doutm2a (2-bit mode 2a)
        Dout_doutm2a2BitMode2a = 1,
        /// dout_doutm2b (2-bit mode 2b)
        Dout_doutm2b2BitMode2b = 2,
        /// dout_doutm2c (2-bit mode 2c)
        Dout_doutm2c2BitMode2c = 3,
        /// gpio_out (level set by GPIO_OUTREG)
        Gpio_outLevelSetByGPIO_OUTREG = 4,
        /// dout_doutm4a (4-bit mode 4a)
        Dout_doutm4a4BitMode4a = 5,
        /// dout_doutm4b (4-bit mode 4b)
        Dout_doutm4b4BitMode4b = 6,
        /// dout_doutm4c (4-bit mode 4c)
        Dout_doutm4c4BitMode4c = 7,
        /// clk_out
        Clk_out = 8,
        /// dout_doutm8a (8-bit mode 8a)
        Dout_doutm8a8BitMode8a = 9,
        /// dout_doutm8b (8-bit mode 8b)
        Dout_doutm8b8BitMode8b = 10,
        /// dout_doutm8c (8-bit mode 8c)
        Dout_doutm8c8BitMode8c = 11
    ],
    /// Output enable source. All other values are reserved.
    P_OE_CFG OFFSET(4) NUMBITS(3) [
        /// gpio_oe (state set by GPIO_OEREG)
        Gpio_oeStateSetByGPIO_OEREG = 0,
        /// dout_oem1 (1-bit mode)
        Dout_oem11BitMode = 4,
        /// dout_oem2 (2-bit mode)
        Dout_oem22BitMode = 5,
        /// dout_oem4 (4-bit mode)
        Dout_oem44BitMode = 6,
        /// dout_oem8 (8-bit mode)
        Dout_oem88BitMode = 7
    ]
],
OUT_MUX_CFG[12] [
    /// Output control of output SGPIOn. All other values are reserved.
    P_OUT_CFG OFFSET(0) NUMBITS(4) [
        /// dout_doutm1 (1-bit mode)
        Dout_doutm11BitMode = 0,
        /// dout_doutm2a (2-bit mode 2a)
        Dout_doutm2a2BitMode2a = 1,
        /// dout_doutm2b (2-bit mode 2b)
        Dout_doutm2b2BitMode2b = 2,
        /// dout_doutm2c (2-bit mode 2c)
        Dout_doutm2c2BitMode2c = 3,
        /// gpio_out (level set by GPIO_OUTREG)
        Gpio_outLevelSetByGPIO_OUTREG = 4,
        /// dout_doutm4a (4-bit mode 4a)
        Dout_doutm4a4BitMode4a = 5,
        /// dout_doutm4b (4-bit mode 4b)
        Dout_doutm4b4BitMode4b = 6,
        /// dout_doutm4c (4-bit mode 4c)
        Dout_doutm4c4BitMode4c = 7,
        /// clk_out
        Clk_out = 8,
        /// dout_doutm8a (8-bit mode 8a)
        Dout_doutm8a8BitMode8a = 9,
        /// dout_doutm8b (8-bit mode 8b)
        Dout_doutm8b8BitMode8b = 10,
        /// dout_doutm8c (8-bit mode 8c)
        Dout_doutm8c8BitMode8c = 11
    ],
    /// Output enable source. All other values are reserved.
    P_OE_CFG OFFSET(4) NUMBITS(3) [
        /// gpio_oe (state set by GPIO_OEREG)
        Gpio_oeStateSetByGPIO_OEREG = 0,
        /// dout_oem1 (1-bit mode)
        Dout_oem11BitMode = 4,
        /// dout_oem2 (2-bit mode)
        Dout_oem22BitMode = 5,
        /// dout_oem4 (4-bit mode)
        Dout_oem44BitMode = 6,
        /// dout_oem8 (8-bit mode)
        Dout_oem88BitMode = 7
    ]
],
OUT_MUX_CFG[13] [
    /// Output control of output SGPIOn. All other values are reserved.
    P_OUT_CFG OFFSET(0) NUMBITS(4) [
        /// dout_doutm1 (1-bit mode)
        Dout_doutm11BitMode = 0,
        /// dout_doutm2a (2-bit mode 2a)
        Dout_doutm2a2BitMode2a = 1,
        /// dout_doutm2b (2-bit mode 2b)
        Dout_doutm2b2BitMode2b = 2,
        /// dout_doutm2c (2-bit mode 2c)
        Dout_doutm2c2BitMode2c = 3,
        /// gpio_out (level set by GPIO_OUTREG)
        Gpio_outLevelSetByGPIO_OUTREG = 4,
        /// dout_doutm4a (4-bit mode 4a)
        Dout_doutm4a4BitMode4a = 5,
        /// dout_doutm4b (4-bit mode 4b)
        Dout_doutm4b4BitMode4b = 6,
        /// dout_doutm4c (4-bit mode 4c)
        Dout_doutm4c4BitMode4c = 7,
        /// clk_out
        Clk_out = 8,
        /// dout_doutm8a (8-bit mode 8a)
        Dout_doutm8a8BitMode8a = 9,
        /// dout_doutm8b (8-bit mode 8b)
        Dout_doutm8b8BitMode8b = 10,
        /// dout_doutm8c (8-bit mode 8c)
        Dout_doutm8c8BitMode8c = 11
    ],
    /// Output enable source. All other values are reserved.
    P_OE_CFG OFFSET(4) NUMBITS(3) [
        /// gpio_oe (state set by GPIO_OEREG)
        Gpio_oeStateSetByGPIO_OEREG = 0,
        /// dout_oem1 (1-bit mode)
        Dout_oem11BitMode = 4,
        /// dout_oem2 (2-bit mode)
        Dout_oem22BitMode = 5,
        /// dout_oem4 (4-bit mode)
        Dout_oem44BitMode = 6,
        /// dout_oem8 (8-bit mode)
        Dout_oem88BitMode = 7
    ]
],
OUT_MUX_CFG[14] [
    /// Output control of output SGPIOn. All other values are reserved.
    P_OUT_CFG OFFSET(0) NUMBITS(4) [
        /// dout_doutm1 (1-bit mode)
        Dout_doutm11BitMode = 0,
        /// dout_doutm2a (2-bit mode 2a)
        Dout_doutm2a2BitMode2a = 1,
        /// dout_doutm2b (2-bit mode 2b)
        Dout_doutm2b2BitMode2b = 2,
        /// dout_doutm2c (2-bit mode 2c)
        Dout_doutm2c2BitMode2c = 3,
        /// gpio_out (level set by GPIO_OUTREG)
        Gpio_outLevelSetByGPIO_OUTREG = 4,
        /// dout_doutm4a (4-bit mode 4a)
        Dout_doutm4a4BitMode4a = 5,
        /// dout_doutm4b (4-bit mode 4b)
        Dout_doutm4b4BitMode4b = 6,
        /// dout_doutm4c (4-bit mode 4c)
        Dout_doutm4c4BitMode4c = 7,
        /// clk_out
        Clk_out = 8,
        /// dout_doutm8a (8-bit mode 8a)
        Dout_doutm8a8BitMode8a = 9,
        /// dout_doutm8b (8-bit mode 8b)
        Dout_doutm8b8BitMode8b = 10,
        /// dout_doutm8c (8-bit mode 8c)
        Dout_doutm8c8BitMode8c = 11
    ],
    /// Output enable source. All other values are reserved.
    P_OE_CFG OFFSET(4) NUMBITS(3) [
        /// gpio_oe (state set by GPIO_OEREG)
        Gpio_oeStateSetByGPIO_OEREG = 0,
        /// dout_oem1 (1-bit mode)
        Dout_oem11BitMode = 4,
        /// dout_oem2 (2-bit mode)
        Dout_oem22BitMode = 5,
        /// dout_oem4 (4-bit mode)
        Dout_oem44BitMode = 6,
        /// dout_oem8 (8-bit mode)
        Dout_oem88BitMode = 7
    ]
],
OUT_MUX_CFG[15] [
    /// Output control of output SGPIOn. All other values are reserved.
    P_OUT_CFG OFFSET(0) NUMBITS(4) [
        /// dout_doutm1 (1-bit mode)
        Dout_doutm11BitMode = 0,
        /// dout_doutm2a (2-bit mode 2a)
        Dout_doutm2a2BitMode2a = 1,
        /// dout_doutm2b (2-bit mode 2b)
        Dout_doutm2b2BitMode2b = 2,
        /// dout_doutm2c (2-bit mode 2c)
        Dout_doutm2c2BitMode2c = 3,
        /// gpio_out (level set by GPIO_OUTREG)
        Gpio_outLevelSetByGPIO_OUTREG = 4,
        /// dout_doutm4a (4-bit mode 4a)
        Dout_doutm4a4BitMode4a = 5,
        /// dout_doutm4b (4-bit mode 4b)
        Dout_doutm4b4BitMode4b = 6,
        /// dout_doutm4c (4-bit mode 4c)
        Dout_doutm4c4BitMode4c = 7,
        /// clk_out
        Clk_out = 8,
        /// dout_doutm8a (8-bit mode 8a)
        Dout_doutm8a8BitMode8a = 9,
        /// dout_doutm8b (8-bit mode 8b)
        Dout_doutm8b8BitMode8b = 10,
        /// dout_doutm8c (8-bit mode 8c)
        Dout_doutm8c8BitMode8c = 11
    ],
    /// Output enable source. All other values are reserved.
    P_OE_CFG OFFSET(4) NUMBITS(3) [
        /// gpio_oe (state set by GPIO_OEREG)
        Gpio_oeStateSetByGPIO_OEREG = 0,
        /// dout_oem1 (1-bit mode)
        Dout_oem11BitMode = 4,
        /// dout_oem2 (2-bit mode)
        Dout_oem22BitMode = 5,
        /// dout_oem4 (4-bit mode)
        Dout_oem44BitMode = 6,
        /// dout_oem8 (8-bit mode)
        Dout_oem88BitMode = 7
    ]
],
SGPIO_MUX_CFG[0] [
    /// Select clock signal.
    EXT_CLK_ENABLE OFFSET(0) NUMBITS(1) [
        /// Internal clock signal (slice)
        InternalClockSignalSlice = 0,
        /// External clock signal (pin)
        ExternalClockSignalPin = 1
    ],
    /// Select source clock pin.
    CLK_SOURCE_PIN_MODE OFFSET(1) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select clock source slice. Note that slices D, H, O and P do not support this mo
    CLK_SOURCE_SLICE_MODE OFFSET(3) NUMBITS(2) [
        /// Slice D
        SliceD = 0,
        /// Slice H
        SliceH = 1,
        /// Slice O
        SliceO = 2,
        /// Slice P
        SliceP = 3
    ],
    /// Select qualifier mode.
    QUALIFIER_MODE OFFSET(5) NUMBITS(2) [
        /// Enable
        Enable = 0,
        /// Disable
        Disable = 1,
        /// Slice (see bits QUALIFIER_SLICE_MODE in this register)
        SliceSeeBitsQUALIFIER_SLICE_MODEInThisRegister = 2,
        /// External SGPIO pin (SGPIO8, SGPIO9, SGPIO10, or SGPIO11)
        ExternalSGPIOPinSGPIO8SGPIO9SGPIO10OrSGPIO11 = 3
    ],
    /// Select qualifier pin.
    QUALIFIER_PIN_MODE OFFSET(7) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select qualifier slice.
    QUALIFIER_SLICE_MODE OFFSET(9) NUMBITS(2) [
        /// Slice A, but for slice A slice D is used.
        SliceAButForSliceASliceDIsUsed = 0,
        /// Slice H, but for slice H slice O is used.
        SliceHButForSliceHSliceOIsUsed = 1,
        /// Slice I, but for slice I slice D is used.
        SliceIButForSliceISliceDIsUsed = 2,
        /// Slice P, but for slice P slice O is used.
        SlicePButForSlicePSliceOIsUsed = 3
    ],
    /// Enable concatenation.
    CONCAT_ENABLE OFFSET(11) NUMBITS(1) [
        /// External data pin
        ExternalDataPin = 0,
        /// Concatenate data
        ConcatenateData = 1
    ],
    /// Select concatenation order
    CONCAT_ORDER OFFSET(12) NUMBITS(2) [
        /// Self-loop
        SelfLoop = 0,
        /// 2 slices
        _2Slices = 1,
        /// 4 slices
        _4Slices = 2,
        /// 8 slices
        _8Slices = 3
    ]
],
SGPIO_MUX_CFG[1] [
    /// Select clock signal.
    EXT_CLK_ENABLE OFFSET(0) NUMBITS(1) [
        /// Internal clock signal (slice)
        InternalClockSignalSlice = 0,
        /// External clock signal (pin)
        ExternalClockSignalPin = 1
    ],
    /// Select source clock pin.
    CLK_SOURCE_PIN_MODE OFFSET(1) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select clock source slice. Note that slices D, H, O and P do not support this mo
    CLK_SOURCE_SLICE_MODE OFFSET(3) NUMBITS(2) [
        /// Slice D
        SliceD = 0,
        /// Slice H
        SliceH = 1,
        /// Slice O
        SliceO = 2,
        /// Slice P
        SliceP = 3
    ],
    /// Select qualifier mode.
    QUALIFIER_MODE OFFSET(5) NUMBITS(2) [
        /// Enable
        Enable = 0,
        /// Disable
        Disable = 1,
        /// Slice (see bits QUALIFIER_SLICE_MODE in this register)
        SliceSeeBitsQUALIFIER_SLICE_MODEInThisRegister = 2,
        /// External SGPIO pin (SGPIO8, SGPIO9, SGPIO10, or SGPIO11)
        ExternalSGPIOPinSGPIO8SGPIO9SGPIO10OrSGPIO11 = 3
    ],
    /// Select qualifier pin.
    QUALIFIER_PIN_MODE OFFSET(7) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select qualifier slice.
    QUALIFIER_SLICE_MODE OFFSET(9) NUMBITS(2) [
        /// Slice A, but for slice A slice D is used.
        SliceAButForSliceASliceDIsUsed = 0,
        /// Slice H, but for slice H slice O is used.
        SliceHButForSliceHSliceOIsUsed = 1,
        /// Slice I, but for slice I slice D is used.
        SliceIButForSliceISliceDIsUsed = 2,
        /// Slice P, but for slice P slice O is used.
        SlicePButForSlicePSliceOIsUsed = 3
    ],
    /// Enable concatenation.
    CONCAT_ENABLE OFFSET(11) NUMBITS(1) [
        /// External data pin
        ExternalDataPin = 0,
        /// Concatenate data
        ConcatenateData = 1
    ],
    /// Select concatenation order
    CONCAT_ORDER OFFSET(12) NUMBITS(2) [
        /// Self-loop
        SelfLoop = 0,
        /// 2 slices
        _2Slices = 1,
        /// 4 slices
        _4Slices = 2,
        /// 8 slices
        _8Slices = 3
    ]
],
SGPIO_MUX_CFG[2] [
    /// Select clock signal.
    EXT_CLK_ENABLE OFFSET(0) NUMBITS(1) [
        /// Internal clock signal (slice)
        InternalClockSignalSlice = 0,
        /// External clock signal (pin)
        ExternalClockSignalPin = 1
    ],
    /// Select source clock pin.
    CLK_SOURCE_PIN_MODE OFFSET(1) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select clock source slice. Note that slices D, H, O and P do not support this mo
    CLK_SOURCE_SLICE_MODE OFFSET(3) NUMBITS(2) [
        /// Slice D
        SliceD = 0,
        /// Slice H
        SliceH = 1,
        /// Slice O
        SliceO = 2,
        /// Slice P
        SliceP = 3
    ],
    /// Select qualifier mode.
    QUALIFIER_MODE OFFSET(5) NUMBITS(2) [
        /// Enable
        Enable = 0,
        /// Disable
        Disable = 1,
        /// Slice (see bits QUALIFIER_SLICE_MODE in this register)
        SliceSeeBitsQUALIFIER_SLICE_MODEInThisRegister = 2,
        /// External SGPIO pin (SGPIO8, SGPIO9, SGPIO10, or SGPIO11)
        ExternalSGPIOPinSGPIO8SGPIO9SGPIO10OrSGPIO11 = 3
    ],
    /// Select qualifier pin.
    QUALIFIER_PIN_MODE OFFSET(7) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select qualifier slice.
    QUALIFIER_SLICE_MODE OFFSET(9) NUMBITS(2) [
        /// Slice A, but for slice A slice D is used.
        SliceAButForSliceASliceDIsUsed = 0,
        /// Slice H, but for slice H slice O is used.
        SliceHButForSliceHSliceOIsUsed = 1,
        /// Slice I, but for slice I slice D is used.
        SliceIButForSliceISliceDIsUsed = 2,
        /// Slice P, but for slice P slice O is used.
        SlicePButForSlicePSliceOIsUsed = 3
    ],
    /// Enable concatenation.
    CONCAT_ENABLE OFFSET(11) NUMBITS(1) [
        /// External data pin
        ExternalDataPin = 0,
        /// Concatenate data
        ConcatenateData = 1
    ],
    /// Select concatenation order
    CONCAT_ORDER OFFSET(12) NUMBITS(2) [
        /// Self-loop
        SelfLoop = 0,
        /// 2 slices
        _2Slices = 1,
        /// 4 slices
        _4Slices = 2,
        /// 8 slices
        _8Slices = 3
    ]
],
SGPIO_MUX_CFG[3] [
    /// Select clock signal.
    EXT_CLK_ENABLE OFFSET(0) NUMBITS(1) [
        /// Internal clock signal (slice)
        InternalClockSignalSlice = 0,
        /// External clock signal (pin)
        ExternalClockSignalPin = 1
    ],
    /// Select source clock pin.
    CLK_SOURCE_PIN_MODE OFFSET(1) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select clock source slice. Note that slices D, H, O and P do not support this mo
    CLK_SOURCE_SLICE_MODE OFFSET(3) NUMBITS(2) [
        /// Slice D
        SliceD = 0,
        /// Slice H
        SliceH = 1,
        /// Slice O
        SliceO = 2,
        /// Slice P
        SliceP = 3
    ],
    /// Select qualifier mode.
    QUALIFIER_MODE OFFSET(5) NUMBITS(2) [
        /// Enable
        Enable = 0,
        /// Disable
        Disable = 1,
        /// Slice (see bits QUALIFIER_SLICE_MODE in this register)
        SliceSeeBitsQUALIFIER_SLICE_MODEInThisRegister = 2,
        /// External SGPIO pin (SGPIO8, SGPIO9, SGPIO10, or SGPIO11)
        ExternalSGPIOPinSGPIO8SGPIO9SGPIO10OrSGPIO11 = 3
    ],
    /// Select qualifier pin.
    QUALIFIER_PIN_MODE OFFSET(7) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select qualifier slice.
    QUALIFIER_SLICE_MODE OFFSET(9) NUMBITS(2) [
        /// Slice A, but for slice A slice D is used.
        SliceAButForSliceASliceDIsUsed = 0,
        /// Slice H, but for slice H slice O is used.
        SliceHButForSliceHSliceOIsUsed = 1,
        /// Slice I, but for slice I slice D is used.
        SliceIButForSliceISliceDIsUsed = 2,
        /// Slice P, but for slice P slice O is used.
        SlicePButForSlicePSliceOIsUsed = 3
    ],
    /// Enable concatenation.
    CONCAT_ENABLE OFFSET(11) NUMBITS(1) [
        /// External data pin
        ExternalDataPin = 0,
        /// Concatenate data
        ConcatenateData = 1
    ],
    /// Select concatenation order
    CONCAT_ORDER OFFSET(12) NUMBITS(2) [
        /// Self-loop
        SelfLoop = 0,
        /// 2 slices
        _2Slices = 1,
        /// 4 slices
        _4Slices = 2,
        /// 8 slices
        _8Slices = 3
    ]
],
SGPIO_MUX_CFG[4] [
    /// Select clock signal.
    EXT_CLK_ENABLE OFFSET(0) NUMBITS(1) [
        /// Internal clock signal (slice)
        InternalClockSignalSlice = 0,
        /// External clock signal (pin)
        ExternalClockSignalPin = 1
    ],
    /// Select source clock pin.
    CLK_SOURCE_PIN_MODE OFFSET(1) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select clock source slice. Note that slices D, H, O and P do not support this mo
    CLK_SOURCE_SLICE_MODE OFFSET(3) NUMBITS(2) [
        /// Slice D
        SliceD = 0,
        /// Slice H
        SliceH = 1,
        /// Slice O
        SliceO = 2,
        /// Slice P
        SliceP = 3
    ],
    /// Select qualifier mode.
    QUALIFIER_MODE OFFSET(5) NUMBITS(2) [
        /// Enable
        Enable = 0,
        /// Disable
        Disable = 1,
        /// Slice (see bits QUALIFIER_SLICE_MODE in this register)
        SliceSeeBitsQUALIFIER_SLICE_MODEInThisRegister = 2,
        /// External SGPIO pin (SGPIO8, SGPIO9, SGPIO10, or SGPIO11)
        ExternalSGPIOPinSGPIO8SGPIO9SGPIO10OrSGPIO11 = 3
    ],
    /// Select qualifier pin.
    QUALIFIER_PIN_MODE OFFSET(7) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select qualifier slice.
    QUALIFIER_SLICE_MODE OFFSET(9) NUMBITS(2) [
        /// Slice A, but for slice A slice D is used.
        SliceAButForSliceASliceDIsUsed = 0,
        /// Slice H, but for slice H slice O is used.
        SliceHButForSliceHSliceOIsUsed = 1,
        /// Slice I, but for slice I slice D is used.
        SliceIButForSliceISliceDIsUsed = 2,
        /// Slice P, but for slice P slice O is used.
        SlicePButForSlicePSliceOIsUsed = 3
    ],
    /// Enable concatenation.
    CONCAT_ENABLE OFFSET(11) NUMBITS(1) [
        /// External data pin
        ExternalDataPin = 0,
        /// Concatenate data
        ConcatenateData = 1
    ],
    /// Select concatenation order
    CONCAT_ORDER OFFSET(12) NUMBITS(2) [
        /// Self-loop
        SelfLoop = 0,
        /// 2 slices
        _2Slices = 1,
        /// 4 slices
        _4Slices = 2,
        /// 8 slices
        _8Slices = 3
    ]
],
SGPIO_MUX_CFG[5] [
    /// Select clock signal.
    EXT_CLK_ENABLE OFFSET(0) NUMBITS(1) [
        /// Internal clock signal (slice)
        InternalClockSignalSlice = 0,
        /// External clock signal (pin)
        ExternalClockSignalPin = 1
    ],
    /// Select source clock pin.
    CLK_SOURCE_PIN_MODE OFFSET(1) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select clock source slice. Note that slices D, H, O and P do not support this mo
    CLK_SOURCE_SLICE_MODE OFFSET(3) NUMBITS(2) [
        /// Slice D
        SliceD = 0,
        /// Slice H
        SliceH = 1,
        /// Slice O
        SliceO = 2,
        /// Slice P
        SliceP = 3
    ],
    /// Select qualifier mode.
    QUALIFIER_MODE OFFSET(5) NUMBITS(2) [
        /// Enable
        Enable = 0,
        /// Disable
        Disable = 1,
        /// Slice (see bits QUALIFIER_SLICE_MODE in this register)
        SliceSeeBitsQUALIFIER_SLICE_MODEInThisRegister = 2,
        /// External SGPIO pin (SGPIO8, SGPIO9, SGPIO10, or SGPIO11)
        ExternalSGPIOPinSGPIO8SGPIO9SGPIO10OrSGPIO11 = 3
    ],
    /// Select qualifier pin.
    QUALIFIER_PIN_MODE OFFSET(7) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select qualifier slice.
    QUALIFIER_SLICE_MODE OFFSET(9) NUMBITS(2) [
        /// Slice A, but for slice A slice D is used.
        SliceAButForSliceASliceDIsUsed = 0,
        /// Slice H, but for slice H slice O is used.
        SliceHButForSliceHSliceOIsUsed = 1,
        /// Slice I, but for slice I slice D is used.
        SliceIButForSliceISliceDIsUsed = 2,
        /// Slice P, but for slice P slice O is used.
        SlicePButForSlicePSliceOIsUsed = 3
    ],
    /// Enable concatenation.
    CONCAT_ENABLE OFFSET(11) NUMBITS(1) [
        /// External data pin
        ExternalDataPin = 0,
        /// Concatenate data
        ConcatenateData = 1
    ],
    /// Select concatenation order
    CONCAT_ORDER OFFSET(12) NUMBITS(2) [
        /// Self-loop
        SelfLoop = 0,
        /// 2 slices
        _2Slices = 1,
        /// 4 slices
        _4Slices = 2,
        /// 8 slices
        _8Slices = 3
    ]
],
SGPIO_MUX_CFG[6] [
    /// Select clock signal.
    EXT_CLK_ENABLE OFFSET(0) NUMBITS(1) [
        /// Internal clock signal (slice)
        InternalClockSignalSlice = 0,
        /// External clock signal (pin)
        ExternalClockSignalPin = 1
    ],
    /// Select source clock pin.
    CLK_SOURCE_PIN_MODE OFFSET(1) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select clock source slice. Note that slices D, H, O and P do not support this mo
    CLK_SOURCE_SLICE_MODE OFFSET(3) NUMBITS(2) [
        /// Slice D
        SliceD = 0,
        /// Slice H
        SliceH = 1,
        /// Slice O
        SliceO = 2,
        /// Slice P
        SliceP = 3
    ],
    /// Select qualifier mode.
    QUALIFIER_MODE OFFSET(5) NUMBITS(2) [
        /// Enable
        Enable = 0,
        /// Disable
        Disable = 1,
        /// Slice (see bits QUALIFIER_SLICE_MODE in this register)
        SliceSeeBitsQUALIFIER_SLICE_MODEInThisRegister = 2,
        /// External SGPIO pin (SGPIO8, SGPIO9, SGPIO10, or SGPIO11)
        ExternalSGPIOPinSGPIO8SGPIO9SGPIO10OrSGPIO11 = 3
    ],
    /// Select qualifier pin.
    QUALIFIER_PIN_MODE OFFSET(7) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select qualifier slice.
    QUALIFIER_SLICE_MODE OFFSET(9) NUMBITS(2) [
        /// Slice A, but for slice A slice D is used.
        SliceAButForSliceASliceDIsUsed = 0,
        /// Slice H, but for slice H slice O is used.
        SliceHButForSliceHSliceOIsUsed = 1,
        /// Slice I, but for slice I slice D is used.
        SliceIButForSliceISliceDIsUsed = 2,
        /// Slice P, but for slice P slice O is used.
        SlicePButForSlicePSliceOIsUsed = 3
    ],
    /// Enable concatenation.
    CONCAT_ENABLE OFFSET(11) NUMBITS(1) [
        /// External data pin
        ExternalDataPin = 0,
        /// Concatenate data
        ConcatenateData = 1
    ],
    /// Select concatenation order
    CONCAT_ORDER OFFSET(12) NUMBITS(2) [
        /// Self-loop
        SelfLoop = 0,
        /// 2 slices
        _2Slices = 1,
        /// 4 slices
        _4Slices = 2,
        /// 8 slices
        _8Slices = 3
    ]
],
SGPIO_MUX_CFG[7] [
    /// Select clock signal.
    EXT_CLK_ENABLE OFFSET(0) NUMBITS(1) [
        /// Internal clock signal (slice)
        InternalClockSignalSlice = 0,
        /// External clock signal (pin)
        ExternalClockSignalPin = 1
    ],
    /// Select source clock pin.
    CLK_SOURCE_PIN_MODE OFFSET(1) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select clock source slice. Note that slices D, H, O and P do not support this mo
    CLK_SOURCE_SLICE_MODE OFFSET(3) NUMBITS(2) [
        /// Slice D
        SliceD = 0,
        /// Slice H
        SliceH = 1,
        /// Slice O
        SliceO = 2,
        /// Slice P
        SliceP = 3
    ],
    /// Select qualifier mode.
    QUALIFIER_MODE OFFSET(5) NUMBITS(2) [
        /// Enable
        Enable = 0,
        /// Disable
        Disable = 1,
        /// Slice (see bits QUALIFIER_SLICE_MODE in this register)
        SliceSeeBitsQUALIFIER_SLICE_MODEInThisRegister = 2,
        /// External SGPIO pin (SGPIO8, SGPIO9, SGPIO10, or SGPIO11)
        ExternalSGPIOPinSGPIO8SGPIO9SGPIO10OrSGPIO11 = 3
    ],
    /// Select qualifier pin.
    QUALIFIER_PIN_MODE OFFSET(7) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select qualifier slice.
    QUALIFIER_SLICE_MODE OFFSET(9) NUMBITS(2) [
        /// Slice A, but for slice A slice D is used.
        SliceAButForSliceASliceDIsUsed = 0,
        /// Slice H, but for slice H slice O is used.
        SliceHButForSliceHSliceOIsUsed = 1,
        /// Slice I, but for slice I slice D is used.
        SliceIButForSliceISliceDIsUsed = 2,
        /// Slice P, but for slice P slice O is used.
        SlicePButForSlicePSliceOIsUsed = 3
    ],
    /// Enable concatenation.
    CONCAT_ENABLE OFFSET(11) NUMBITS(1) [
        /// External data pin
        ExternalDataPin = 0,
        /// Concatenate data
        ConcatenateData = 1
    ],
    /// Select concatenation order
    CONCAT_ORDER OFFSET(12) NUMBITS(2) [
        /// Self-loop
        SelfLoop = 0,
        /// 2 slices
        _2Slices = 1,
        /// 4 slices
        _4Slices = 2,
        /// 8 slices
        _8Slices = 3
    ]
],
SGPIO_MUX_CFG[8] [
    /// Select clock signal.
    EXT_CLK_ENABLE OFFSET(0) NUMBITS(1) [
        /// Internal clock signal (slice)
        InternalClockSignalSlice = 0,
        /// External clock signal (pin)
        ExternalClockSignalPin = 1
    ],
    /// Select source clock pin.
    CLK_SOURCE_PIN_MODE OFFSET(1) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select clock source slice. Note that slices D, H, O and P do not support this mo
    CLK_SOURCE_SLICE_MODE OFFSET(3) NUMBITS(2) [
        /// Slice D
        SliceD = 0,
        /// Slice H
        SliceH = 1,
        /// Slice O
        SliceO = 2,
        /// Slice P
        SliceP = 3
    ],
    /// Select qualifier mode.
    QUALIFIER_MODE OFFSET(5) NUMBITS(2) [
        /// Enable
        Enable = 0,
        /// Disable
        Disable = 1,
        /// Slice (see bits QUALIFIER_SLICE_MODE in this register)
        SliceSeeBitsQUALIFIER_SLICE_MODEInThisRegister = 2,
        /// External SGPIO pin (SGPIO8, SGPIO9, SGPIO10, or SGPIO11)
        ExternalSGPIOPinSGPIO8SGPIO9SGPIO10OrSGPIO11 = 3
    ],
    /// Select qualifier pin.
    QUALIFIER_PIN_MODE OFFSET(7) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select qualifier slice.
    QUALIFIER_SLICE_MODE OFFSET(9) NUMBITS(2) [
        /// Slice A, but for slice A slice D is used.
        SliceAButForSliceASliceDIsUsed = 0,
        /// Slice H, but for slice H slice O is used.
        SliceHButForSliceHSliceOIsUsed = 1,
        /// Slice I, but for slice I slice D is used.
        SliceIButForSliceISliceDIsUsed = 2,
        /// Slice P, but for slice P slice O is used.
        SlicePButForSlicePSliceOIsUsed = 3
    ],
    /// Enable concatenation.
    CONCAT_ENABLE OFFSET(11) NUMBITS(1) [
        /// External data pin
        ExternalDataPin = 0,
        /// Concatenate data
        ConcatenateData = 1
    ],
    /// Select concatenation order
    CONCAT_ORDER OFFSET(12) NUMBITS(2) [
        /// Self-loop
        SelfLoop = 0,
        /// 2 slices
        _2Slices = 1,
        /// 4 slices
        _4Slices = 2,
        /// 8 slices
        _8Slices = 3
    ]
],
SGPIO_MUX_CFG[9] [
    /// Select clock signal.
    EXT_CLK_ENABLE OFFSET(0) NUMBITS(1) [
        /// Internal clock signal (slice)
        InternalClockSignalSlice = 0,
        /// External clock signal (pin)
        ExternalClockSignalPin = 1
    ],
    /// Select source clock pin.
    CLK_SOURCE_PIN_MODE OFFSET(1) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select clock source slice. Note that slices D, H, O and P do not support this mo
    CLK_SOURCE_SLICE_MODE OFFSET(3) NUMBITS(2) [
        /// Slice D
        SliceD = 0,
        /// Slice H
        SliceH = 1,
        /// Slice O
        SliceO = 2,
        /// Slice P
        SliceP = 3
    ],
    /// Select qualifier mode.
    QUALIFIER_MODE OFFSET(5) NUMBITS(2) [
        /// Enable
        Enable = 0,
        /// Disable
        Disable = 1,
        /// Slice (see bits QUALIFIER_SLICE_MODE in this register)
        SliceSeeBitsQUALIFIER_SLICE_MODEInThisRegister = 2,
        /// External SGPIO pin (SGPIO8, SGPIO9, SGPIO10, or SGPIO11)
        ExternalSGPIOPinSGPIO8SGPIO9SGPIO10OrSGPIO11 = 3
    ],
    /// Select qualifier pin.
    QUALIFIER_PIN_MODE OFFSET(7) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select qualifier slice.
    QUALIFIER_SLICE_MODE OFFSET(9) NUMBITS(2) [
        /// Slice A, but for slice A slice D is used.
        SliceAButForSliceASliceDIsUsed = 0,
        /// Slice H, but for slice H slice O is used.
        SliceHButForSliceHSliceOIsUsed = 1,
        /// Slice I, but for slice I slice D is used.
        SliceIButForSliceISliceDIsUsed = 2,
        /// Slice P, but for slice P slice O is used.
        SlicePButForSlicePSliceOIsUsed = 3
    ],
    /// Enable concatenation.
    CONCAT_ENABLE OFFSET(11) NUMBITS(1) [
        /// External data pin
        ExternalDataPin = 0,
        /// Concatenate data
        ConcatenateData = 1
    ],
    /// Select concatenation order
    CONCAT_ORDER OFFSET(12) NUMBITS(2) [
        /// Self-loop
        SelfLoop = 0,
        /// 2 slices
        _2Slices = 1,
        /// 4 slices
        _4Slices = 2,
        /// 8 slices
        _8Slices = 3
    ]
],
SGPIO_MUX_CFG[10] [
    /// Select clock signal.
    EXT_CLK_ENABLE OFFSET(0) NUMBITS(1) [
        /// Internal clock signal (slice)
        InternalClockSignalSlice = 0,
        /// External clock signal (pin)
        ExternalClockSignalPin = 1
    ],
    /// Select source clock pin.
    CLK_SOURCE_PIN_MODE OFFSET(1) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select clock source slice. Note that slices D, H, O and P do not support this mo
    CLK_SOURCE_SLICE_MODE OFFSET(3) NUMBITS(2) [
        /// Slice D
        SliceD = 0,
        /// Slice H
        SliceH = 1,
        /// Slice O
        SliceO = 2,
        /// Slice P
        SliceP = 3
    ],
    /// Select qualifier mode.
    QUALIFIER_MODE OFFSET(5) NUMBITS(2) [
        /// Enable
        Enable = 0,
        /// Disable
        Disable = 1,
        /// Slice (see bits QUALIFIER_SLICE_MODE in this register)
        SliceSeeBitsQUALIFIER_SLICE_MODEInThisRegister = 2,
        /// External SGPIO pin (SGPIO8, SGPIO9, SGPIO10, or SGPIO11)
        ExternalSGPIOPinSGPIO8SGPIO9SGPIO10OrSGPIO11 = 3
    ],
    /// Select qualifier pin.
    QUALIFIER_PIN_MODE OFFSET(7) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select qualifier slice.
    QUALIFIER_SLICE_MODE OFFSET(9) NUMBITS(2) [
        /// Slice A, but for slice A slice D is used.
        SliceAButForSliceASliceDIsUsed = 0,
        /// Slice H, but for slice H slice O is used.
        SliceHButForSliceHSliceOIsUsed = 1,
        /// Slice I, but for slice I slice D is used.
        SliceIButForSliceISliceDIsUsed = 2,
        /// Slice P, but for slice P slice O is used.
        SlicePButForSlicePSliceOIsUsed = 3
    ],
    /// Enable concatenation.
    CONCAT_ENABLE OFFSET(11) NUMBITS(1) [
        /// External data pin
        ExternalDataPin = 0,
        /// Concatenate data
        ConcatenateData = 1
    ],
    /// Select concatenation order
    CONCAT_ORDER OFFSET(12) NUMBITS(2) [
        /// Self-loop
        SelfLoop = 0,
        /// 2 slices
        _2Slices = 1,
        /// 4 slices
        _4Slices = 2,
        /// 8 slices
        _8Slices = 3
    ]
],
SGPIO_MUX_CFG[11] [
    /// Select clock signal.
    EXT_CLK_ENABLE OFFSET(0) NUMBITS(1) [
        /// Internal clock signal (slice)
        InternalClockSignalSlice = 0,
        /// External clock signal (pin)
        ExternalClockSignalPin = 1
    ],
    /// Select source clock pin.
    CLK_SOURCE_PIN_MODE OFFSET(1) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select clock source slice. Note that slices D, H, O and P do not support this mo
    CLK_SOURCE_SLICE_MODE OFFSET(3) NUMBITS(2) [
        /// Slice D
        SliceD = 0,
        /// Slice H
        SliceH = 1,
        /// Slice O
        SliceO = 2,
        /// Slice P
        SliceP = 3
    ],
    /// Select qualifier mode.
    QUALIFIER_MODE OFFSET(5) NUMBITS(2) [
        /// Enable
        Enable = 0,
        /// Disable
        Disable = 1,
        /// Slice (see bits QUALIFIER_SLICE_MODE in this register)
        SliceSeeBitsQUALIFIER_SLICE_MODEInThisRegister = 2,
        /// External SGPIO pin (SGPIO8, SGPIO9, SGPIO10, or SGPIO11)
        ExternalSGPIOPinSGPIO8SGPIO9SGPIO10OrSGPIO11 = 3
    ],
    /// Select qualifier pin.
    QUALIFIER_PIN_MODE OFFSET(7) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select qualifier slice.
    QUALIFIER_SLICE_MODE OFFSET(9) NUMBITS(2) [
        /// Slice A, but for slice A slice D is used.
        SliceAButForSliceASliceDIsUsed = 0,
        /// Slice H, but for slice H slice O is used.
        SliceHButForSliceHSliceOIsUsed = 1,
        /// Slice I, but for slice I slice D is used.
        SliceIButForSliceISliceDIsUsed = 2,
        /// Slice P, but for slice P slice O is used.
        SlicePButForSlicePSliceOIsUsed = 3
    ],
    /// Enable concatenation.
    CONCAT_ENABLE OFFSET(11) NUMBITS(1) [
        /// External data pin
        ExternalDataPin = 0,
        /// Concatenate data
        ConcatenateData = 1
    ],
    /// Select concatenation order
    CONCAT_ORDER OFFSET(12) NUMBITS(2) [
        /// Self-loop
        SelfLoop = 0,
        /// 2 slices
        _2Slices = 1,
        /// 4 slices
        _4Slices = 2,
        /// 8 slices
        _8Slices = 3
    ]
],
SGPIO_MUX_CFG[12] [
    /// Select clock signal.
    EXT_CLK_ENABLE OFFSET(0) NUMBITS(1) [
        /// Internal clock signal (slice)
        InternalClockSignalSlice = 0,
        /// External clock signal (pin)
        ExternalClockSignalPin = 1
    ],
    /// Select source clock pin.
    CLK_SOURCE_PIN_MODE OFFSET(1) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select clock source slice. Note that slices D, H, O and P do not support this mo
    CLK_SOURCE_SLICE_MODE OFFSET(3) NUMBITS(2) [
        /// Slice D
        SliceD = 0,
        /// Slice H
        SliceH = 1,
        /// Slice O
        SliceO = 2,
        /// Slice P
        SliceP = 3
    ],
    /// Select qualifier mode.
    QUALIFIER_MODE OFFSET(5) NUMBITS(2) [
        /// Enable
        Enable = 0,
        /// Disable
        Disable = 1,
        /// Slice (see bits QUALIFIER_SLICE_MODE in this register)
        SliceSeeBitsQUALIFIER_SLICE_MODEInThisRegister = 2,
        /// External SGPIO pin (SGPIO8, SGPIO9, SGPIO10, or SGPIO11)
        ExternalSGPIOPinSGPIO8SGPIO9SGPIO10OrSGPIO11 = 3
    ],
    /// Select qualifier pin.
    QUALIFIER_PIN_MODE OFFSET(7) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select qualifier slice.
    QUALIFIER_SLICE_MODE OFFSET(9) NUMBITS(2) [
        /// Slice A, but for slice A slice D is used.
        SliceAButForSliceASliceDIsUsed = 0,
        /// Slice H, but for slice H slice O is used.
        SliceHButForSliceHSliceOIsUsed = 1,
        /// Slice I, but for slice I slice D is used.
        SliceIButForSliceISliceDIsUsed = 2,
        /// Slice P, but for slice P slice O is used.
        SlicePButForSlicePSliceOIsUsed = 3
    ],
    /// Enable concatenation.
    CONCAT_ENABLE OFFSET(11) NUMBITS(1) [
        /// External data pin
        ExternalDataPin = 0,
        /// Concatenate data
        ConcatenateData = 1
    ],
    /// Select concatenation order
    CONCAT_ORDER OFFSET(12) NUMBITS(2) [
        /// Self-loop
        SelfLoop = 0,
        /// 2 slices
        _2Slices = 1,
        /// 4 slices
        _4Slices = 2,
        /// 8 slices
        _8Slices = 3
    ]
],
SGPIO_MUX_CFG[13] [
    /// Select clock signal.
    EXT_CLK_ENABLE OFFSET(0) NUMBITS(1) [
        /// Internal clock signal (slice)
        InternalClockSignalSlice = 0,
        /// External clock signal (pin)
        ExternalClockSignalPin = 1
    ],
    /// Select source clock pin.
    CLK_SOURCE_PIN_MODE OFFSET(1) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select clock source slice. Note that slices D, H, O and P do not support this mo
    CLK_SOURCE_SLICE_MODE OFFSET(3) NUMBITS(2) [
        /// Slice D
        SliceD = 0,
        /// Slice H
        SliceH = 1,
        /// Slice O
        SliceO = 2,
        /// Slice P
        SliceP = 3
    ],
    /// Select qualifier mode.
    QUALIFIER_MODE OFFSET(5) NUMBITS(2) [
        /// Enable
        Enable = 0,
        /// Disable
        Disable = 1,
        /// Slice (see bits QUALIFIER_SLICE_MODE in this register)
        SliceSeeBitsQUALIFIER_SLICE_MODEInThisRegister = 2,
        /// External SGPIO pin (SGPIO8, SGPIO9, SGPIO10, or SGPIO11)
        ExternalSGPIOPinSGPIO8SGPIO9SGPIO10OrSGPIO11 = 3
    ],
    /// Select qualifier pin.
    QUALIFIER_PIN_MODE OFFSET(7) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select qualifier slice.
    QUALIFIER_SLICE_MODE OFFSET(9) NUMBITS(2) [
        /// Slice A, but for slice A slice D is used.
        SliceAButForSliceASliceDIsUsed = 0,
        /// Slice H, but for slice H slice O is used.
        SliceHButForSliceHSliceOIsUsed = 1,
        /// Slice I, but for slice I slice D is used.
        SliceIButForSliceISliceDIsUsed = 2,
        /// Slice P, but for slice P slice O is used.
        SlicePButForSlicePSliceOIsUsed = 3
    ],
    /// Enable concatenation.
    CONCAT_ENABLE OFFSET(11) NUMBITS(1) [
        /// External data pin
        ExternalDataPin = 0,
        /// Concatenate data
        ConcatenateData = 1
    ],
    /// Select concatenation order
    CONCAT_ORDER OFFSET(12) NUMBITS(2) [
        /// Self-loop
        SelfLoop = 0,
        /// 2 slices
        _2Slices = 1,
        /// 4 slices
        _4Slices = 2,
        /// 8 slices
        _8Slices = 3
    ]
],
SGPIO_MUX_CFG[14] [
    /// Select clock signal.
    EXT_CLK_ENABLE OFFSET(0) NUMBITS(1) [
        /// Internal clock signal (slice)
        InternalClockSignalSlice = 0,
        /// External clock signal (pin)
        ExternalClockSignalPin = 1
    ],
    /// Select source clock pin.
    CLK_SOURCE_PIN_MODE OFFSET(1) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select clock source slice. Note that slices D, H, O and P do not support this mo
    CLK_SOURCE_SLICE_MODE OFFSET(3) NUMBITS(2) [
        /// Slice D
        SliceD = 0,
        /// Slice H
        SliceH = 1,
        /// Slice O
        SliceO = 2,
        /// Slice P
        SliceP = 3
    ],
    /// Select qualifier mode.
    QUALIFIER_MODE OFFSET(5) NUMBITS(2) [
        /// Enable
        Enable = 0,
        /// Disable
        Disable = 1,
        /// Slice (see bits QUALIFIER_SLICE_MODE in this register)
        SliceSeeBitsQUALIFIER_SLICE_MODEInThisRegister = 2,
        /// External SGPIO pin (SGPIO8, SGPIO9, SGPIO10, or SGPIO11)
        ExternalSGPIOPinSGPIO8SGPIO9SGPIO10OrSGPIO11 = 3
    ],
    /// Select qualifier pin.
    QUALIFIER_PIN_MODE OFFSET(7) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select qualifier slice.
    QUALIFIER_SLICE_MODE OFFSET(9) NUMBITS(2) [
        /// Slice A, but for slice A slice D is used.
        SliceAButForSliceASliceDIsUsed = 0,
        /// Slice H, but for slice H slice O is used.
        SliceHButForSliceHSliceOIsUsed = 1,
        /// Slice I, but for slice I slice D is used.
        SliceIButForSliceISliceDIsUsed = 2,
        /// Slice P, but for slice P slice O is used.
        SlicePButForSlicePSliceOIsUsed = 3
    ],
    /// Enable concatenation.
    CONCAT_ENABLE OFFSET(11) NUMBITS(1) [
        /// External data pin
        ExternalDataPin = 0,
        /// Concatenate data
        ConcatenateData = 1
    ],
    /// Select concatenation order
    CONCAT_ORDER OFFSET(12) NUMBITS(2) [
        /// Self-loop
        SelfLoop = 0,
        /// 2 slices
        _2Slices = 1,
        /// 4 slices
        _4Slices = 2,
        /// 8 slices
        _8Slices = 3
    ]
],
SGPIO_MUX_CFG[15] [
    /// Select clock signal.
    EXT_CLK_ENABLE OFFSET(0) NUMBITS(1) [
        /// Internal clock signal (slice)
        InternalClockSignalSlice = 0,
        /// External clock signal (pin)
        ExternalClockSignalPin = 1
    ],
    /// Select source clock pin.
    CLK_SOURCE_PIN_MODE OFFSET(1) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select clock source slice. Note that slices D, H, O and P do not support this mo
    CLK_SOURCE_SLICE_MODE OFFSET(3) NUMBITS(2) [
        /// Slice D
        SliceD = 0,
        /// Slice H
        SliceH = 1,
        /// Slice O
        SliceO = 2,
        /// Slice P
        SliceP = 3
    ],
    /// Select qualifier mode.
    QUALIFIER_MODE OFFSET(5) NUMBITS(2) [
        /// Enable
        Enable = 0,
        /// Disable
        Disable = 1,
        /// Slice (see bits QUALIFIER_SLICE_MODE in this register)
        SliceSeeBitsQUALIFIER_SLICE_MODEInThisRegister = 2,
        /// External SGPIO pin (SGPIO8, SGPIO9, SGPIO10, or SGPIO11)
        ExternalSGPIOPinSGPIO8SGPIO9SGPIO10OrSGPIO11 = 3
    ],
    /// Select qualifier pin.
    QUALIFIER_PIN_MODE OFFSET(7) NUMBITS(2) [
        /// SGPIO8
        SGPIO8 = 0,
        /// SGPIO9
        SGPIO9 = 1,
        /// SGPIO10
        SGPIO10 = 2,
        /// SGPIO11
        SGPIO11 = 3
    ],
    /// Select qualifier slice.
    QUALIFIER_SLICE_MODE OFFSET(9) NUMBITS(2) [
        /// Slice A, but for slice A slice D is used.
        SliceAButForSliceASliceDIsUsed = 0,
        /// Slice H, but for slice H slice O is used.
        SliceHButForSliceHSliceOIsUsed = 1,
        /// Slice I, but for slice I slice D is used.
        SliceIButForSliceISliceDIsUsed = 2,
        /// Slice P, but for slice P slice O is used.
        SlicePButForSlicePSliceOIsUsed = 3
    ],
    /// Enable concatenation.
    CONCAT_ENABLE OFFSET(11) NUMBITS(1) [
        /// External data pin
        ExternalDataPin = 0,
        /// Concatenate data
        ConcatenateData = 1
    ],
    /// Select concatenation order
    CONCAT_ORDER OFFSET(12) NUMBITS(2) [
        /// Self-loop
        SelfLoop = 0,
        /// 2 slices
        _2Slices = 1,
        /// 4 slices
        _4Slices = 2,
        /// 8 slices
        _8Slices = 3
    ]
],
SLICE_MUX_CFG[0] [
    /// Match mode. Selects whether the match filter is active or whether data is captur
    MATCH_MODE OFFSET(0) NUMBITS(1) [
        /// Do not match data.
        DoNotMatchData = 0,
        /// Match data.
        MatchData = 1
    ],
    /// Capture clock mode
    CLK_CAPTURE_MODE OFFSET(1) NUMBITS(1) [
        /// Use rising clock edge.
        UseRisingClockEdge = 0,
        /// Use falling clock edge.
        UseFallingClockEdge = 1
    ],
    /// Clock generation mode. Selects the clock generated by the slice counter or by an
    CLKGEN_MODE OFFSET(2) NUMBITS(1) [
        /// Use clock internally generated by COUNTER.
        UseClockInternallyGeneratedByCOUNTER = 0,
        /// Use external clock from a pin or other slice.
        UseExternalClockFromAPinOrOtherSlice = 1
    ],
    /// Invert output clock
    INV_OUT_CLK OFFSET(3) NUMBITS(1) [
        /// Normal clock.
        NormalClock = 0,
        /// Inverted clock.
        InvertedClock = 1
    ],
    /// Condition for input bit match interrupt
    DATA_CAPTURE_MODE OFFSET(4) NUMBITS(2) [
        /// Detect rising edge.
        DetectRisingEdge = 0,
        /// Detect falling edge.
        DetectFallingEdge = 1,
        /// Detect LOW level.
        DetectLOWLevel = 2,
        /// Detect HIGH level.
        DetectHIGHLevel = 3
    ],
    /// Parallel mode
    PARALLEL_MODE OFFSET(6) NUMBITS(2) [
        /// Shift 1 bit per clock.
        Shift1BitPerClock = 0,
        /// Shift 2 bits per clock.
        Shift2BitsPerClock = 1,
        /// Shift 4 bits per clock.
        Shift4BitsPerClock = 2,
        /// Shift 1 byte per clock.
        Shift1BytePerClock = 3
    ],
    /// Inversion qualifier
    INV_QUALIFIER OFFSET(8) NUMBITS(1) [
        /// Use normal qualifier.
        UseNormalQualifier = 0,
        /// Use inverted qualifier.
        UseInvertedQualifier = 1
    ]
],
SLICE_MUX_CFG[1] [
    /// Match mode. Selects whether the match filter is active or whether data is captur
    MATCH_MODE OFFSET(0) NUMBITS(1) [
        /// Do not match data.
        DoNotMatchData = 0,
        /// Match data.
        MatchData = 1
    ],
    /// Capture clock mode
    CLK_CAPTURE_MODE OFFSET(1) NUMBITS(1) [
        /// Use rising clock edge.
        UseRisingClockEdge = 0,
        /// Use falling clock edge.
        UseFallingClockEdge = 1
    ],
    /// Clock generation mode. Selects the clock generated by the slice counter or by an
    CLKGEN_MODE OFFSET(2) NUMBITS(1) [
        /// Use clock internally generated by COUNTER.
        UseClockInternallyGeneratedByCOUNTER = 0,
        /// Use external clock from a pin or other slice.
        UseExternalClockFromAPinOrOtherSlice = 1
    ],
    /// Invert output clock
    INV_OUT_CLK OFFSET(3) NUMBITS(1) [
        /// Normal clock.
        NormalClock = 0,
        /// Inverted clock.
        InvertedClock = 1
    ],
    /// Condition for input bit match interrupt
    DATA_CAPTURE_MODE OFFSET(4) NUMBITS(2) [
        /// Detect rising edge.
        DetectRisingEdge = 0,
        /// Detect falling edge.
        DetectFallingEdge = 1,
        /// Detect LOW level.
        DetectLOWLevel = 2,
        /// Detect HIGH level.
        DetectHIGHLevel = 3
    ],
    /// Parallel mode
    PARALLEL_MODE OFFSET(6) NUMBITS(2) [
        /// Shift 1 bit per clock.
        Shift1BitPerClock = 0,
        /// Shift 2 bits per clock.
        Shift2BitsPerClock = 1,
        /// Shift 4 bits per clock.
        Shift4BitsPerClock = 2,
        /// Shift 1 byte per clock.
        Shift1BytePerClock = 3
    ],
    /// Inversion qualifier
    INV_QUALIFIER OFFSET(8) NUMBITS(1) [
        /// Use normal qualifier.
        UseNormalQualifier = 0,
        /// Use inverted qualifier.
        UseInvertedQualifier = 1
    ]
],
SLICE_MUX_CFG[2] [
    /// Match mode. Selects whether the match filter is active or whether data is captur
    MATCH_MODE OFFSET(0) NUMBITS(1) [
        /// Do not match data.
        DoNotMatchData = 0,
        /// Match data.
        MatchData = 1
    ],
    /// Capture clock mode
    CLK_CAPTURE_MODE OFFSET(1) NUMBITS(1) [
        /// Use rising clock edge.
        UseRisingClockEdge = 0,
        /// Use falling clock edge.
        UseFallingClockEdge = 1
    ],
    /// Clock generation mode. Selects the clock generated by the slice counter or by an
    CLKGEN_MODE OFFSET(2) NUMBITS(1) [
        /// Use clock internally generated by COUNTER.
        UseClockInternallyGeneratedByCOUNTER = 0,
        /// Use external clock from a pin or other slice.
        UseExternalClockFromAPinOrOtherSlice = 1
    ],
    /// Invert output clock
    INV_OUT_CLK OFFSET(3) NUMBITS(1) [
        /// Normal clock.
        NormalClock = 0,
        /// Inverted clock.
        InvertedClock = 1
    ],
    /// Condition for input bit match interrupt
    DATA_CAPTURE_MODE OFFSET(4) NUMBITS(2) [
        /// Detect rising edge.
        DetectRisingEdge = 0,
        /// Detect falling edge.
        DetectFallingEdge = 1,
        /// Detect LOW level.
        DetectLOWLevel = 2,
        /// Detect HIGH level.
        DetectHIGHLevel = 3
    ],
    /// Parallel mode
    PARALLEL_MODE OFFSET(6) NUMBITS(2) [
        /// Shift 1 bit per clock.
        Shift1BitPerClock = 0,
        /// Shift 2 bits per clock.
        Shift2BitsPerClock = 1,
        /// Shift 4 bits per clock.
        Shift4BitsPerClock = 2,
        /// Shift 1 byte per clock.
        Shift1BytePerClock = 3
    ],
    /// Inversion qualifier
    INV_QUALIFIER OFFSET(8) NUMBITS(1) [
        /// Use normal qualifier.
        UseNormalQualifier = 0,
        /// Use inverted qualifier.
        UseInvertedQualifier = 1
    ]
],
SLICE_MUX_CFG[3] [
    /// Match mode. Selects whether the match filter is active or whether data is captur
    MATCH_MODE OFFSET(0) NUMBITS(1) [
        /// Do not match data.
        DoNotMatchData = 0,
        /// Match data.
        MatchData = 1
    ],
    /// Capture clock mode
    CLK_CAPTURE_MODE OFFSET(1) NUMBITS(1) [
        /// Use rising clock edge.
        UseRisingClockEdge = 0,
        /// Use falling clock edge.
        UseFallingClockEdge = 1
    ],
    /// Clock generation mode. Selects the clock generated by the slice counter or by an
    CLKGEN_MODE OFFSET(2) NUMBITS(1) [
        /// Use clock internally generated by COUNTER.
        UseClockInternallyGeneratedByCOUNTER = 0,
        /// Use external clock from a pin or other slice.
        UseExternalClockFromAPinOrOtherSlice = 1
    ],
    /// Invert output clock
    INV_OUT_CLK OFFSET(3) NUMBITS(1) [
        /// Normal clock.
        NormalClock = 0,
        /// Inverted clock.
        InvertedClock = 1
    ],
    /// Condition for input bit match interrupt
    DATA_CAPTURE_MODE OFFSET(4) NUMBITS(2) [
        /// Detect rising edge.
        DetectRisingEdge = 0,
        /// Detect falling edge.
        DetectFallingEdge = 1,
        /// Detect LOW level.
        DetectLOWLevel = 2,
        /// Detect HIGH level.
        DetectHIGHLevel = 3
    ],
    /// Parallel mode
    PARALLEL_MODE OFFSET(6) NUMBITS(2) [
        /// Shift 1 bit per clock.
        Shift1BitPerClock = 0,
        /// Shift 2 bits per clock.
        Shift2BitsPerClock = 1,
        /// Shift 4 bits per clock.
        Shift4BitsPerClock = 2,
        /// Shift 1 byte per clock.
        Shift1BytePerClock = 3
    ],
    /// Inversion qualifier
    INV_QUALIFIER OFFSET(8) NUMBITS(1) [
        /// Use normal qualifier.
        UseNormalQualifier = 0,
        /// Use inverted qualifier.
        UseInvertedQualifier = 1
    ]
],
SLICE_MUX_CFG[4] [
    /// Match mode. Selects whether the match filter is active or whether data is captur
    MATCH_MODE OFFSET(0) NUMBITS(1) [
        /// Do not match data.
        DoNotMatchData = 0,
        /// Match data.
        MatchData = 1
    ],
    /// Capture clock mode
    CLK_CAPTURE_MODE OFFSET(1) NUMBITS(1) [
        /// Use rising clock edge.
        UseRisingClockEdge = 0,
        /// Use falling clock edge.
        UseFallingClockEdge = 1
    ],
    /// Clock generation mode. Selects the clock generated by the slice counter or by an
    CLKGEN_MODE OFFSET(2) NUMBITS(1) [
        /// Use clock internally generated by COUNTER.
        UseClockInternallyGeneratedByCOUNTER = 0,
        /// Use external clock from a pin or other slice.
        UseExternalClockFromAPinOrOtherSlice = 1
    ],
    /// Invert output clock
    INV_OUT_CLK OFFSET(3) NUMBITS(1) [
        /// Normal clock.
        NormalClock = 0,
        /// Inverted clock.
        InvertedClock = 1
    ],
    /// Condition for input bit match interrupt
    DATA_CAPTURE_MODE OFFSET(4) NUMBITS(2) [
        /// Detect rising edge.
        DetectRisingEdge = 0,
        /// Detect falling edge.
        DetectFallingEdge = 1,
        /// Detect LOW level.
        DetectLOWLevel = 2,
        /// Detect HIGH level.
        DetectHIGHLevel = 3
    ],
    /// Parallel mode
    PARALLEL_MODE OFFSET(6) NUMBITS(2) [
        /// Shift 1 bit per clock.
        Shift1BitPerClock = 0,
        /// Shift 2 bits per clock.
        Shift2BitsPerClock = 1,
        /// Shift 4 bits per clock.
        Shift4BitsPerClock = 2,
        /// Shift 1 byte per clock.
        Shift1BytePerClock = 3
    ],
    /// Inversion qualifier
    INV_QUALIFIER OFFSET(8) NUMBITS(1) [
        /// Use normal qualifier.
        UseNormalQualifier = 0,
        /// Use inverted qualifier.
        UseInvertedQualifier = 1
    ]
],
SLICE_MUX_CFG[5] [
    /// Match mode. Selects whether the match filter is active or whether data is captur
    MATCH_MODE OFFSET(0) NUMBITS(1) [
        /// Do not match data.
        DoNotMatchData = 0,
        /// Match data.
        MatchData = 1
    ],
    /// Capture clock mode
    CLK_CAPTURE_MODE OFFSET(1) NUMBITS(1) [
        /// Use rising clock edge.
        UseRisingClockEdge = 0,
        /// Use falling clock edge.
        UseFallingClockEdge = 1
    ],
    /// Clock generation mode. Selects the clock generated by the slice counter or by an
    CLKGEN_MODE OFFSET(2) NUMBITS(1) [
        /// Use clock internally generated by COUNTER.
        UseClockInternallyGeneratedByCOUNTER = 0,
        /// Use external clock from a pin or other slice.
        UseExternalClockFromAPinOrOtherSlice = 1
    ],
    /// Invert output clock
    INV_OUT_CLK OFFSET(3) NUMBITS(1) [
        /// Normal clock.
        NormalClock = 0,
        /// Inverted clock.
        InvertedClock = 1
    ],
    /// Condition for input bit match interrupt
    DATA_CAPTURE_MODE OFFSET(4) NUMBITS(2) [
        /// Detect rising edge.
        DetectRisingEdge = 0,
        /// Detect falling edge.
        DetectFallingEdge = 1,
        /// Detect LOW level.
        DetectLOWLevel = 2,
        /// Detect HIGH level.
        DetectHIGHLevel = 3
    ],
    /// Parallel mode
    PARALLEL_MODE OFFSET(6) NUMBITS(2) [
        /// Shift 1 bit per clock.
        Shift1BitPerClock = 0,
        /// Shift 2 bits per clock.
        Shift2BitsPerClock = 1,
        /// Shift 4 bits per clock.
        Shift4BitsPerClock = 2,
        /// Shift 1 byte per clock.
        Shift1BytePerClock = 3
    ],
    /// Inversion qualifier
    INV_QUALIFIER OFFSET(8) NUMBITS(1) [
        /// Use normal qualifier.
        UseNormalQualifier = 0,
        /// Use inverted qualifier.
        UseInvertedQualifier = 1
    ]
],
SLICE_MUX_CFG[6] [
    /// Match mode. Selects whether the match filter is active or whether data is captur
    MATCH_MODE OFFSET(0) NUMBITS(1) [
        /// Do not match data.
        DoNotMatchData = 0,
        /// Match data.
        MatchData = 1
    ],
    /// Capture clock mode
    CLK_CAPTURE_MODE OFFSET(1) NUMBITS(1) [
        /// Use rising clock edge.
        UseRisingClockEdge = 0,
        /// Use falling clock edge.
        UseFallingClockEdge = 1
    ],
    /// Clock generation mode. Selects the clock generated by the slice counter or by an
    CLKGEN_MODE OFFSET(2) NUMBITS(1) [
        /// Use clock internally generated by COUNTER.
        UseClockInternallyGeneratedByCOUNTER = 0,
        /// Use external clock from a pin or other slice.
        UseExternalClockFromAPinOrOtherSlice = 1
    ],
    /// Invert output clock
    INV_OUT_CLK OFFSET(3) NUMBITS(1) [
        /// Normal clock.
        NormalClock = 0,
        /// Inverted clock.
        InvertedClock = 1
    ],
    /// Condition for input bit match interrupt
    DATA_CAPTURE_MODE OFFSET(4) NUMBITS(2) [
        /// Detect rising edge.
        DetectRisingEdge = 0,
        /// Detect falling edge.
        DetectFallingEdge = 1,
        /// Detect LOW level.
        DetectLOWLevel = 2,
        /// Detect HIGH level.
        DetectHIGHLevel = 3
    ],
    /// Parallel mode
    PARALLEL_MODE OFFSET(6) NUMBITS(2) [
        /// Shift 1 bit per clock.
        Shift1BitPerClock = 0,
        /// Shift 2 bits per clock.
        Shift2BitsPerClock = 1,
        /// Shift 4 bits per clock.
        Shift4BitsPerClock = 2,
        /// Shift 1 byte per clock.
        Shift1BytePerClock = 3
    ],
    /// Inversion qualifier
    INV_QUALIFIER OFFSET(8) NUMBITS(1) [
        /// Use normal qualifier.
        UseNormalQualifier = 0,
        /// Use inverted qualifier.
        UseInvertedQualifier = 1
    ]
],
SLICE_MUX_CFG[7] [
    /// Match mode. Selects whether the match filter is active or whether data is captur
    MATCH_MODE OFFSET(0) NUMBITS(1) [
        /// Do not match data.
        DoNotMatchData = 0,
        /// Match data.
        MatchData = 1
    ],
    /// Capture clock mode
    CLK_CAPTURE_MODE OFFSET(1) NUMBITS(1) [
        /// Use rising clock edge.
        UseRisingClockEdge = 0,
        /// Use falling clock edge.
        UseFallingClockEdge = 1
    ],
    /// Clock generation mode. Selects the clock generated by the slice counter or by an
    CLKGEN_MODE OFFSET(2) NUMBITS(1) [
        /// Use clock internally generated by COUNTER.
        UseClockInternallyGeneratedByCOUNTER = 0,
        /// Use external clock from a pin or other slice.
        UseExternalClockFromAPinOrOtherSlice = 1
    ],
    /// Invert output clock
    INV_OUT_CLK OFFSET(3) NUMBITS(1) [
        /// Normal clock.
        NormalClock = 0,
        /// Inverted clock.
        InvertedClock = 1
    ],
    /// Condition for input bit match interrupt
    DATA_CAPTURE_MODE OFFSET(4) NUMBITS(2) [
        /// Detect rising edge.
        DetectRisingEdge = 0,
        /// Detect falling edge.
        DetectFallingEdge = 1,
        /// Detect LOW level.
        DetectLOWLevel = 2,
        /// Detect HIGH level.
        DetectHIGHLevel = 3
    ],
    /// Parallel mode
    PARALLEL_MODE OFFSET(6) NUMBITS(2) [
        /// Shift 1 bit per clock.
        Shift1BitPerClock = 0,
        /// Shift 2 bits per clock.
        Shift2BitsPerClock = 1,
        /// Shift 4 bits per clock.
        Shift4BitsPerClock = 2,
        /// Shift 1 byte per clock.
        Shift1BytePerClock = 3
    ],
    /// Inversion qualifier
    INV_QUALIFIER OFFSET(8) NUMBITS(1) [
        /// Use normal qualifier.
        UseNormalQualifier = 0,
        /// Use inverted qualifier.
        UseInvertedQualifier = 1
    ]
],
SLICE_MUX_CFG[8] [
    /// Match mode. Selects whether the match filter is active or whether data is captur
    MATCH_MODE OFFSET(0) NUMBITS(1) [
        /// Do not match data.
        DoNotMatchData = 0,
        /// Match data.
        MatchData = 1
    ],
    /// Capture clock mode
    CLK_CAPTURE_MODE OFFSET(1) NUMBITS(1) [
        /// Use rising clock edge.
        UseRisingClockEdge = 0,
        /// Use falling clock edge.
        UseFallingClockEdge = 1
    ],
    /// Clock generation mode. Selects the clock generated by the slice counter or by an
    CLKGEN_MODE OFFSET(2) NUMBITS(1) [
        /// Use clock internally generated by COUNTER.
        UseClockInternallyGeneratedByCOUNTER = 0,
        /// Use external clock from a pin or other slice.
        UseExternalClockFromAPinOrOtherSlice = 1
    ],
    /// Invert output clock
    INV_OUT_CLK OFFSET(3) NUMBITS(1) [
        /// Normal clock.
        NormalClock = 0,
        /// Inverted clock.
        InvertedClock = 1
    ],
    /// Condition for input bit match interrupt
    DATA_CAPTURE_MODE OFFSET(4) NUMBITS(2) [
        /// Detect rising edge.
        DetectRisingEdge = 0,
        /// Detect falling edge.
        DetectFallingEdge = 1,
        /// Detect LOW level.
        DetectLOWLevel = 2,
        /// Detect HIGH level.
        DetectHIGHLevel = 3
    ],
    /// Parallel mode
    PARALLEL_MODE OFFSET(6) NUMBITS(2) [
        /// Shift 1 bit per clock.
        Shift1BitPerClock = 0,
        /// Shift 2 bits per clock.
        Shift2BitsPerClock = 1,
        /// Shift 4 bits per clock.
        Shift4BitsPerClock = 2,
        /// Shift 1 byte per clock.
        Shift1BytePerClock = 3
    ],
    /// Inversion qualifier
    INV_QUALIFIER OFFSET(8) NUMBITS(1) [
        /// Use normal qualifier.
        UseNormalQualifier = 0,
        /// Use inverted qualifier.
        UseInvertedQualifier = 1
    ]
],
SLICE_MUX_CFG[9] [
    /// Match mode. Selects whether the match filter is active or whether data is captur
    MATCH_MODE OFFSET(0) NUMBITS(1) [
        /// Do not match data.
        DoNotMatchData = 0,
        /// Match data.
        MatchData = 1
    ],
    /// Capture clock mode
    CLK_CAPTURE_MODE OFFSET(1) NUMBITS(1) [
        /// Use rising clock edge.
        UseRisingClockEdge = 0,
        /// Use falling clock edge.
        UseFallingClockEdge = 1
    ],
    /// Clock generation mode. Selects the clock generated by the slice counter or by an
    CLKGEN_MODE OFFSET(2) NUMBITS(1) [
        /// Use clock internally generated by COUNTER.
        UseClockInternallyGeneratedByCOUNTER = 0,
        /// Use external clock from a pin or other slice.
        UseExternalClockFromAPinOrOtherSlice = 1
    ],
    /// Invert output clock
    INV_OUT_CLK OFFSET(3) NUMBITS(1) [
        /// Normal clock.
        NormalClock = 0,
        /// Inverted clock.
        InvertedClock = 1
    ],
    /// Condition for input bit match interrupt
    DATA_CAPTURE_MODE OFFSET(4) NUMBITS(2) [
        /// Detect rising edge.
        DetectRisingEdge = 0,
        /// Detect falling edge.
        DetectFallingEdge = 1,
        /// Detect LOW level.
        DetectLOWLevel = 2,
        /// Detect HIGH level.
        DetectHIGHLevel = 3
    ],
    /// Parallel mode
    PARALLEL_MODE OFFSET(6) NUMBITS(2) [
        /// Shift 1 bit per clock.
        Shift1BitPerClock = 0,
        /// Shift 2 bits per clock.
        Shift2BitsPerClock = 1,
        /// Shift 4 bits per clock.
        Shift4BitsPerClock = 2,
        /// Shift 1 byte per clock.
        Shift1BytePerClock = 3
    ],
    /// Inversion qualifier
    INV_QUALIFIER OFFSET(8) NUMBITS(1) [
        /// Use normal qualifier.
        UseNormalQualifier = 0,
        /// Use inverted qualifier.
        UseInvertedQualifier = 1
    ]
],
SLICE_MUX_CFG[10] [
    /// Match mode. Selects whether the match filter is active or whether data is captur
    MATCH_MODE OFFSET(0) NUMBITS(1) [
        /// Do not match data.
        DoNotMatchData = 0,
        /// Match data.
        MatchData = 1
    ],
    /// Capture clock mode
    CLK_CAPTURE_MODE OFFSET(1) NUMBITS(1) [
        /// Use rising clock edge.
        UseRisingClockEdge = 0,
        /// Use falling clock edge.
        UseFallingClockEdge = 1
    ],
    /// Clock generation mode. Selects the clock generated by the slice counter or by an
    CLKGEN_MODE OFFSET(2) NUMBITS(1) [
        /// Use clock internally generated by COUNTER.
        UseClockInternallyGeneratedByCOUNTER = 0,
        /// Use external clock from a pin or other slice.
        UseExternalClockFromAPinOrOtherSlice = 1
    ],
    /// Invert output clock
    INV_OUT_CLK OFFSET(3) NUMBITS(1) [
        /// Normal clock.
        NormalClock = 0,
        /// Inverted clock.
        InvertedClock = 1
    ],
    /// Condition for input bit match interrupt
    DATA_CAPTURE_MODE OFFSET(4) NUMBITS(2) [
        /// Detect rising edge.
        DetectRisingEdge = 0,
        /// Detect falling edge.
        DetectFallingEdge = 1,
        /// Detect LOW level.
        DetectLOWLevel = 2,
        /// Detect HIGH level.
        DetectHIGHLevel = 3
    ],
    /// Parallel mode
    PARALLEL_MODE OFFSET(6) NUMBITS(2) [
        /// Shift 1 bit per clock.
        Shift1BitPerClock = 0,
        /// Shift 2 bits per clock.
        Shift2BitsPerClock = 1,
        /// Shift 4 bits per clock.
        Shift4BitsPerClock = 2,
        /// Shift 1 byte per clock.
        Shift1BytePerClock = 3
    ],
    /// Inversion qualifier
    INV_QUALIFIER OFFSET(8) NUMBITS(1) [
        /// Use normal qualifier.
        UseNormalQualifier = 0,
        /// Use inverted qualifier.
        UseInvertedQualifier = 1
    ]
],
SLICE_MUX_CFG[11] [
    /// Match mode. Selects whether the match filter is active or whether data is captur
    MATCH_MODE OFFSET(0) NUMBITS(1) [
        /// Do not match data.
        DoNotMatchData = 0,
        /// Match data.
        MatchData = 1
    ],
    /// Capture clock mode
    CLK_CAPTURE_MODE OFFSET(1) NUMBITS(1) [
        /// Use rising clock edge.
        UseRisingClockEdge = 0,
        /// Use falling clock edge.
        UseFallingClockEdge = 1
    ],
    /// Clock generation mode. Selects the clock generated by the slice counter or by an
    CLKGEN_MODE OFFSET(2) NUMBITS(1) [
        /// Use clock internally generated by COUNTER.
        UseClockInternallyGeneratedByCOUNTER = 0,
        /// Use external clock from a pin or other slice.
        UseExternalClockFromAPinOrOtherSlice = 1
    ],
    /// Invert output clock
    INV_OUT_CLK OFFSET(3) NUMBITS(1) [
        /// Normal clock.
        NormalClock = 0,
        /// Inverted clock.
        InvertedClock = 1
    ],
    /// Condition for input bit match interrupt
    DATA_CAPTURE_MODE OFFSET(4) NUMBITS(2) [
        /// Detect rising edge.
        DetectRisingEdge = 0,
        /// Detect falling edge.
        DetectFallingEdge = 1,
        /// Detect LOW level.
        DetectLOWLevel = 2,
        /// Detect HIGH level.
        DetectHIGHLevel = 3
    ],
    /// Parallel mode
    PARALLEL_MODE OFFSET(6) NUMBITS(2) [
        /// Shift 1 bit per clock.
        Shift1BitPerClock = 0,
        /// Shift 2 bits per clock.
        Shift2BitsPerClock = 1,
        /// Shift 4 bits per clock.
        Shift4BitsPerClock = 2,
        /// Shift 1 byte per clock.
        Shift1BytePerClock = 3
    ],
    /// Inversion qualifier
    INV_QUALIFIER OFFSET(8) NUMBITS(1) [
        /// Use normal qualifier.
        UseNormalQualifier = 0,
        /// Use inverted qualifier.
        UseInvertedQualifier = 1
    ]
],
SLICE_MUX_CFG[12] [
    /// Match mode. Selects whether the match filter is active or whether data is captur
    MATCH_MODE OFFSET(0) NUMBITS(1) [
        /// Do not match data.
        DoNotMatchData = 0,
        /// Match data.
        MatchData = 1
    ],
    /// Capture clock mode
    CLK_CAPTURE_MODE OFFSET(1) NUMBITS(1) [
        /// Use rising clock edge.
        UseRisingClockEdge = 0,
        /// Use falling clock edge.
        UseFallingClockEdge = 1
    ],
    /// Clock generation mode. Selects the clock generated by the slice counter or by an
    CLKGEN_MODE OFFSET(2) NUMBITS(1) [
        /// Use clock internally generated by COUNTER.
        UseClockInternallyGeneratedByCOUNTER = 0,
        /// Use external clock from a pin or other slice.
        UseExternalClockFromAPinOrOtherSlice = 1
    ],
    /// Invert output clock
    INV_OUT_CLK OFFSET(3) NUMBITS(1) [
        /// Normal clock.
        NormalClock = 0,
        /// Inverted clock.
        InvertedClock = 1
    ],
    /// Condition for input bit match interrupt
    DATA_CAPTURE_MODE OFFSET(4) NUMBITS(2) [
        /// Detect rising edge.
        DetectRisingEdge = 0,
        /// Detect falling edge.
        DetectFallingEdge = 1,
        /// Detect LOW level.
        DetectLOWLevel = 2,
        /// Detect HIGH level.
        DetectHIGHLevel = 3
    ],
    /// Parallel mode
    PARALLEL_MODE OFFSET(6) NUMBITS(2) [
        /// Shift 1 bit per clock.
        Shift1BitPerClock = 0,
        /// Shift 2 bits per clock.
        Shift2BitsPerClock = 1,
        /// Shift 4 bits per clock.
        Shift4BitsPerClock = 2,
        /// Shift 1 byte per clock.
        Shift1BytePerClock = 3
    ],
    /// Inversion qualifier
    INV_QUALIFIER OFFSET(8) NUMBITS(1) [
        /// Use normal qualifier.
        UseNormalQualifier = 0,
        /// Use inverted qualifier.
        UseInvertedQualifier = 1
    ]
],
SLICE_MUX_CFG[13] [
    /// Match mode. Selects whether the match filter is active or whether data is captur
    MATCH_MODE OFFSET(0) NUMBITS(1) [
        /// Do not match data.
        DoNotMatchData = 0,
        /// Match data.
        MatchData = 1
    ],
    /// Capture clock mode
    CLK_CAPTURE_MODE OFFSET(1) NUMBITS(1) [
        /// Use rising clock edge.
        UseRisingClockEdge = 0,
        /// Use falling clock edge.
        UseFallingClockEdge = 1
    ],
    /// Clock generation mode. Selects the clock generated by the slice counter or by an
    CLKGEN_MODE OFFSET(2) NUMBITS(1) [
        /// Use clock internally generated by COUNTER.
        UseClockInternallyGeneratedByCOUNTER = 0,
        /// Use external clock from a pin or other slice.
        UseExternalClockFromAPinOrOtherSlice = 1
    ],
    /// Invert output clock
    INV_OUT_CLK OFFSET(3) NUMBITS(1) [
        /// Normal clock.
        NormalClock = 0,
        /// Inverted clock.
        InvertedClock = 1
    ],
    /// Condition for input bit match interrupt
    DATA_CAPTURE_MODE OFFSET(4) NUMBITS(2) [
        /// Detect rising edge.
        DetectRisingEdge = 0,
        /// Detect falling edge.
        DetectFallingEdge = 1,
        /// Detect LOW level.
        DetectLOWLevel = 2,
        /// Detect HIGH level.
        DetectHIGHLevel = 3
    ],
    /// Parallel mode
    PARALLEL_MODE OFFSET(6) NUMBITS(2) [
        /// Shift 1 bit per clock.
        Shift1BitPerClock = 0,
        /// Shift 2 bits per clock.
        Shift2BitsPerClock = 1,
        /// Shift 4 bits per clock.
        Shift4BitsPerClock = 2,
        /// Shift 1 byte per clock.
        Shift1BytePerClock = 3
    ],
    /// Inversion qualifier
    INV_QUALIFIER OFFSET(8) NUMBITS(1) [
        /// Use normal qualifier.
        UseNormalQualifier = 0,
        /// Use inverted qualifier.
        UseInvertedQualifier = 1
    ]
],
SLICE_MUX_CFG[14] [
    /// Match mode. Selects whether the match filter is active or whether data is captur
    MATCH_MODE OFFSET(0) NUMBITS(1) [
        /// Do not match data.
        DoNotMatchData = 0,
        /// Match data.
        MatchData = 1
    ],
    /// Capture clock mode
    CLK_CAPTURE_MODE OFFSET(1) NUMBITS(1) [
        /// Use rising clock edge.
        UseRisingClockEdge = 0,
        /// Use falling clock edge.
        UseFallingClockEdge = 1
    ],
    /// Clock generation mode. Selects the clock generated by the slice counter or by an
    CLKGEN_MODE OFFSET(2) NUMBITS(1) [
        /// Use clock internally generated by COUNTER.
        UseClockInternallyGeneratedByCOUNTER = 0,
        /// Use external clock from a pin or other slice.
        UseExternalClockFromAPinOrOtherSlice = 1
    ],
    /// Invert output clock
    INV_OUT_CLK OFFSET(3) NUMBITS(1) [
        /// Normal clock.
        NormalClock = 0,
        /// Inverted clock.
        InvertedClock = 1
    ],
    /// Condition for input bit match interrupt
    DATA_CAPTURE_MODE OFFSET(4) NUMBITS(2) [
        /// Detect rising edge.
        DetectRisingEdge = 0,
        /// Detect falling edge.
        DetectFallingEdge = 1,
        /// Detect LOW level.
        DetectLOWLevel = 2,
        /// Detect HIGH level.
        DetectHIGHLevel = 3
    ],
    /// Parallel mode
    PARALLEL_MODE OFFSET(6) NUMBITS(2) [
        /// Shift 1 bit per clock.
        Shift1BitPerClock = 0,
        /// Shift 2 bits per clock.
        Shift2BitsPerClock = 1,
        /// Shift 4 bits per clock.
        Shift4BitsPerClock = 2,
        /// Shift 1 byte per clock.
        Shift1BytePerClock = 3
    ],
    /// Inversion qualifier
    INV_QUALIFIER OFFSET(8) NUMBITS(1) [
        /// Use normal qualifier.
        UseNormalQualifier = 0,
        /// Use inverted qualifier.
        UseInvertedQualifier = 1
    ]
],
SLICE_MUX_CFG[15] [
    /// Match mode. Selects whether the match filter is active or whether data is captur
    MATCH_MODE OFFSET(0) NUMBITS(1) [
        /// Do not match data.
        DoNotMatchData = 0,
        /// Match data.
        MatchData = 1
    ],
    /// Capture clock mode
    CLK_CAPTURE_MODE OFFSET(1) NUMBITS(1) [
        /// Use rising clock edge.
        UseRisingClockEdge = 0,
        /// Use falling clock edge.
        UseFallingClockEdge = 1
    ],
    /// Clock generation mode. Selects the clock generated by the slice counter or by an
    CLKGEN_MODE OFFSET(2) NUMBITS(1) [
        /// Use clock internally generated by COUNTER.
        UseClockInternallyGeneratedByCOUNTER = 0,
        /// Use external clock from a pin or other slice.
        UseExternalClockFromAPinOrOtherSlice = 1
    ],
    /// Invert output clock
    INV_OUT_CLK OFFSET(3) NUMBITS(1) [
        /// Normal clock.
        NormalClock = 0,
        /// Inverted clock.
        InvertedClock = 1
    ],
    /// Condition for input bit match interrupt
    DATA_CAPTURE_MODE OFFSET(4) NUMBITS(2) [
        /// Detect rising edge.
        DetectRisingEdge = 0,
        /// Detect falling edge.
        DetectFallingEdge = 1,
        /// Detect LOW level.
        DetectLOWLevel = 2,
        /// Detect HIGH level.
        DetectHIGHLevel = 3
    ],
    /// Parallel mode
    PARALLEL_MODE OFFSET(6) NUMBITS(2) [
        /// Shift 1 bit per clock.
        Shift1BitPerClock = 0,
        /// Shift 2 bits per clock.
        Shift2BitsPerClock = 1,
        /// Shift 4 bits per clock.
        Shift4BitsPerClock = 2,
        /// Shift 1 byte per clock.
        Shift1BytePerClock = 3
    ],
    /// Inversion qualifier
    INV_QUALIFIER OFFSET(8) NUMBITS(1) [
        /// Use normal qualifier.
        UseNormalQualifier = 0,
        /// Use inverted qualifier.
        UseInvertedQualifier = 1
    ]
],
POS[0] [
    /// Each time COUNT reaches 0x0 POS counts down.
    POS OFFSET(0) NUMBITS(8) [],
    /// Reload value for POS after POS reaches 0x0.
    POS_RESET OFFSET(8) NUMBITS(8) []
],
POS[1] [
    /// Each time COUNT reaches 0x0 POS counts down.
    POS OFFSET(0) NUMBITS(8) [],
    /// Reload value for POS after POS reaches 0x0.
    POS_RESET OFFSET(8) NUMBITS(8) []
],
POS[2] [
    /// Each time COUNT reaches 0x0 POS counts down.
    POS OFFSET(0) NUMBITS(8) [],
    /// Reload value for POS after POS reaches 0x0.
    POS_RESET OFFSET(8) NUMBITS(8) []
],
POS[3] [
    /// Each time COUNT reaches 0x0 POS counts down.
    POS OFFSET(0) NUMBITS(8) [],
    /// Reload value for POS after POS reaches 0x0.
    POS_RESET OFFSET(8) NUMBITS(8) []
],
POS[4] [
    /// Each time COUNT reaches 0x0 POS counts down.
    POS OFFSET(0) NUMBITS(8) [],
    /// Reload value for POS after POS reaches 0x0.
    POS_RESET OFFSET(8) NUMBITS(8) []
],
POS[5] [
    /// Each time COUNT reaches 0x0 POS counts down.
    POS OFFSET(0) NUMBITS(8) [],
    /// Reload value for POS after POS reaches 0x0.
    POS_RESET OFFSET(8) NUMBITS(8) []
],
POS[6] [
    /// Each time COUNT reaches 0x0 POS counts down.
    POS OFFSET(0) NUMBITS(8) [],
    /// Reload value for POS after POS reaches 0x0.
    POS_RESET OFFSET(8) NUMBITS(8) []
],
POS[7] [
    /// Each time COUNT reaches 0x0 POS counts down.
    POS OFFSET(0) NUMBITS(8) [],
    /// Reload value for POS after POS reaches 0x0.
    POS_RESET OFFSET(8) NUMBITS(8) []
],
POS[8] [
    /// Each time COUNT reaches 0x0 POS counts down.
    POS OFFSET(0) NUMBITS(8) [],
    /// Reload value for POS after POS reaches 0x0.
    POS_RESET OFFSET(8) NUMBITS(8) []
],
POS[9] [
    /// Each time COUNT reaches 0x0 POS counts down.
    POS OFFSET(0) NUMBITS(8) [],
    /// Reload value for POS after POS reaches 0x0.
    POS_RESET OFFSET(8) NUMBITS(8) []
],
POS[10] [
    /// Each time COUNT reaches 0x0 POS counts down.
    POS OFFSET(0) NUMBITS(8) [],
    /// Reload value for POS after POS reaches 0x0.
    POS_RESET OFFSET(8) NUMBITS(8) []
],
POS[11] [
    /// Each time COUNT reaches 0x0 POS counts down.
    POS OFFSET(0) NUMBITS(8) [],
    /// Reload value for POS after POS reaches 0x0.
    POS_RESET OFFSET(8) NUMBITS(8) []
],
POS[12] [
    /// Each time COUNT reaches 0x0 POS counts down.
    POS OFFSET(0) NUMBITS(8) [],
    /// Reload value for POS after POS reaches 0x0.
    POS_RESET OFFSET(8) NUMBITS(8) []
],
POS[13] [
    /// Each time COUNT reaches 0x0 POS counts down.
    POS OFFSET(0) NUMBITS(8) [],
    /// Reload value for POS after POS reaches 0x0.
    POS_RESET OFFSET(8) NUMBITS(8) []
],
POS[14] [
    /// Each time COUNT reaches 0x0 POS counts down.
    POS OFFSET(0) NUMBITS(8) [],
    /// Reload value for POS after POS reaches 0x0.
    POS_RESET OFFSET(8) NUMBITS(8) []
],
POS[15] [
    /// Each time COUNT reaches 0x0 POS counts down.
    POS OFFSET(0) NUMBITS(8) [],
    /// Reload value for POS after POS reaches 0x0.
    POS_RESET OFFSET(8) NUMBITS(8) []
]
];
const SGPIO_BASE: StaticRef<SgpioRegisters> =
    unsafe { StaticRef::new(0x40101000 as *const SgpioRegisters) };
