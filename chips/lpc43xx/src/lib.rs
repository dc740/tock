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
pub mod ccu2;*/
pub mod cgu;
/*pub mod creg;
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
pub mod mcpwm;*/
pub mod pmc;
/*pub mod qei;
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

use cortexm4::{generic_isr, hard_fault_handler, svc_handler, systick_handler};

unsafe extern "C" fn unhandled_interrupt() {
    let mut interrupt_number: u32;
    //TODO: copy pasted from SAM4L just to test something

    // IPSR[8:0] holds the currently active interrupt
    asm!(
    "mrs    r0, ipsr                    "
    : "={r0}"(interrupt_number)
    :
    : "r0"
    :
    );

    interrupt_number = interrupt_number & 0x1ff;

    panic!("Unhandled Interrupt. ISR {} is active.", interrupt_number);
}

extern "C" {
    // _estack is not really a function, but it makes the types work
    // You should never actually invoke it!!
    fn _estack();

    // Defined by platform
    fn reset_handler();

    static mut _szero: u32;
    static mut _ezero: u32;
    static mut _etext: u32;
    static mut _srelocate: u32;
    static mut _erelocate: u32;
}


#[link_section = ".vectors"]
// used Ensures that the symbol is kept until the final binary
#[used]
pub static BASE_VECTORS: [unsafe extern "C" fn(); 16] = [
    _estack,
    reset_handler,
    unhandled_interrupt, // NMI
    hard_fault_handler,  // Hard Fault
    unhandled_interrupt, // MemManage
    unhandled_interrupt, // BusFault
    unhandled_interrupt, // UsageFault
    unhandled_interrupt,
    unhandled_interrupt,
    unhandled_interrupt,
    unhandled_interrupt,
    svc_handler,         // SVC
    unhandled_interrupt, // DebugMon
    unhandled_interrupt,
    unhandled_interrupt, // PendSV
    systick_handler,     // SysTick
];

#[link_section = ".vectors"]
#[used] // Ensures that the symbol is kept until the final binary
pub static IRQS: [unsafe extern "C" fn(); 80] = [generic_isr; 80];

pub unsafe fn init() {
    tock_rt0::init_data(&mut _etext, &mut _srelocate, &mut _erelocate);
    tock_rt0::zero_bss(&mut _szero, &mut _ezero);

    cortexm4::nvic::disable_all();
    cortexm4::nvic::clear_all_pending();
    cortexm4::nvic::enable_all();
}