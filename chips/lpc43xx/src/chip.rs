//! Interrupt mapping and DMA channel setup.
use cortexm4;
use kernel::common::deferred_call;
use kernel::Chip;


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

    fn service_pending_interrupts(&self) {
        unsafe {
            loop {
                if let Some(interrupt) = cortexm4::nvic::next_pending() {
                    match interrupt {
                        _ => {
                            (); //panic!("unhandled interrupt {}", interrupt);
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

}