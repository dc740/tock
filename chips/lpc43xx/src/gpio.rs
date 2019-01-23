use core::cell::Cell;
use core::ops::{Index, IndexMut};
use core::sync::atomic::AtomicUsize;
use kernel::common::cells::OptionalCell;
use kernel::hil;

use kernel::common::StaticRef;
use kernel::common::registers::{ReadWrite, WriteOnly, FieldValue};
use scu::{SCU_BASE, SFSP};

/// GPIO port register
#[repr(C)]
struct GpioRegisters {
/// Byte pin registers
b: [[ReadWrite<u8>; 32]; 128],   // Offset 0x0000
/// Word pin registers
w: [[ReadWrite<u32>; 32]; 32],  // Offset 0x1000
/// Direction registers port m
dir: [ReadWrite<u32>; 32], // Offset 0x2000
/// Mask register port m
mask: [ReadWrite<u32>; 32],// Offset 0x2080
/// Port pin register port m
pin: [ReadWrite<u32>; 32], // Offset 0x2100
/// Masked port register port m
mpin: [ReadWrite<u32>; 32],// Offset 0x2180
/// Write: Set register for port m  Read: output bits for port m
set: [ReadWrite<u32>; 32],// Offset 0x2200
/// Clear port m
clr: [WriteOnly<u32>; 32],// Offset 0x2280
/// Toggle port m
not: [WriteOnly<u32>; 32],// Offset 0x2300
}

const GPIO_PORT_BASE: StaticRef<GpioRegisters> =
    unsafe { StaticRef::new(0x400F4000 as *const GpioRegisters) };
	
/// Reference count for the number of GPIO interrupts currently active.
///
/// This is used to determine if it's possible for the SAM4L to go into
/// WAIT/RETENTION mode, since those modes will not be woken up by GPIO
/// interrupts.
///
/// This is an `AtomicUsize` because it has to be a `Sync` type to live in a
/// global---Rust has no way of knowing we're not going to use it across
/// threads. Use `Ordering::Relaxed` when reading/writing the value to get LLVM
/// to just use plain loads and stores instead of atomic operations.
pub static INTERRUPT_COUNT: AtomicUsize = AtomicUsize::new(0);


/// GPIO port that manages up to 31 pins [30:0].
///
/// LPC43xx ports are divided into 8 smaller groups.
///
pub struct Port {
    pins: [Option<GPIOPin>; 31],
}

impl Index<usize> for Port {
    type Output = GPIOPin;

    fn index<'a>(&'a self, index: usize) -> &'a GPIOPin {
        self.pins[index].as_ref().expect("Tried to use a non existing GPIO pin for this port.") //this will panic if the board is poorly coded
    }
}

impl IndexMut<usize> for Port {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut GPIOPin {
        self.pins[index].as_mut().expect("Tried to use a non existing GPIO pin for this port.") //this will panic if the board is poorly coded
    }
}

impl Port {
    pub fn handle_interrupt(&self) {
        ()
    }
}

pub static mut GPIO0: Port = Port {
pins: [
    Some(GPIOPin::new(0,0,SFSP::MODE::Function0Default,0,0)),
    Some(GPIOPin::new(0,1,SFSP::MODE::Function0Default,0,1)),
    Some(GPIOPin::new(1,0,SFSP::MODE::Function0Default,0,4)),
    Some(GPIOPin::new(6,6,SFSP::MODE::Function0Default,0,5)),
    Some(GPIOPin::new(3,6,SFSP::MODE::Function0Default,0,6)),
    Some(GPIOPin::new(2,7,SFSP::MODE::Function0Default,0,7)),
    Some(GPIOPin::new(1,1,SFSP::MODE::Function0Default,0,8)),
    Some(GPIOPin::new(1,2,SFSP::MODE::Function0Default,0,9)),
    Some(GPIOPin::new(1,3,SFSP::MODE::Function0Default,0,10)),
    Some(GPIOPin::new(1,4,SFSP::MODE::Function0Default,0,11)),
    Some(GPIOPin::new(1,15,SFSP::MODE::Function0Default,0,2)),
    Some(GPIOPin::new(1,16,SFSP::MODE::Function0Default,0,3)),
    Some(GPIOPin::new(1,17,SFSP::MODE::Function0Default,0,12)),
    Some(GPIOPin::new(1,18,SFSP::MODE::Function0Default,0,13)),
    Some(GPIOPin::new(1,20,SFSP::MODE::Function0Default,0,15)),
    Some(GPIOPin::new(2,10,SFSP::MODE::Function0Default,0,14)),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    ],
};


pub static mut GPIO1: Port = Port {
pins: [
    Some(GPIOPin::new(1,7,SFSP::MODE::Function0Default,1,0)),
    Some(GPIOPin::new(1,8,SFSP::MODE::Function0Default,1,1)),
    Some(GPIOPin::new(1,9,SFSP::MODE::Function0Default,1,2)),
    Some(GPIOPin::new(1,10,SFSP::MODE::Function0Default,1,3)),
    Some(GPIOPin::new(1,11,SFSP::MODE::Function0Default,1,4)),
    Some(GPIOPin::new(1,12,SFSP::MODE::Function0Default,1,5)),
    Some(GPIOPin::new(1,13,SFSP::MODE::Function0Default,1,6)),
    Some(GPIOPin::new(1,14,SFSP::MODE::Function0Default,1,7)),
    Some(GPIOPin::new(1,5,SFSP::MODE::Function0Default,1,8)),
    Some(GPIOPin::new(1,6,SFSP::MODE::Function0Default,1,9)),
    Some(GPIOPin::new(2,9,SFSP::MODE::Function0Default,1,10)),
    Some(GPIOPin::new(2,11,SFSP::MODE::Function0Default,1,11)),
    Some(GPIOPin::new(2,12,SFSP::MODE::Function0Default,1,12)),
    Some(GPIOPin::new(2,13,SFSP::MODE::Function0Default,1,13)),
    Some(GPIOPin::new(3,4,SFSP::MODE::Function0Default,1,14)),
    Some(GPIOPin::new(3,5,SFSP::MODE::Function0Default,1,15)),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    ],
};


pub static mut GPIO2: Port = Port {
pins: [
    Some(GPIOPin::new(4,0,SFSP::MODE::Function0Default,2,0)),
    Some(GPIOPin::new(4,1,SFSP::MODE::Function0Default,2,1)),
    Some(GPIOPin::new(4,2,SFSP::MODE::Function0Default,2,2)),
    Some(GPIOPin::new(4,3,SFSP::MODE::Function0Default,2,3)),
    Some(GPIOPin::new(4,4,SFSP::MODE::Function0Default,2,4)),
    Some(GPIOPin::new(4,5,SFSP::MODE::Function0Default,2,5)),
    Some(GPIOPin::new(4,6,SFSP::MODE::Function0Default,2,6)),
    Some(GPIOPin::new(5,7,SFSP::MODE::Function0Default,2,7)),
    Some(GPIOPin::new(6,12,SFSP::MODE::Function0Default,2,8)),
    Some(GPIOPin::new(5,0,SFSP::MODE::Function0Default,2,9)),
    Some(GPIOPin::new(5,1,SFSP::MODE::Function0Default,2,10)),
    Some(GPIOPin::new(5,2,SFSP::MODE::Function0Default,2,11)),
    Some(GPIOPin::new(5,3,SFSP::MODE::Function0Default,2,12)),
    Some(GPIOPin::new(5,4,SFSP::MODE::Function0Default,2,13)),
    Some(GPIOPin::new(5,5,SFSP::MODE::Function0Default,2,14)),
    Some(GPIOPin::new(5,6,SFSP::MODE::Function0Default,2,15)),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    ],
};


pub static mut GPIO3: Port = Port {
pins: [
    Some(GPIOPin::new(6,1,SFSP::MODE::Function0Default,3,0)),
    Some(GPIOPin::new(6,2,SFSP::MODE::Function0Default,3,1)),
    Some(GPIOPin::new(6,3,SFSP::MODE::Function0Default,3,2)),
    Some(GPIOPin::new(6,4,SFSP::MODE::Function0Default,3,3)),
    Some(GPIOPin::new(6,5,SFSP::MODE::Function0Default,3,4)),
    Some(GPIOPin::new(6,9,SFSP::MODE::Function0Default,3,5)),
    Some(GPIOPin::new(6,10,SFSP::MODE::Function0Default,3,6)),
    Some(GPIOPin::new(6,11,SFSP::MODE::Function0Default,3,7)),
    Some(GPIOPin::new(7,0,SFSP::MODE::Function0Default,3,8)),
    Some(GPIOPin::new(7,1,SFSP::MODE::Function0Default,3,9)),
    Some(GPIOPin::new(7,2,SFSP::MODE::Function0Default,3,10)),
    Some(GPIOPin::new(7,3,SFSP::MODE::Function0Default,3,11)),
    Some(GPIOPin::new(7,4,SFSP::MODE::Function0Default,3,12)),
    Some(GPIOPin::new(7,5,SFSP::MODE::Function0Default,3,13)),
    Some(GPIOPin::new(7,6,SFSP::MODE::Function0Default,3,14)),
    Some(GPIOPin::new(7,7,SFSP::MODE::Function0Default,3,15)),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    ],
};


pub static mut GPIO4: Port = Port {
pins: [
    Some(GPIOPin::new(8,0,SFSP::MODE::Function0Default,4,0)),
    Some(GPIOPin::new(8,1,SFSP::MODE::Function0Default,4,1)),
    Some(GPIOPin::new(8,2,SFSP::MODE::Function0Default,4,2)),
    Some(GPIOPin::new(8,3,SFSP::MODE::Function0Default,4,3)),
    Some(GPIOPin::new(8,4,SFSP::MODE::Function0Default,4,4)),
    Some(GPIOPin::new(8,5,SFSP::MODE::Function0Default,4,5)),
    Some(GPIOPin::new(8,6,SFSP::MODE::Function0Default,4,6)),
    Some(GPIOPin::new(8,7,SFSP::MODE::Function0Default,4,7)),
    Some(GPIOPin::new(10,1,SFSP::MODE::Function0Default,4,8)),
    Some(GPIOPin::new(10,2,SFSP::MODE::Function0Default,4,9)),
    Some(GPIOPin::new(10,3,SFSP::MODE::Function0Default,4,10)),
    Some(GPIOPin::new(9,6,SFSP::MODE::Function0Default,4,11)),
    Some(GPIOPin::new(9,0,SFSP::MODE::Function0Default,4,12)),
    Some(GPIOPin::new(9,1,SFSP::MODE::Function0Default,4,13)),
    Some(GPIOPin::new(9,2,SFSP::MODE::Function0Default,4,14)),
    Some(GPIOPin::new(9,3,SFSP::MODE::Function0Default,4,15)),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    ],
};


pub static mut GPIO5: Port = Port {
pins: [
    Some(GPIOPin::new(2,0,SFSP::MODE::Function4,5,0)),
    Some(GPIOPin::new(2,1,SFSP::MODE::Function4,5,1)),
    Some(GPIOPin::new(2,2,SFSP::MODE::Function4,5,2)),
    Some(GPIOPin::new(2,3,SFSP::MODE::Function4,5,3)),
    Some(GPIOPin::new(2,4,SFSP::MODE::Function4,5,4)),
    Some(GPIOPin::new(2,5,SFSP::MODE::Function4,5,5)),
    Some(GPIOPin::new(2,6,SFSP::MODE::Function4,5,6)),
    Some(GPIOPin::new(2,8,SFSP::MODE::Function4,5,7)),
    Some(GPIOPin::new(3,1,SFSP::MODE::Function4,5,8)),
    Some(GPIOPin::new(3,2,SFSP::MODE::Function4,5,9)),
    Some(GPIOPin::new(3,7,SFSP::MODE::Function4,5,10)),
    Some(GPIOPin::new(3,8,SFSP::MODE::Function4,5,11)),
    Some(GPIOPin::new(4,8,SFSP::MODE::Function4,5,12)),
    Some(GPIOPin::new(4,9,SFSP::MODE::Function4,5,13)),
    Some(GPIOPin::new(4,10,SFSP::MODE::Function4,5,14)),
    Some(GPIOPin::new(6,7,SFSP::MODE::Function4,5,15)),
    Some(GPIOPin::new(6,8,SFSP::MODE::Function4,5,16)),
    Some(GPIOPin::new(9,4,SFSP::MODE::Function4,5,17)),
    Some(GPIOPin::new(9,5,SFSP::MODE::Function4,5,18)),
    Some(GPIOPin::new(10,4,SFSP::MODE::Function4,5,19)),
    Some(GPIOPin::new(11,0,SFSP::MODE::Function4,5,20)),
    Some(GPIOPin::new(11,1,SFSP::MODE::Function4,5,21)),
    Some(GPIOPin::new(11,2,SFSP::MODE::Function4,5,22)),
    Some(GPIOPin::new(11,3,SFSP::MODE::Function4,5,23)),
    Some(GPIOPin::new(11,4,SFSP::MODE::Function4,5,24)),
    Some(GPIOPin::new(11,5,SFSP::MODE::Function4,5,25)),
    Some(GPIOPin::new(11,6,SFSP::MODE::Function4,5,26)),
    None,
    None,
    None,
    None,
    ],
};


pub static mut GPIO6: Port = Port {
pins: [
    Some(GPIOPin::new(12,1,SFSP::MODE::Function4,6,0)),
    Some(GPIOPin::new(12,2,SFSP::MODE::Function4,6,1)),
    Some(GPIOPin::new(12,3,SFSP::MODE::Function4,6,2)),
    Some(GPIOPin::new(12,4,SFSP::MODE::Function4,6,3)),
    Some(GPIOPin::new(12,5,SFSP::MODE::Function4,6,4)),
    Some(GPIOPin::new(12,6,SFSP::MODE::Function4,6,5)),
    Some(GPIOPin::new(12,7,SFSP::MODE::Function4,6,6)),
    Some(GPIOPin::new(12,8,SFSP::MODE::Function4,6,7)),
    Some(GPIOPin::new(12,9,SFSP::MODE::Function4,6,8)),
    Some(GPIOPin::new(12,10,SFSP::MODE::Function4,6,9)),
    Some(GPIOPin::new(12,11,SFSP::MODE::Function4,6,10)),
    Some(GPIOPin::new(12,12,SFSP::MODE::Function4,6,11)),
    Some(GPIOPin::new(12,13,SFSP::MODE::Function4,6,12)),
    Some(GPIOPin::new(12,14,SFSP::MODE::Function4,6,13)),
    Some(GPIOPin::new(13,0,SFSP::MODE::Function4,6,14)),
    Some(GPIOPin::new(13,1,SFSP::MODE::Function4,6,15)),
    Some(GPIOPin::new(13,2,SFSP::MODE::Function4,6,16)),
    Some(GPIOPin::new(13,3,SFSP::MODE::Function4,6,17)),
    Some(GPIOPin::new(13,4,SFSP::MODE::Function4,6,18)),
    Some(GPIOPin::new(13,5,SFSP::MODE::Function4,6,19)),
    Some(GPIOPin::new(13,6,SFSP::MODE::Function4,6,20)),
    Some(GPIOPin::new(13,7,SFSP::MODE::Function4,6,21)),
    Some(GPIOPin::new(13,8,SFSP::MODE::Function4,6,22)),
    Some(GPIOPin::new(13,9,SFSP::MODE::Function4,6,23)),
    Some(GPIOPin::new(13,10,SFSP::MODE::Function4,6,24)),
    Some(GPIOPin::new(13,11,SFSP::MODE::Function4,6,25)),
    Some(GPIOPin::new(13,12,SFSP::MODE::Function4,6,26)),
    Some(GPIOPin::new(13,13,SFSP::MODE::Function4,6,27)),
    Some(GPIOPin::new(13,14,SFSP::MODE::Function4,6,28)),
    Some(GPIOPin::new(13,15,SFSP::MODE::Function4,6,29)),
    Some(GPIOPin::new(13,16,SFSP::MODE::Function4,6,30)),
    ],
};


pub static mut GPIO7: Port = Port {
pins: [
    Some(GPIOPin::new(14,0,SFSP::MODE::Function4,7,0)),
    Some(GPIOPin::new(14,1,SFSP::MODE::Function4,7,1)),
    Some(GPIOPin::new(14,2,SFSP::MODE::Function4,7,2)),
    Some(GPIOPin::new(14,3,SFSP::MODE::Function4,7,3)),
    Some(GPIOPin::new(14,4,SFSP::MODE::Function4,7,4)),
    Some(GPIOPin::new(14,5,SFSP::MODE::Function4,7,5)),
    Some(GPIOPin::new(14,6,SFSP::MODE::Function4,7,6)),
    Some(GPIOPin::new(14,7,SFSP::MODE::Function4,7,7)),
    Some(GPIOPin::new(14,8,SFSP::MODE::Function4,7,8)),
    Some(GPIOPin::new(14,9,SFSP::MODE::Function4,7,9)),
    Some(GPIOPin::new(14,10,SFSP::MODE::Function4,7,10)),
    Some(GPIOPin::new(14,11,SFSP::MODE::Function4,7,11)),
    Some(GPIOPin::new(14,12,SFSP::MODE::Function4,7,12)),
    Some(GPIOPin::new(14,13,SFSP::MODE::Function4,7,13)),
    Some(GPIOPin::new(14,14,SFSP::MODE::Function4,7,14)),
    Some(GPIOPin::new(14,15,SFSP::MODE::Function4,7,15)),
    Some(GPIOPin::new(15,1,SFSP::MODE::Function4,7,16)),
    Some(GPIOPin::new(15,2,SFSP::MODE::Function4,7,17)),
    Some(GPIOPin::new(15,3,SFSP::MODE::Function4,7,18)),
    Some(GPIOPin::new(15,5,SFSP::MODE::Function4,7,19)),
    Some(GPIOPin::new(15,6,SFSP::MODE::Function4,7,20)),
    Some(GPIOPin::new(15,7,SFSP::MODE::Function4,7,21)),
    Some(GPIOPin::new(15,8,SFSP::MODE::Function4,7,22)),
    Some(GPIOPin::new(15,9,SFSP::MODE::Function4,7,23)),
    Some(GPIOPin::new(15,10,SFSP::MODE::Function4,7,24)),
    Some(GPIOPin::new(15,11,SFSP::MODE::Function4,7,25)),
    None,
    None,
    None,
    None,
    None,
    ],
};

//TODO: Write a new port with all non-GPIO pins so we can use the
//same interface to configure them. Or find where else to put them.

/// Each pin consists of a name for the port and pin that is globally
/// used to refer to it even when the functionality may be different.
/// For example: P2_14 (port 2, pin 14)
/// When the pin gets configured as GPIO, it also has a GPIO port pin and port.
/// For example: For P2_14, the corresponding GPIO is GPIO0[14].
/// Since each pin can be configured as any different types, we need to pass down
/// the default Function that sets it as GPIO, as documented in:
/// UM10503 Chapter 17: LPC43xx/LPC43Sxx System Control Unit (SCU)/ IO configuration
pub struct GPIOPin {
	port_name: u32,
	pin_name: u32,
	func: FieldValue<u32, SFSP::Register>,
	gpio_port: u32,
	gpio_pin: u32,
    client_data: Cell<usize>,
    client: OptionalCell<&'static hil::gpio::Client>,
}

impl GPIOPin {
    const fn new(
	port_name: u32,
	pin_name: u32,
	func: FieldValue<u32, SFSP::Register>,
	gpio_port: u32,
	gpio_pin: u32) -> GPIOPin {
        GPIOPin {
			port_name: port_name,
			pin_name: pin_name,
			func: func,
			gpio_port: gpio_port,
			gpio_pin: gpio_pin,
            client_data: Cell::new(0),
            client: OptionalCell::empty(),
        }
    }

    pub fn set_client<C: hil::gpio::Client>(&self, client: &'static C) {
        self.client.set(client);
    }

	pub fn enable(&self) {
        let sfsp = &SCU_BASE.sfsp[self.port_name as usize][self.pin_name as usize];
		sfsp.write(self.func + SFSP::ZIF::DisableInputGlitchFilter + SFSP::EZI::EnableInputBuffer); 
    }

    pub fn disable(&self) {
        let sfsp = &SCU_BASE.sfsp[self.port_name as usize][self.pin_name as usize];
		sfsp.write(SFSP::MODE::Function0Default +
					SFSP::EPD::DisablePullDown +
					SFSP::EPUN::EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode +
					SFSP::EZI::DisableInputBuffer +
					SFSP::ZIF::EnableInputGlitchFilter +
					SFSP::EHD::NormalDrive4MADriveStrength);
    }
    pub fn make_output(&self) {
		let sfsp = &SCU_BASE.sfsp[self.port_name as usize][self.pin_name as usize];
		sfsp.modify(SFSP::EPUN::DisablePullUp + SFSP::EPD::DisablePullDown);
		// Set pin bit in dir to 1 to set an output direction
		// and use these extra steps to leave the other values untouched
		let gpio_dir = &GPIO_PORT_BASE.dir[self.gpio_port as usize];
		let output_mode = FieldValue::<u32, ()>::new(1 << self.gpio_pin, 0x0, 1);
		gpio_dir.modify(output_mode)
    }

    pub fn make_input(&self) {
		let sfsp = &SCU_BASE.sfsp[self.port_name as usize][self.pin_name as usize];
        sfsp.modify(SFSP::EPUN::DisablePullUp + SFSP::EPD::DisablePullDown);
		// Set pin bit in dir to 0 to set an input direction
		// and use these extra steps to leave the other values untouched
		let gpio_dir = &GPIO_PORT_BASE.dir[self.gpio_port as usize];
		let input_mode = FieldValue::<u32, ()>::new(1 << self.gpio_pin, 0x0, 0);
		gpio_dir.modify(input_mode)
    }

	/// Called by hil::Controller::configure to set pin function
    pub fn select_peripheral(&self, function: FieldValue<u32, SFSP::Register>) {
		//let sfsp: unsafe { StaticRef::new((SCU_BASE + self.port_name * 32 * 4 + self.pin_name * 4) as *const ReadWrite) }
		let sfsp = &SCU_BASE.sfsp[self.port_name as usize][self.pin_name as usize];
		sfsp.write(function);
    }

    pub fn enable_pull_down(&self) {
		let sfsp = &SCU_BASE.sfsp[self.port_name as usize][self.pin_name as usize];
		sfsp.modify(SFSP::EPD::EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode);
    }

    pub fn disable_pull_down(&self) {
        let sfsp = &SCU_BASE.sfsp[self.port_name as usize][self.pin_name as usize];
		sfsp.modify(SFSP::EPD::DisablePullDown);
    }

    pub fn enable_pull_up(&self) {
        let sfsp = &SCU_BASE.sfsp[self.port_name as usize][self.pin_name as usize];
		sfsp.modify(SFSP::EPUN::EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode);
    }

    pub fn disable_pull_up(&self) {
        let sfsp = &SCU_BASE.sfsp[self.port_name as usize][self.pin_name as usize];
		sfsp.modify(SFSP::EPUN::DisablePullUp);
    }

    pub fn read(&self) -> bool {
        let b = &GPIO_PORT_BASE.b[self.gpio_port as usize][self.gpio_pin as usize];
		b.get() != 0
    }

    pub fn toggle(&self) {
		let b = &GPIO_PORT_BASE.b[self.gpio_port as usize][self.gpio_pin as usize];
        let val = self.read();
		b.set((val as u8 == 0) as u8)
    }

    pub fn set(&self) {
        let b = &GPIO_PORT_BASE.b[self.gpio_port as usize][self.gpio_pin as usize];
		b.set(1)
    }

    pub fn clear(&self) {
        let b = &GPIO_PORT_BASE.b[self.gpio_port as usize][self.gpio_pin as usize];
		b.set(0)
    }
}

impl hil::Controller for GPIOPin {
    type Config = Option<FieldValue<u32, SFSP::Register>>;
	/// Configure a function, or enable the pin as a GPIO
    fn configure(&self, config: Self::Config) {
        match config {
            Some(c) => self.select_peripheral(c),
            None => self.enable(),
        }
    }
}

impl hil::gpio::PinCtl for GPIOPin {
    fn set_input_mode(&self, mode: hil::gpio::InputMode) {
		let sfsp = &SCU_BASE.sfsp[self.port_name as usize][self.pin_name as usize];
        match mode {
            hil::gpio::InputMode::PullUp => {
				sfsp.modify(SFSP::EPD::DisablePullDown + SFSP::EPUN::EnablePullUpEnableBothPullDownResistorAndPullUpResistorForRepeaterMode);
            }
            hil::gpio::InputMode::PullDown => {
				sfsp.modify(SFSP::EPUN::DisablePullUp + SFSP::EPD::EnablePullDownEnableBothPullDownResistorAndPullUpResistorForRepeaterMode);
            }
            hil::gpio::InputMode::PullNone => {
                sfsp.modify(SFSP::EPUN::DisablePullUp + SFSP::EPD::DisablePullDown);
            }
        }
    }

}

impl hil::gpio::Pin for GPIOPin {
    fn disable(&self) {
        GPIOPin::disable(self);
    }

    fn make_output(&self) {
        self.enable();
        GPIOPin::make_output(self);
    }

    fn make_input(&self) {
        self.enable();
        GPIOPin::make_input(self);
    }

    fn read(&self) -> bool {
        GPIOPin::read(self)
    }

    fn toggle(&self) {
        GPIOPin::toggle(self);
    }

    fn set(&self) {
        GPIOPin::set(self);
    }

    fn clear(&self) {
        GPIOPin::clear(self);
    }

    fn enable_interrupt(&self, client_data: usize, mode: hil::gpio::InterruptMode) {
        let mode_bits = match mode {
            hil::gpio::InterruptMode::EitherEdge => 0b00,
            hil::gpio::InterruptMode::RisingEdge => 0b01,
            hil::gpio::InterruptMode::FallingEdge => 0b10,
        };
        self.client_data.set(client_data);
        /*GPIOPin::set_interrupt_mode(self, mode_bits);
        GPIOPin::enable_interrupt(self);*/
    }

    fn disable_interrupt(&self) {
        //GPIOPin::disable_interrupt(self);
    }
}