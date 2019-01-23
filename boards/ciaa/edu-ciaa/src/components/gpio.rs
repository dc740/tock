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
// Last modified: 1/30/2019

#![allow(dead_code)] // Components are intended to be conditionally included

use capsules::gpio;
use kernel::capabilities;
use kernel::component::Component;
use kernel::create_capability;
use kernel::static_init;

pub struct GpioComponent {
    board_kernel: &'static kernel::Kernel,
}

impl GpioComponent {
    pub fn new(board_kernel: &'static kernel::Kernel) -> GpioComponent {
        GpioComponent {
            board_kernel: board_kernel,
        }
    }
}
/// GPIO component that lists the same GPIO ports as the sAPI for the edu-ciaa-nxp
/// LEDs and buttons have their own separate capsule.
impl Component for GpioComponent {
    type Output = &'static gpio::GPIO<'static, lpc43xx::gpio::GPIOPin>;

    unsafe fn finalize(&mut self) -> Self::Output {
        let grant_cap = create_capability!(capabilities::MemoryAllocationCapability);

        let gpio_pins = static_init!(
            [&'static lpc43xx::gpio::GPIOPin; 36],
            [
				/*LPC GPIO port             |Index|Connector|Serigraphy|      */
                &lpc43xx::gpio::GPIO2[1],  /*   0   CON1_36   T_FIL1           */
   				&lpc43xx::gpio::GPIO3[13], /*   1   CON1_34   T_COL2           */

			   &lpc43xx::gpio::GPIO1[8],   /*   2   CON1_39   T_COL0           */
			   &lpc43xx::gpio::GPIO2[2],   /*   3   CON1_37   T_FIL2           */
			   &lpc43xx::gpio::GPIO2[3],   /*   4   CON1_35   T_FIL3           */
			   &lpc43xx::gpio::GPIO2[0],   /*   5   CON1_33   T_FIL0           */
			   &lpc43xx::gpio::GPIO3[12],  /*   6   CON1_31   T_COL1           */
			
			   &lpc43xx::gpio::GPIO5[9],   /*   7   CON1_29   CAN_TD           */
			   &lpc43xx::gpio::GPIO5[8],   /*   8   CON1_27   CAN_RD           */
			
			   &lpc43xx::gpio::GPIO5[3],   /*   9   CON1_25   RS232_TXD        */
			   &lpc43xx::gpio::GPIO5[4],   /*  10   CON1_23   RS232_RXD        */
			
			   &lpc43xx::gpio::GPIO2[8],   /*  11   CON2_40   GPIO8            */
			   &lpc43xx::gpio::GPIO3[7],   /*  12   CON2_38   GPIO7            */
			   &lpc43xx::gpio::GPIO3[5],   /*  13   CON2_36   GPIO5            */
			   &lpc43xx::gpio::GPIO5[15],  /*  14   CON2_34   GPIO3            */
			   &lpc43xx::gpio::GPIO3[3],   /*  15   CON2_32   GPIO1            */
			
			   &lpc43xx::gpio::GPIO2[4],   /*  16   CON2_30   LCD1             */
			   &lpc43xx::gpio::GPIO2[5],   /*  17   CON2_28   LCD2             */
			   &lpc43xx::gpio::GPIO2[6],   /*  18   CON2_26   LCD3             */
			   &lpc43xx::gpio::GPIO5[12],  /*  19   CON2_24   LCDRS            */
			   &lpc43xx::gpio::GPIO5[14],  /*  20   CON2_22   LCD4             */
			
			   &lpc43xx::gpio::GPIO0[10],  /*  21   CON2_18   SPI_MISO         */
			
			   &lpc43xx::gpio::GPIO0[15],  /*  22   CON2_16   ENET_TXD1        */
			   &lpc43xx::gpio::GPIO0[13],  /*  23   CON2_14   ENET_TXD0        */
			   &lpc43xx::gpio::GPIO0[12],  /*  24   CON2_12   ENET_MDIO        */
			   &lpc43xx::gpio::GPIO0[3],   /*  25   CON2_10   ENET_CRS_DV      */
			   &lpc43xx::gpio::GPIO3[15],  /*  26   CON2_08   ENET_MDC         */
			   &lpc43xx::gpio::GPIO0[1],   /*  27   CON2_06   ENET_TXEN        */
			   &lpc43xx::gpio::GPIO0[0],   /*  28   CON2_04   ENET_RXD1        */
			
			   &lpc43xx::gpio::GPIO3[6],   /*  29   CON2_35   GPIO6            */
			   &lpc43xx::gpio::GPIO5[16],  /*  30   CON2_33   GPIO4            */
			   &lpc43xx::gpio::GPIO3[4],   /*  31   CON2_31   GPIO2            */
			   &lpc43xx::gpio::GPIO3[0],   /*  32   CON2_29   GPIO0            */
			
			   &lpc43xx::gpio::GPIO5[13],  /*  33   CON2_23   LCDEN            */
			
			   &lpc43xx::gpio::GPIO0[11],  /*  34   CON2_21   SPI_MOSI         */
			
			   &lpc43xx::gpio::GPIO0[2],   /*  35   CON2_09   ENET_RXD0        */
            ]
        );

        let gpio = static_init!(
            gpio::GPIO<'static, lpc43xx::gpio::GPIOPin>,
            gpio::GPIO::new(gpio_pins, self.board_kernel.create_grant(&grant_cap))
        );
        for pin in gpio_pins.iter() {
            pin.set_client(gpio);
        }

        gpio
    }
}
