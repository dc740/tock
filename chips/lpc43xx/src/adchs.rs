
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// 12-bit Analog-to-Digital Converter  High-Speed (ADCHS)
#[repr(C)]
struct AdchsRegisters {
/// Flushes FIFO
flush: WriteOnly<u32>,
/// Set or clear DMA write request
dma_req: ReadWrite<u32>,
/// Indicates FIFO fill level status
fifo_sts: ReadOnly<u32>,
/// Configures FIFO fill level that triggers interrupt and packing 1 or 2 samples pe
fifo_cfg: ReadWrite<u32, FIFO_CFG::Register>,
/// Enable software trigger to start descriptor processing
trigger: WriteOnly<u32>,
/// Indicates active descriptor table and descriptor entry
dscr_sts: ReadWrite<u32, DSCR_STS::Register>,
/// Set or clear power down mode
power_down: ReadWrite<u32>,
/// Configures external trigger mode, store channel ID in FIFO and walk-up recovery
config: ReadWrite<u32, CONFIG::Register>,
/// Configures window comparator A levels.
thr_a: ReadWrite<u32, THR_A::Register>,
/// Configures window comparator B levels.
thr_b: ReadWrite<u32, THR_B::Register>,
/// Contains last converted sample of input M [M=0..5) and result of window comparat
last_sample_0: ReadOnly<u32, LAST_SAMPLE[0]::Register>,
/// Contains last converted sample of input M [M=0..5) and result of window comparat
last_sample_1: ReadOnly<u32, LAST_SAMPLE[1]::Register>,
/// Contains last converted sample of input M [M=0..5) and result of window comparat
last_sample_2: ReadOnly<u32, LAST_SAMPLE[2]::Register>,
/// Contains last converted sample of input M [M=0..5) and result of window comparat
last_sample_3: ReadOnly<u32, LAST_SAMPLE[3]::Register>,
/// Contains last converted sample of input M [M=0..5) and result of window comparat
last_sample_4: ReadOnly<u32, LAST_SAMPLE[4]::Register>,
/// Contains last converted sample of input M [M=0..5) and result of window comparat
last_sample_5: ReadOnly<u32, LAST_SAMPLE[5]::Register>,
_reserved0: [u8; 196],
/// ADC speed control
adc_speed: ReadWrite<u32, ADC_SPEED::Register>,
/// Configures ADC power vs. speed, DC-in biasing, output format and power gating.
power_control: ReadWrite<u32, POWER_CONTROL::Register>,
_reserved1: [u8; 244],
/// FIFO output mapped to 16 consecutive address locations. An output contains the v
fifo_output_0: ReadOnly<u32, FIFO_OUTPUT[0]::Register>,
/// FIFO output mapped to 16 consecutive address locations. An output contains the v
fifo_output_1: ReadOnly<u32, FIFO_OUTPUT[1]::Register>,
/// FIFO output mapped to 16 consecutive address locations. An output contains the v
fifo_output_2: ReadOnly<u32, FIFO_OUTPUT[2]::Register>,
/// FIFO output mapped to 16 consecutive address locations. An output contains the v
fifo_output_3: ReadOnly<u32, FIFO_OUTPUT[3]::Register>,
/// FIFO output mapped to 16 consecutive address locations. An output contains the v
fifo_output_4: ReadOnly<u32, FIFO_OUTPUT[4]::Register>,
/// FIFO output mapped to 16 consecutive address locations. An output contains the v
fifo_output_5: ReadOnly<u32, FIFO_OUTPUT[5]::Register>,
/// FIFO output mapped to 16 consecutive address locations. An output contains the v
fifo_output_6: ReadOnly<u32, FIFO_OUTPUT[6]::Register>,
/// FIFO output mapped to 16 consecutive address locations. An output contains the v
fifo_output_7: ReadOnly<u32, FIFO_OUTPUT[7]::Register>,
/// FIFO output mapped to 16 consecutive address locations. An output contains the v
fifo_output_8: ReadOnly<u32, FIFO_OUTPUT[8]::Register>,
/// FIFO output mapped to 16 consecutive address locations. An output contains the v
fifo_output_9: ReadOnly<u32, FIFO_OUTPUT[9]::Register>,
/// FIFO output mapped to 16 consecutive address locations. An output contains the v
fifo_output_10: ReadOnly<u32, FIFO_OUTPUT[10]::Register>,
/// FIFO output mapped to 16 consecutive address locations. An output contains the v
fifo_output_11: ReadOnly<u32, FIFO_OUTPUT[11]::Register>,
/// FIFO output mapped to 16 consecutive address locations. An output contains the v
fifo_output_12: ReadOnly<u32, FIFO_OUTPUT[12]::Register>,
/// FIFO output mapped to 16 consecutive address locations. An output contains the v
fifo_output_13: ReadOnly<u32, FIFO_OUTPUT[13]::Register>,
/// FIFO output mapped to 16 consecutive address locations. An output contains the v
fifo_output_14: ReadOnly<u32, FIFO_OUTPUT[14]::Register>,
/// FIFO output mapped to 16 consecutive address locations. An output contains the v
fifo_output_15: ReadOnly<u32, FIFO_OUTPUT[15]::Register>,
_reserved2: [u8; 192],
/// Table 0 descriptor n, n= 0 to 7
descriptor0__0: ReadWrite<u32, DESCRIPTOR0_[0]::Register>,
/// Table 0 descriptor n, n= 0 to 7
descriptor0__1: ReadWrite<u32, DESCRIPTOR0_[1]::Register>,
/// Table 0 descriptor n, n= 0 to 7
descriptor0__2: ReadWrite<u32, DESCRIPTOR0_[2]::Register>,
/// Table 0 descriptor n, n= 0 to 7
descriptor0__3: ReadWrite<u32, DESCRIPTOR0_[3]::Register>,
/// Table 0 descriptor n, n= 0 to 7
descriptor0__4: ReadWrite<u32, DESCRIPTOR0_[4]::Register>,
/// Table 0 descriptor n, n= 0 to 7
descriptor0__5: ReadWrite<u32, DESCRIPTOR0_[5]::Register>,
/// Table 0 descriptor n, n= 0 to 7
descriptor0__6: ReadWrite<u32, DESCRIPTOR0_[6]::Register>,
/// Table 0 descriptor n, n= 0 to 7
descriptor0__7: ReadWrite<u32, DESCRIPTOR0_[7]::Register>,
/// Table 1 descriptors n, n=0 to 7
descriptor1__0: ReadWrite<u32, DESCRIPTOR1_[0]::Register>,
/// Table 1 descriptors n, n=0 to 7
descriptor1__1: ReadWrite<u32, DESCRIPTOR1_[1]::Register>,
/// Table 1 descriptors n, n=0 to 7
descriptor1__2: ReadWrite<u32, DESCRIPTOR1_[2]::Register>,
/// Table 1 descriptors n, n=0 to 7
descriptor1__3: ReadWrite<u32, DESCRIPTOR1_[3]::Register>,
/// Table 1 descriptors n, n=0 to 7
descriptor1__4: ReadWrite<u32, DESCRIPTOR1_[4]::Register>,
/// Table 1 descriptors n, n=0 to 7
descriptor1__5: ReadWrite<u32, DESCRIPTOR1_[5]::Register>,
/// Table 1 descriptors n, n=0 to 7
descriptor1__6: ReadWrite<u32, DESCRIPTOR1_[6]::Register>,
/// Table 1 descriptors n, n=0 to 7
descriptor1__7: ReadWrite<u32, DESCRIPTOR1_[7]::Register>,
_reserved3: [u8; 3008],
/// Interrupt 0 clear mask
clr_en0: WriteOnly<u32>,
/// Interrupt 0 set mask
set_en0: WriteOnly<u32>,
/// Interrupt 0 mask
mask0: ReadOnly<u32>,
/// Interrupt 0 status. Interrupt 0 contains FIFO fill level, descriptor status and
status0: ReadOnly<u32, STATUS0::Register>,
/// Interrupt 0 clear status
clr_stat0: WriteOnly<u32>,
/// Interrupt 0 set status
set_stat0: WriteOnly<u32>,
_reserved4: [u8; 8],
/// Interrupt 1 mask clear enable.
clr_en1: WriteOnly<u32>,
/// Interrupt 1 mask set enable
set_en1: WriteOnly<u32>,
/// Interrupt 1 mask
mask1: ReadOnly<u32>,
/// Interrupt 1 status. Interrupt 1 contains window comparator results and register
status1: ReadOnly<u32, STATUS1::Register>,
/// Interrupt 1 clear status
clr_stat1: WriteOnly<u32>,
/// Interrupt 1 set status
set_stat1: WriteOnly<u32>,
}
register_bitfields![u32,
FIFO_CFG [
    /// 0 = one sample is packed in one 32-bit read cycle  1 = two samples are packed in
    PACKED_READ OFFSET(0) NUMBITS(1) [],
    /// When the FIFO contains more or equal than FIFO_LEVEL samples interrupt flag FIFO
    LEVEL OFFSET(1) NUMBITS(5) []
],
DSCR_STS [
    /// 0 = table 0 is active  1 = table 1 is active.
    ACT_TABLE OFFSET(0) NUMBITS(1) [],
    /// ID of the descriptor that is active.
    ACT_DESCRIPTOR OFFSET(1) NUMBITS(3) []
],
CONFIG [
    /// 00 = triggers off  01 = software trigger only  10 = external trigger only  11 =
    TRIGGER__MASK OFFSET(0) NUMBITS(2) [],
    /// 00 = rising external trigger  01 = falling external trigger  10 = low external t
    TRIGGER_MODE OFFSET(2) NUMBITS(2) [],
    /// 0  = do not synchronize external  trigger input  1 = synchronize external  trigg
    TRIGGER_SYNC OFFSET(4) NUMBITS(1) [],
    /// 0 = do not add channel ID to FIFO output data  1 = add channel ID to FIFO output
    CHANNEL_ID_EN OFFSET(5) NUMBITS(1) [],
    /// ADC recovery time from power down
    RECOVERY_TIME OFFSET(6) NUMBITS(8) []
],
THR_A [
    /// Low Compare Threshold Register A:   Contains the lower  threshold  level for aut
    THR_LOW_A OFFSET(0) NUMBITS(12) [],
    /// High Compare Threshold Register A:   Contains the upper threshold  level for aut
    THR_HIGH_A OFFSET(16) NUMBITS(12) []
],
THR_B [
    /// Low Compare Threshold Register B:   Contains the lower  threshold  level for aut
    THR_LOW_B OFFSET(0) NUMBITS(12) [],
    /// High Compare Threshold Register B:   Contains the upper threshold  level for aut
    THR_HIGH_B OFFSET(16) NUMBITS(12) []
],
ADC_SPEED [
    /// Speed0
    DGEC0 OFFSET(0) NUMBITS(4) [],
    /// Speed1
    DGEC1 OFFSET(4) NUMBITS(4) [],
    /// Speed2
    DGEC2 OFFSET(8) NUMBITS(4) [],
    /// Speed3
    DGEC3 OFFSET(12) NUMBITS(4) [],
    /// Speed4
    DGEC4 OFFSET(16) NUMBITS(4) [],
    /// Speed5
    DGEC5 OFFSET(20) NUMBITS(4) []
],
POWER_CONTROL [
    /// current setting for power versus speed programming
    CRS OFFSET(0) NUMBITS(4) [],
    /// AC-DC coupling selection  0 = No dc bias  1 = DC bias on vin_neg side
    DCINNEG OFFSET(4) NUMBITS(6) [],
    /// AC-DC coupling selection  0 = No dc bias  1 = DC bias on vin_pos side
    DCINPOS OFFSET(10) NUMBITS(6) [],
    /// Output data format selection  0 = offset binary  1 = two's complement
    TWOS OFFSET(16) NUMBITS(1) [],
    /// 0 = ADC is powered down 1 = ADC is active
    POWER_SWITCH OFFSET(17) NUMBITS(1) [],
    /// 0 = ADC band gap reference is powered down  1 = ADC band gap reference is active
    BGAP_SWITCH OFFSET(18) NUMBITS(1) []
],
STATUS0 [
    /// 0: number of samples in FIFO less than or equal to FIFO_LEVEL  1: number of samp
    FIFO_FULL OFFSET(0) NUMBITS(1) [],
    /// 0: FIFO is not empty   1: FIFO is empty
    FIFO_EMPTY OFFSET(1) NUMBITS(1) [],
    /// FIFO was full; conversion sample is not stored  and lost
    FIFO_OVERFLOW OFFSET(2) NUMBITS(1) [],
    /// The descriptor INTERRUPT field was enabled and its sample is converted.
    DSCR_DONE OFFSET(3) NUMBITS(1) [],
    /// The ADC was not fully woken up when a sample was converted and the conversion re
    DSCR_ERROR OFFSET(4) NUMBITS(1) [],
    /// Converted sample value was over range of the 12 bit output code.
    ADC_OVF OFFSET(5) NUMBITS(1) [],
    /// Converted sample value was under range of the 12 bit output code.
    ADC_UNF OFFSET(6) NUMBITS(1) []
],
STATUS1 [
    /// Input channel 0 result below  range
    THCMP_BRANGE0 OFFSET(0) NUMBITS(1) [],
    /// Input channel 0 result above range
    THCMP_ARANGE0 OFFSET(1) NUMBITS(1) [],
    /// Input channel 0 result  downward threshold crossing detected
    THCMP_DCROSS0 OFFSET(2) NUMBITS(1) [],
    /// Input channel 0 result upward threshold crossing detected
    THCMP_UCROSS0 OFFSET(3) NUMBITS(1) [],
    /// A new conversion on channel m completed and has overwritten the previous content
    OVERRUN_0 OFFSET(4) NUMBITS(1) [],
    /// Input channel 1 result below  range
    THCMP_BRANGE1 OFFSET(5) NUMBITS(1) [],
    /// Input channel 1 result above range
    THCMP_ARANGE1 OFFSET(6) NUMBITS(1) [],
    /// Input channel 1 result  downward threshold crossing detected
    THCMP_DCROSS1 OFFSET(7) NUMBITS(1) [],
    /// Input channel 1 result upward threshold crossing detected
    THCMP_UCROSS1 OFFSET(8) NUMBITS(1) [],
    /// A new conversion on channel m completed and has overwritten the previous content
    OVERRUN_1 OFFSET(9) NUMBITS(1) [],
    /// Input channel 2 result below  range
    THCMP_BRANGE2 OFFSET(10) NUMBITS(1) [],
    /// Input channel 2 result above range
    THCMP_ARANGE2 OFFSET(11) NUMBITS(1) [],
    /// Input channel 2 result  downward threshold crossing detected
    THCMP_DCROSS2 OFFSET(12) NUMBITS(1) [],
    /// Input channel 2 result upward threshold crossing detected
    THCMP_UCROSS2 OFFSET(13) NUMBITS(1) [],
    /// A new conversion on channel m completed and has overwritten the previous content
    OVERRUN_2 OFFSET(14) NUMBITS(1) [],
    /// Input channel 3 result below  range
    THCMP_BRANGE3 OFFSET(15) NUMBITS(1) [],
    /// Input channel 3 result above range
    THCMP_ARANGE3 OFFSET(16) NUMBITS(1) [],
    /// Input channel 3 result  downward threshold crossing detected
    THCMP_DCROSS3 OFFSET(17) NUMBITS(1) [],
    /// Input channel 3 result upward threshold crossing detected
    THCMP_UCROSS3 OFFSET(18) NUMBITS(1) [],
    /// A new conversion on channel m completed and has overwritten the previous content
    OVERRUN_3 OFFSET(19) NUMBITS(1) [],
    /// Input channel 4 result below  range
    THCMP_BRANGE4 OFFSET(20) NUMBITS(1) [],
    /// Input channel 4 result above range
    THCMP_ARANGE4 OFFSET(21) NUMBITS(1) [],
    /// Input channel 4 result  downward threshold crossing detected
    THCMP_DCROSS4 OFFSET(22) NUMBITS(1) [],
    /// Input channel 4 result upward threshold crossing detected
    THCMP_UCROSS4 OFFSET(23) NUMBITS(1) [],
    /// A new conversion on channel m completed and has overwritten the previous content
    OVERRUN_4 OFFSET(24) NUMBITS(1) [],
    /// Input channel 5 result below  range
    THCMP_BRANGE5 OFFSET(25) NUMBITS(1) [],
    /// Input channel 5 result above range
    THCMP_ARANGE5 OFFSET(26) NUMBITS(1) [],
    /// Input channel 5 result  downward threshold crossing detected
    THCMP_DCROSS5 OFFSET(27) NUMBITS(1) [],
    /// Input channel 5 result upward threshold crossing detected
    THCMP_UCROSS5 OFFSET(28) NUMBITS(1) [],
    /// A new conversion on channel m completed and has overwritten the previous content
    OVERRUN_5 OFFSET(29) NUMBITS(1) []
],
LAST_SAMPLE[0] [
    /// This bit is set to 1 when an A/D conversion on this channel completes.  This bit
    DONE OFFSET(0) NUMBITS(1) [],
    /// This bit will be set to a 1 if a new conversion on this channel completes and ov
    OVERRUN OFFSET(1) NUMBITS(1) [],
    /// Threshold Range Comparison result  00: In Range  01: Below Range  10: Above Rang
    THCMP_RANGE OFFSET(2) NUMBITS(2) [],
    /// Threshold Crossing Comparison result  00: No Threshold Crossing detected  01: Do
    THCMP_CROSS OFFSET(4) NUMBITS(2) [],
    /// 12-Bit value of last converted sample for this channel
    SAMPLE OFFSET(6) NUMBITS(12) []
],
LAST_SAMPLE[1] [
    /// This bit is set to 1 when an A/D conversion on this channel completes.  This bit
    DONE OFFSET(0) NUMBITS(1) [],
    /// This bit will be set to a 1 if a new conversion on this channel completes and ov
    OVERRUN OFFSET(1) NUMBITS(1) [],
    /// Threshold Range Comparison result  00: In Range  01: Below Range  10: Above Rang
    THCMP_RANGE OFFSET(2) NUMBITS(2) [],
    /// Threshold Crossing Comparison result  00: No Threshold Crossing detected  01: Do
    THCMP_CROSS OFFSET(4) NUMBITS(2) [],
    /// 12-Bit value of last converted sample for this channel
    SAMPLE OFFSET(6) NUMBITS(12) []
],
LAST_SAMPLE[2] [
    /// This bit is set to 1 when an A/D conversion on this channel completes.  This bit
    DONE OFFSET(0) NUMBITS(1) [],
    /// This bit will be set to a 1 if a new conversion on this channel completes and ov
    OVERRUN OFFSET(1) NUMBITS(1) [],
    /// Threshold Range Comparison result  00: In Range  01: Below Range  10: Above Rang
    THCMP_RANGE OFFSET(2) NUMBITS(2) [],
    /// Threshold Crossing Comparison result  00: No Threshold Crossing detected  01: Do
    THCMP_CROSS OFFSET(4) NUMBITS(2) [],
    /// 12-Bit value of last converted sample for this channel
    SAMPLE OFFSET(6) NUMBITS(12) []
],
LAST_SAMPLE[3] [
    /// This bit is set to 1 when an A/D conversion on this channel completes.  This bit
    DONE OFFSET(0) NUMBITS(1) [],
    /// This bit will be set to a 1 if a new conversion on this channel completes and ov
    OVERRUN OFFSET(1) NUMBITS(1) [],
    /// Threshold Range Comparison result  00: In Range  01: Below Range  10: Above Rang
    THCMP_RANGE OFFSET(2) NUMBITS(2) [],
    /// Threshold Crossing Comparison result  00: No Threshold Crossing detected  01: Do
    THCMP_CROSS OFFSET(4) NUMBITS(2) [],
    /// 12-Bit value of last converted sample for this channel
    SAMPLE OFFSET(6) NUMBITS(12) []
],
LAST_SAMPLE[4] [
    /// This bit is set to 1 when an A/D conversion on this channel completes.  This bit
    DONE OFFSET(0) NUMBITS(1) [],
    /// This bit will be set to a 1 if a new conversion on this channel completes and ov
    OVERRUN OFFSET(1) NUMBITS(1) [],
    /// Threshold Range Comparison result  00: In Range  01: Below Range  10: Above Rang
    THCMP_RANGE OFFSET(2) NUMBITS(2) [],
    /// Threshold Crossing Comparison result  00: No Threshold Crossing detected  01: Do
    THCMP_CROSS OFFSET(4) NUMBITS(2) [],
    /// 12-Bit value of last converted sample for this channel
    SAMPLE OFFSET(6) NUMBITS(12) []
],
LAST_SAMPLE[5] [
    /// This bit is set to 1 when an A/D conversion on this channel completes.  This bit
    DONE OFFSET(0) NUMBITS(1) [],
    /// This bit will be set to a 1 if a new conversion on this channel completes and ov
    OVERRUN OFFSET(1) NUMBITS(1) [],
    /// Threshold Range Comparison result  00: In Range  01: Below Range  10: Above Rang
    THCMP_RANGE OFFSET(2) NUMBITS(2) [],
    /// Threshold Crossing Comparison result  00: No Threshold Crossing detected  01: Do
    THCMP_CROSS OFFSET(4) NUMBITS(2) [],
    /// 12-Bit value of last converted sample for this channel
    SAMPLE OFFSET(6) NUMBITS(12) []
],
FIFO_OUTPUT[0] [
    /// Value of first converted sample
    SAMPLE OFFSET(0) NUMBITS(12) [],
    /// Channel number of first converted sample:  000: channel _0 or CHANNEL_ID_EN =0
    CHAN_ID OFFSET(12) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty
    EMPTY OFFSET(15) NUMBITS(1) [],
    /// Value of second converted sample.  This field is only valid if PACKED_READ is se
    SAMPLE2 OFFSET(16) NUMBITS(12) [],
    /// Channel number of second converted sample   This field is only valid if CHANNEL_
    CHAN_ID2 OFFSET(28) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty and PACKED_READ is set
    EMPTY2 OFFSET(31) NUMBITS(1) []
],
FIFO_OUTPUT[1] [
    /// Value of first converted sample
    SAMPLE OFFSET(0) NUMBITS(12) [],
    /// Channel number of first converted sample:  000: channel _0 or CHANNEL_ID_EN =0
    CHAN_ID OFFSET(12) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty
    EMPTY OFFSET(15) NUMBITS(1) [],
    /// Value of second converted sample.  This field is only valid if PACKED_READ is se
    SAMPLE2 OFFSET(16) NUMBITS(12) [],
    /// Channel number of second converted sample   This field is only valid if CHANNEL_
    CHAN_ID2 OFFSET(28) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty and PACKED_READ is set
    EMPTY2 OFFSET(31) NUMBITS(1) []
],
FIFO_OUTPUT[2] [
    /// Value of first converted sample
    SAMPLE OFFSET(0) NUMBITS(12) [],
    /// Channel number of first converted sample:  000: channel _0 or CHANNEL_ID_EN =0
    CHAN_ID OFFSET(12) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty
    EMPTY OFFSET(15) NUMBITS(1) [],
    /// Value of second converted sample.  This field is only valid if PACKED_READ is se
    SAMPLE2 OFFSET(16) NUMBITS(12) [],
    /// Channel number of second converted sample   This field is only valid if CHANNEL_
    CHAN_ID2 OFFSET(28) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty and PACKED_READ is set
    EMPTY2 OFFSET(31) NUMBITS(1) []
],
FIFO_OUTPUT[3] [
    /// Value of first converted sample
    SAMPLE OFFSET(0) NUMBITS(12) [],
    /// Channel number of first converted sample:  000: channel _0 or CHANNEL_ID_EN =0
    CHAN_ID OFFSET(12) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty
    EMPTY OFFSET(15) NUMBITS(1) [],
    /// Value of second converted sample.  This field is only valid if PACKED_READ is se
    SAMPLE2 OFFSET(16) NUMBITS(12) [],
    /// Channel number of second converted sample   This field is only valid if CHANNEL_
    CHAN_ID2 OFFSET(28) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty and PACKED_READ is set
    EMPTY2 OFFSET(31) NUMBITS(1) []
],
FIFO_OUTPUT[4] [
    /// Value of first converted sample
    SAMPLE OFFSET(0) NUMBITS(12) [],
    /// Channel number of first converted sample:  000: channel _0 or CHANNEL_ID_EN =0
    CHAN_ID OFFSET(12) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty
    EMPTY OFFSET(15) NUMBITS(1) [],
    /// Value of second converted sample.  This field is only valid if PACKED_READ is se
    SAMPLE2 OFFSET(16) NUMBITS(12) [],
    /// Channel number of second converted sample   This field is only valid if CHANNEL_
    CHAN_ID2 OFFSET(28) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty and PACKED_READ is set
    EMPTY2 OFFSET(31) NUMBITS(1) []
],
FIFO_OUTPUT[5] [
    /// Value of first converted sample
    SAMPLE OFFSET(0) NUMBITS(12) [],
    /// Channel number of first converted sample:  000: channel _0 or CHANNEL_ID_EN =0
    CHAN_ID OFFSET(12) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty
    EMPTY OFFSET(15) NUMBITS(1) [],
    /// Value of second converted sample.  This field is only valid if PACKED_READ is se
    SAMPLE2 OFFSET(16) NUMBITS(12) [],
    /// Channel number of second converted sample   This field is only valid if CHANNEL_
    CHAN_ID2 OFFSET(28) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty and PACKED_READ is set
    EMPTY2 OFFSET(31) NUMBITS(1) []
],
FIFO_OUTPUT[6] [
    /// Value of first converted sample
    SAMPLE OFFSET(0) NUMBITS(12) [],
    /// Channel number of first converted sample:  000: channel _0 or CHANNEL_ID_EN =0
    CHAN_ID OFFSET(12) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty
    EMPTY OFFSET(15) NUMBITS(1) [],
    /// Value of second converted sample.  This field is only valid if PACKED_READ is se
    SAMPLE2 OFFSET(16) NUMBITS(12) [],
    /// Channel number of second converted sample   This field is only valid if CHANNEL_
    CHAN_ID2 OFFSET(28) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty and PACKED_READ is set
    EMPTY2 OFFSET(31) NUMBITS(1) []
],
FIFO_OUTPUT[7] [
    /// Value of first converted sample
    SAMPLE OFFSET(0) NUMBITS(12) [],
    /// Channel number of first converted sample:  000: channel _0 or CHANNEL_ID_EN =0
    CHAN_ID OFFSET(12) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty
    EMPTY OFFSET(15) NUMBITS(1) [],
    /// Value of second converted sample.  This field is only valid if PACKED_READ is se
    SAMPLE2 OFFSET(16) NUMBITS(12) [],
    /// Channel number of second converted sample   This field is only valid if CHANNEL_
    CHAN_ID2 OFFSET(28) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty and PACKED_READ is set
    EMPTY2 OFFSET(31) NUMBITS(1) []
],
FIFO_OUTPUT[8] [
    /// Value of first converted sample
    SAMPLE OFFSET(0) NUMBITS(12) [],
    /// Channel number of first converted sample:  000: channel _0 or CHANNEL_ID_EN =0
    CHAN_ID OFFSET(12) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty
    EMPTY OFFSET(15) NUMBITS(1) [],
    /// Value of second converted sample.  This field is only valid if PACKED_READ is se
    SAMPLE2 OFFSET(16) NUMBITS(12) [],
    /// Channel number of second converted sample   This field is only valid if CHANNEL_
    CHAN_ID2 OFFSET(28) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty and PACKED_READ is set
    EMPTY2 OFFSET(31) NUMBITS(1) []
],
FIFO_OUTPUT[9] [
    /// Value of first converted sample
    SAMPLE OFFSET(0) NUMBITS(12) [],
    /// Channel number of first converted sample:  000: channel _0 or CHANNEL_ID_EN =0
    CHAN_ID OFFSET(12) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty
    EMPTY OFFSET(15) NUMBITS(1) [],
    /// Value of second converted sample.  This field is only valid if PACKED_READ is se
    SAMPLE2 OFFSET(16) NUMBITS(12) [],
    /// Channel number of second converted sample   This field is only valid if CHANNEL_
    CHAN_ID2 OFFSET(28) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty and PACKED_READ is set
    EMPTY2 OFFSET(31) NUMBITS(1) []
],
FIFO_OUTPUT[10] [
    /// Value of first converted sample
    SAMPLE OFFSET(0) NUMBITS(12) [],
    /// Channel number of first converted sample:  000: channel _0 or CHANNEL_ID_EN =0
    CHAN_ID OFFSET(12) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty
    EMPTY OFFSET(15) NUMBITS(1) [],
    /// Value of second converted sample.  This field is only valid if PACKED_READ is se
    SAMPLE2 OFFSET(16) NUMBITS(12) [],
    /// Channel number of second converted sample   This field is only valid if CHANNEL_
    CHAN_ID2 OFFSET(28) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty and PACKED_READ is set
    EMPTY2 OFFSET(31) NUMBITS(1) []
],
FIFO_OUTPUT[11] [
    /// Value of first converted sample
    SAMPLE OFFSET(0) NUMBITS(12) [],
    /// Channel number of first converted sample:  000: channel _0 or CHANNEL_ID_EN =0
    CHAN_ID OFFSET(12) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty
    EMPTY OFFSET(15) NUMBITS(1) [],
    /// Value of second converted sample.  This field is only valid if PACKED_READ is se
    SAMPLE2 OFFSET(16) NUMBITS(12) [],
    /// Channel number of second converted sample   This field is only valid if CHANNEL_
    CHAN_ID2 OFFSET(28) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty and PACKED_READ is set
    EMPTY2 OFFSET(31) NUMBITS(1) []
],
FIFO_OUTPUT[12] [
    /// Value of first converted sample
    SAMPLE OFFSET(0) NUMBITS(12) [],
    /// Channel number of first converted sample:  000: channel _0 or CHANNEL_ID_EN =0
    CHAN_ID OFFSET(12) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty
    EMPTY OFFSET(15) NUMBITS(1) [],
    /// Value of second converted sample.  This field is only valid if PACKED_READ is se
    SAMPLE2 OFFSET(16) NUMBITS(12) [],
    /// Channel number of second converted sample   This field is only valid if CHANNEL_
    CHAN_ID2 OFFSET(28) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty and PACKED_READ is set
    EMPTY2 OFFSET(31) NUMBITS(1) []
],
FIFO_OUTPUT[13] [
    /// Value of first converted sample
    SAMPLE OFFSET(0) NUMBITS(12) [],
    /// Channel number of first converted sample:  000: channel _0 or CHANNEL_ID_EN =0
    CHAN_ID OFFSET(12) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty
    EMPTY OFFSET(15) NUMBITS(1) [],
    /// Value of second converted sample.  This field is only valid if PACKED_READ is se
    SAMPLE2 OFFSET(16) NUMBITS(12) [],
    /// Channel number of second converted sample   This field is only valid if CHANNEL_
    CHAN_ID2 OFFSET(28) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty and PACKED_READ is set
    EMPTY2 OFFSET(31) NUMBITS(1) []
],
FIFO_OUTPUT[14] [
    /// Value of first converted sample
    SAMPLE OFFSET(0) NUMBITS(12) [],
    /// Channel number of first converted sample:  000: channel _0 or CHANNEL_ID_EN =0
    CHAN_ID OFFSET(12) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty
    EMPTY OFFSET(15) NUMBITS(1) [],
    /// Value of second converted sample.  This field is only valid if PACKED_READ is se
    SAMPLE2 OFFSET(16) NUMBITS(12) [],
    /// Channel number of second converted sample   This field is only valid if CHANNEL_
    CHAN_ID2 OFFSET(28) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty and PACKED_READ is set
    EMPTY2 OFFSET(31) NUMBITS(1) []
],
FIFO_OUTPUT[15] [
    /// Value of first converted sample
    SAMPLE OFFSET(0) NUMBITS(12) [],
    /// Channel number of first converted sample:  000: channel _0 or CHANNEL_ID_EN =0
    CHAN_ID OFFSET(12) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty
    EMPTY OFFSET(15) NUMBITS(1) [],
    /// Value of second converted sample.  This field is only valid if PACKED_READ is se
    SAMPLE2 OFFSET(16) NUMBITS(12) [],
    /// Channel number of second converted sample   This field is only valid if CHANNEL_
    CHAN_ID2 OFFSET(28) NUMBITS(3) [],
    /// 0: FIFO not empty  1: FIFO empty and PACKED_READ is set
    EMPTY2 OFFSET(31) NUMBITS(1) []
],
DESCRIPTOR0_[0] [
    /// 0: convert input  0  1: convert input  1  2: convert input  2  3: convert input
    CHANNEL_NR OFFSET(0) NUMBITS(3) [],
    /// 0: After this descriptor continue with the next descriptor.  1: halt after this
    HALT OFFSET(3) NUMBITS(1) [],
    /// 1: Raise interrupt when ADC result is available
    INTERRUPT OFFSET(4) NUMBITS(1) [],
    /// 1: Power down after this conversion.
    POWER_DOWN OFFSET(5) NUMBITS(1) [],
    /// 00: Continue with next descriptor (wraps around after top).  01: Branch to the f
    BRANCH OFFSET(6) NUMBITS(2) [],
    /// Evaluate this descriptor when descriptor timer value is equal to match value.
    MATCH_VALUE OFFSET(8) NUMBITS(14) [],
    /// Indicates which threshold comparison level register set is to be used:  00: no c
    THRESHOLD_SEL OFFSET(22) NUMBITS(2) [],
    /// 1: reset descriptor timer.
    RESET_TIMER OFFSET(24) NUMBITS(1) [],
    /// 1: Update table with all 8 descriptors of this table. Descriptors of this table
    UPDATE_TABLE OFFSET(31) NUMBITS(1) []
],
DESCRIPTOR0_[1] [
    /// 0: convert input  0  1: convert input  1  2: convert input  2  3: convert input
    CHANNEL_NR OFFSET(0) NUMBITS(3) [],
    /// 0: After this descriptor continue with the next descriptor.  1: halt after this
    HALT OFFSET(3) NUMBITS(1) [],
    /// 1: Raise interrupt when ADC result is available
    INTERRUPT OFFSET(4) NUMBITS(1) [],
    /// 1: Power down after this conversion.
    POWER_DOWN OFFSET(5) NUMBITS(1) [],
    /// 00: Continue with next descriptor (wraps around after top).  01: Branch to the f
    BRANCH OFFSET(6) NUMBITS(2) [],
    /// Evaluate this descriptor when descriptor timer value is equal to match value.
    MATCH_VALUE OFFSET(8) NUMBITS(14) [],
    /// Indicates which threshold comparison level register set is to be used:  00: no c
    THRESHOLD_SEL OFFSET(22) NUMBITS(2) [],
    /// 1: reset descriptor timer.
    RESET_TIMER OFFSET(24) NUMBITS(1) [],
    /// 1: Update table with all 8 descriptors of this table. Descriptors of this table
    UPDATE_TABLE OFFSET(31) NUMBITS(1) []
],
DESCRIPTOR0_[2] [
    /// 0: convert input  0  1: convert input  1  2: convert input  2  3: convert input
    CHANNEL_NR OFFSET(0) NUMBITS(3) [],
    /// 0: After this descriptor continue with the next descriptor.  1: halt after this
    HALT OFFSET(3) NUMBITS(1) [],
    /// 1: Raise interrupt when ADC result is available
    INTERRUPT OFFSET(4) NUMBITS(1) [],
    /// 1: Power down after this conversion.
    POWER_DOWN OFFSET(5) NUMBITS(1) [],
    /// 00: Continue with next descriptor (wraps around after top).  01: Branch to the f
    BRANCH OFFSET(6) NUMBITS(2) [],
    /// Evaluate this descriptor when descriptor timer value is equal to match value.
    MATCH_VALUE OFFSET(8) NUMBITS(14) [],
    /// Indicates which threshold comparison level register set is to be used:  00: no c
    THRESHOLD_SEL OFFSET(22) NUMBITS(2) [],
    /// 1: reset descriptor timer.
    RESET_TIMER OFFSET(24) NUMBITS(1) [],
    /// 1: Update table with all 8 descriptors of this table. Descriptors of this table
    UPDATE_TABLE OFFSET(31) NUMBITS(1) []
],
DESCRIPTOR0_[3] [
    /// 0: convert input  0  1: convert input  1  2: convert input  2  3: convert input
    CHANNEL_NR OFFSET(0) NUMBITS(3) [],
    /// 0: After this descriptor continue with the next descriptor.  1: halt after this
    HALT OFFSET(3) NUMBITS(1) [],
    /// 1: Raise interrupt when ADC result is available
    INTERRUPT OFFSET(4) NUMBITS(1) [],
    /// 1: Power down after this conversion.
    POWER_DOWN OFFSET(5) NUMBITS(1) [],
    /// 00: Continue with next descriptor (wraps around after top).  01: Branch to the f
    BRANCH OFFSET(6) NUMBITS(2) [],
    /// Evaluate this descriptor when descriptor timer value is equal to match value.
    MATCH_VALUE OFFSET(8) NUMBITS(14) [],
    /// Indicates which threshold comparison level register set is to be used:  00: no c
    THRESHOLD_SEL OFFSET(22) NUMBITS(2) [],
    /// 1: reset descriptor timer.
    RESET_TIMER OFFSET(24) NUMBITS(1) [],
    /// 1: Update table with all 8 descriptors of this table. Descriptors of this table
    UPDATE_TABLE OFFSET(31) NUMBITS(1) []
],
DESCRIPTOR0_[4] [
    /// 0: convert input  0  1: convert input  1  2: convert input  2  3: convert input
    CHANNEL_NR OFFSET(0) NUMBITS(3) [],
    /// 0: After this descriptor continue with the next descriptor.  1: halt after this
    HALT OFFSET(3) NUMBITS(1) [],
    /// 1: Raise interrupt when ADC result is available
    INTERRUPT OFFSET(4) NUMBITS(1) [],
    /// 1: Power down after this conversion.
    POWER_DOWN OFFSET(5) NUMBITS(1) [],
    /// 00: Continue with next descriptor (wraps around after top).  01: Branch to the f
    BRANCH OFFSET(6) NUMBITS(2) [],
    /// Evaluate this descriptor when descriptor timer value is equal to match value.
    MATCH_VALUE OFFSET(8) NUMBITS(14) [],
    /// Indicates which threshold comparison level register set is to be used:  00: no c
    THRESHOLD_SEL OFFSET(22) NUMBITS(2) [],
    /// 1: reset descriptor timer.
    RESET_TIMER OFFSET(24) NUMBITS(1) [],
    /// 1: Update table with all 8 descriptors of this table. Descriptors of this table
    UPDATE_TABLE OFFSET(31) NUMBITS(1) []
],
DESCRIPTOR0_[5] [
    /// 0: convert input  0  1: convert input  1  2: convert input  2  3: convert input
    CHANNEL_NR OFFSET(0) NUMBITS(3) [],
    /// 0: After this descriptor continue with the next descriptor.  1: halt after this
    HALT OFFSET(3) NUMBITS(1) [],
    /// 1: Raise interrupt when ADC result is available
    INTERRUPT OFFSET(4) NUMBITS(1) [],
    /// 1: Power down after this conversion.
    POWER_DOWN OFFSET(5) NUMBITS(1) [],
    /// 00: Continue with next descriptor (wraps around after top).  01: Branch to the f
    BRANCH OFFSET(6) NUMBITS(2) [],
    /// Evaluate this descriptor when descriptor timer value is equal to match value.
    MATCH_VALUE OFFSET(8) NUMBITS(14) [],
    /// Indicates which threshold comparison level register set is to be used:  00: no c
    THRESHOLD_SEL OFFSET(22) NUMBITS(2) [],
    /// 1: reset descriptor timer.
    RESET_TIMER OFFSET(24) NUMBITS(1) [],
    /// 1: Update table with all 8 descriptors of this table. Descriptors of this table
    UPDATE_TABLE OFFSET(31) NUMBITS(1) []
],
DESCRIPTOR0_[6] [
    /// 0: convert input  0  1: convert input  1  2: convert input  2  3: convert input
    CHANNEL_NR OFFSET(0) NUMBITS(3) [],
    /// 0: After this descriptor continue with the next descriptor.  1: halt after this
    HALT OFFSET(3) NUMBITS(1) [],
    /// 1: Raise interrupt when ADC result is available
    INTERRUPT OFFSET(4) NUMBITS(1) [],
    /// 1: Power down after this conversion.
    POWER_DOWN OFFSET(5) NUMBITS(1) [],
    /// 00: Continue with next descriptor (wraps around after top).  01: Branch to the f
    BRANCH OFFSET(6) NUMBITS(2) [],
    /// Evaluate this descriptor when descriptor timer value is equal to match value.
    MATCH_VALUE OFFSET(8) NUMBITS(14) [],
    /// Indicates which threshold comparison level register set is to be used:  00: no c
    THRESHOLD_SEL OFFSET(22) NUMBITS(2) [],
    /// 1: reset descriptor timer.
    RESET_TIMER OFFSET(24) NUMBITS(1) [],
    /// 1: Update table with all 8 descriptors of this table. Descriptors of this table
    UPDATE_TABLE OFFSET(31) NUMBITS(1) []
],
DESCRIPTOR0_[7] [
    /// 0: convert input  0  1: convert input  1  2: convert input  2  3: convert input
    CHANNEL_NR OFFSET(0) NUMBITS(3) [],
    /// 0: After this descriptor continue with the next descriptor.  1: halt after this
    HALT OFFSET(3) NUMBITS(1) [],
    /// 1: Raise interrupt when ADC result is available
    INTERRUPT OFFSET(4) NUMBITS(1) [],
    /// 1: Power down after this conversion.
    POWER_DOWN OFFSET(5) NUMBITS(1) [],
    /// 00: Continue with next descriptor (wraps around after top).  01: Branch to the f
    BRANCH OFFSET(6) NUMBITS(2) [],
    /// Evaluate this descriptor when descriptor timer value is equal to match value.
    MATCH_VALUE OFFSET(8) NUMBITS(14) [],
    /// Indicates which threshold comparison level register set is to be used:  00: no c
    THRESHOLD_SEL OFFSET(22) NUMBITS(2) [],
    /// 1: reset descriptor timer.
    RESET_TIMER OFFSET(24) NUMBITS(1) [],
    /// 1: Update table with all 8 descriptors of this table. Descriptors of this table
    UPDATE_TABLE OFFSET(31) NUMBITS(1) []
],
DESCRIPTOR1_[0] [
    /// 0: convert input 0  1: convert input 1  2: convert input 2  3: convert input 3
    CHANNEL_NR OFFSET(0) NUMBITS(3) [],
    /// 0: After this descriptor continue with the next descriptor.  1: halt after this
    HALT OFFSET(3) NUMBITS(1) [],
    /// 1: Raise interrupt when ADC result is available
    INTERRUPT OFFSET(4) NUMBITS(1) [],
    /// 1: Power down after this conversion.
    POWER_DOWN OFFSET(5) NUMBITS(1) [],
    /// 00: Continue with next descriptor (wraps around after top).  01: Branch to the f
    BRANCH OFFSET(6) NUMBITS(2) [],
    /// Evaluate this descriptor when descriptor timer value is equal to match value.
    MATCH_VALUE OFFSET(8) NUMBITS(14) [],
    /// Indicates which threshold comparison level register set is to be used:  00: no c
    THRESHOLD_SEL OFFSET(22) NUMBITS(2) [],
    /// 1: reset descriptor timer.
    RESET_TIMER OFFSET(24) NUMBITS(1) [],
    /// 1: Update table with all 8 descriptors of this table. Descriptors of this table
    UPDATE_TABLE OFFSET(31) NUMBITS(1) []
],
DESCRIPTOR1_[1] [
    /// 0: convert input 0  1: convert input 1  2: convert input 2  3: convert input 3
    CHANNEL_NR OFFSET(0) NUMBITS(3) [],
    /// 0: After this descriptor continue with the next descriptor.  1: halt after this
    HALT OFFSET(3) NUMBITS(1) [],
    /// 1: Raise interrupt when ADC result is available
    INTERRUPT OFFSET(4) NUMBITS(1) [],
    /// 1: Power down after this conversion.
    POWER_DOWN OFFSET(5) NUMBITS(1) [],
    /// 00: Continue with next descriptor (wraps around after top).  01: Branch to the f
    BRANCH OFFSET(6) NUMBITS(2) [],
    /// Evaluate this descriptor when descriptor timer value is equal to match value.
    MATCH_VALUE OFFSET(8) NUMBITS(14) [],
    /// Indicates which threshold comparison level register set is to be used:  00: no c
    THRESHOLD_SEL OFFSET(22) NUMBITS(2) [],
    /// 1: reset descriptor timer.
    RESET_TIMER OFFSET(24) NUMBITS(1) [],
    /// 1: Update table with all 8 descriptors of this table. Descriptors of this table
    UPDATE_TABLE OFFSET(31) NUMBITS(1) []
],
DESCRIPTOR1_[2] [
    /// 0: convert input 0  1: convert input 1  2: convert input 2  3: convert input 3
    CHANNEL_NR OFFSET(0) NUMBITS(3) [],
    /// 0: After this descriptor continue with the next descriptor.  1: halt after this
    HALT OFFSET(3) NUMBITS(1) [],
    /// 1: Raise interrupt when ADC result is available
    INTERRUPT OFFSET(4) NUMBITS(1) [],
    /// 1: Power down after this conversion.
    POWER_DOWN OFFSET(5) NUMBITS(1) [],
    /// 00: Continue with next descriptor (wraps around after top).  01: Branch to the f
    BRANCH OFFSET(6) NUMBITS(2) [],
    /// Evaluate this descriptor when descriptor timer value is equal to match value.
    MATCH_VALUE OFFSET(8) NUMBITS(14) [],
    /// Indicates which threshold comparison level register set is to be used:  00: no c
    THRESHOLD_SEL OFFSET(22) NUMBITS(2) [],
    /// 1: reset descriptor timer.
    RESET_TIMER OFFSET(24) NUMBITS(1) [],
    /// 1: Update table with all 8 descriptors of this table. Descriptors of this table
    UPDATE_TABLE OFFSET(31) NUMBITS(1) []
],
DESCRIPTOR1_[3] [
    /// 0: convert input 0  1: convert input 1  2: convert input 2  3: convert input 3
    CHANNEL_NR OFFSET(0) NUMBITS(3) [],
    /// 0: After this descriptor continue with the next descriptor.  1: halt after this
    HALT OFFSET(3) NUMBITS(1) [],
    /// 1: Raise interrupt when ADC result is available
    INTERRUPT OFFSET(4) NUMBITS(1) [],
    /// 1: Power down after this conversion.
    POWER_DOWN OFFSET(5) NUMBITS(1) [],
    /// 00: Continue with next descriptor (wraps around after top).  01: Branch to the f
    BRANCH OFFSET(6) NUMBITS(2) [],
    /// Evaluate this descriptor when descriptor timer value is equal to match value.
    MATCH_VALUE OFFSET(8) NUMBITS(14) [],
    /// Indicates which threshold comparison level register set is to be used:  00: no c
    THRESHOLD_SEL OFFSET(22) NUMBITS(2) [],
    /// 1: reset descriptor timer.
    RESET_TIMER OFFSET(24) NUMBITS(1) [],
    /// 1: Update table with all 8 descriptors of this table. Descriptors of this table
    UPDATE_TABLE OFFSET(31) NUMBITS(1) []
],
DESCRIPTOR1_[4] [
    /// 0: convert input 0  1: convert input 1  2: convert input 2  3: convert input 3
    CHANNEL_NR OFFSET(0) NUMBITS(3) [],
    /// 0: After this descriptor continue with the next descriptor.  1: halt after this
    HALT OFFSET(3) NUMBITS(1) [],
    /// 1: Raise interrupt when ADC result is available
    INTERRUPT OFFSET(4) NUMBITS(1) [],
    /// 1: Power down after this conversion.
    POWER_DOWN OFFSET(5) NUMBITS(1) [],
    /// 00: Continue with next descriptor (wraps around after top).  01: Branch to the f
    BRANCH OFFSET(6) NUMBITS(2) [],
    /// Evaluate this descriptor when descriptor timer value is equal to match value.
    MATCH_VALUE OFFSET(8) NUMBITS(14) [],
    /// Indicates which threshold comparison level register set is to be used:  00: no c
    THRESHOLD_SEL OFFSET(22) NUMBITS(2) [],
    /// 1: reset descriptor timer.
    RESET_TIMER OFFSET(24) NUMBITS(1) [],
    /// 1: Update table with all 8 descriptors of this table. Descriptors of this table
    UPDATE_TABLE OFFSET(31) NUMBITS(1) []
],
DESCRIPTOR1_[5] [
    /// 0: convert input 0  1: convert input 1  2: convert input 2  3: convert input 3
    CHANNEL_NR OFFSET(0) NUMBITS(3) [],
    /// 0: After this descriptor continue with the next descriptor.  1: halt after this
    HALT OFFSET(3) NUMBITS(1) [],
    /// 1: Raise interrupt when ADC result is available
    INTERRUPT OFFSET(4) NUMBITS(1) [],
    /// 1: Power down after this conversion.
    POWER_DOWN OFFSET(5) NUMBITS(1) [],
    /// 00: Continue with next descriptor (wraps around after top).  01: Branch to the f
    BRANCH OFFSET(6) NUMBITS(2) [],
    /// Evaluate this descriptor when descriptor timer value is equal to match value.
    MATCH_VALUE OFFSET(8) NUMBITS(14) [],
    /// Indicates which threshold comparison level register set is to be used:  00: no c
    THRESHOLD_SEL OFFSET(22) NUMBITS(2) [],
    /// 1: reset descriptor timer.
    RESET_TIMER OFFSET(24) NUMBITS(1) [],
    /// 1: Update table with all 8 descriptors of this table. Descriptors of this table
    UPDATE_TABLE OFFSET(31) NUMBITS(1) []
],
DESCRIPTOR1_[6] [
    /// 0: convert input 0  1: convert input 1  2: convert input 2  3: convert input 3
    CHANNEL_NR OFFSET(0) NUMBITS(3) [],
    /// 0: After this descriptor continue with the next descriptor.  1: halt after this
    HALT OFFSET(3) NUMBITS(1) [],
    /// 1: Raise interrupt when ADC result is available
    INTERRUPT OFFSET(4) NUMBITS(1) [],
    /// 1: Power down after this conversion.
    POWER_DOWN OFFSET(5) NUMBITS(1) [],
    /// 00: Continue with next descriptor (wraps around after top).  01: Branch to the f
    BRANCH OFFSET(6) NUMBITS(2) [],
    /// Evaluate this descriptor when descriptor timer value is equal to match value.
    MATCH_VALUE OFFSET(8) NUMBITS(14) [],
    /// Indicates which threshold comparison level register set is to be used:  00: no c
    THRESHOLD_SEL OFFSET(22) NUMBITS(2) [],
    /// 1: reset descriptor timer.
    RESET_TIMER OFFSET(24) NUMBITS(1) [],
    /// 1: Update table with all 8 descriptors of this table. Descriptors of this table
    UPDATE_TABLE OFFSET(31) NUMBITS(1) []
],
DESCRIPTOR1_[7] [
    /// 0: convert input 0  1: convert input 1  2: convert input 2  3: convert input 3
    CHANNEL_NR OFFSET(0) NUMBITS(3) [],
    /// 0: After this descriptor continue with the next descriptor.  1: halt after this
    HALT OFFSET(3) NUMBITS(1) [],
    /// 1: Raise interrupt when ADC result is available
    INTERRUPT OFFSET(4) NUMBITS(1) [],
    /// 1: Power down after this conversion.
    POWER_DOWN OFFSET(5) NUMBITS(1) [],
    /// 00: Continue with next descriptor (wraps around after top).  01: Branch to the f
    BRANCH OFFSET(6) NUMBITS(2) [],
    /// Evaluate this descriptor when descriptor timer value is equal to match value.
    MATCH_VALUE OFFSET(8) NUMBITS(14) [],
    /// Indicates which threshold comparison level register set is to be used:  00: no c
    THRESHOLD_SEL OFFSET(22) NUMBITS(2) [],
    /// 1: reset descriptor timer.
    RESET_TIMER OFFSET(24) NUMBITS(1) [],
    /// 1: Update table with all 8 descriptors of this table. Descriptors of this table
    UPDATE_TABLE OFFSET(31) NUMBITS(1) []
]
];
const ADCHS_BASE: StaticRef<AdchsRegisters> =
    unsafe { StaticRef::new(0x400F0000 as *const AdchsRegisters) };
