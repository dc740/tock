#![no_std]
#![no_main]
#![feature(panic_info_message)]
//#![deny(missing_docs)]
#![feature(asm)]
#![feature(in_band_lifetimes)]

extern crate capsules;
#[allow(unused_imports)]
use kernel::{create_capability, debug, debug_gpio, static_init};
extern crate kernel;
extern crate cortexm4;
extern crate lpc43xx;

use kernel::capabilities;


use capsules::alarm::AlarmDriver;
use capsules::virtual_alarm::{MuxAlarm, VirtualMuxAlarm};
use components;
use components::console::{ConsoleComponent, UartMuxComponent};
use components::debug_writer::DebugWriterComponent;
use components::process_console::ProcessConsoleComponent;
use kernel::hil::Controller;
use kernel::common::dynamic_deferred_call::{DynamicDeferredCall, DynamicDeferredCallClientState};
use kernel::component::Component;

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
    pconsole: &'static capsules::process_console::ProcessConsole<
        'static,
        components::process_console::Capability,
    >,
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
    // GPIO
    let gpio = components::gpio::GpioComponent::new(board_kernel).finalize(components::gpio_component_helper!(
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
                                
                    &lpc43xx::gpio::GPIO0[2]   /*  35   CON2_09   ENET_RXD0 */
                ));
    // LEDS
    let led = components::led::LedsComponent::new().finalize(components::led_component_helper!(
             (&lpc43xx::gpio::GPIO5[0],  kernel::hil::gpio::ActivationMode::ActiveHigh),
             (&lpc43xx::gpio::GPIO5[1],  kernel::hil::gpio::ActivationMode::ActiveHigh),
             (&lpc43xx::gpio::GPIO5[2],  kernel::hil::gpio::ActivationMode::ActiveHigh),
             (&lpc43xx::gpio::GPIO0[14], kernel::hil::gpio::ActivationMode::ActiveHigh),
             (&lpc43xx::gpio::GPIO1[11], kernel::hil::gpio::ActivationMode::ActiveHigh),
             (&lpc43xx::gpio::GPIO1[12], kernel::hil::gpio::ActivationMode::ActiveHigh)
            )
        );
    // BUTTONS
    let button = components::button::ButtonComponent::new(board_kernel).finalize(
        components::button_component_helper!((
            &lpc43xx::gpio::GPIO0[4],kernel::hil::gpio::ActivationMode::ActiveLow,kernel::hil::gpio::FloatingState::PullNone
        ),(
            &lpc43xx::gpio::GPIO0[8],kernel::hil::gpio::ActivationMode::ActiveLow,kernel::hil::gpio::FloatingState::PullNone
        ),(
            &lpc43xx::gpio::GPIO0[9],kernel::hil::gpio::ActivationMode::ActiveLow,kernel::hil::gpio::FloatingState::PullNone
        ),(
            &lpc43xx::gpio::GPIO1[9],kernel::hil::gpio::ActivationMode::ActiveLow,kernel::hil::gpio::FloatingState::PullNone
        )),
    );
    // Create capabilities that the board needs to call certain protected kernel
    // functions.
    let process_management_capability =
        create_capability!(capabilities::ProcessManagementCapability);
    let main_loop_capability = create_capability!(capabilities::MainLoopCapability);
    let memory_allocation_capability = create_capability!(capabilities::MemoryAllocationCapability);



    let dynamic_deferred_call_clients =
        static_init!([DynamicDeferredCallClientState; 2], Default::default());
    let dynamic_deferred_caller = static_init!(
        DynamicDeferredCall,
        DynamicDeferredCall::new(dynamic_deferred_call_clients)
    );
    DynamicDeferredCall::set_global_instance(dynamic_deferred_caller);
    
    // # CONSOLE
    // Create a shared UART channel for the consoles and for kernel debug.
    let uart_mux =
        UartMuxComponent::new(&lpc43xx::usart::USART2, 115200, dynamic_deferred_caller).finalize(());

    let pconsole = ProcessConsoleComponent::new(board_kernel, uart_mux).finalize(());
    let console = ConsoleComponent::new(board_kernel, uart_mux).finalize(());
    DebugWriterComponent::new(uart_mux).finalize(());

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
            pconsole: pconsole,
            console: console,
            button: button,
            gpio: gpio,
            led: led,
            ipc: kernel::ipc::IPC::new(board_kernel, &memory_allocation_capability),
            alarm: alarm,
        };
    let chip = static_init!(lpc43xx::chip::Lpc43xx, lpc43xx::chip::Lpc43xx::new());
    
    platform.pconsole.start();
    
    debug!("Initialization complete. Entering main loop");
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
