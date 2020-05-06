//! Interrupt mapping and DMA channel setup.
use cortexm4;
use kernel::Chip;
use core::fmt::Write;
use crate::adc;
use crate::atimer;
use crate::gpio;
use crate::nvic;

use crate::usart;
pub struct Lpc43xx {
    mpu: cortexm4::mpu::MPU,
	userspace_kernel_boundary: cortexm4::syscall::SysCall,
    systick: cortexm4::systick::SysTick,
}

impl Lpc43xx {
    pub unsafe fn new() -> Lpc43xx {
        Lpc43xx {
            mpu: cortexm4::mpu::MPU::new(),
			userspace_kernel_boundary: cortexm4::syscall::SysCall::new(),
            systick: cortexm4::systick::SysTick::new(),
        }
    }
}

impl Chip for Lpc43xx {
    type MPU = cortexm4::mpu::MPU;
    type UserspaceKernelBoundary = cortexm4::syscall::SysCall;
    type SysTick = cortexm4::systick::SysTick;
    
    #[no_mangle]
    #[inline(never)]
    fn service_pending_interrupts(&self) {
        unsafe {
            loop {
                if let Some(interrupt) = cortexm4::nvic::next_pending() {
                    match interrupt {
                        nvic::ADC0 => adc::ADC0.handle_interrupt(),
                        nvic::ADC1 => adc::ADC1.handle_interrupt(),
                        nvic::ATIMER => atimer::ATIMER.handle_interrupt(),
                        nvic::PIN_INT0 => gpio::GPIO0[4].handle_interrupt(),
                        nvic::PIN_INT1 => gpio::GPIO0[8].handle_interrupt(),
                        nvic::PIN_INT2 => gpio::GPIO0[9].handle_interrupt(),
                        nvic::PIN_INT3 => gpio::GPIO1[9].handle_interrupt(),
                        nvic::USART2 => usart::USART2.handle_interrupt(),
                        _ => {
                            // This handler should work with JLink GDB to unwind the stack trace
//                            asm!("bkpt #10
//                            bx lr"::::);
                            panic!("unhandled interrupt {}", interrupt);
                        }
                    }
                    
                    let n = cortexm4::nvic::Nvic::new(interrupt);
                    n.clear_pending();
                    n.enable();
                } else {
                    break;
                }
            }
        }
    }

    fn has_pending_interrupts(&self) -> bool {
        unsafe { cortexm4::nvic::has_pending() }
    }

    fn sleep(&self) {
        unsafe {
            cortexm4::support::wfi();
        }
    }
    
    fn mpu(&self) -> &cortexm4::mpu::MPU {
        &self.mpu
    }

    fn systick(&self) -> &cortexm4::systick::SysTick {
        &self.systick
    }

    unsafe fn atomic<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        cortexm4::support::atomic(f)
    }

    fn userspace_kernel_boundary(&self) -> &cortexm4::syscall::SysCall {
        &self.userspace_kernel_boundary
    }

    unsafe fn print_state(&self, write: &mut dyn Write) {
        cortexm4::print_cortexm4_state(write);
    }
}