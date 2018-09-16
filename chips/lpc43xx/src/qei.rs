
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// Quadrature Encoder Interface (QEI)
#[repr(C)]
struct QeiRegisters {
/// Control register
con: WriteOnly<u32, CON::Register>,
/// Encoder status register
stat: ReadOnly<u32>,
/// Configuration register
conf: ReadWrite<u32, CONF::Register>,
/// Position register
pos: ReadOnly<u32>,
/// Maximum position register
maxpos: ReadWrite<u32>,
/// position compare register 0
cmpos0: ReadWrite<u32>,
/// position compare register 1
cmpos1: ReadWrite<u32>,
/// position compare register 2
cmpos2: ReadWrite<u32>,
/// Index count register
inxcnt: ReadOnly<u32>,
/// Index compare register 0
inxcmp0: ReadWrite<u32>,
/// Velocity timer reload register
load: ReadWrite<u32>,
/// Velocity timer register
time: ReadOnly<u32>,
/// Velocity counter register
vel: ReadOnly<u32>,
/// Velocity capture register
cap: ReadOnly<u32>,
/// Velocity compare register
velcomp: ReadWrite<u32>,
/// Digital filter register on input phase A (QEI_A)
filterpha: ReadWrite<u32>,
/// Digital filter register on input phase B (QEI_B)
filterphb: ReadWrite<u32>,
/// Digital filter register on input index (QEI_IDX)
filterinx: ReadWrite<u32>,
/// Index acceptance window register
window: ReadWrite<u32>,
/// Index compare register 1
inxcmp1: ReadWrite<u32>,
/// Index compare register 2
inxcmp2: ReadWrite<u32>,
_reserved0: [u8; 3972],
/// Interrupt enable clear register
iec: WriteOnly<u32, IEC::Register>,
/// Interrupt enable set register
ies: WriteOnly<u32, IES::Register>,
/// Interrupt status register
intstat: ReadOnly<u32, INTSTAT::Register>,
/// Interrupt enable register
ie: ReadOnly<u32, IE::Register>,
/// Interrupt status clear register
clr: WriteOnly<u32, CLR::Register>,
/// Interrupt status set register
set: WriteOnly<u32, SET::Register>,
}
register_bitfields![u32,
CON [
    /// Reset position counter. When set = 1, resets the position counter to all zeros.
    RESP OFFSET(0) NUMBITS(1) [],
    /// Reset position counter on index. When set = 1, resets the position counter to al
    RESPI OFFSET(1) NUMBITS(1) [],
    /// Reset velocity. When set = 1, resets the velocity counter to all zeros and reloa
    RESV OFFSET(2) NUMBITS(1) [],
    /// Reset index counter. When set = 1, resets the index counter to all zeros. Autocl
    RESI OFFSET(3) NUMBITS(1) []
],
CONF [
    /// Direction invert. When = 1, complements the DIR bit.
    DIRINV OFFSET(0) NUMBITS(1) [],
    /// Signal Mode. When = 0, PhA and PhB function as quadrature encoder inputs. When =
    SIGMODE OFFSET(1) NUMBITS(1) [],
    /// Capture Mode. When = 0, only PhA edges are counted (2X). When = 1, BOTH PhA and
    CAPMODE OFFSET(2) NUMBITS(1) [],
    /// Invert Index. When set, inverts the sense of the index input.
    INVINX OFFSET(3) NUMBITS(1) [],
    /// Continuously reset position counter on index. When set = 1, resets the position
    CRESPI OFFSET(4) NUMBITS(1) [],
    /// Index gating configuration: when INXGATE(19)=1, pass the index when Pha=0 and Ph
    INXGATE OFFSET(16) NUMBITS(4) []
],
IEC [
    /// Indicates that an index pulse was detected.
    INX_EN OFFSET(0) NUMBITS(1) [],
    /// Indicates that a velocity timer overflow occurred
    TIM_EN OFFSET(1) NUMBITS(1) [],
    /// Indicates that captured velocity is less than compare velocity.
    VELC_EN OFFSET(2) NUMBITS(1) [],
    /// Indicates that a change of direction was detected.
    DIR_EN OFFSET(3) NUMBITS(1) [],
    /// Indicates that an encoder phase error was detected.
    ERR_EN OFFSET(4) NUMBITS(1) [],
    /// Indicates that and encoder clock pulse was detected.
    ENCLK_EN OFFSET(5) NUMBITS(1) [],
    /// Indicates that the position 0 compare value is equal to the current position.
    POS0_Int OFFSET(6) NUMBITS(1) [],
    /// Indicates that the position 1compare value is equal to the current position.
    POS1_Int OFFSET(7) NUMBITS(1) [],
    /// Indicates that the position 2 compare value is equal to the current position.
    POS2_Int OFFSET(8) NUMBITS(1) [],
    /// Indicates that the index compare value is equal to the current index count.
    REV_Int OFFSET(9) NUMBITS(1) [],
    /// Combined position 0 and revolution count interrupt. Set when both the POS0_Int b
    POS0REV_Int OFFSET(10) NUMBITS(1) [],
    /// Combined position 1 and revolution count interrupt. Set when both the POS1_Int b
    POS1REV_Int OFFSET(11) NUMBITS(1) [],
    /// Combined position 2 and revolution count interrupt. Set when both the POS2_Int b
    POS2REV_Int OFFSET(12) NUMBITS(1) [],
    /// Indicates that the index 1 compare value is equal to the current index count.
    REV1_Int OFFSET(13) NUMBITS(1) [],
    /// Indicates that the index 2 compare value is equal to the current index count.
    REV2_Int OFFSET(14) NUMBITS(1) [],
    /// Indicates that the current position count goes through the MAXPOS value to zero
    MAXPOS_Int OFFSET(15) NUMBITS(1) []
],
IES [
    /// Indicates that an index pulse was detected.
    INX_EN OFFSET(0) NUMBITS(1) [],
    /// Indicates that a velocity timer overflow occurred
    TIM_EN OFFSET(1) NUMBITS(1) [],
    /// Indicates that captured velocity is less than compare velocity.
    VELC_EN OFFSET(2) NUMBITS(1) [],
    /// Indicates that a change of direction was detected.
    DIR_EN OFFSET(3) NUMBITS(1) [],
    /// Indicates that an encoder phase error was detected.
    ERR_EN OFFSET(4) NUMBITS(1) [],
    /// Indicates that and encoder clock pulse was detected.
    ENCLK_EN OFFSET(5) NUMBITS(1) [],
    /// Indicates that the position 0 compare value is equal to the current position.
    POS0_Int OFFSET(6) NUMBITS(1) [],
    /// Indicates that the position 1compare value is equal to the current position.
    POS1_Int OFFSET(7) NUMBITS(1) [],
    /// Indicates that the position 2 compare value is equal to the current position.
    POS2_Int OFFSET(8) NUMBITS(1) [],
    /// Indicates that the index compare value is equal to the current index count.
    REV_Int OFFSET(9) NUMBITS(1) [],
    /// Combined position 0 and revolution count interrupt. Set when both the POS0_Int b
    POS0REV_Int OFFSET(10) NUMBITS(1) [],
    /// Combined position 1 and revolution count interrupt. Set when both the POS1_Int b
    POS1REV_Int OFFSET(11) NUMBITS(1) [],
    /// Combined position 2 and revolution count interrupt. Set when both the POS2_Int b
    POS2REV_Int OFFSET(12) NUMBITS(1) [],
    /// Indicates that the index 1 compare value is equal to the current index count.
    REV1_Int OFFSET(13) NUMBITS(1) [],
    /// Indicates that the index 2 compare value is equal to the current index count.
    REV2_Int OFFSET(14) NUMBITS(1) [],
    /// Indicates that the current position count goes through the MAXPOS value to zero
    MAXPOS_Int OFFSET(15) NUMBITS(1) []
],
INTSTAT [
    /// Indicates that an index pulse was detected.
    INX_Int OFFSET(0) NUMBITS(1) [],
    /// Indicates that a velocity timer overflow occurred
    TIM_Int OFFSET(1) NUMBITS(1) [],
    /// Indicates that captured velocity is less than compare velocity.
    VELC_Int OFFSET(2) NUMBITS(1) [],
    /// Indicates that a change of direction was detected.
    DIR_Int OFFSET(3) NUMBITS(1) [],
    /// Indicates that an encoder phase error was detected.
    ERR_Int OFFSET(4) NUMBITS(1) [],
    /// Indicates that and encoder clock pulse was detected.
    ENCLK_Int OFFSET(5) NUMBITS(1) [],
    /// Indicates that the position 0 compare value is equal to the current position.
    POS0_Int OFFSET(6) NUMBITS(1) [],
    /// Indicates that the position 1compare value is equal to the current position.
    POS1_Int OFFSET(7) NUMBITS(1) [],
    /// Indicates that the position 2 compare value is equal to the current position.
    POS2_Int OFFSET(8) NUMBITS(1) [],
    /// Indicates that the index compare value is equal to the current index count.
    REV_Int OFFSET(9) NUMBITS(1) [],
    /// Combined position 0 and revolution count interrupt. Set when both the POS0_Int b
    POS0REV_Int OFFSET(10) NUMBITS(1) [],
    /// Combined position 1 and revolution count interrupt. Set when both the POS1_Int b
    POS1REV_Int OFFSET(11) NUMBITS(1) [],
    /// Combined position 2 and revolution count interrupt. Set when both the POS2_Int b
    POS2REV_Int OFFSET(12) NUMBITS(1) [],
    /// Indicates that the index 1 compare value is equal to the current index count.
    REV1_Int OFFSET(13) NUMBITS(1) [],
    /// Indicates that the index 2 compare value is equal to the current index count.
    REV2_Int OFFSET(14) NUMBITS(1) [],
    /// Indicates that the current position count goes through the MAXPOS value to zero
    MAXPOS_Int OFFSET(15) NUMBITS(1) []
],
IE [
    /// Indicates that an index pulse was detected.
    INX_Int OFFSET(0) NUMBITS(1) [],
    /// Indicates that a velocity timer overflow occurred
    TIM_Int OFFSET(1) NUMBITS(1) [],
    /// Indicates that captured velocity is less than compare velocity.
    VELC_Int OFFSET(2) NUMBITS(1) [],
    /// Indicates that a change of direction was detected.
    DIR_Int OFFSET(3) NUMBITS(1) [],
    /// Indicates that an encoder phase error was detected.
    ERR_Int OFFSET(4) NUMBITS(1) [],
    /// Indicates that and encoder clock pulse was detected.
    ENCLK_Int OFFSET(5) NUMBITS(1) [],
    /// Indicates that the position 0 compare value is equal to the current position.
    POS0_Int OFFSET(6) NUMBITS(1) [],
    /// Indicates that the position 1compare value is equal to the current position.
    POS1_Int OFFSET(7) NUMBITS(1) [],
    /// Indicates that the position 2 compare value is equal to the current position.
    POS2_Int OFFSET(8) NUMBITS(1) [],
    /// Indicates that the index compare value is equal to the current index count.
    REV_Int OFFSET(9) NUMBITS(1) [],
    /// Combined position 0 and revolution count interrupt. Set when both the POS0_Int b
    POS0REV_Int OFFSET(10) NUMBITS(1) [],
    /// Combined position 1 and revolution count interrupt. Set when both the POS1_Int b
    POS1REV_Int OFFSET(11) NUMBITS(1) [],
    /// Combined position 2 and revolution count interrupt. Set when both the POS2_Int b
    POS2REV_Int OFFSET(12) NUMBITS(1) [],
    /// Indicates that the index 1 compare value is equal to the current index count.
    REV1_Int OFFSET(13) NUMBITS(1) [],
    /// Indicates that the index 2 compare value is equal to the current index count.
    REV2_Int OFFSET(14) NUMBITS(1) [],
    /// Indicates that the current position count goes through the MAXPOS value to zero
    MAXPOS_Int OFFSET(15) NUMBITS(1) []
],
CLR [
    /// Indicates that an index pulse was detected.
    INX_Int OFFSET(0) NUMBITS(1) [],
    /// Indicates that a velocity timer overflow occurred
    TIM_Int OFFSET(1) NUMBITS(1) [],
    /// Indicates that captured velocity is less than compare velocity.
    VELC_Int OFFSET(2) NUMBITS(1) [],
    /// Indicates that a change of direction was detected.
    DIR_Int OFFSET(3) NUMBITS(1) [],
    /// Indicates that an encoder phase error was detected.
    ERR_Int OFFSET(4) NUMBITS(1) [],
    /// Indicates that and encoder clock pulse was detected.
    ENCLK_Int OFFSET(5) NUMBITS(1) [],
    /// Indicates that the position 0 compare value is equal to the current position.
    POS0_Int OFFSET(6) NUMBITS(1) [],
    /// Indicates that the position 1compare value is equal to the current position.
    POS1_Int OFFSET(7) NUMBITS(1) [],
    /// Indicates that the position 2 compare value is equal to the current position.
    POS2_Int OFFSET(8) NUMBITS(1) [],
    /// Indicates that the index compare value is equal to the current index count.
    REV_Int OFFSET(9) NUMBITS(1) [],
    /// Combined position 0 and revolution count interrupt. Set when both the POS0_Int b
    POS0REV_Int OFFSET(10) NUMBITS(1) [],
    /// Combined position 1 and revolution count interrupt. Set when both the POS1_Int b
    POS1REV_Int OFFSET(11) NUMBITS(1) [],
    /// Indicates that the index 1 compare value is equal to the current index count.
    REV1_Int OFFSET(13) NUMBITS(1) [],
    /// Indicates that the index 2 compare value is equal to the current index count.
    REV2_Int OFFSET(14) NUMBITS(1) [],
    /// Indicates that the current position count goes through the MAXPOS value to zero
    MAXPOS_Int OFFSET(15) NUMBITS(1) []
],
SET [
    /// Indicates that an index pulse was detected.
    INX_Int OFFSET(0) NUMBITS(1) [],
    /// Indicates that a velocity timer overflow occurred
    TIM_Int OFFSET(1) NUMBITS(1) [],
    /// Indicates that captured velocity is less than compare velocity.
    VELC_Int OFFSET(2) NUMBITS(1) [],
    /// Indicates that a change of direction was detected.
    DIR_Int OFFSET(3) NUMBITS(1) [],
    /// Indicates that an encoder phase error was detected.
    ERR_Int OFFSET(4) NUMBITS(1) [],
    /// Indicates that and encoder clock pulse was detected.
    ENCLK_Int OFFSET(5) NUMBITS(1) [],
    /// Indicates that the position 0 compare value is equal to the current position.
    POS0_Int OFFSET(6) NUMBITS(1) [],
    /// Indicates that the position 1compare value is equal to the current position.
    POS1_Int OFFSET(7) NUMBITS(1) [],
    /// Indicates that the position 2 compare value is equal to the current position.
    POS2_Int OFFSET(8) NUMBITS(1) [],
    /// Indicates that the index compare value is equal to the current index count.
    REV_Int OFFSET(9) NUMBITS(1) [],
    /// Combined position 0 and revolution count interrupt. Set when both the POS0_Int b
    POS0REV_Int OFFSET(10) NUMBITS(1) [],
    /// Combined position 1 and revolution count interrupt. Set when both the POS1_Int b
    POS1REV_Int OFFSET(11) NUMBITS(1) [],
    /// Combined position 2 and revolution count interrupt. Set when both the POS2_Int b
    POS2REV_Int OFFSET(12) NUMBITS(1) [],
    /// Indicates that the index 1 compare value is equal to the current index count.
    REV1_Int OFFSET(13) NUMBITS(1) [],
    /// Indicates that the index 2 compare value is equal to the current index count.
    REV2_Int OFFSET(14) NUMBITS(1) [],
    /// Indicates that the current position count goes through the MAXPOS value to zero
    MAXPOS_Int OFFSET(15) NUMBITS(1) []
]
];
const QEI_BASE: StaticRef<QeiRegisters> =
    unsafe { StaticRef::new(0x400C6000 as *const QeiRegisters) };
