//! Component for imix board LEDs.
//!
//! This provides one Component, LedComponent, which implements
//! a userspace syscall interface to the two imix on-board LEDs.
//!
//! Usage
//! -----
//! ```rust
//! let led = LedComponent::new().finalize();
//! ```

// Author: Philip Levis <pal@cs.stanford.edu>
// Last modified: 6/20/2018

#![allow(dead_code)] // Components are intended to be conditionally included

use capsules::led;
use kernel::component::Component;
use kernel::static_init;

pub struct LedComponent {}

impl LedComponent {
    pub fn new() -> LedComponent {
        LedComponent {}
    }
}

impl Component for LedComponent {
    type Output = &'static led::LED<'static, lpc43xx::gpio::GPIOPin>;

    unsafe fn finalize(&mut self) -> Self::Output {
        let led_pins = static_init!(
            [(&'static lpc43xx::gpio::GPIOPin, led::ActivationMode); 6],
            [(&lpc43xx::gpio::GPIO5[0], led::ActivationMode::ActiveHigh),
			(&lpc43xx::gpio::GPIO5[1], led::ActivationMode::ActiveHigh),
			(&lpc43xx::gpio::GPIO5[2], led::ActivationMode::ActiveHigh),
			(&lpc43xx::gpio::GPIO0[14], led::ActivationMode::ActiveHigh),
			(&lpc43xx::gpio::GPIO1[11], led::ActivationMode::ActiveHigh),
			(&lpc43xx::gpio::GPIO1[12], led::ActivationMode::ActiveHigh)]
        );
        let led = static_init!(
            led::LED<'static, lpc43xx::gpio::GPIOPin>,
            led::LED::new(led_pins)
        );
		for &(led, _) in led_pins.iter() {
			led.make_output();
			led.clear()
        }
        led
    }
}
