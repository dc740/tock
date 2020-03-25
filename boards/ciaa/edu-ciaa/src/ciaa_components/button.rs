//! Component for imix board buttons.
//!
//! This provides one Component, ButtonComponent, which implements a
//! userspace syscall interface to the one imix on-board button (pin
//! 24).
//!
//! Usage
//! -----
//! ```rust
//! let button = ButtonComponent::new(board_kernel).finalize();
//! ```

// Author: Philip Levis <pal@cs.stanford.edu>
// Last modified: 6/20/2018

#![allow(dead_code)] // Components are intended to be conditionally included

use capsules::button;
use kernel::capabilities;
use kernel::component::Component;
use kernel::create_capability;
use kernel::static_init;

pub struct ButtonComponent {
    board_kernel: &'static kernel::Kernel,
}

impl ButtonComponent {
    pub fn new(board_kernel: &'static kernel::Kernel) -> ButtonComponent {
        ButtonComponent {
            board_kernel,
        }
    }
}

impl Component for ButtonComponent {
    type StaticInput = ();
    type Output = &'static button::Button<'static>;

    unsafe fn finalize(self, _s: Self::StaticInput) -> Self::Output {
        let grant_cap = create_capability!(capabilities::MemoryAllocationCapability);
        let button = components::button::ButtonComponent::new(self.board_kernel).finalize(
        components::button_component_helper!((
            &lpc43xx::gpio::GPIO0[4],
            kernel::hil::gpio::ActivationMode::ActiveLow,
            kernel::hil::gpio::FloatingState::PullNone
        ),(
            &lpc43xx::gpio::GPIO0[8],
            kernel::hil::gpio::ActivationMode::ActiveLow,
            kernel::hil::gpio::FloatingState::PullNone
        ),(
            &lpc43xx::gpio::GPIO0[9],
            kernel::hil::gpio::ActivationMode::ActiveLow,
            kernel::hil::gpio::FloatingState::PullNone
        ),(
            &lpc43xx::gpio::GPIO1[9],
            kernel::hil::gpio::ActivationMode::ActiveLow,
            kernel::hil::gpio::FloatingState::PullNone
        )),
    );

        button
    }
}
