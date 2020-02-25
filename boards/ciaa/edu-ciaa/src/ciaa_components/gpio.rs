//! Component for GPIO on the EDU-CIAA-NXP board.
//!
//! This provides one Component, GpioComponent, which implements
//! a userspace syscall interface to a subset of the LPC4337 GPIO
//! pins provided on the board headers.
//!
//! Usage
//! -----
//! ```rust
//! let gpio = GpioComponent::new(board_kernel).finalize();
//! ```

// Author: Emilio Moretti <emilio.moretti@gmail.com>

#![allow(dead_code)] // Components are intended to be conditionally included

use capsules::gpio;
use kernel::capabilities;
use kernel::component::Component;
use kernel::create_capability;
use kernel::static_init;
use kernel;

pub struct GpioComponent {
    board_kernel: &'static kernel::Kernel,
}

impl GpioComponent {
    pub fn new(board_kernel: &'static kernel::Kernel) -> GpioComponent {
        GpioComponent {
            board_kernel,
        }
    }
}
/// GPIO component that lists the same GPIO ports as the sAPI for the edu-ciaa-nxp
/// LEDs and buttons have their own separate capsule.
impl Component for GpioComponent {
    type StaticInput = ();
    type Output = &'static gpio::GPIO<'static>;

    unsafe fn finalize(&mut self, _s: Self::StaticInput) -> Self::Output {
        let grant_cap = create_capability!(capabilities::MemoryAllocationCapability);

        let gpio_pins = static_init!(
            [&'static dyn kernel::hil::gpio::InterruptValuePin; 36],
            [
				/*LPC GPIO port             |Index|Connector|Serigraphy|      */
                static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO2[1])).finalize(),  /*   0   CON1_36   T_FIL1           */
                static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO3[13])).finalize(), /*   1   CON1_34   T_COL2           */

               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO1[8])).finalize(),   /*   2   CON1_39   T_COL0           */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO2[2])).finalize(),   /*   3   CON1_37   T_FIL2           */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO2[3])).finalize(),   /*   4   CON1_35   T_FIL3           */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO2[0])).finalize(),   /*   5   CON1_33   T_FIL0           */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO3[12])).finalize(),  /*   6   CON1_31   T_COL1           */
            
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO5[9])).finalize(),   /*   7   CON1_29   CAN_TD           */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO5[8])).finalize(),   /*   8   CON1_27   CAN_RD           */
            
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO5[3])).finalize(),   /*   9   CON1_25   RS232_TXD        */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO5[4])).finalize(),   /*  10   CON1_23   RS232_RXD        */
            
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO2[8])).finalize(),   /*  11   CON2_40   GPIO8            */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO3[7])).finalize(),   /*  12   CON2_38   GPIO7            */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO3[5])).finalize(),   /*  13   CON2_36   GPIO5            */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO5[15])).finalize(),  /*  14   CON2_34   GPIO3            */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO3[3])).finalize(),   /*  15   CON2_32   GPIO1            */
            
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO2[4])).finalize(),   /*  16   CON2_30   LCD1             */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO2[5])).finalize(),   /*  17   CON2_28   LCD2             */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO2[6])).finalize(),   /*  18   CON2_26   LCD3             */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO5[12])).finalize(),  /*  19   CON2_24   LCDRS            */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO5[14])).finalize(),  /*  20   CON2_22   LCD4             */
            
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO0[10])).finalize(),  /*  21   CON2_18   SPI_MISO         */
            
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO0[15])).finalize(),  /*  22   CON2_16   ENET_TXD1        */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO0[13])).finalize(),  /*  23   CON2_14   ENET_TXD0        */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO0[12])).finalize(),  /*  24   CON2_12   ENET_MDIO        */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO0[3])).finalize(),   /*  25   CON2_10   ENET_CRS_DV      */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO3[15])).finalize(),  /*  26   CON2_08   ENET_MDC         */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO0[1])).finalize(),   /*  27   CON2_06   ENET_TXEN        */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO0[0])).finalize(),   /*  28   CON2_04   ENET_RXD1        */
            
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO3[6])).finalize(),   /*  29   CON2_35   GPIO6            */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO5[16])).finalize(),  /*  30   CON2_33   GPIO4            */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO3[4])).finalize(),   /*  31   CON2_31   GPIO2            */
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO3[0])).finalize(),   /*  32   CON2_29   GPIO0            */
            
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO5[13])).finalize(),  /*  33   CON2_23   LCDEN            */
            
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO0[11])).finalize(),  /*  34   CON2_21   SPI_MOSI         */
            
               static_init!(kernel::hil::gpio::InterruptValueWrapper, kernel::hil::gpio::InterruptValueWrapper::new(&lpc43xx::gpio::GPIO0[2])).finalize(),   /*  35   CON2_09   ENET_RXD0        */
            ]
        );

        let gpio = static_init!(
            gpio::GPIO<'static>,
            gpio::GPIO::new(&gpio_pins[..], self.board_kernel.create_grant(&grant_cap))
        );
        for pin in gpio_pins.iter() { 
            pin.set_client(gpio);
        }

        gpio
    }
}
