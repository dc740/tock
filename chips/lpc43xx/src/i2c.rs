
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// I2C-bus interface
#[repr(C)]
struct I2CRegisters {
/// I2C Control Set Register. When a one is written to a bit of this register, the c
conset: ReadWrite<u32, CONSET::Register>,
/// I2C Status Register. During I2C operation, this register provides detailed statu
stat: ReadOnly<u32>,
/// I2C Data Register. During master or slave transmit mode, data to be transmitted
dat: ReadWrite<u32>,
/// I2C Slave Address Register 0. Contains the 7-bit slave address for operation of
adr0: ReadWrite<u32, ADR0::Register>,
/// SCH Duty Cycle Register High Half Word. Determines the high time of the I2C cloc
sclh: ReadWrite<u32>,
/// SCL Duty Cycle Register Low Half Word. Determines the low time of the I2C clock.
scll: ReadWrite<u32>,
/// I2C Control Clear Register. When a one is written to a bit of this register, the
conclr: WriteOnly<u32, CONCLR::Register>,
/// Monitor mode control register.
mmctrl: ReadWrite<u32, MMCTRL::Register>,
/// I2C Slave Address Register. Contains the 7-bit slave address for operation of th
adr1: ReadWrite<u32, ADR1::Register>,
/// I2C Slave Address Register. Contains the 7-bit slave address for operation of th
adr2: ReadWrite<u32, ADR2::Register>,
/// I2C Slave Address Register. Contains the 7-bit slave address for operation of th
adr3: ReadWrite<u32, ADR3::Register>,
/// Data buffer register. The contents of the 8 MSBs of the DAT shift register will
data_buffer: ReadOnly<u32>,
/// I2C Slave address mask register
mask0: ReadWrite<u32>,
/// I2C Slave address mask register
mask1: ReadWrite<u32>,
/// I2C Slave address mask register
mask2: ReadWrite<u32>,
/// I2C Slave address mask register
mask3: ReadWrite<u32>,
}
register_bitfields![u32,
CONSET [
    /// Assert acknowledge flag.
    AA OFFSET(2) NUMBITS(1) [],
    /// I2C interrupt flag.
    SI OFFSET(3) NUMBITS(1) [],
    /// STOP flag.
    STO OFFSET(4) NUMBITS(1) [],
    /// START flag.
    STA OFFSET(5) NUMBITS(1) [],
    /// I2C interface enable.
    I2EN OFFSET(6) NUMBITS(1) []
],
ADR0 [
    /// General Call enable bit.
    GC OFFSET(0) NUMBITS(1) [],
    /// The I2C device address for slave mode.
    Address OFFSET(1) NUMBITS(7) []
],
CONCLR [
    /// Assert acknowledge Clear bit.
    AAC OFFSET(2) NUMBITS(1) [],
    /// I2C interrupt Clear bit.
    SIC OFFSET(3) NUMBITS(1) [],
    /// START flag Clear bit.
    STAC OFFSET(5) NUMBITS(1) [],
    /// I2C interface Disable bit.
    I2ENC OFFSET(6) NUMBITS(1) []
],
MMCTRL [
    /// Monitor mode enable.
    MM_ENA OFFSET(0) NUMBITS(1) [
        /// Monitor mode disabled.
        MonitorModeDisabled = 0,
        /// The I 2C module will enter monitor mode. In this mode the SDA output will be for
        THE_I_2C_MODULE_WILL = 1
    ],
    /// SCL output enable.
    ENA_SCL OFFSET(1) NUMBITS(1) [
        /// When this bit is cleared to 0, the SCL output will be forced high when the modul
        WHEN_THIS_BIT_IS_CLE = 0,
        /// When this bit is set, the I2C module may exercise the same control over the cloc
        WHEN_THIS_BIT_IS_SET = 1
    ],
    /// Select interrupt register match.
    MATCH_ALL OFFSET(2) NUMBITS(1) [
        /// When this bit is cleared, an interrupt will only be generated when a match occur
        WHEN_THIS_BIT_IS_CLE = 0,
        /// When this bit is set to 1 and the I2C is in monitor mode, an interrupt will be g
        WHEN_THIS_BIT_IS_SET = 1
    ]
],
ADR1 [
    /// General Call enable bit.
    GC OFFSET(0) NUMBITS(1) [],
    /// The I2C device address for slave mode.
    Address OFFSET(1) NUMBITS(7) []
],
ADR2 [
    /// General Call enable bit.
    GC OFFSET(0) NUMBITS(1) [],
    /// The I2C device address for slave mode.
    Address OFFSET(1) NUMBITS(7) []
],
ADR3 [
    /// General Call enable bit.
    GC OFFSET(0) NUMBITS(1) [],
    /// The I2C device address for slave mode.
    Address OFFSET(1) NUMBITS(7) []
]
];
const I2C0_BASE: StaticRef<I2CRegisters> =
    unsafe { StaticRef::new(0x400A1000 as *const I2CRegisters) };


const I2C1_BASE: StaticRef<I2CRegisters> =
    unsafe { StaticRef::new(0x400E0000 as *const I2CRegisters) };
