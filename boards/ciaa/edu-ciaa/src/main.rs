#![no_std]
#![no_main]
//#![deny(missing_docs)]

extern crate capsules;
#[allow(unused_imports)]
#[macro_use(create_capability, debug, debug_gpio, static_init)]
extern crate kernel;
extern crate cortexm4;
extern crate lpc43xx;

use kernel::capabilities;

mod components;
use components::button::ButtonComponent;
use components::gpio::GpioComponent;
use components::led::LedComponent;

use kernel::component::Component;
// how should the kernel respond when a process faults
const FAULT_RESPONSE: kernel::procs::FaultResponse = kernel::procs::FaultResponse::Panic;

#[macro_use]
pub mod panic;

/// Dummy buffer that causes the linker to reserve enough space for the stack.
#[no_mangle]
#[link_section = ".stack_buffer"]
pub static mut STACK_MEMORY: [u8; 0x1000] = [0; 0x1000];

// Number of concurrent processes this platform supports. 1 by now. we can increase this later
const NUM_PROCS: usize = 1;

#[link_section = ".app_memory"]
static mut APP_MEMORY: [u8; 8192] = [0; 8192];

static mut PROCESSES: [Option<&'static kernel::procs::ProcessType>; NUM_PROCS] = [None; NUM_PROCS];

/// Supported drivers by the platform
pub struct Platform {
	button: &'static capsules::button::Button<'static, lpc43xx::gpio::GPIOPin>,
	//gpio: &'static capsules::gpio::GPIO<'static, lpc43xx::gpio::GPIOPin>,
    led: &'static capsules::led::LED<'static, lpc43xx::gpio::GPIOPin>,
    
}

impl kernel::Platform for Platform {
    fn with_driver<F, R>(&self, driver_num: usize, f: F) -> R
    where
        F: FnOnce(Option<&kernel::Driver>) -> R,
    {
        match driver_num {
            capsules::button::DRIVER_NUM => f(Some(self.button)),
			//capsules::gpio::DRIVER_NUM => f(Some(self.gpio)),
            capsules::led::DRIVER_NUM => f(Some(self.led)),
            _ => f(None),
        }
    }
}

/// Reset Handler.
///
/// This symbol is loaded into vector table by the SAM4L chip crate.
/// When the chip first powers on or later does a hard reset, after the core
/// initializes all the hardware, the address of this function is loaded and
/// execution begins here.
#[no_mangle]
#[inline(never)]
pub unsafe fn reset_handler() {
	lpc43xx::init();

/*    lpc43xx::pmc::PM.setup_system_clock(sam4l::pm::SystemClockSource::PllExternalOscillatorAt48MHz {
        frequency: sam4l::pm::OscillatorFrequency::Frequency16MHz,
        startup_mode: sam4l::pm::OscillatorStartup::FastStart,
    });

    // Source 32Khz and 1Khz clocks from RC23K (SAM4L Datasheet 11.6.8)
    sam4l::bpm::set_ck32source(sam4l::bpm::CK32Source::RC32K);
*/
    //set_pin_primary_functions();
    //    debug!("Starting virtual read test.");
    //    virtual_uart_rx_test::run_virtual_uart_receive(uart_mux);
    let board_kernel = static_init!(kernel::Kernel, kernel::Kernel::new(&PROCESSES));

	let button = ButtonComponent::new(board_kernel).finalize();
	//let gpio = GpioComponent::new(board_kernel).finalize();
    let led = LedComponent::new().finalize();
    

    // Create capabilities that the board needs to call certain protected kernel
    // functions.
    let process_management_capability =
        create_capability!(capabilities::ProcessManagementCapability);
    let main_loop_capability = create_capability!(capabilities::MainLoopCapability);
    let memory_allocation_capability = create_capability!(capabilities::MemoryAllocationCapability);

	let platform = Platform {
			button: button,
	        //gpio: gpio,
			led: led,
	    };
	let chip = static_init!(lpc43xx::chip::Lpc43xx, lpc43xx::chip::Lpc43xx::new());
	
	/* TODO: implement UART so we get debugging messages there
	DO NOT USE debug! until we do this!
	let debug_wrapper = static_init!(
        kernel::debug::DebugWriterWrapper,
        kernel::debug::DebugWriterWrapper::new(debugger)
    );
    kernel::debug::set_debug_writer_wrapper(debug_wrapper);
    debug!("Initialization complete. Entering main loop");*/

    extern "C" {
        /// Beginning of the ROM region containing app images.
        static _sapps: u8;
    }
    kernel::procs::load_processes(
        board_kernel,
        chip,
        &_sapps as *const u8,
        &mut APP_MEMORY,
        &mut PROCESSES,
        FAULT_RESPONSE,
        &process_management_capability,
    );

    board_kernel.kernel_loop(
        &platform,
        chip,
        Some(&kernel::ipc::IPC::new(
            board_kernel,
            &memory_allocation_capability,
        )),
        &main_loop_capability,
    );
}
