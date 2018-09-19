#![no_std]
#![no_main]
#![feature(panic_implementation)]
//#![deny(missing_docs)]

extern crate capsules;
#[allow(unused_imports)]
#[macro_use(create_capability, debug, debug_gpio, static_init)]
extern crate kernel;
extern crate cortexm4;

use core::panic::PanicInfo;
#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub unsafe extern "C" fn panic_fmt(pi: &PanicInfo) -> ! {
	loop { }
}