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

    unsafe fn finalize(self, _s: Self::StaticInput) -> Self::Output {
        let led = components::led::LedsComponent::new().finalize(components::led_component_helper!(
             (&lpc43xx::gpio::GPIO5[0],  kernel::hil::gpio::ActivationMode::ActiveHigh),
			 (&lpc43xx::gpio::GPIO5[1],  kernel::hil::gpio::ActivationMode::ActiveHigh),
			 (&lpc43xx::gpio::GPIO5[2],  kernel::hil::gpio::ActivationMode::ActiveHigh),
			 (&lpc43xx::gpio::GPIO0[14], kernel::hil::gpio::ActivationMode::ActiveHigh),
			 (&lpc43xx::gpio::GPIO1[11], kernel::hil::gpio::ActivationMode::ActiveHigh),
			 (&lpc43xx::gpio::GPIO1[12], kernel::hil::gpio::ActivationMode::ActiveHigh)
			)
        );
        led
    }
}
