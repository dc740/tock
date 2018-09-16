
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// GPIO port
#[repr(C)]
struct Gpio_PortRegisters {
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_0: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_1: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_2: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_3: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_4: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_5: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_6: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_7: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_8: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_9: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_10: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_11: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_12: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_13: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_14: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_15: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_16: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_17: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_18: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_19: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_20: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_21: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_22: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_23: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_24: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_25: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_26: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_27: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_28: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_29: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_30: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_31: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_32: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_33: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_34: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_35: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_36: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_37: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_38: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_39: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_40: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_41: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_42: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_43: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_44: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_45: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_46: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_47: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_48: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_49: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_50: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_51: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_52: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_53: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_54: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_55: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_56: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_57: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_58: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_59: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_60: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_61: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_62: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_63: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_64: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_65: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_66: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_67: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_68: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_69: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_70: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_71: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_72: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_73: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_74: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_75: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_76: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_77: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_78: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_79: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_80: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_81: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_82: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_83: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_84: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_85: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_86: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_87: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_88: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_89: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_90: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_91: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_92: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_93: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_94: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_95: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_96: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_97: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_98: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_99: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_100: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_101: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_102: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_103: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_104: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_105: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_106: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_107: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_108: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_109: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_110: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_111: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_112: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_113: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_114: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_115: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_116: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_117: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_118: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_119: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_120: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_121: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_122: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_123: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_124: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_125: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_126: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_127: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_128: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_129: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_130: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_131: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_132: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_133: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_134: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_135: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_136: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_137: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_138: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_139: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_140: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_141: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_142: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_143: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_144: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_145: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_146: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_147: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_148: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_149: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_150: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_151: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_152: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_153: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_154: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_155: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_156: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_157: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_158: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_159: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_160: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_161: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_162: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_163: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_164: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_165: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_166: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_167: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_168: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_169: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_170: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_171: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_172: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_173: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_174: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_175: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_176: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_177: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_178: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_179: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_180: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_181: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_182: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_183: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_184: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_185: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_186: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_187: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_188: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_189: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_190: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_191: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_192: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_193: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_194: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_195: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_196: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_197: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_198: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_199: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_200: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_201: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_202: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_203: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_204: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_205: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_206: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_207: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_208: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_209: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_210: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_211: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_212: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_213: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_214: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_215: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_216: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_217: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_218: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_219: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_220: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_221: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_222: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_223: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_224: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_225: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_226: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_227: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_228: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_229: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_230: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_231: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_232: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_233: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_234: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_235: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_236: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_237: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_238: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_239: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_240: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_241: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_242: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_243: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_244: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_245: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_246: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_247: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_248: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_249: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_250: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_251: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_252: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_253: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_254: ReadWrite<u8>,
/// Byte pin registers port 0 to 5; pins PIOn_0 to PIOn_31
b_255: ReadWrite<u8>,
_reserved0: [u8; 3840],
/// Word pin registers port 0 to 5
w_0: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_1: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_2: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_3: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_4: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_5: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_6: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_7: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_8: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_9: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_10: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_11: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_12: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_13: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_14: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_15: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_16: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_17: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_18: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_19: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_20: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_21: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_22: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_23: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_24: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_25: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_26: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_27: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_28: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_29: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_30: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_31: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_32: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_33: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_34: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_35: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_36: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_37: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_38: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_39: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_40: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_41: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_42: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_43: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_44: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_45: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_46: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_47: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_48: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_49: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_50: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_51: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_52: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_53: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_54: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_55: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_56: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_57: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_58: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_59: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_60: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_61: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_62: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_63: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_64: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_65: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_66: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_67: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_68: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_69: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_70: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_71: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_72: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_73: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_74: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_75: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_76: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_77: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_78: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_79: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_80: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_81: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_82: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_83: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_84: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_85: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_86: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_87: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_88: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_89: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_90: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_91: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_92: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_93: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_94: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_95: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_96: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_97: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_98: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_99: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_100: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_101: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_102: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_103: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_104: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_105: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_106: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_107: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_108: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_109: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_110: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_111: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_112: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_113: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_114: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_115: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_116: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_117: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_118: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_119: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_120: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_121: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_122: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_123: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_124: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_125: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_126: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_127: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_128: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_129: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_130: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_131: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_132: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_133: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_134: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_135: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_136: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_137: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_138: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_139: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_140: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_141: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_142: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_143: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_144: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_145: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_146: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_147: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_148: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_149: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_150: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_151: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_152: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_153: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_154: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_155: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_156: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_157: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_158: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_159: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_160: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_161: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_162: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_163: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_164: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_165: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_166: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_167: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_168: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_169: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_170: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_171: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_172: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_173: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_174: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_175: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_176: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_177: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_178: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_179: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_180: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_181: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_182: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_183: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_184: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_185: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_186: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_187: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_188: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_189: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_190: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_191: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_192: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_193: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_194: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_195: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_196: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_197: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_198: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_199: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_200: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_201: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_202: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_203: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_204: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_205: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_206: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_207: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_208: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_209: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_210: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_211: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_212: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_213: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_214: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_215: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_216: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_217: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_218: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_219: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_220: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_221: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_222: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_223: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_224: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_225: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_226: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_227: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_228: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_229: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_230: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_231: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_232: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_233: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_234: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_235: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_236: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_237: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_238: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_239: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_240: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_241: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_242: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_243: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_244: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_245: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_246: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_247: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_248: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_249: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_250: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_251: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_252: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_253: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_254: ReadWrite<u32>,
/// Word pin registers port 0 to 5
w_255: ReadWrite<u32>,
_reserved1: [u8; 3072],
/// Direction registers port m
dir0: ReadWrite<u32, DIR0::Register>,
/// Direction registers port m
dir1: ReadWrite<u32, DIR1::Register>,
/// Direction registers port m
dir2: ReadWrite<u32, DIR2::Register>,
/// Direction registers port m
dir3: ReadWrite<u32, DIR3::Register>,
/// Direction registers port m
dir4: ReadWrite<u32, DIR4::Register>,
/// Direction registers port m
dir5: ReadWrite<u32, DIR5::Register>,
/// Direction registers port m
dir6: ReadWrite<u32, DIR6::Register>,
/// Direction registers port m
dir7: ReadWrite<u32, DIR7::Register>,
_reserved2: [u8; 96],
/// Mask register port m
mask0: ReadWrite<u32, MASK0::Register>,
/// Mask register port m
mask1: ReadWrite<u32, MASK1::Register>,
/// Mask register port m
mask2: ReadWrite<u32, MASK2::Register>,
/// Mask register port m
mask3: ReadWrite<u32, MASK3::Register>,
/// Mask register port m
mask4: ReadWrite<u32, MASK4::Register>,
/// Mask register port m
mask5: ReadWrite<u32, MASK5::Register>,
/// Mask register port m
mask6: ReadWrite<u32, MASK6::Register>,
/// Mask register port m
mask7: ReadWrite<u32, MASK7::Register>,
_reserved3: [u8; 96],
/// Port pin register port m
pin0: ReadWrite<u32, PIN0::Register>,
/// Port pin register port m
pin1: ReadWrite<u32, PIN1::Register>,
/// Port pin register port m
pin2: ReadWrite<u32, PIN2::Register>,
/// Port pin register port m
pin3: ReadWrite<u32, PIN3::Register>,
/// Port pin register port m
pin4: ReadWrite<u32, PIN4::Register>,
/// Port pin register port m
pin5: ReadWrite<u32, PIN5::Register>,
/// Port pin register port m
pin6: ReadWrite<u32, PIN6::Register>,
/// Port pin register port m
pin7: ReadWrite<u32, PIN7::Register>,
_reserved4: [u8; 96],
/// Masked port register port m
mpin0: ReadWrite<u32, MPIN0::Register>,
/// Masked port register port m
mpin1: ReadWrite<u32, MPIN1::Register>,
/// Masked port register port m
mpin2: ReadWrite<u32, MPIN2::Register>,
/// Masked port register port m
mpin3: ReadWrite<u32, MPIN3::Register>,
/// Masked port register port m
mpin4: ReadWrite<u32, MPIN4::Register>,
/// Masked port register port m
mpin5: ReadWrite<u32, MPIN5::Register>,
/// Masked port register port m
mpin6: ReadWrite<u32, MPIN6::Register>,
/// Masked port register port m
mpin7: ReadWrite<u32, MPIN7::Register>,
_reserved5: [u8; 96],
/// Write: Set register for port m  Read: output bits for port m
set0: ReadWrite<u32, SET0::Register>,
/// Write: Set register for port m  Read: output bits for port m
set1: ReadWrite<u32, SET1::Register>,
/// Write: Set register for port m  Read: output bits for port m
set2: ReadWrite<u32, SET2::Register>,
/// Write: Set register for port m  Read: output bits for port m
set3: ReadWrite<u32, SET3::Register>,
/// Write: Set register for port m  Read: output bits for port m
set4: ReadWrite<u32, SET4::Register>,
/// Write: Set register for port m  Read: output bits for port m
set5: ReadWrite<u32, SET5::Register>,
/// Write: Set register for port m  Read: output bits for port m
set6: ReadWrite<u32, SET6::Register>,
/// Write: Set register for port m  Read: output bits for port m
set7: ReadWrite<u32, SET7::Register>,
_reserved6: [u8; 96],
/// Clear port m
clr0: WriteOnly<u32, CLR0::Register>,
/// Clear port m
clr1: WriteOnly<u32, CLR1::Register>,
/// Clear port m
clr2: WriteOnly<u32, CLR2::Register>,
/// Clear port m
clr3: WriteOnly<u32, CLR3::Register>,
/// Clear port m
clr4: WriteOnly<u32, CLR4::Register>,
/// Clear port m
clr5: WriteOnly<u32, CLR5::Register>,
/// Clear port m
clr6: WriteOnly<u32, CLR6::Register>,
/// Clear port m
clr7: WriteOnly<u32, CLR7::Register>,
_reserved7: [u8; 96],
/// Toggle port m
not0: WriteOnly<u32, NOT0::Register>,
/// Toggle port m
not1: WriteOnly<u32, NOT1::Register>,
/// Toggle port m
not2: WriteOnly<u32, NOT2::Register>,
/// Toggle port m
not3: WriteOnly<u32, NOT3::Register>,
/// Toggle port m
not4: WriteOnly<u32, NOT4::Register>,
/// Toggle port m
not5: WriteOnly<u32, NOT5::Register>,
/// Toggle port m
not6: WriteOnly<u32, NOT6::Register>,
/// Toggle port m
not7: WriteOnly<u32, NOT7::Register>,
}
register_bitfields![u32,
DIR0 [
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP0 OFFSET(0) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP1 OFFSET(1) NUMBITS(1) [],
    /// Selects pin direction for  GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...
    DIRP2 OFFSET(2) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP3 OFFSET(3) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP4 OFFSET(4) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP5 OFFSET(5) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP6 OFFSET(6) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP7 OFFSET(7) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP8 OFFSET(8) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP9 OFFSET(9) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP10 OFFSET(10) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP11 OFFSET(11) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP12 OFFSET(12) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP13 OFFSET(13) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP14 OFFSET(14) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP15 OFFSET(15) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP16 OFFSET(16) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP17 OFFSET(17) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP18 OFFSET(18) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP19 OFFSET(19) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP20 OFFSET(20) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP21 OFFSET(21) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP22 OFFSET(22) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP23 OFFSET(23) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP24 OFFSET(24) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP25 OFFSET(25) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP26 OFFSET(26) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP27 OFFSET(27) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP28 OFFSET(28) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP29 OFFSET(29) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP30 OFFSET(30) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP31 OFFSET(31) NUMBITS(1) []
],
DIR1 [
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP0 OFFSET(0) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP1 OFFSET(1) NUMBITS(1) [],
    /// Selects pin direction for  GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...
    DIRP2 OFFSET(2) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP3 OFFSET(3) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP4 OFFSET(4) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP5 OFFSET(5) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP6 OFFSET(6) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP7 OFFSET(7) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP8 OFFSET(8) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP9 OFFSET(9) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP10 OFFSET(10) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP11 OFFSET(11) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP12 OFFSET(12) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP13 OFFSET(13) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP14 OFFSET(14) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP15 OFFSET(15) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP16 OFFSET(16) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP17 OFFSET(17) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP18 OFFSET(18) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP19 OFFSET(19) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP20 OFFSET(20) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP21 OFFSET(21) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP22 OFFSET(22) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP23 OFFSET(23) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP24 OFFSET(24) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP25 OFFSET(25) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP26 OFFSET(26) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP27 OFFSET(27) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP28 OFFSET(28) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP29 OFFSET(29) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP30 OFFSET(30) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP31 OFFSET(31) NUMBITS(1) []
],
DIR2 [
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP0 OFFSET(0) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP1 OFFSET(1) NUMBITS(1) [],
    /// Selects pin direction for  GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...
    DIRP2 OFFSET(2) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP3 OFFSET(3) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP4 OFFSET(4) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP5 OFFSET(5) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP6 OFFSET(6) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP7 OFFSET(7) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP8 OFFSET(8) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP9 OFFSET(9) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP10 OFFSET(10) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP11 OFFSET(11) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP12 OFFSET(12) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP13 OFFSET(13) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP14 OFFSET(14) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP15 OFFSET(15) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP16 OFFSET(16) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP17 OFFSET(17) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP18 OFFSET(18) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP19 OFFSET(19) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP20 OFFSET(20) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP21 OFFSET(21) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP22 OFFSET(22) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP23 OFFSET(23) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP24 OFFSET(24) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP25 OFFSET(25) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP26 OFFSET(26) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP27 OFFSET(27) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP28 OFFSET(28) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP29 OFFSET(29) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP30 OFFSET(30) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP31 OFFSET(31) NUMBITS(1) []
],
DIR3 [
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP0 OFFSET(0) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP1 OFFSET(1) NUMBITS(1) [],
    /// Selects pin direction for  GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...
    DIRP2 OFFSET(2) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP3 OFFSET(3) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP4 OFFSET(4) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP5 OFFSET(5) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP6 OFFSET(6) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP7 OFFSET(7) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP8 OFFSET(8) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP9 OFFSET(9) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP10 OFFSET(10) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP11 OFFSET(11) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP12 OFFSET(12) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP13 OFFSET(13) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP14 OFFSET(14) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP15 OFFSET(15) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP16 OFFSET(16) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP17 OFFSET(17) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP18 OFFSET(18) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP19 OFFSET(19) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP20 OFFSET(20) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP21 OFFSET(21) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP22 OFFSET(22) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP23 OFFSET(23) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP24 OFFSET(24) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP25 OFFSET(25) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP26 OFFSET(26) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP27 OFFSET(27) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP28 OFFSET(28) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP29 OFFSET(29) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP30 OFFSET(30) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP31 OFFSET(31) NUMBITS(1) []
],
DIR4 [
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP0 OFFSET(0) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP1 OFFSET(1) NUMBITS(1) [],
    /// Selects pin direction for  GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...
    DIRP2 OFFSET(2) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP3 OFFSET(3) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP4 OFFSET(4) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP5 OFFSET(5) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP6 OFFSET(6) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP7 OFFSET(7) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP8 OFFSET(8) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP9 OFFSET(9) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP10 OFFSET(10) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP11 OFFSET(11) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP12 OFFSET(12) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP13 OFFSET(13) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP14 OFFSET(14) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP15 OFFSET(15) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP16 OFFSET(16) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP17 OFFSET(17) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP18 OFFSET(18) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP19 OFFSET(19) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP20 OFFSET(20) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP21 OFFSET(21) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP22 OFFSET(22) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP23 OFFSET(23) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP24 OFFSET(24) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP25 OFFSET(25) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP26 OFFSET(26) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP27 OFFSET(27) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP28 OFFSET(28) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP29 OFFSET(29) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP30 OFFSET(30) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP31 OFFSET(31) NUMBITS(1) []
],
DIR5 [
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP0 OFFSET(0) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP1 OFFSET(1) NUMBITS(1) [],
    /// Selects pin direction for  GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...
    DIRP2 OFFSET(2) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP3 OFFSET(3) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP4 OFFSET(4) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP5 OFFSET(5) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP6 OFFSET(6) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP7 OFFSET(7) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP8 OFFSET(8) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP9 OFFSET(9) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP10 OFFSET(10) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP11 OFFSET(11) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP12 OFFSET(12) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP13 OFFSET(13) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP14 OFFSET(14) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP15 OFFSET(15) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP16 OFFSET(16) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP17 OFFSET(17) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP18 OFFSET(18) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP19 OFFSET(19) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP20 OFFSET(20) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP21 OFFSET(21) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP22 OFFSET(22) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP23 OFFSET(23) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP24 OFFSET(24) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP25 OFFSET(25) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP26 OFFSET(26) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP27 OFFSET(27) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP28 OFFSET(28) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP29 OFFSET(29) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP30 OFFSET(30) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP31 OFFSET(31) NUMBITS(1) []
],
DIR6 [
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP0 OFFSET(0) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP1 OFFSET(1) NUMBITS(1) [],
    /// Selects pin direction for  GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...
    DIRP2 OFFSET(2) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP3 OFFSET(3) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP4 OFFSET(4) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP5 OFFSET(5) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP6 OFFSET(6) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP7 OFFSET(7) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP8 OFFSET(8) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP9 OFFSET(9) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP10 OFFSET(10) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP11 OFFSET(11) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP12 OFFSET(12) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP13 OFFSET(13) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP14 OFFSET(14) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP15 OFFSET(15) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP16 OFFSET(16) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP17 OFFSET(17) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP18 OFFSET(18) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP19 OFFSET(19) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP20 OFFSET(20) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP21 OFFSET(21) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP22 OFFSET(22) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP23 OFFSET(23) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP24 OFFSET(24) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP25 OFFSET(25) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP26 OFFSET(26) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP27 OFFSET(27) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP28 OFFSET(28) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP29 OFFSET(29) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP30 OFFSET(30) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP31 OFFSET(31) NUMBITS(1) []
],
DIR7 [
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP0 OFFSET(0) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP1 OFFSET(1) NUMBITS(1) [],
    /// Selects pin direction for  GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...
    DIRP2 OFFSET(2) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP3 OFFSET(3) NUMBITS(1) [],
    /// Selects pin direction for GPIOm[n] pin (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP4 OFFSET(4) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP5 OFFSET(5) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP6 OFFSET(6) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP7 OFFSET(7) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP8 OFFSET(8) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP9 OFFSET(9) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP10 OFFSET(10) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP11 OFFSET(11) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP12 OFFSET(12) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP13 OFFSET(13) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP14 OFFSET(14) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP15 OFFSET(15) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP16 OFFSET(16) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP17 OFFSET(17) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP18 OFFSET(18) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP19 OFFSET(19) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP20 OFFSET(20) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP21 OFFSET(21) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP22 OFFSET(22) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP23 OFFSET(23) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP24 OFFSET(24) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP25 OFFSET(25) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP26 OFFSET(26) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP27 OFFSET(27) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP28 OFFSET(28) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP29 OFFSET(29) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP30 OFFSET(30) NUMBITS(1) [],
    /// Selects pin direction for pin GPIOm[n] (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    DIRP31 OFFSET(31) NUMBITS(1) []
],
MASK0 [
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP0 OFFSET(0) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP1 OFFSET(1) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP2 OFFSET(2) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the PIN register (bi
    MASKP3 OFFSET(3) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the P0/1PIN register
    MASKP4 OFFSET(4) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP5 OFFSET(5) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP6 OFFSET(6) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP7 OFFSET(7) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP8 OFFSET(8) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP9 OFFSET(9) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP10 OFFSET(10) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP11 OFFSET(11) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP12 OFFSET(12) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP13 OFFSET(13) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP14 OFFSET(14) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP15 OFFSET(15) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP16 OFFSET(16) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP17 OFFSET(17) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP18 OFFSET(18) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP19 OFFSET(19) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP20 OFFSET(20) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP21 OFFSET(21) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP22 OFFSET(22) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP23 OFFSET(23) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP24 OFFSET(24) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP25 OFFSET(25) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP26 OFFSET(26) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP27 OFFSET(27) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP28 OFFSET(28) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP29 OFFSET(29) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP30 OFFSET(30) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP31 OFFSET(31) NUMBITS(1) []
],
MASK1 [
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP0 OFFSET(0) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP1 OFFSET(1) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP2 OFFSET(2) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the PIN register (bi
    MASKP3 OFFSET(3) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the P0/1PIN register
    MASKP4 OFFSET(4) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP5 OFFSET(5) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP6 OFFSET(6) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP7 OFFSET(7) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP8 OFFSET(8) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP9 OFFSET(9) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP10 OFFSET(10) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP11 OFFSET(11) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP12 OFFSET(12) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP13 OFFSET(13) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP14 OFFSET(14) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP15 OFFSET(15) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP16 OFFSET(16) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP17 OFFSET(17) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP18 OFFSET(18) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP19 OFFSET(19) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP20 OFFSET(20) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP21 OFFSET(21) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP22 OFFSET(22) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP23 OFFSET(23) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP24 OFFSET(24) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP25 OFFSET(25) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP26 OFFSET(26) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP27 OFFSET(27) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP28 OFFSET(28) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP29 OFFSET(29) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP30 OFFSET(30) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP31 OFFSET(31) NUMBITS(1) []
],
MASK2 [
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP0 OFFSET(0) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP1 OFFSET(1) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP2 OFFSET(2) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the PIN register (bi
    MASKP3 OFFSET(3) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the P0/1PIN register
    MASKP4 OFFSET(4) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP5 OFFSET(5) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP6 OFFSET(6) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP7 OFFSET(7) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP8 OFFSET(8) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP9 OFFSET(9) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP10 OFFSET(10) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP11 OFFSET(11) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP12 OFFSET(12) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP13 OFFSET(13) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP14 OFFSET(14) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP15 OFFSET(15) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP16 OFFSET(16) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP17 OFFSET(17) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP18 OFFSET(18) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP19 OFFSET(19) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP20 OFFSET(20) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP21 OFFSET(21) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP22 OFFSET(22) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP23 OFFSET(23) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP24 OFFSET(24) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP25 OFFSET(25) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP26 OFFSET(26) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP27 OFFSET(27) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP28 OFFSET(28) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP29 OFFSET(29) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP30 OFFSET(30) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP31 OFFSET(31) NUMBITS(1) []
],
MASK3 [
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP0 OFFSET(0) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP1 OFFSET(1) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP2 OFFSET(2) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the PIN register (bi
    MASKP3 OFFSET(3) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the P0/1PIN register
    MASKP4 OFFSET(4) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP5 OFFSET(5) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP6 OFFSET(6) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP7 OFFSET(7) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP8 OFFSET(8) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP9 OFFSET(9) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP10 OFFSET(10) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP11 OFFSET(11) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP12 OFFSET(12) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP13 OFFSET(13) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP14 OFFSET(14) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP15 OFFSET(15) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP16 OFFSET(16) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP17 OFFSET(17) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP18 OFFSET(18) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP19 OFFSET(19) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP20 OFFSET(20) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP21 OFFSET(21) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP22 OFFSET(22) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP23 OFFSET(23) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP24 OFFSET(24) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP25 OFFSET(25) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP26 OFFSET(26) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP27 OFFSET(27) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP28 OFFSET(28) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP29 OFFSET(29) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP30 OFFSET(30) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP31 OFFSET(31) NUMBITS(1) []
],
MASK4 [
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP0 OFFSET(0) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP1 OFFSET(1) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP2 OFFSET(2) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the PIN register (bi
    MASKP3 OFFSET(3) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the P0/1PIN register
    MASKP4 OFFSET(4) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP5 OFFSET(5) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP6 OFFSET(6) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP7 OFFSET(7) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP8 OFFSET(8) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP9 OFFSET(9) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP10 OFFSET(10) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP11 OFFSET(11) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP12 OFFSET(12) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP13 OFFSET(13) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP14 OFFSET(14) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP15 OFFSET(15) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP16 OFFSET(16) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP17 OFFSET(17) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP18 OFFSET(18) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP19 OFFSET(19) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP20 OFFSET(20) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP21 OFFSET(21) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP22 OFFSET(22) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP23 OFFSET(23) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP24 OFFSET(24) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP25 OFFSET(25) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP26 OFFSET(26) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP27 OFFSET(27) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP28 OFFSET(28) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP29 OFFSET(29) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP30 OFFSET(30) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP31 OFFSET(31) NUMBITS(1) []
],
MASK5 [
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP0 OFFSET(0) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP1 OFFSET(1) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP2 OFFSET(2) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the PIN register (bi
    MASKP3 OFFSET(3) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the P0/1PIN register
    MASKP4 OFFSET(4) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP5 OFFSET(5) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP6 OFFSET(6) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP7 OFFSET(7) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP8 OFFSET(8) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP9 OFFSET(9) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP10 OFFSET(10) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP11 OFFSET(11) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP12 OFFSET(12) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP13 OFFSET(13) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP14 OFFSET(14) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP15 OFFSET(15) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP16 OFFSET(16) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP17 OFFSET(17) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP18 OFFSET(18) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP19 OFFSET(19) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP20 OFFSET(20) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP21 OFFSET(21) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP22 OFFSET(22) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP23 OFFSET(23) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP24 OFFSET(24) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP25 OFFSET(25) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP26 OFFSET(26) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP27 OFFSET(27) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP28 OFFSET(28) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP29 OFFSET(29) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP30 OFFSET(30) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP31 OFFSET(31) NUMBITS(1) []
],
MASK6 [
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP0 OFFSET(0) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP1 OFFSET(1) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP2 OFFSET(2) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the PIN register (bi
    MASKP3 OFFSET(3) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the P0/1PIN register
    MASKP4 OFFSET(4) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP5 OFFSET(5) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP6 OFFSET(6) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP7 OFFSET(7) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP8 OFFSET(8) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP9 OFFSET(9) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP10 OFFSET(10) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP11 OFFSET(11) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP12 OFFSET(12) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP13 OFFSET(13) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP14 OFFSET(14) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP15 OFFSET(15) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP16 OFFSET(16) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP17 OFFSET(17) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP18 OFFSET(18) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP19 OFFSET(19) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP20 OFFSET(20) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP21 OFFSET(21) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP22 OFFSET(22) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP23 OFFSET(23) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP24 OFFSET(24) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP25 OFFSET(25) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP26 OFFSET(26) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP27 OFFSET(27) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP28 OFFSET(28) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP29 OFFSET(29) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP30 OFFSET(30) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP31 OFFSET(31) NUMBITS(1) []
],
MASK7 [
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP0 OFFSET(0) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP1 OFFSET(1) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP2 OFFSET(2) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the PIN register (bi
    MASKP3 OFFSET(3) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the P0/1PIN register
    MASKP4 OFFSET(4) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP5 OFFSET(5) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP6 OFFSET(6) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP7 OFFSET(7) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP8 OFFSET(8) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP9 OFFSET(9) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP10 OFFSET(10) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP11 OFFSET(11) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP12 OFFSET(12) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP13 OFFSET(13) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP14 OFFSET(14) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP15 OFFSET(15) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP16 OFFSET(16) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP17 OFFSET(17) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP18 OFFSET(18) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP19 OFFSET(19) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP20 OFFSET(20) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP21 OFFSET(21) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP22 OFFSET(22) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP23 OFFSET(23) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP24 OFFSET(24) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP25 OFFSET(25) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP26 OFFSET(26) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP27 OFFSET(27) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP28 OFFSET(28) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP29 OFFSET(29) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP30 OFFSET(30) NUMBITS(1) [],
    /// Controls which bits corresponding to GPIOm[n] are active in the  PIN register (b
    MASKP31 OFFSET(31) NUMBITS(1) []
],
PIN0 [
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT0 OFFSET(0) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT1 OFFSET(1) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT2 OFFSET(2) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT3 OFFSET(3) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT4 OFFSET(4) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT5 OFFSET(5) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT6 OFFSET(6) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT7 OFFSET(7) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT8 OFFSET(8) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT9 OFFSET(9) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT10 OFFSET(10) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT11 OFFSET(11) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT12 OFFSET(12) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT13 OFFSET(13) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT14 OFFSET(14) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT15 OFFSET(15) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT16 OFFSET(16) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT17 OFFSET(17) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT18 OFFSET(18) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT19 OFFSET(19) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT20 OFFSET(20) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT21 OFFSET(21) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT22 OFFSET(22) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT23 OFFSET(23) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT24 OFFSET(24) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT25 OFFSET(25) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT26 OFFSET(26) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT27 OFFSET(27) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT28 OFFSET(28) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT29 OFFSET(29) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT30 OFFSET(30) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT31 OFFSET(31) NUMBITS(1) []
],
PIN1 [
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT0 OFFSET(0) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT1 OFFSET(1) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT2 OFFSET(2) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT3 OFFSET(3) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT4 OFFSET(4) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT5 OFFSET(5) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT6 OFFSET(6) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT7 OFFSET(7) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT8 OFFSET(8) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT9 OFFSET(9) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT10 OFFSET(10) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT11 OFFSET(11) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT12 OFFSET(12) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT13 OFFSET(13) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT14 OFFSET(14) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT15 OFFSET(15) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT16 OFFSET(16) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT17 OFFSET(17) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT18 OFFSET(18) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT19 OFFSET(19) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT20 OFFSET(20) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT21 OFFSET(21) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT22 OFFSET(22) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT23 OFFSET(23) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT24 OFFSET(24) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT25 OFFSET(25) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT26 OFFSET(26) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT27 OFFSET(27) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT28 OFFSET(28) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT29 OFFSET(29) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT30 OFFSET(30) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT31 OFFSET(31) NUMBITS(1) []
],
PIN2 [
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT0 OFFSET(0) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT1 OFFSET(1) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT2 OFFSET(2) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT3 OFFSET(3) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT4 OFFSET(4) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT5 OFFSET(5) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT6 OFFSET(6) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT7 OFFSET(7) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT8 OFFSET(8) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT9 OFFSET(9) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT10 OFFSET(10) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT11 OFFSET(11) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT12 OFFSET(12) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT13 OFFSET(13) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT14 OFFSET(14) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT15 OFFSET(15) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT16 OFFSET(16) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT17 OFFSET(17) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT18 OFFSET(18) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT19 OFFSET(19) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT20 OFFSET(20) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT21 OFFSET(21) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT22 OFFSET(22) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT23 OFFSET(23) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT24 OFFSET(24) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT25 OFFSET(25) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT26 OFFSET(26) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT27 OFFSET(27) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT28 OFFSET(28) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT29 OFFSET(29) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT30 OFFSET(30) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT31 OFFSET(31) NUMBITS(1) []
],
PIN3 [
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT0 OFFSET(0) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT1 OFFSET(1) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT2 OFFSET(2) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT3 OFFSET(3) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT4 OFFSET(4) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT5 OFFSET(5) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT6 OFFSET(6) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT7 OFFSET(7) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT8 OFFSET(8) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT9 OFFSET(9) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT10 OFFSET(10) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT11 OFFSET(11) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT12 OFFSET(12) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT13 OFFSET(13) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT14 OFFSET(14) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT15 OFFSET(15) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT16 OFFSET(16) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT17 OFFSET(17) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT18 OFFSET(18) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT19 OFFSET(19) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT20 OFFSET(20) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT21 OFFSET(21) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT22 OFFSET(22) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT23 OFFSET(23) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT24 OFFSET(24) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT25 OFFSET(25) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT26 OFFSET(26) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT27 OFFSET(27) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT28 OFFSET(28) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT29 OFFSET(29) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT30 OFFSET(30) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT31 OFFSET(31) NUMBITS(1) []
],
PIN4 [
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT0 OFFSET(0) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT1 OFFSET(1) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT2 OFFSET(2) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT3 OFFSET(3) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT4 OFFSET(4) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT5 OFFSET(5) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT6 OFFSET(6) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT7 OFFSET(7) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT8 OFFSET(8) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT9 OFFSET(9) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT10 OFFSET(10) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT11 OFFSET(11) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT12 OFFSET(12) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT13 OFFSET(13) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT14 OFFSET(14) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT15 OFFSET(15) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT16 OFFSET(16) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT17 OFFSET(17) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT18 OFFSET(18) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT19 OFFSET(19) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT20 OFFSET(20) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT21 OFFSET(21) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT22 OFFSET(22) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT23 OFFSET(23) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT24 OFFSET(24) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT25 OFFSET(25) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT26 OFFSET(26) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT27 OFFSET(27) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT28 OFFSET(28) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT29 OFFSET(29) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT30 OFFSET(30) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT31 OFFSET(31) NUMBITS(1) []
],
PIN5 [
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT0 OFFSET(0) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT1 OFFSET(1) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT2 OFFSET(2) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT3 OFFSET(3) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT4 OFFSET(4) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT5 OFFSET(5) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT6 OFFSET(6) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT7 OFFSET(7) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT8 OFFSET(8) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT9 OFFSET(9) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT10 OFFSET(10) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT11 OFFSET(11) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT12 OFFSET(12) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT13 OFFSET(13) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT14 OFFSET(14) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT15 OFFSET(15) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT16 OFFSET(16) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT17 OFFSET(17) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT18 OFFSET(18) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT19 OFFSET(19) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT20 OFFSET(20) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT21 OFFSET(21) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT22 OFFSET(22) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT23 OFFSET(23) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT24 OFFSET(24) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT25 OFFSET(25) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT26 OFFSET(26) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT27 OFFSET(27) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT28 OFFSET(28) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT29 OFFSET(29) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT30 OFFSET(30) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT31 OFFSET(31) NUMBITS(1) []
],
PIN6 [
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT0 OFFSET(0) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT1 OFFSET(1) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT2 OFFSET(2) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT3 OFFSET(3) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT4 OFFSET(4) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT5 OFFSET(5) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT6 OFFSET(6) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT7 OFFSET(7) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT8 OFFSET(8) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT9 OFFSET(9) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT10 OFFSET(10) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT11 OFFSET(11) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT12 OFFSET(12) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT13 OFFSET(13) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT14 OFFSET(14) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT15 OFFSET(15) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT16 OFFSET(16) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT17 OFFSET(17) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT18 OFFSET(18) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT19 OFFSET(19) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT20 OFFSET(20) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT21 OFFSET(21) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT22 OFFSET(22) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT23 OFFSET(23) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT24 OFFSET(24) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT25 OFFSET(25) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT26 OFFSET(26) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT27 OFFSET(27) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT28 OFFSET(28) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT29 OFFSET(29) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT30 OFFSET(30) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT31 OFFSET(31) NUMBITS(1) []
],
PIN7 [
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT0 OFFSET(0) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT1 OFFSET(1) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT2 OFFSET(2) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT3 OFFSET(3) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT4 OFFSET(4) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT5 OFFSET(5) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT6 OFFSET(6) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT7 OFFSET(7) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT8 OFFSET(8) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT9 OFFSET(9) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT10 OFFSET(10) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT11 OFFSET(11) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT12 OFFSET(12) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT13 OFFSET(13) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT14 OFFSET(14) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT15 OFFSET(15) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT16 OFFSET(16) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT17 OFFSET(17) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT18 OFFSET(18) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT19 OFFSET(19) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT20 OFFSET(20) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT21 OFFSET(21) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT22 OFFSET(22) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT23 OFFSET(23) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT24 OFFSET(24) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT25 OFFSET(25) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT26 OFFSET(26) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT27 OFFSET(27) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT28 OFFSET(28) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT29 OFFSET(29) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT30 OFFSET(30) NUMBITS(1) [],
    /// Reads pin states or loads output bits (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ...,
    PORT31 OFFSET(31) NUMBITS(1) []
],
MPIN0 [
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP0 OFFSET(0) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP1 OFFSET(1) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP2 OFFSET(2) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP3 OFFSET(3) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP4 OFFSET(4) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP5 OFFSET(5) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP6 OFFSET(6) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP7 OFFSET(7) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP8 OFFSET(8) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP9 OFFSET(9) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP10 OFFSET(10) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP11 OFFSET(11) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP12 OFFSET(12) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP13 OFFSET(13) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP14 OFFSET(14) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP15 OFFSET(15) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP16 OFFSET(16) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP17 OFFSET(17) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP18 OFFSET(18) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP19 OFFSET(19) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP20 OFFSET(20) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP21 OFFSET(21) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP22 OFFSET(22) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP23 OFFSET(23) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP24 OFFSET(24) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP25 OFFSET(25) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP26 OFFSET(26) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP27 OFFSET(27) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP28 OFFSET(28) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP29 OFFSET(29) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP30 OFFSET(30) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP31 OFFSET(31) NUMBITS(1) []
],
MPIN1 [
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP0 OFFSET(0) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP1 OFFSET(1) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP2 OFFSET(2) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP3 OFFSET(3) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP4 OFFSET(4) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP5 OFFSET(5) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP6 OFFSET(6) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP7 OFFSET(7) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP8 OFFSET(8) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP9 OFFSET(9) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP10 OFFSET(10) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP11 OFFSET(11) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP12 OFFSET(12) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP13 OFFSET(13) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP14 OFFSET(14) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP15 OFFSET(15) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP16 OFFSET(16) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP17 OFFSET(17) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP18 OFFSET(18) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP19 OFFSET(19) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP20 OFFSET(20) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP21 OFFSET(21) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP22 OFFSET(22) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP23 OFFSET(23) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP24 OFFSET(24) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP25 OFFSET(25) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP26 OFFSET(26) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP27 OFFSET(27) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP28 OFFSET(28) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP29 OFFSET(29) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP30 OFFSET(30) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP31 OFFSET(31) NUMBITS(1) []
],
MPIN2 [
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP0 OFFSET(0) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP1 OFFSET(1) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP2 OFFSET(2) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP3 OFFSET(3) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP4 OFFSET(4) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP5 OFFSET(5) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP6 OFFSET(6) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP7 OFFSET(7) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP8 OFFSET(8) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP9 OFFSET(9) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP10 OFFSET(10) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP11 OFFSET(11) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP12 OFFSET(12) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP13 OFFSET(13) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP14 OFFSET(14) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP15 OFFSET(15) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP16 OFFSET(16) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP17 OFFSET(17) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP18 OFFSET(18) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP19 OFFSET(19) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP20 OFFSET(20) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP21 OFFSET(21) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP22 OFFSET(22) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP23 OFFSET(23) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP24 OFFSET(24) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP25 OFFSET(25) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP26 OFFSET(26) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP27 OFFSET(27) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP28 OFFSET(28) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP29 OFFSET(29) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP30 OFFSET(30) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP31 OFFSET(31) NUMBITS(1) []
],
MPIN3 [
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP0 OFFSET(0) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP1 OFFSET(1) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP2 OFFSET(2) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP3 OFFSET(3) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP4 OFFSET(4) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP5 OFFSET(5) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP6 OFFSET(6) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP7 OFFSET(7) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP8 OFFSET(8) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP9 OFFSET(9) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP10 OFFSET(10) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP11 OFFSET(11) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP12 OFFSET(12) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP13 OFFSET(13) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP14 OFFSET(14) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP15 OFFSET(15) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP16 OFFSET(16) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP17 OFFSET(17) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP18 OFFSET(18) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP19 OFFSET(19) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP20 OFFSET(20) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP21 OFFSET(21) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP22 OFFSET(22) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP23 OFFSET(23) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP24 OFFSET(24) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP25 OFFSET(25) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP26 OFFSET(26) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP27 OFFSET(27) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP28 OFFSET(28) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP29 OFFSET(29) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP30 OFFSET(30) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP31 OFFSET(31) NUMBITS(1) []
],
MPIN4 [
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP0 OFFSET(0) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP1 OFFSET(1) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP2 OFFSET(2) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP3 OFFSET(3) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP4 OFFSET(4) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP5 OFFSET(5) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP6 OFFSET(6) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP7 OFFSET(7) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP8 OFFSET(8) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP9 OFFSET(9) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP10 OFFSET(10) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP11 OFFSET(11) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP12 OFFSET(12) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP13 OFFSET(13) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP14 OFFSET(14) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP15 OFFSET(15) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP16 OFFSET(16) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP17 OFFSET(17) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP18 OFFSET(18) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP19 OFFSET(19) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP20 OFFSET(20) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP21 OFFSET(21) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP22 OFFSET(22) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP23 OFFSET(23) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP24 OFFSET(24) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP25 OFFSET(25) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP26 OFFSET(26) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP27 OFFSET(27) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP28 OFFSET(28) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP29 OFFSET(29) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP30 OFFSET(30) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP31 OFFSET(31) NUMBITS(1) []
],
MPIN5 [
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP0 OFFSET(0) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP1 OFFSET(1) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP2 OFFSET(2) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP3 OFFSET(3) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP4 OFFSET(4) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP5 OFFSET(5) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP6 OFFSET(6) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP7 OFFSET(7) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP8 OFFSET(8) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP9 OFFSET(9) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP10 OFFSET(10) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP11 OFFSET(11) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP12 OFFSET(12) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP13 OFFSET(13) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP14 OFFSET(14) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP15 OFFSET(15) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP16 OFFSET(16) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP17 OFFSET(17) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP18 OFFSET(18) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP19 OFFSET(19) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP20 OFFSET(20) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP21 OFFSET(21) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP22 OFFSET(22) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP23 OFFSET(23) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP24 OFFSET(24) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP25 OFFSET(25) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP26 OFFSET(26) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP27 OFFSET(27) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP28 OFFSET(28) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP29 OFFSET(29) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP30 OFFSET(30) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP31 OFFSET(31) NUMBITS(1) []
],
MPIN6 [
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP0 OFFSET(0) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP1 OFFSET(1) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP2 OFFSET(2) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP3 OFFSET(3) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP4 OFFSET(4) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP5 OFFSET(5) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP6 OFFSET(6) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP7 OFFSET(7) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP8 OFFSET(8) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP9 OFFSET(9) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP10 OFFSET(10) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP11 OFFSET(11) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP12 OFFSET(12) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP13 OFFSET(13) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP14 OFFSET(14) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP15 OFFSET(15) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP16 OFFSET(16) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP17 OFFSET(17) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP18 OFFSET(18) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP19 OFFSET(19) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP20 OFFSET(20) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP21 OFFSET(21) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP22 OFFSET(22) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP23 OFFSET(23) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP24 OFFSET(24) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP25 OFFSET(25) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP26 OFFSET(26) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP27 OFFSET(27) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP28 OFFSET(28) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP29 OFFSET(29) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP30 OFFSET(30) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP31 OFFSET(31) NUMBITS(1) []
],
MPIN7 [
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP0 OFFSET(0) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP1 OFFSET(1) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP2 OFFSET(2) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP3 OFFSET(3) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP4 OFFSET(4) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP5 OFFSET(5) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP6 OFFSET(6) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP7 OFFSET(7) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP8 OFFSET(8) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP9 OFFSET(9) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP10 OFFSET(10) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP11 OFFSET(11) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP12 OFFSET(12) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP13 OFFSET(13) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP14 OFFSET(14) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP15 OFFSET(15) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP16 OFFSET(16) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP17 OFFSET(17) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP18 OFFSET(18) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP19 OFFSET(19) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP20 OFFSET(20) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP21 OFFSET(21) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP22 OFFSET(22) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP23 OFFSET(23) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP24 OFFSET(24) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP25 OFFSET(25) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP26 OFFSET(26) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP27 OFFSET(27) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP28 OFFSET(28) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP29 OFFSET(29) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP30 OFFSET(30) NUMBITS(1) [],
    /// Masked port register (bit 0 = GPIOm[0], bit 1 = GPIOm[1], ..., bit 31 = GPIOm[31
    MPORTP31 OFFSET(31) NUMBITS(1) []
],
SET0 [
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP0 OFFSET(0) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP1 OFFSET(1) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP2 OFFSET(2) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP3 OFFSET(3) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP4 OFFSET(4) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP5 OFFSET(5) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP6 OFFSET(6) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP7 OFFSET(7) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP8 OFFSET(8) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP9 OFFSET(9) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP10 OFFSET(10) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP11 OFFSET(11) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP12 OFFSET(12) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP13 OFFSET(13) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP14 OFFSET(14) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP15 OFFSET(15) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP16 OFFSET(16) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP17 OFFSET(17) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP18 OFFSET(18) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP19 OFFSET(19) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP20 OFFSET(20) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP21 OFFSET(21) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP22 OFFSET(22) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP23 OFFSET(23) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP24 OFFSET(24) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP25 OFFSET(25) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP26 OFFSET(26) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP27 OFFSET(27) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP28 OFFSET(28) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP29 OFFSET(29) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP30 OFFSET(30) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP31 OFFSET(31) NUMBITS(1) []
],
SET1 [
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP0 OFFSET(0) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP1 OFFSET(1) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP2 OFFSET(2) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP3 OFFSET(3) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP4 OFFSET(4) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP5 OFFSET(5) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP6 OFFSET(6) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP7 OFFSET(7) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP8 OFFSET(8) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP9 OFFSET(9) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP10 OFFSET(10) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP11 OFFSET(11) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP12 OFFSET(12) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP13 OFFSET(13) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP14 OFFSET(14) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP15 OFFSET(15) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP16 OFFSET(16) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP17 OFFSET(17) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP18 OFFSET(18) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP19 OFFSET(19) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP20 OFFSET(20) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP21 OFFSET(21) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP22 OFFSET(22) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP23 OFFSET(23) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP24 OFFSET(24) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP25 OFFSET(25) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP26 OFFSET(26) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP27 OFFSET(27) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP28 OFFSET(28) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP29 OFFSET(29) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP30 OFFSET(30) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP31 OFFSET(31) NUMBITS(1) []
],
SET2 [
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP0 OFFSET(0) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP1 OFFSET(1) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP2 OFFSET(2) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP3 OFFSET(3) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP4 OFFSET(4) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP5 OFFSET(5) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP6 OFFSET(6) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP7 OFFSET(7) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP8 OFFSET(8) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP9 OFFSET(9) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP10 OFFSET(10) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP11 OFFSET(11) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP12 OFFSET(12) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP13 OFFSET(13) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP14 OFFSET(14) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP15 OFFSET(15) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP16 OFFSET(16) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP17 OFFSET(17) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP18 OFFSET(18) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP19 OFFSET(19) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP20 OFFSET(20) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP21 OFFSET(21) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP22 OFFSET(22) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP23 OFFSET(23) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP24 OFFSET(24) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP25 OFFSET(25) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP26 OFFSET(26) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP27 OFFSET(27) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP28 OFFSET(28) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP29 OFFSET(29) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP30 OFFSET(30) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP31 OFFSET(31) NUMBITS(1) []
],
SET3 [
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP0 OFFSET(0) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP1 OFFSET(1) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP2 OFFSET(2) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP3 OFFSET(3) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP4 OFFSET(4) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP5 OFFSET(5) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP6 OFFSET(6) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP7 OFFSET(7) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP8 OFFSET(8) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP9 OFFSET(9) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP10 OFFSET(10) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP11 OFFSET(11) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP12 OFFSET(12) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP13 OFFSET(13) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP14 OFFSET(14) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP15 OFFSET(15) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP16 OFFSET(16) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP17 OFFSET(17) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP18 OFFSET(18) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP19 OFFSET(19) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP20 OFFSET(20) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP21 OFFSET(21) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP22 OFFSET(22) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP23 OFFSET(23) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP24 OFFSET(24) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP25 OFFSET(25) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP26 OFFSET(26) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP27 OFFSET(27) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP28 OFFSET(28) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP29 OFFSET(29) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP30 OFFSET(30) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP31 OFFSET(31) NUMBITS(1) []
],
SET4 [
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP0 OFFSET(0) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP1 OFFSET(1) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP2 OFFSET(2) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP3 OFFSET(3) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP4 OFFSET(4) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP5 OFFSET(5) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP6 OFFSET(6) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP7 OFFSET(7) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP8 OFFSET(8) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP9 OFFSET(9) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP10 OFFSET(10) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP11 OFFSET(11) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP12 OFFSET(12) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP13 OFFSET(13) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP14 OFFSET(14) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP15 OFFSET(15) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP16 OFFSET(16) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP17 OFFSET(17) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP18 OFFSET(18) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP19 OFFSET(19) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP20 OFFSET(20) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP21 OFFSET(21) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP22 OFFSET(22) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP23 OFFSET(23) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP24 OFFSET(24) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP25 OFFSET(25) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP26 OFFSET(26) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP27 OFFSET(27) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP28 OFFSET(28) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP29 OFFSET(29) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP30 OFFSET(30) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP31 OFFSET(31) NUMBITS(1) []
],
SET5 [
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP0 OFFSET(0) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP1 OFFSET(1) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP2 OFFSET(2) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP3 OFFSET(3) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP4 OFFSET(4) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP5 OFFSET(5) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP6 OFFSET(6) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP7 OFFSET(7) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP8 OFFSET(8) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP9 OFFSET(9) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP10 OFFSET(10) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP11 OFFSET(11) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP12 OFFSET(12) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP13 OFFSET(13) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP14 OFFSET(14) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP15 OFFSET(15) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP16 OFFSET(16) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP17 OFFSET(17) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP18 OFFSET(18) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP19 OFFSET(19) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP20 OFFSET(20) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP21 OFFSET(21) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP22 OFFSET(22) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP23 OFFSET(23) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP24 OFFSET(24) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP25 OFFSET(25) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP26 OFFSET(26) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP27 OFFSET(27) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP28 OFFSET(28) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP29 OFFSET(29) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP30 OFFSET(30) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP31 OFFSET(31) NUMBITS(1) []
],
SET6 [
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP0 OFFSET(0) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP1 OFFSET(1) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP2 OFFSET(2) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP3 OFFSET(3) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP4 OFFSET(4) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP5 OFFSET(5) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP6 OFFSET(6) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP7 OFFSET(7) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP8 OFFSET(8) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP9 OFFSET(9) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP10 OFFSET(10) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP11 OFFSET(11) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP12 OFFSET(12) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP13 OFFSET(13) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP14 OFFSET(14) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP15 OFFSET(15) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP16 OFFSET(16) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP17 OFFSET(17) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP18 OFFSET(18) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP19 OFFSET(19) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP20 OFFSET(20) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP21 OFFSET(21) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP22 OFFSET(22) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP23 OFFSET(23) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP24 OFFSET(24) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP25 OFFSET(25) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP26 OFFSET(26) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP27 OFFSET(27) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP28 OFFSET(28) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP29 OFFSET(29) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP30 OFFSET(30) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP31 OFFSET(31) NUMBITS(1) []
],
SET7 [
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP0 OFFSET(0) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP1 OFFSET(1) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP2 OFFSET(2) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP3 OFFSET(3) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP4 OFFSET(4) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP5 OFFSET(5) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP6 OFFSET(6) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP7 OFFSET(7) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP8 OFFSET(8) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP9 OFFSET(9) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP10 OFFSET(10) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP11 OFFSET(11) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP12 OFFSET(12) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP13 OFFSET(13) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP14 OFFSET(14) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP15 OFFSET(15) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP16 OFFSET(16) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP17 OFFSET(17) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP18 OFFSET(18) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP19 OFFSET(19) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP20 OFFSET(20) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP21 OFFSET(21) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP22 OFFSET(22) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP23 OFFSET(23) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP24 OFFSET(24) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP25 OFFSET(25) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP26 OFFSET(26) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP27 OFFSET(27) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP28 OFFSET(28) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP29 OFFSET(29) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP30 OFFSET(30) NUMBITS(1) [],
    /// Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: ou
    SETP31 OFFSET(31) NUMBITS(1) []
],
CLR0 [
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP00 OFFSET(0) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP01 OFFSET(1) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP02 OFFSET(2) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP03 OFFSET(3) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP04 OFFSET(4) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP05 OFFSET(5) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP06 OFFSET(6) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP07 OFFSET(7) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP08 OFFSET(8) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP09 OFFSET(9) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP010 OFFSET(10) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP011 OFFSET(11) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP012 OFFSET(12) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP013 OFFSET(13) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP014 OFFSET(14) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP015 OFFSET(15) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP016 OFFSET(16) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP017 OFFSET(17) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP018 OFFSET(18) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP019 OFFSET(19) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP020 OFFSET(20) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP021 OFFSET(21) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP022 OFFSET(22) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP023 OFFSET(23) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP024 OFFSET(24) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP025 OFFSET(25) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP026 OFFSET(26) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP027 OFFSET(27) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP028 OFFSET(28) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP029 OFFSET(29) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP030 OFFSET(30) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP031 OFFSET(31) NUMBITS(1) []
],
CLR1 [
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP00 OFFSET(0) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP01 OFFSET(1) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP02 OFFSET(2) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP03 OFFSET(3) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP04 OFFSET(4) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP05 OFFSET(5) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP06 OFFSET(6) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP07 OFFSET(7) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP08 OFFSET(8) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP09 OFFSET(9) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP010 OFFSET(10) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP011 OFFSET(11) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP012 OFFSET(12) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP013 OFFSET(13) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP014 OFFSET(14) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP015 OFFSET(15) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP016 OFFSET(16) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP017 OFFSET(17) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP018 OFFSET(18) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP019 OFFSET(19) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP020 OFFSET(20) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP021 OFFSET(21) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP022 OFFSET(22) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP023 OFFSET(23) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP024 OFFSET(24) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP025 OFFSET(25) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP026 OFFSET(26) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP027 OFFSET(27) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP028 OFFSET(28) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP029 OFFSET(29) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP030 OFFSET(30) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP031 OFFSET(31) NUMBITS(1) []
],
CLR2 [
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP00 OFFSET(0) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP01 OFFSET(1) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP02 OFFSET(2) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP03 OFFSET(3) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP04 OFFSET(4) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP05 OFFSET(5) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP06 OFFSET(6) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP07 OFFSET(7) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP08 OFFSET(8) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP09 OFFSET(9) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP010 OFFSET(10) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP011 OFFSET(11) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP012 OFFSET(12) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP013 OFFSET(13) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP014 OFFSET(14) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP015 OFFSET(15) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP016 OFFSET(16) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP017 OFFSET(17) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP018 OFFSET(18) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP019 OFFSET(19) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP020 OFFSET(20) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP021 OFFSET(21) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP022 OFFSET(22) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP023 OFFSET(23) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP024 OFFSET(24) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP025 OFFSET(25) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP026 OFFSET(26) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP027 OFFSET(27) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP028 OFFSET(28) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP029 OFFSET(29) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP030 OFFSET(30) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP031 OFFSET(31) NUMBITS(1) []
],
CLR3 [
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP00 OFFSET(0) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP01 OFFSET(1) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP02 OFFSET(2) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP03 OFFSET(3) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP04 OFFSET(4) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP05 OFFSET(5) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP06 OFFSET(6) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP07 OFFSET(7) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP08 OFFSET(8) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP09 OFFSET(9) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP010 OFFSET(10) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP011 OFFSET(11) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP012 OFFSET(12) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP013 OFFSET(13) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP014 OFFSET(14) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP015 OFFSET(15) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP016 OFFSET(16) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP017 OFFSET(17) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP018 OFFSET(18) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP019 OFFSET(19) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP020 OFFSET(20) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP021 OFFSET(21) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP022 OFFSET(22) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP023 OFFSET(23) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP024 OFFSET(24) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP025 OFFSET(25) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP026 OFFSET(26) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP027 OFFSET(27) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP028 OFFSET(28) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP029 OFFSET(29) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP030 OFFSET(30) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP031 OFFSET(31) NUMBITS(1) []
],
CLR4 [
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP00 OFFSET(0) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP01 OFFSET(1) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP02 OFFSET(2) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP03 OFFSET(3) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP04 OFFSET(4) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP05 OFFSET(5) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP06 OFFSET(6) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP07 OFFSET(7) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP08 OFFSET(8) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP09 OFFSET(9) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP010 OFFSET(10) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP011 OFFSET(11) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP012 OFFSET(12) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP013 OFFSET(13) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP014 OFFSET(14) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP015 OFFSET(15) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP016 OFFSET(16) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP017 OFFSET(17) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP018 OFFSET(18) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP019 OFFSET(19) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP020 OFFSET(20) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP021 OFFSET(21) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP022 OFFSET(22) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP023 OFFSET(23) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP024 OFFSET(24) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP025 OFFSET(25) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP026 OFFSET(26) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP027 OFFSET(27) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP028 OFFSET(28) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP029 OFFSET(29) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP030 OFFSET(30) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP031 OFFSET(31) NUMBITS(1) []
],
CLR5 [
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP00 OFFSET(0) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP01 OFFSET(1) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP02 OFFSET(2) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP03 OFFSET(3) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP04 OFFSET(4) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP05 OFFSET(5) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP06 OFFSET(6) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP07 OFFSET(7) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP08 OFFSET(8) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP09 OFFSET(9) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP010 OFFSET(10) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP011 OFFSET(11) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP012 OFFSET(12) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP013 OFFSET(13) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP014 OFFSET(14) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP015 OFFSET(15) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP016 OFFSET(16) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP017 OFFSET(17) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP018 OFFSET(18) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP019 OFFSET(19) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP020 OFFSET(20) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP021 OFFSET(21) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP022 OFFSET(22) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP023 OFFSET(23) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP024 OFFSET(24) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP025 OFFSET(25) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP026 OFFSET(26) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP027 OFFSET(27) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP028 OFFSET(28) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP029 OFFSET(29) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP030 OFFSET(30) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP031 OFFSET(31) NUMBITS(1) []
],
CLR6 [
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP00 OFFSET(0) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP01 OFFSET(1) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP02 OFFSET(2) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP03 OFFSET(3) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP04 OFFSET(4) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP05 OFFSET(5) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP06 OFFSET(6) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP07 OFFSET(7) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP08 OFFSET(8) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP09 OFFSET(9) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP010 OFFSET(10) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP011 OFFSET(11) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP012 OFFSET(12) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP013 OFFSET(13) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP014 OFFSET(14) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP015 OFFSET(15) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP016 OFFSET(16) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP017 OFFSET(17) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP018 OFFSET(18) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP019 OFFSET(19) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP020 OFFSET(20) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP021 OFFSET(21) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP022 OFFSET(22) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP023 OFFSET(23) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP024 OFFSET(24) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP025 OFFSET(25) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP026 OFFSET(26) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP027 OFFSET(27) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP028 OFFSET(28) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP029 OFFSET(29) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP030 OFFSET(30) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP031 OFFSET(31) NUMBITS(1) []
],
CLR7 [
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP00 OFFSET(0) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP01 OFFSET(1) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP02 OFFSET(2) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP03 OFFSET(3) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP04 OFFSET(4) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP05 OFFSET(5) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP06 OFFSET(6) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP07 OFFSET(7) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP08 OFFSET(8) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP09 OFFSET(9) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP010 OFFSET(10) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP011 OFFSET(11) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP012 OFFSET(12) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP013 OFFSET(13) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP014 OFFSET(14) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP015 OFFSET(15) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP016 OFFSET(16) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP017 OFFSET(17) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP018 OFFSET(18) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP019 OFFSET(19) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP020 OFFSET(20) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP021 OFFSET(21) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP022 OFFSET(22) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP023 OFFSET(23) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP024 OFFSET(24) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP025 OFFSET(25) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP026 OFFSET(26) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP027 OFFSET(27) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP028 OFFSET(28) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP029 OFFSET(29) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP030 OFFSET(30) NUMBITS(1) [],
    /// Clear output bits: 0 = No operation. 1 = Clear output bit.
    CLRP031 OFFSET(31) NUMBITS(1) []
],
NOT0 [
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP0 OFFSET(0) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP1 OFFSET(1) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP2 OFFSET(2) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP3 OFFSET(3) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP4 OFFSET(4) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP5 OFFSET(5) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP6 OFFSET(6) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP7 OFFSET(7) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP8 OFFSET(8) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP9 OFFSET(9) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP10 OFFSET(10) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP11 OFFSET(11) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP12 OFFSET(12) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP13 OFFSET(13) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP14 OFFSET(14) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP15 OFFSET(15) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP16 OFFSET(16) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP17 OFFSET(17) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP18 OFFSET(18) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP19 OFFSET(19) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP20 OFFSET(20) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP21 OFFSET(21) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP22 OFFSET(22) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP23 OFFSET(23) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP24 OFFSET(24) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP25 OFFSET(25) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP26 OFFSET(26) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP27 OFFSET(27) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP28 OFFSET(28) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP29 OFFSET(29) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP30 OFFSET(30) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP31 OFFSET(31) NUMBITS(1) []
],
NOT1 [
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP0 OFFSET(0) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP1 OFFSET(1) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP2 OFFSET(2) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP3 OFFSET(3) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP4 OFFSET(4) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP5 OFFSET(5) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP6 OFFSET(6) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP7 OFFSET(7) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP8 OFFSET(8) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP9 OFFSET(9) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP10 OFFSET(10) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP11 OFFSET(11) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP12 OFFSET(12) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP13 OFFSET(13) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP14 OFFSET(14) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP15 OFFSET(15) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP16 OFFSET(16) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP17 OFFSET(17) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP18 OFFSET(18) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP19 OFFSET(19) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP20 OFFSET(20) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP21 OFFSET(21) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP22 OFFSET(22) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP23 OFFSET(23) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP24 OFFSET(24) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP25 OFFSET(25) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP26 OFFSET(26) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP27 OFFSET(27) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP28 OFFSET(28) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP29 OFFSET(29) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP30 OFFSET(30) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP31 OFFSET(31) NUMBITS(1) []
],
NOT2 [
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP0 OFFSET(0) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP1 OFFSET(1) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP2 OFFSET(2) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP3 OFFSET(3) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP4 OFFSET(4) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP5 OFFSET(5) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP6 OFFSET(6) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP7 OFFSET(7) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP8 OFFSET(8) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP9 OFFSET(9) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP10 OFFSET(10) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP11 OFFSET(11) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP12 OFFSET(12) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP13 OFFSET(13) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP14 OFFSET(14) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP15 OFFSET(15) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP16 OFFSET(16) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP17 OFFSET(17) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP18 OFFSET(18) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP19 OFFSET(19) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP20 OFFSET(20) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP21 OFFSET(21) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP22 OFFSET(22) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP23 OFFSET(23) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP24 OFFSET(24) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP25 OFFSET(25) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP26 OFFSET(26) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP27 OFFSET(27) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP28 OFFSET(28) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP29 OFFSET(29) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP30 OFFSET(30) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP31 OFFSET(31) NUMBITS(1) []
],
NOT3 [
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP0 OFFSET(0) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP1 OFFSET(1) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP2 OFFSET(2) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP3 OFFSET(3) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP4 OFFSET(4) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP5 OFFSET(5) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP6 OFFSET(6) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP7 OFFSET(7) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP8 OFFSET(8) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP9 OFFSET(9) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP10 OFFSET(10) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP11 OFFSET(11) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP12 OFFSET(12) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP13 OFFSET(13) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP14 OFFSET(14) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP15 OFFSET(15) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP16 OFFSET(16) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP17 OFFSET(17) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP18 OFFSET(18) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP19 OFFSET(19) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP20 OFFSET(20) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP21 OFFSET(21) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP22 OFFSET(22) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP23 OFFSET(23) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP24 OFFSET(24) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP25 OFFSET(25) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP26 OFFSET(26) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP27 OFFSET(27) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP28 OFFSET(28) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP29 OFFSET(29) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP30 OFFSET(30) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP31 OFFSET(31) NUMBITS(1) []
],
NOT4 [
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP0 OFFSET(0) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP1 OFFSET(1) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP2 OFFSET(2) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP3 OFFSET(3) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP4 OFFSET(4) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP5 OFFSET(5) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP6 OFFSET(6) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP7 OFFSET(7) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP8 OFFSET(8) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP9 OFFSET(9) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP10 OFFSET(10) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP11 OFFSET(11) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP12 OFFSET(12) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP13 OFFSET(13) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP14 OFFSET(14) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP15 OFFSET(15) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP16 OFFSET(16) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP17 OFFSET(17) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP18 OFFSET(18) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP19 OFFSET(19) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP20 OFFSET(20) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP21 OFFSET(21) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP22 OFFSET(22) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP23 OFFSET(23) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP24 OFFSET(24) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP25 OFFSET(25) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP26 OFFSET(26) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP27 OFFSET(27) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP28 OFFSET(28) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP29 OFFSET(29) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP30 OFFSET(30) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP31 OFFSET(31) NUMBITS(1) []
],
NOT5 [
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP0 OFFSET(0) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP1 OFFSET(1) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP2 OFFSET(2) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP3 OFFSET(3) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP4 OFFSET(4) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP5 OFFSET(5) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP6 OFFSET(6) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP7 OFFSET(7) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP8 OFFSET(8) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP9 OFFSET(9) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP10 OFFSET(10) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP11 OFFSET(11) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP12 OFFSET(12) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP13 OFFSET(13) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP14 OFFSET(14) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP15 OFFSET(15) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP16 OFFSET(16) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP17 OFFSET(17) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP18 OFFSET(18) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP19 OFFSET(19) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP20 OFFSET(20) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP21 OFFSET(21) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP22 OFFSET(22) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP23 OFFSET(23) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP24 OFFSET(24) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP25 OFFSET(25) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP26 OFFSET(26) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP27 OFFSET(27) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP28 OFFSET(28) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP29 OFFSET(29) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP30 OFFSET(30) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP31 OFFSET(31) NUMBITS(1) []
],
NOT6 [
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP0 OFFSET(0) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP1 OFFSET(1) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP2 OFFSET(2) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP3 OFFSET(3) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP4 OFFSET(4) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP5 OFFSET(5) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP6 OFFSET(6) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP7 OFFSET(7) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP8 OFFSET(8) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP9 OFFSET(9) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP10 OFFSET(10) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP11 OFFSET(11) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP12 OFFSET(12) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP13 OFFSET(13) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP14 OFFSET(14) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP15 OFFSET(15) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP16 OFFSET(16) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP17 OFFSET(17) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP18 OFFSET(18) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP19 OFFSET(19) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP20 OFFSET(20) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP21 OFFSET(21) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP22 OFFSET(22) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP23 OFFSET(23) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP24 OFFSET(24) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP25 OFFSET(25) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP26 OFFSET(26) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP27 OFFSET(27) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP28 OFFSET(28) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP29 OFFSET(29) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP30 OFFSET(30) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP31 OFFSET(31) NUMBITS(1) []
],
NOT7 [
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP0 OFFSET(0) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP1 OFFSET(1) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP2 OFFSET(2) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP3 OFFSET(3) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP4 OFFSET(4) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP5 OFFSET(5) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP6 OFFSET(6) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP7 OFFSET(7) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP8 OFFSET(8) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP9 OFFSET(9) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP10 OFFSET(10) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP11 OFFSET(11) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP12 OFFSET(12) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP13 OFFSET(13) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP14 OFFSET(14) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP15 OFFSET(15) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP16 OFFSET(16) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP17 OFFSET(17) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP18 OFFSET(18) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP19 OFFSET(19) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP20 OFFSET(20) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP21 OFFSET(21) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP22 OFFSET(22) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP23 OFFSET(23) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP24 OFFSET(24) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP25 OFFSET(25) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP26 OFFSET(26) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP27 OFFSET(27) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP28 OFFSET(28) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP29 OFFSET(29) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP30 OFFSET(30) NUMBITS(1) [],
    /// Toggle output bits: 0 = no operation. 1 = Toggle output bit.
    NOTP31 OFFSET(31) NUMBITS(1) []
]
];
const GPIO_PORT_BASE: StaticRef<Gpio_PortRegisters> =
    unsafe { StaticRef::new(0x400F4000 as *const Gpio_PortRegisters) };
