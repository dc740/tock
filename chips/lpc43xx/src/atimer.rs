/**
 * The Alarm Timer is a 16 bit timer
 * that runs on 1024Hz
 */

use kernel::common::StaticRef;
use kernel::common::registers::{ReadOnly, ReadWrite, WriteOnly};

use kernel::common::cells::OptionalCell;
use kernel::hil::time::{self, Alarm, Time, Frequency};
use kernel::hil::Controller;

use crate::{eventrouter,nvic};

    /// Alarm timer
#[repr(C)]
struct AtimerRegisters {
/// Downcounter register
downcounter: ReadWrite<u32>,
/// Preset value register
preset: ReadWrite<u32>,
_reserved0: [u8; 4048],
/// Interrupt clear enable register
clr_en: WriteOnly<u32>,
/// Interrupt set enable register
set_en: WriteOnly<u32>,
/// Status register
status: ReadOnly<u32>,
/// Enable register
enable: ReadOnly<u32>,
/// Clear register
clr_stat: WriteOnly<u32>,
/// Set register
set_stat: WriteOnly<u32>,
}


const ATIMER_BASE: StaticRef<AtimerRegisters> =
    unsafe { StaticRef::new(0x40040000 as *const AtimerRegisters) };

pub struct AlarmTimer<'a> {
    registers: StaticRef<AtimerRegisters>,
    callback: OptionalCell<&'a dyn time::AlarmClient>,
}

pub static mut ATIMER: AlarmTimer<'static> = AlarmTimer {
    registers: ATIMER_BASE,
    callback: OptionalCell::empty(),
};

impl Controller for AlarmTimer<'a> {
    type Config = &'static dyn time::AlarmClient;
#[no_mangle]
#[inline(never)]
    fn configure(&self, client: &'a dyn time::AlarmClient) {
        let regs: &AtimerRegisters = &*self.registers;
        self.callback.set(client);
        regs.preset.set(0);
        self.clear_register();
        eventrouter::atimer_setup();
        self.clear();
        self.disable();
    }
}

impl AlarmTimer<'a> {
    fn set_client(&self, client: &'a dyn time::AlarmClient) {
        self.callback.set(client);
    }
    
#[no_mangle]
#[inline(never)]    
    fn clear(&self) {
        self.clear_register();
        eventrouter::clear_pending_atimer_interrupt_evrt_source();
    }
    /// Clears the alarm bit in the status register (indicating the alarm value
    /// has been reached).
#[no_mangle]
#[inline(never)]
    fn clear_register(&self) {
        let regs: &AtimerRegisters = &*self.registers;
        regs.clr_stat.set(1);
    }

    /// Enables the AlarmTimer registers
    fn enable_registers(&self) {
        let regs: &AtimerRegisters = &*self.registers;
        regs.set_en.set(1);
    }

    /// Disable the AlarmTimer registers
    fn disable_registers(&self) {
        let regs: &AtimerRegisters = &*self.registers;
        regs.clr_en.set(1);
    }

    /// Returns if an alarm is currently set
    fn is_alarm_enabled(&self) -> bool {
        let regs: &AtimerRegisters = &*self.registers;
        regs.enable.get() !=  0
    }

    fn enable_alarm_irq(&self) {
        //let regs: &AtimerRegisters = &*self.registers;
        unsafe {
            let n = cortexm4::nvic::Nvic::new(nvic::ATIMER);
            n.enable();
        }
    }

    fn disable_alarm_irq(&self) {
        //let regs: &AtimerRegisters = &*self.registers;
        unsafe {
            let n = cortexm4::nvic::Nvic::new(nvic::ATIMER);
            n.clear_pending();
            n.disable();
        }
    }

    fn get_counter(&self) -> u32 {
        let regs: &AtimerRegisters = &*self.registers;
        regs.downcounter.get()
    }

    pub fn handle_interrupt(&mut self) {
        self.clear();
        self.callback.map(|cb| {
            cb.fired();
        });
    }
}

/// Freq1024Hz `Frequency` derived from the 32kHz clock
#[derive(Debug)]
pub struct Freq1024Hz;
impl Frequency for Freq1024Hz {
    fn frequency() -> u32 {
        1024
    }
}


impl Time for AlarmTimer<'a> {
    type Frequency = Freq1024Hz;

    fn now(&self) -> u32 {
        self.get_counter()
    }

    fn max_tics(&self) -> u32 {
        core::u16::MAX.into()
    }
}

impl Alarm<'a> for AlarmTimer<'a> {
    fn set_client(&self, client: &'a dyn time::AlarmClient) {
        AlarmTimer::set_client(self, client);
    }

    fn set_alarm(&self, tics: u32) {
        let regs: &AtimerRegisters = &*self.registers;
        self.clear();
        regs.preset.set(tics);
        self.enable_alarm_irq();
        self.enable_registers();
    }

    fn get_alarm(&self) -> u32 {
        let regs: &AtimerRegisters = &*self.registers;
        regs.preset.get()
    }

    fn disable(&self) {
        self.disable_alarm_irq();
        self.clear();
        self.disable_registers();
    }

    fn is_enabled(&self) -> bool {
        AlarmTimer::is_alarm_enabled(self)
    }
}
