#![no_std]
#![no_main]
#![feature(panic_info_message)]
//#![deny(missing_docs)]
#![feature(asm)]
#![feature(in_band_lifetimes)]

extern crate capsules;
#[allow(unused_imports)]
#[macro_use(create_capability, debug, debug_gpio, static_init)]
extern crate kernel;
extern crate cortexm4;
extern crate lpc43xx;

use kernel::capabilities;

use capsules::alarm::AlarmDriver;
use capsules::virtual_uart::{MuxUart, UartDevice};
use capsules::virtual_alarm::{MuxAlarm, VirtualMuxAlarm};
mod ciaa_components;
use ciaa_components::button::ButtonComponent;
use ciaa_components::gpio::GpioComponent;
use ciaa_components::led::LedComponent;
use kernel::hil::Controller;
use kernel::component::Component;
use kernel::hil;

use components::alarm::AlarmDriverComponent;
// how should the kernel respond when a process faults
const FAULT_RESPONSE: kernel::procs::FaultResponse = kernel::procs::FaultResponse::Panic;

#[macro_use]
pub mod panic;

/// Dummy buffer that causes the linker to reserve enough space for the stack.
#[no_mangle]
#[link_section = ".stack_buffer"]
pub static mut STACK_MEMORY: [u8; 0x1000] = [0; 0x1000];

// Number of concurrent processes this platform supports. 4 for now. we can increase this later
const NUM_PROCS: usize = 4;

#[link_section = ".app_memory"]
static mut APP_MEMORY: [u8; 8192] = [0; 8192];

static mut PROCESSES: [Option<&'static dyn kernel::procs::ProcessType>; NUM_PROCS] = [None; NUM_PROCS];

/// Supported drivers by the platform
pub struct EduCiaaNXP {
    console: &'static capsules::console::Console<'static>,
    button: &'static capsules::button::Button<'static>,
    gpio: &'static capsules::gpio::GPIO<'static>,
    led: &'static capsules::led::LED<'static>,
    ipc: kernel::ipc::IPC,
    alarm: &'static AlarmDriver<'static, VirtualMuxAlarm<'static, lpc43xx::atimer::AlarmTimer<'static>>>,
}

impl kernel::Platform for EduCiaaNXP {
    fn with_driver<F, R>(&self, driver_num: usize, f: F) -> R
    where
        F: FnOnce(Option<&dyn kernel::Driver>) -> R,
    {
        match driver_num {
            capsules::console::DRIVER_NUM => f(Some(self.console)),
            capsules::button::DRIVER_NUM => f(Some(self.button)),
            capsules::gpio::DRIVER_NUM => f(Some(self.gpio)),
            capsules::led::DRIVER_NUM => f(Some(self.led)),
            capsules::alarm::DRIVER_NUM => f(Some(self.alarm)),
            kernel::ipc::DRIVER_NUM => f(Some(&self.ipc)),
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
    lpc43xx::creg::set_flash_acceleration(lpc43xx::creg::FLASHCFG::FLASHTIM::_10_BASE_M4_CLK_CLOCK);
    lpc43xx::cgu::board_setup_clocking(lpc43xx::cgu::BASE_CLK::CLK_SEL::CrystalOscillator, lpc43xx::cgu::MAX_CLOCK_FREQ, true);
    lpc43xx::creg::enable_32khz_1khz_osc();
    lpc43xx::creg::enable_creg6_rmii_mode();
    lpc43xx::ritimer::disable_rit(); //TODO find why this is enabled at all
    let board_kernel = static_init!(kernel::Kernel, kernel::Kernel::new(&PROCESSES));

    let button = ButtonComponent::new(board_kernel).finalize(());
    let gpio = GpioComponent::new(board_kernel).finalize(());
    let led = LedComponent::new().finalize(());
    

    // Create capabilities that the board needs to call certain protected kernel
    // functions.
    let process_management_capability =
        create_capability!(capabilities::ProcessManagementCapability);
    let main_loop_capability = create_capability!(capabilities::MainLoopCapability);
    let memory_allocation_capability = create_capability!(capabilities::MemoryAllocationCapability);


    // Create a shared UART channel for the console and for kernel debug.
    let uart_mux = static_init!(
        MuxUart<'static>,
        MuxUart::new(
            &lpc43xx::usart::USART2,
            &mut capsules::virtual_uart::RX_BUF,
            115200
        )
    );
    uart_mux.initialize();
    
    hil::uart::Transmit::set_transmit_client(&lpc43xx::usart::USART2, uart_mux);
    hil::uart::Receive::set_receive_client(&lpc43xx::usart::USART2, uart_mux);
    
    // Create a UartDevice for the kernel debugger.
    let debugger_uart = static_init!(UartDevice, UartDevice::new(uart_mux, false));
    debugger_uart.setup();
    let debugger = static_init!(
        kernel::debug::DebugWriter,
        kernel::debug::DebugWriter::new(
            debugger_uart,
            &mut kernel::debug::OUTPUT_BUF,
            &mut kernel::debug::INTERNAL_BUF,
        )
    );
    hil::uart::Transmit::set_transmit_client(debugger_uart, debugger);

    let debug_wrapper = static_init!(
        kernel::debug::DebugWriterWrapper,
        kernel::debug::DebugWriterWrapper::new(debugger)
    );
    kernel::debug::set_debug_writer_wrapper(debug_wrapper);
    debug!("Initialization complete. Entering main loop");
    
    // Create a UartDevice for the console.
    let console_uart = static_init!(UartDevice, UartDevice::new(uart_mux, true));
    console_uart.setup();
    let console = static_init!(
        capsules::console::Console<'static>,
        capsules::console::Console::new(
            console_uart,
            &mut capsules::console::WRITE_BUF,
            &mut capsules::console::READ_BUF,
            board_kernel.create_grant(&memory_allocation_capability)
        )
    );
    hil::uart::Transmit::set_transmit_client(console_uart, console);
    hil::uart::Receive::set_receive_client(console_uart, console);
    
    // # TIMER
    let atimer = &lpc43xx::atimer::ATIMER;
    let mux_alarm = static_init!(
        MuxAlarm<'static, lpc43xx::atimer::AlarmTimer>,
        MuxAlarm::new(&lpc43xx::atimer::ATIMER)
    );
    atimer.configure(mux_alarm);
    let alarm = AlarmDriverComponent::new(board_kernel, mux_alarm)
        .finalize(components::alarm_component_helper!(lpc43xx::atimer::AlarmTimer));

    
    let platform = EduCiaaNXP {
            console: console,
            button: button,
            gpio: gpio,
            led: led,
            ipc: kernel::ipc::IPC::new(board_kernel, &memory_allocation_capability),
            alarm: alarm,
        };
    let chip = static_init!(lpc43xx::chip::Lpc43xx, lpc43xx::chip::Lpc43xx::new());

    //platform.console.initialize();
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
        Some(&platform.ipc),
        &main_loop_capability,
    );
}
