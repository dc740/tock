#![feature(asm, concat_idents, const_fn, try_from)]
#![no_std]
#![crate_name = "lpc43xx"]
#![crate_type = "rlib"]

extern crate cortexm4;
#[allow(unused_imports)]
#[macro_use(
    debug,
    debug_verbose,
    debug_gpio
)]
extern crate kernel;

pub mod chip;
// I commented out the files that have invalid definitions from svd2regs.py
/*pub mod adchs;
pub mod adc;
pub mod atimer;
pub mod c_can0;
pub mod c_can1;
pub mod ccu1;
pub mod ccu2;
pub mod cgu;
pub mod creg;
pub mod dac;
pub mod eeprom;
pub mod emc;
pub mod ethernet;
pub mod eventrouter;
pub mod gima;
pub mod gpdma;
pub mod gpio_group_int0;
pub mod gpio_group_int1;
pub mod gpio_pin_int;*/
pub mod gpio_port;
/*pub mod i2c;
pub mod i2s;
pub mod lcd;
pub mod mcpwm;
pub mod pmc;
pub mod qei;
pub mod regfile;
pub mod rgu;
pub mod ritimer;
pub mod rtc;
pub mod sct;
pub mod scu;
pub mod sdmmc;
pub mod sgpio;
pub mod spifi;
pub mod spi;
pub mod ssp;
pub mod timer;
pub mod uart1;
pub mod usart;
pub mod usb0;
pub mod usb1;
pub mod wwdt;*/
