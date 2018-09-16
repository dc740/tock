
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// RTC REGFILE
#[repr(C)]
struct RegfileRegisters {
/// General purpose storage register
regfile_0: ReadWrite<u32>,
/// General purpose storage register
regfile_1: ReadWrite<u32>,
/// General purpose storage register
regfile_2: ReadWrite<u32>,
/// General purpose storage register
regfile_3: ReadWrite<u32>,
/// General purpose storage register
regfile_4: ReadWrite<u32>,
/// General purpose storage register
regfile_5: ReadWrite<u32>,
/// General purpose storage register
regfile_6: ReadWrite<u32>,
/// General purpose storage register
regfile_7: ReadWrite<u32>,
/// General purpose storage register
regfile_8: ReadWrite<u32>,
/// General purpose storage register
regfile_9: ReadWrite<u32>,
/// General purpose storage register
regfile_10: ReadWrite<u32>,
/// General purpose storage register
regfile_11: ReadWrite<u32>,
/// General purpose storage register
regfile_12: ReadWrite<u32>,
/// General purpose storage register
regfile_13: ReadWrite<u32>,
/// General purpose storage register
regfile_14: ReadWrite<u32>,
/// General purpose storage register
regfile_15: ReadWrite<u32>,
/// General purpose storage register
regfile_16: ReadWrite<u32>,
/// General purpose storage register
regfile_17: ReadWrite<u32>,
/// General purpose storage register
regfile_18: ReadWrite<u32>,
/// General purpose storage register
regfile_19: ReadWrite<u32>,
/// General purpose storage register
regfile_20: ReadWrite<u32>,
/// General purpose storage register
regfile_21: ReadWrite<u32>,
/// General purpose storage register
regfile_22: ReadWrite<u32>,
/// General purpose storage register
regfile_23: ReadWrite<u32>,
/// General purpose storage register
regfile_24: ReadWrite<u32>,
/// General purpose storage register
regfile_25: ReadWrite<u32>,
/// General purpose storage register
regfile_26: ReadWrite<u32>,
/// General purpose storage register
regfile_27: ReadWrite<u32>,
/// General purpose storage register
regfile_28: ReadWrite<u32>,
/// General purpose storage register
regfile_29: ReadWrite<u32>,
/// General purpose storage register
regfile_30: ReadWrite<u32>,
/// General purpose storage register
regfile_31: ReadWrite<u32>,
/// General purpose storage register
regfile_32: ReadWrite<u32>,
/// General purpose storage register
regfile_33: ReadWrite<u32>,
/// General purpose storage register
regfile_34: ReadWrite<u32>,
/// General purpose storage register
regfile_35: ReadWrite<u32>,
/// General purpose storage register
regfile_36: ReadWrite<u32>,
/// General purpose storage register
regfile_37: ReadWrite<u32>,
/// General purpose storage register
regfile_38: ReadWrite<u32>,
/// General purpose storage register
regfile_39: ReadWrite<u32>,
/// General purpose storage register
regfile_40: ReadWrite<u32>,
/// General purpose storage register
regfile_41: ReadWrite<u32>,
/// General purpose storage register
regfile_42: ReadWrite<u32>,
/// General purpose storage register
regfile_43: ReadWrite<u32>,
/// General purpose storage register
regfile_44: ReadWrite<u32>,
/// General purpose storage register
regfile_45: ReadWrite<u32>,
/// General purpose storage register
regfile_46: ReadWrite<u32>,
/// General purpose storage register
regfile_47: ReadWrite<u32>,
/// General purpose storage register
regfile_48: ReadWrite<u32>,
/// General purpose storage register
regfile_49: ReadWrite<u32>,
/// General purpose storage register
regfile_50: ReadWrite<u32>,
/// General purpose storage register
regfile_51: ReadWrite<u32>,
/// General purpose storage register
regfile_52: ReadWrite<u32>,
/// General purpose storage register
regfile_53: ReadWrite<u32>,
/// General purpose storage register
regfile_54: ReadWrite<u32>,
/// General purpose storage register
regfile_55: ReadWrite<u32>,
/// General purpose storage register
regfile_56: ReadWrite<u32>,
/// General purpose storage register
regfile_57: ReadWrite<u32>,
/// General purpose storage register
regfile_58: ReadWrite<u32>,
/// General purpose storage register
regfile_59: ReadWrite<u32>,
/// General purpose storage register
regfile_60: ReadWrite<u32>,
/// General purpose storage register
regfile_61: ReadWrite<u32>,
/// General purpose storage register
regfile_62: ReadWrite<u32>,
/// General purpose storage register
regfile_63: ReadWrite<u32>,
}

const REGFILE_BASE: StaticRef<RegfileRegisters> =
    unsafe { StaticRef::new(0x40041000 as *const RegfileRegisters) };
