use core::panic::PanicInfo;
use core::fmt::Write;
use core::str;
use kernel::debug;
use kernel::hil::led;
use lpc43xx;


use crate::PROCESSES;

struct Writer {
    index : usize,
    panic_buffer : [u8;1024],
}

static mut WRITER: Writer = Writer {index:0, panic_buffer:[0; 1024]};

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        if s.bytes().count() < 1024-self.index {
            for c in s.bytes() {
                self.panic_buffer[self.index] = c;
                self.index+=1;
            }
        }
        Ok(())
    }

}

fn flush(writer : &mut Writer)  {
    let message = match str::from_utf8(&writer.panic_buffer){
           Err(_e) => "Unable to read kernel panic dump",
           Ok(m) => m,
    };
    unsafe {
        asm!(
            "mov r0, $0
            bkpt #128"
            :                                          // outputs
            :  "r"(&message)                             // inputs
            :   "r0"                                       // clobbers
            :                                          // options
            );
    }
}
/// Panic.
#[cfg(not(test))]
#[no_mangle]
#[panic_handler]
pub unsafe extern "C" fn panic_fmt(panic_info: &PanicInfo) -> ! {
    let led1 = &mut led::LedHigh::new(&mut lpc43xx::gpio::GPIO0[14]);
    //let led2 = &mut led::LedHigh::new(&mut lpc43xx::gpio::GPIO1[11]);
    let led3 = &mut led::LedHigh::new(&mut lpc43xx::gpio::GPIO1[12]);
    /*let ledr = &mut led::LedHigh::new(&mut lpc43xx::gpio::GPIO5[0]);
    let ledg = &mut led::LedHigh::new(&mut lpc43xx::gpio::GPIO5[1]);
    let ledb = &mut led::LedHigh::new(&mut lpc43xx::gpio::GPIO5[2]);*/
//    debug::panic_blink_forever(&mut [led3, led1])
    let writer = &mut WRITER;
    debug::panic_begin(&cortexm4::support::nop);
    debug::panic_banner(writer, panic_info);
    debug::panic_process_info(&PROCESSES, writer);
    flush(writer);
    debug::panic_blink_forever(&mut [led3, led1])
}
