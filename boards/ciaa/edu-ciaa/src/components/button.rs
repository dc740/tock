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
            board_kernel: board_kernel,
        }
    }
}

impl Component for ButtonComponent {
    type Output = &'static button::Button<'static, lpc43xx::gpio::GPIOPin>;

    unsafe fn finalize(&mut self) -> Self::Output {
        let grant_cap = create_capability!(capabilities::MemoryAllocationCapability);

        let button_pins = static_init!(
            [(&'static lpc43xx::gpio::GPIOPin, button::GpioMode); 4],
            [(&lpc43xx::gpio::GPIO0[4], button::GpioMode::LowWhenPressed),
			(&lpc43xx::gpio::GPIO0[8], button::GpioMode::LowWhenPressed),
			(&lpc43xx::gpio::GPIO0[9], button::GpioMode::LowWhenPressed),
			(&lpc43xx::gpio::GPIO1[9], button::GpioMode::LowWhenPressed)]
        );

        let button = static_init!(
            button::Button<'static, lpc43xx::gpio::GPIOPin>,
            button::Button::new(button_pins, self.board_kernel.create_grant(&grant_cap))
        );
        for &(btn, _) in button_pins.iter() {
            btn.set_client(button);
			btn.make_input();
        }

        button
    }
}
