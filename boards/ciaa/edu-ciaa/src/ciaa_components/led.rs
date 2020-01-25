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
    type StaticInput = ();
    type Output = &'static led::LED<'static>;

    unsafe fn finalize(&mut self, _s: Self::StaticInput) -> Self::Output {
        let led_pins = static_init!(
            [(&'static dyn kernel::hil::gpio::Pin, led::ActivationMode); 6],
            [(&lpc43xx::gpio::GPIO5[0],  led::ActivationMode::ActiveHigh),
			 (&lpc43xx::gpio::GPIO5[1],  led::ActivationMode::ActiveHigh),
			 (&lpc43xx::gpio::GPIO5[2],  led::ActivationMode::ActiveHigh),
			 (&lpc43xx::gpio::GPIO0[14], led::ActivationMode::ActiveHigh),
			 (&lpc43xx::gpio::GPIO1[11], led::ActivationMode::ActiveHigh),
			 (&lpc43xx::gpio::GPIO1[12], led::ActivationMode::ActiveHigh)
			]
        );
        let led = static_init!(
            led::LED<'static>,
            led::LED::new(&led_pins[..])
        );
        led
    }
}
