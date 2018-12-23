
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly, register_bitfields};
    /// Power Management Controller (PMC)
#[repr(C)]
struct PmcRegisters {
/// Hardware sleep event enable register
pd0_sleep0_hw_ena: ReadWrite<u32, PD0_SLEEP0_HW_ENA::Register>,
_reserved0: [u8; 24],
/// Sleep power mode register
pd0_sleep0_mode: ReadWrite<u32>,
}
register_bitfields![u32,
PD0_SLEEP0_HW_ENA [
    /// Writing a 1 enables the Cortex-M4 core to put the part
								into any of the P
    ENA_EVENT0 OFFSET(0) NUMBITS(1) [],
    /// Writing a 1 enables the Cortex-M0 core and the
								Cortex-M0 subsystem core
    ENA_EVENT1 OFFSET(1) NUMBITS(1) []
]
];
const PMC_BASE: StaticRef<PmcRegisters> =
    unsafe { StaticRef::new(0x40042000 as *const PmcRegisters) };
