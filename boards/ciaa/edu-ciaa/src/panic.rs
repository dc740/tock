use core::panic::PanicInfo;

use kernel::debug;
use kernel::hil::led;
use lpc43xx;

/// Panic.
#[cfg(not(test))]
#[no_mangle]
#[panic_handler]
pub unsafe extern "C" fn panic_fmt(_pi: &PanicInfo) -> ! {
    let led1 = &mut led::LedHigh::new(&mut lpc43xx::gpio::GPIO0[14]);
	let led2 = &mut led::LedHigh::new(&mut lpc43xx::gpio::GPIO1[11]);
	let led3 = &mut led::LedHigh::new(&mut lpc43xx::gpio::GPIO1[12]);
	let ledr = &mut led::LedHigh::new(&mut lpc43xx::gpio::GPIO5[0]);
	let ledg = &mut led::LedHigh::new(&mut lpc43xx::gpio::GPIO5[1]);
	let ledb = &mut led::LedHigh::new(&mut lpc43xx::gpio::GPIO5[2]);
    debug::panic_blink_forever(&mut [led1, led2, led3, ledr, ledg, ledb])
}
