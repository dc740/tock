
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly, register_bitfields};
    /// LCD controller
#[repr(C)]
struct LcdRegisters {
/// Horizontal Timing Control register
timh: ReadWrite<u32, TIMH::Register>,
/// Vertical Timing Control register
timv: ReadWrite<u32, TIMV::Register>,
/// Clock and Signal Polarity Control register
pol: ReadWrite<u32, POL::Register>,
/// Line End Control register
le: ReadWrite<u32, LE::Register>,
/// Upper Panel Frame Base Address register
upbase: ReadWrite<u32>,
/// Lower Panel Frame Base Address register
lpbase: ReadWrite<u32>,
/// LCD Control register
ctrl: ReadWrite<u32, CTRL::Register>,
/// Interrupt Mask register
intmsk: ReadWrite<u32, INTMSK::Register>,
/// Raw Interrupt Status register
intraw: ReadOnly<u32, INTRAW::Register>,
/// Masked Interrupt Status register
intstat: ReadOnly<u32, INTSTAT::Register>,
/// Interrupt Clear register
intclr: WriteOnly<u32, INTCLR::Register>,
/// Upper Panel Current Address Value register
upcurr: ReadOnly<u32>,
/// Lower Panel Current Address Value register
lpcurr: ReadOnly<u32>,
_reserved0: [u8; 460],
/// 256x16-bit Color Palette registers
pal_0: ReadWrite<u32, PAL[0]::Register>,
/// 256x16-bit Color Palette registers
pal_1: ReadWrite<u32, PAL[1]::Register>,
/// 256x16-bit Color Palette registers
pal_2: ReadWrite<u32, PAL[2]::Register>,
/// 256x16-bit Color Palette registers
pal_3: ReadWrite<u32, PAL[3]::Register>,
/// 256x16-bit Color Palette registers
pal_4: ReadWrite<u32, PAL[4]::Register>,
/// 256x16-bit Color Palette registers
pal_5: ReadWrite<u32, PAL[5]::Register>,
/// 256x16-bit Color Palette registers
pal_6: ReadWrite<u32, PAL[6]::Register>,
/// 256x16-bit Color Palette registers
pal_7: ReadWrite<u32, PAL[7]::Register>,
/// 256x16-bit Color Palette registers
pal_8: ReadWrite<u32, PAL[8]::Register>,
/// 256x16-bit Color Palette registers
pal_9: ReadWrite<u32, PAL[9]::Register>,
/// 256x16-bit Color Palette registers
pal_10: ReadWrite<u32, PAL[10]::Register>,
/// 256x16-bit Color Palette registers
pal_11: ReadWrite<u32, PAL[11]::Register>,
/// 256x16-bit Color Palette registers
pal_12: ReadWrite<u32, PAL[12]::Register>,
/// 256x16-bit Color Palette registers
pal_13: ReadWrite<u32, PAL[13]::Register>,
/// 256x16-bit Color Palette registers
pal_14: ReadWrite<u32, PAL[14]::Register>,
/// 256x16-bit Color Palette registers
pal_15: ReadWrite<u32, PAL[15]::Register>,
/// 256x16-bit Color Palette registers
pal_16: ReadWrite<u32, PAL[16]::Register>,
/// 256x16-bit Color Palette registers
pal_17: ReadWrite<u32, PAL[17]::Register>,
/// 256x16-bit Color Palette registers
pal_18: ReadWrite<u32, PAL[18]::Register>,
/// 256x16-bit Color Palette registers
pal_19: ReadWrite<u32, PAL[19]::Register>,
/// 256x16-bit Color Palette registers
pal_20: ReadWrite<u32, PAL[20]::Register>,
/// 256x16-bit Color Palette registers
pal_21: ReadWrite<u32, PAL[21]::Register>,
/// 256x16-bit Color Palette registers
pal_22: ReadWrite<u32, PAL[22]::Register>,
/// 256x16-bit Color Palette registers
pal_23: ReadWrite<u32, PAL[23]::Register>,
/// 256x16-bit Color Palette registers
pal_24: ReadWrite<u32, PAL[24]::Register>,
/// 256x16-bit Color Palette registers
pal_25: ReadWrite<u32, PAL[25]::Register>,
/// 256x16-bit Color Palette registers
pal_26: ReadWrite<u32, PAL[26]::Register>,
/// 256x16-bit Color Palette registers
pal_27: ReadWrite<u32, PAL[27]::Register>,
/// 256x16-bit Color Palette registers
pal_28: ReadWrite<u32, PAL[28]::Register>,
/// 256x16-bit Color Palette registers
pal_29: ReadWrite<u32, PAL[29]::Register>,
/// 256x16-bit Color Palette registers
pal_30: ReadWrite<u32, PAL[30]::Register>,
/// 256x16-bit Color Palette registers
pal_31: ReadWrite<u32, PAL[31]::Register>,
/// 256x16-bit Color Palette registers
pal_32: ReadWrite<u32, PAL[32]::Register>,
/// 256x16-bit Color Palette registers
pal_33: ReadWrite<u32, PAL[33]::Register>,
/// 256x16-bit Color Palette registers
pal_34: ReadWrite<u32, PAL[34]::Register>,
/// 256x16-bit Color Palette registers
pal_35: ReadWrite<u32, PAL[35]::Register>,
/// 256x16-bit Color Palette registers
pal_36: ReadWrite<u32, PAL[36]::Register>,
/// 256x16-bit Color Palette registers
pal_37: ReadWrite<u32, PAL[37]::Register>,
/// 256x16-bit Color Palette registers
pal_38: ReadWrite<u32, PAL[38]::Register>,
/// 256x16-bit Color Palette registers
pal_39: ReadWrite<u32, PAL[39]::Register>,
/// 256x16-bit Color Palette registers
pal_40: ReadWrite<u32, PAL[40]::Register>,
/// 256x16-bit Color Palette registers
pal_41: ReadWrite<u32, PAL[41]::Register>,
/// 256x16-bit Color Palette registers
pal_42: ReadWrite<u32, PAL[42]::Register>,
/// 256x16-bit Color Palette registers
pal_43: ReadWrite<u32, PAL[43]::Register>,
/// 256x16-bit Color Palette registers
pal_44: ReadWrite<u32, PAL[44]::Register>,
/// 256x16-bit Color Palette registers
pal_45: ReadWrite<u32, PAL[45]::Register>,
/// 256x16-bit Color Palette registers
pal_46: ReadWrite<u32, PAL[46]::Register>,
/// 256x16-bit Color Palette registers
pal_47: ReadWrite<u32, PAL[47]::Register>,
/// 256x16-bit Color Palette registers
pal_48: ReadWrite<u32, PAL[48]::Register>,
/// 256x16-bit Color Palette registers
pal_49: ReadWrite<u32, PAL[49]::Register>,
/// 256x16-bit Color Palette registers
pal_50: ReadWrite<u32, PAL[50]::Register>,
/// 256x16-bit Color Palette registers
pal_51: ReadWrite<u32, PAL[51]::Register>,
/// 256x16-bit Color Palette registers
pal_52: ReadWrite<u32, PAL[52]::Register>,
/// 256x16-bit Color Palette registers
pal_53: ReadWrite<u32, PAL[53]::Register>,
/// 256x16-bit Color Palette registers
pal_54: ReadWrite<u32, PAL[54]::Register>,
/// 256x16-bit Color Palette registers
pal_55: ReadWrite<u32, PAL[55]::Register>,
/// 256x16-bit Color Palette registers
pal_56: ReadWrite<u32, PAL[56]::Register>,
/// 256x16-bit Color Palette registers
pal_57: ReadWrite<u32, PAL[57]::Register>,
/// 256x16-bit Color Palette registers
pal_58: ReadWrite<u32, PAL[58]::Register>,
/// 256x16-bit Color Palette registers
pal_59: ReadWrite<u32, PAL[59]::Register>,
/// 256x16-bit Color Palette registers
pal_60: ReadWrite<u32, PAL[60]::Register>,
/// 256x16-bit Color Palette registers
pal_61: ReadWrite<u32, PAL[61]::Register>,
/// 256x16-bit Color Palette registers
pal_62: ReadWrite<u32, PAL[62]::Register>,
/// 256x16-bit Color Palette registers
pal_63: ReadWrite<u32, PAL[63]::Register>,
/// 256x16-bit Color Palette registers
pal_64: ReadWrite<u32, PAL[64]::Register>,
/// 256x16-bit Color Palette registers
pal_65: ReadWrite<u32, PAL[65]::Register>,
/// 256x16-bit Color Palette registers
pal_66: ReadWrite<u32, PAL[66]::Register>,
/// 256x16-bit Color Palette registers
pal_67: ReadWrite<u32, PAL[67]::Register>,
/// 256x16-bit Color Palette registers
pal_68: ReadWrite<u32, PAL[68]::Register>,
/// 256x16-bit Color Palette registers
pal_69: ReadWrite<u32, PAL[69]::Register>,
/// 256x16-bit Color Palette registers
pal_70: ReadWrite<u32, PAL[70]::Register>,
/// 256x16-bit Color Palette registers
pal_71: ReadWrite<u32, PAL[71]::Register>,
/// 256x16-bit Color Palette registers
pal_72: ReadWrite<u32, PAL[72]::Register>,
/// 256x16-bit Color Palette registers
pal_73: ReadWrite<u32, PAL[73]::Register>,
/// 256x16-bit Color Palette registers
pal_74: ReadWrite<u32, PAL[74]::Register>,
/// 256x16-bit Color Palette registers
pal_75: ReadWrite<u32, PAL[75]::Register>,
/// 256x16-bit Color Palette registers
pal_76: ReadWrite<u32, PAL[76]::Register>,
/// 256x16-bit Color Palette registers
pal_77: ReadWrite<u32, PAL[77]::Register>,
/// 256x16-bit Color Palette registers
pal_78: ReadWrite<u32, PAL[78]::Register>,
/// 256x16-bit Color Palette registers
pal_79: ReadWrite<u32, PAL[79]::Register>,
/// 256x16-bit Color Palette registers
pal_80: ReadWrite<u32, PAL[80]::Register>,
/// 256x16-bit Color Palette registers
pal_81: ReadWrite<u32, PAL[81]::Register>,
/// 256x16-bit Color Palette registers
pal_82: ReadWrite<u32, PAL[82]::Register>,
/// 256x16-bit Color Palette registers
pal_83: ReadWrite<u32, PAL[83]::Register>,
/// 256x16-bit Color Palette registers
pal_84: ReadWrite<u32, PAL[84]::Register>,
/// 256x16-bit Color Palette registers
pal_85: ReadWrite<u32, PAL[85]::Register>,
/// 256x16-bit Color Palette registers
pal_86: ReadWrite<u32, PAL[86]::Register>,
/// 256x16-bit Color Palette registers
pal_87: ReadWrite<u32, PAL[87]::Register>,
/// 256x16-bit Color Palette registers
pal_88: ReadWrite<u32, PAL[88]::Register>,
/// 256x16-bit Color Palette registers
pal_89: ReadWrite<u32, PAL[89]::Register>,
/// 256x16-bit Color Palette registers
pal_90: ReadWrite<u32, PAL[90]::Register>,
/// 256x16-bit Color Palette registers
pal_91: ReadWrite<u32, PAL[91]::Register>,
/// 256x16-bit Color Palette registers
pal_92: ReadWrite<u32, PAL[92]::Register>,
/// 256x16-bit Color Palette registers
pal_93: ReadWrite<u32, PAL[93]::Register>,
/// 256x16-bit Color Palette registers
pal_94: ReadWrite<u32, PAL[94]::Register>,
/// 256x16-bit Color Palette registers
pal_95: ReadWrite<u32, PAL[95]::Register>,
/// 256x16-bit Color Palette registers
pal_96: ReadWrite<u32, PAL[96]::Register>,
/// 256x16-bit Color Palette registers
pal_97: ReadWrite<u32, PAL[97]::Register>,
/// 256x16-bit Color Palette registers
pal_98: ReadWrite<u32, PAL[98]::Register>,
/// 256x16-bit Color Palette registers
pal_99: ReadWrite<u32, PAL[99]::Register>,
/// 256x16-bit Color Palette registers
pal_100: ReadWrite<u32, PAL[100]::Register>,
/// 256x16-bit Color Palette registers
pal_101: ReadWrite<u32, PAL[101]::Register>,
/// 256x16-bit Color Palette registers
pal_102: ReadWrite<u32, PAL[102]::Register>,
/// 256x16-bit Color Palette registers
pal_103: ReadWrite<u32, PAL[103]::Register>,
/// 256x16-bit Color Palette registers
pal_104: ReadWrite<u32, PAL[104]::Register>,
/// 256x16-bit Color Palette registers
pal_105: ReadWrite<u32, PAL[105]::Register>,
/// 256x16-bit Color Palette registers
pal_106: ReadWrite<u32, PAL[106]::Register>,
/// 256x16-bit Color Palette registers
pal_107: ReadWrite<u32, PAL[107]::Register>,
/// 256x16-bit Color Palette registers
pal_108: ReadWrite<u32, PAL[108]::Register>,
/// 256x16-bit Color Palette registers
pal_109: ReadWrite<u32, PAL[109]::Register>,
/// 256x16-bit Color Palette registers
pal_110: ReadWrite<u32, PAL[110]::Register>,
/// 256x16-bit Color Palette registers
pal_111: ReadWrite<u32, PAL[111]::Register>,
/// 256x16-bit Color Palette registers
pal_112: ReadWrite<u32, PAL[112]::Register>,
/// 256x16-bit Color Palette registers
pal_113: ReadWrite<u32, PAL[113]::Register>,
/// 256x16-bit Color Palette registers
pal_114: ReadWrite<u32, PAL[114]::Register>,
/// 256x16-bit Color Palette registers
pal_115: ReadWrite<u32, PAL[115]::Register>,
/// 256x16-bit Color Palette registers
pal_116: ReadWrite<u32, PAL[116]::Register>,
/// 256x16-bit Color Palette registers
pal_117: ReadWrite<u32, PAL[117]::Register>,
/// 256x16-bit Color Palette registers
pal_118: ReadWrite<u32, PAL[118]::Register>,
/// 256x16-bit Color Palette registers
pal_119: ReadWrite<u32, PAL[119]::Register>,
/// 256x16-bit Color Palette registers
pal_120: ReadWrite<u32, PAL[120]::Register>,
/// 256x16-bit Color Palette registers
pal_121: ReadWrite<u32, PAL[121]::Register>,
/// 256x16-bit Color Palette registers
pal_122: ReadWrite<u32, PAL[122]::Register>,
/// 256x16-bit Color Palette registers
pal_123: ReadWrite<u32, PAL[123]::Register>,
/// 256x16-bit Color Palette registers
pal_124: ReadWrite<u32, PAL[124]::Register>,
/// 256x16-bit Color Palette registers
pal_125: ReadWrite<u32, PAL[125]::Register>,
/// 256x16-bit Color Palette registers
pal_126: ReadWrite<u32, PAL[126]::Register>,
/// 256x16-bit Color Palette registers
pal_127: ReadWrite<u32, PAL[127]::Register>,
/// 256x16-bit Color Palette registers
pal_128: ReadWrite<u32, PAL[128]::Register>,
/// 256x16-bit Color Palette registers
pal_129: ReadWrite<u32, PAL[129]::Register>,
/// 256x16-bit Color Palette registers
pal_130: ReadWrite<u32, PAL[130]::Register>,
/// 256x16-bit Color Palette registers
pal_131: ReadWrite<u32, PAL[131]::Register>,
/// 256x16-bit Color Palette registers
pal_132: ReadWrite<u32, PAL[132]::Register>,
/// 256x16-bit Color Palette registers
pal_133: ReadWrite<u32, PAL[133]::Register>,
/// 256x16-bit Color Palette registers
pal_134: ReadWrite<u32, PAL[134]::Register>,
/// 256x16-bit Color Palette registers
pal_135: ReadWrite<u32, PAL[135]::Register>,
/// 256x16-bit Color Palette registers
pal_136: ReadWrite<u32, PAL[136]::Register>,
/// 256x16-bit Color Palette registers
pal_137: ReadWrite<u32, PAL[137]::Register>,
/// 256x16-bit Color Palette registers
pal_138: ReadWrite<u32, PAL[138]::Register>,
/// 256x16-bit Color Palette registers
pal_139: ReadWrite<u32, PAL[139]::Register>,
/// 256x16-bit Color Palette registers
pal_140: ReadWrite<u32, PAL[140]::Register>,
/// 256x16-bit Color Palette registers
pal_141: ReadWrite<u32, PAL[141]::Register>,
/// 256x16-bit Color Palette registers
pal_142: ReadWrite<u32, PAL[142]::Register>,
/// 256x16-bit Color Palette registers
pal_143: ReadWrite<u32, PAL[143]::Register>,
/// 256x16-bit Color Palette registers
pal_144: ReadWrite<u32, PAL[144]::Register>,
/// 256x16-bit Color Palette registers
pal_145: ReadWrite<u32, PAL[145]::Register>,
/// 256x16-bit Color Palette registers
pal_146: ReadWrite<u32, PAL[146]::Register>,
/// 256x16-bit Color Palette registers
pal_147: ReadWrite<u32, PAL[147]::Register>,
/// 256x16-bit Color Palette registers
pal_148: ReadWrite<u32, PAL[148]::Register>,
/// 256x16-bit Color Palette registers
pal_149: ReadWrite<u32, PAL[149]::Register>,
/// 256x16-bit Color Palette registers
pal_150: ReadWrite<u32, PAL[150]::Register>,
/// 256x16-bit Color Palette registers
pal_151: ReadWrite<u32, PAL[151]::Register>,
/// 256x16-bit Color Palette registers
pal_152: ReadWrite<u32, PAL[152]::Register>,
/// 256x16-bit Color Palette registers
pal_153: ReadWrite<u32, PAL[153]::Register>,
/// 256x16-bit Color Palette registers
pal_154: ReadWrite<u32, PAL[154]::Register>,
/// 256x16-bit Color Palette registers
pal_155: ReadWrite<u32, PAL[155]::Register>,
/// 256x16-bit Color Palette registers
pal_156: ReadWrite<u32, PAL[156]::Register>,
/// 256x16-bit Color Palette registers
pal_157: ReadWrite<u32, PAL[157]::Register>,
/// 256x16-bit Color Palette registers
pal_158: ReadWrite<u32, PAL[158]::Register>,
/// 256x16-bit Color Palette registers
pal_159: ReadWrite<u32, PAL[159]::Register>,
/// 256x16-bit Color Palette registers
pal_160: ReadWrite<u32, PAL[160]::Register>,
/// 256x16-bit Color Palette registers
pal_161: ReadWrite<u32, PAL[161]::Register>,
/// 256x16-bit Color Palette registers
pal_162: ReadWrite<u32, PAL[162]::Register>,
/// 256x16-bit Color Palette registers
pal_163: ReadWrite<u32, PAL[163]::Register>,
/// 256x16-bit Color Palette registers
pal_164: ReadWrite<u32, PAL[164]::Register>,
/// 256x16-bit Color Palette registers
pal_165: ReadWrite<u32, PAL[165]::Register>,
/// 256x16-bit Color Palette registers
pal_166: ReadWrite<u32, PAL[166]::Register>,
/// 256x16-bit Color Palette registers
pal_167: ReadWrite<u32, PAL[167]::Register>,
/// 256x16-bit Color Palette registers
pal_168: ReadWrite<u32, PAL[168]::Register>,
/// 256x16-bit Color Palette registers
pal_169: ReadWrite<u32, PAL[169]::Register>,
/// 256x16-bit Color Palette registers
pal_170: ReadWrite<u32, PAL[170]::Register>,
/// 256x16-bit Color Palette registers
pal_171: ReadWrite<u32, PAL[171]::Register>,
/// 256x16-bit Color Palette registers
pal_172: ReadWrite<u32, PAL[172]::Register>,
/// 256x16-bit Color Palette registers
pal_173: ReadWrite<u32, PAL[173]::Register>,
/// 256x16-bit Color Palette registers
pal_174: ReadWrite<u32, PAL[174]::Register>,
/// 256x16-bit Color Palette registers
pal_175: ReadWrite<u32, PAL[175]::Register>,
/// 256x16-bit Color Palette registers
pal_176: ReadWrite<u32, PAL[176]::Register>,
/// 256x16-bit Color Palette registers
pal_177: ReadWrite<u32, PAL[177]::Register>,
/// 256x16-bit Color Palette registers
pal_178: ReadWrite<u32, PAL[178]::Register>,
/// 256x16-bit Color Palette registers
pal_179: ReadWrite<u32, PAL[179]::Register>,
/// 256x16-bit Color Palette registers
pal_180: ReadWrite<u32, PAL[180]::Register>,
/// 256x16-bit Color Palette registers
pal_181: ReadWrite<u32, PAL[181]::Register>,
/// 256x16-bit Color Palette registers
pal_182: ReadWrite<u32, PAL[182]::Register>,
/// 256x16-bit Color Palette registers
pal_183: ReadWrite<u32, PAL[183]::Register>,
/// 256x16-bit Color Palette registers
pal_184: ReadWrite<u32, PAL[184]::Register>,
/// 256x16-bit Color Palette registers
pal_185: ReadWrite<u32, PAL[185]::Register>,
/// 256x16-bit Color Palette registers
pal_186: ReadWrite<u32, PAL[186]::Register>,
/// 256x16-bit Color Palette registers
pal_187: ReadWrite<u32, PAL[187]::Register>,
/// 256x16-bit Color Palette registers
pal_188: ReadWrite<u32, PAL[188]::Register>,
/// 256x16-bit Color Palette registers
pal_189: ReadWrite<u32, PAL[189]::Register>,
/// 256x16-bit Color Palette registers
pal_190: ReadWrite<u32, PAL[190]::Register>,
/// 256x16-bit Color Palette registers
pal_191: ReadWrite<u32, PAL[191]::Register>,
/// 256x16-bit Color Palette registers
pal_192: ReadWrite<u32, PAL[192]::Register>,
/// 256x16-bit Color Palette registers
pal_193: ReadWrite<u32, PAL[193]::Register>,
/// 256x16-bit Color Palette registers
pal_194: ReadWrite<u32, PAL[194]::Register>,
/// 256x16-bit Color Palette registers
pal_195: ReadWrite<u32, PAL[195]::Register>,
/// 256x16-bit Color Palette registers
pal_196: ReadWrite<u32, PAL[196]::Register>,
/// 256x16-bit Color Palette registers
pal_197: ReadWrite<u32, PAL[197]::Register>,
/// 256x16-bit Color Palette registers
pal_198: ReadWrite<u32, PAL[198]::Register>,
/// 256x16-bit Color Palette registers
pal_199: ReadWrite<u32, PAL[199]::Register>,
/// 256x16-bit Color Palette registers
pal_200: ReadWrite<u32, PAL[200]::Register>,
/// 256x16-bit Color Palette registers
pal_201: ReadWrite<u32, PAL[201]::Register>,
/// 256x16-bit Color Palette registers
pal_202: ReadWrite<u32, PAL[202]::Register>,
/// 256x16-bit Color Palette registers
pal_203: ReadWrite<u32, PAL[203]::Register>,
/// 256x16-bit Color Palette registers
pal_204: ReadWrite<u32, PAL[204]::Register>,
/// 256x16-bit Color Palette registers
pal_205: ReadWrite<u32, PAL[205]::Register>,
/// 256x16-bit Color Palette registers
pal_206: ReadWrite<u32, PAL[206]::Register>,
/// 256x16-bit Color Palette registers
pal_207: ReadWrite<u32, PAL[207]::Register>,
/// 256x16-bit Color Palette registers
pal_208: ReadWrite<u32, PAL[208]::Register>,
/// 256x16-bit Color Palette registers
pal_209: ReadWrite<u32, PAL[209]::Register>,
/// 256x16-bit Color Palette registers
pal_210: ReadWrite<u32, PAL[210]::Register>,
/// 256x16-bit Color Palette registers
pal_211: ReadWrite<u32, PAL[211]::Register>,
/// 256x16-bit Color Palette registers
pal_212: ReadWrite<u32, PAL[212]::Register>,
/// 256x16-bit Color Palette registers
pal_213: ReadWrite<u32, PAL[213]::Register>,
/// 256x16-bit Color Palette registers
pal_214: ReadWrite<u32, PAL[214]::Register>,
/// 256x16-bit Color Palette registers
pal_215: ReadWrite<u32, PAL[215]::Register>,
/// 256x16-bit Color Palette registers
pal_216: ReadWrite<u32, PAL[216]::Register>,
/// 256x16-bit Color Palette registers
pal_217: ReadWrite<u32, PAL[217]::Register>,
/// 256x16-bit Color Palette registers
pal_218: ReadWrite<u32, PAL[218]::Register>,
/// 256x16-bit Color Palette registers
pal_219: ReadWrite<u32, PAL[219]::Register>,
/// 256x16-bit Color Palette registers
pal_220: ReadWrite<u32, PAL[220]::Register>,
/// 256x16-bit Color Palette registers
pal_221: ReadWrite<u32, PAL[221]::Register>,
/// 256x16-bit Color Palette registers
pal_222: ReadWrite<u32, PAL[222]::Register>,
/// 256x16-bit Color Palette registers
pal_223: ReadWrite<u32, PAL[223]::Register>,
/// 256x16-bit Color Palette registers
pal_224: ReadWrite<u32, PAL[224]::Register>,
/// 256x16-bit Color Palette registers
pal_225: ReadWrite<u32, PAL[225]::Register>,
/// 256x16-bit Color Palette registers
pal_226: ReadWrite<u32, PAL[226]::Register>,
/// 256x16-bit Color Palette registers
pal_227: ReadWrite<u32, PAL[227]::Register>,
/// 256x16-bit Color Palette registers
pal_228: ReadWrite<u32, PAL[228]::Register>,
/// 256x16-bit Color Palette registers
pal_229: ReadWrite<u32, PAL[229]::Register>,
/// 256x16-bit Color Palette registers
pal_230: ReadWrite<u32, PAL[230]::Register>,
/// 256x16-bit Color Palette registers
pal_231: ReadWrite<u32, PAL[231]::Register>,
/// 256x16-bit Color Palette registers
pal_232: ReadWrite<u32, PAL[232]::Register>,
/// 256x16-bit Color Palette registers
pal_233: ReadWrite<u32, PAL[233]::Register>,
/// 256x16-bit Color Palette registers
pal_234: ReadWrite<u32, PAL[234]::Register>,
/// 256x16-bit Color Palette registers
pal_235: ReadWrite<u32, PAL[235]::Register>,
/// 256x16-bit Color Palette registers
pal_236: ReadWrite<u32, PAL[236]::Register>,
/// 256x16-bit Color Palette registers
pal_237: ReadWrite<u32, PAL[237]::Register>,
/// 256x16-bit Color Palette registers
pal_238: ReadWrite<u32, PAL[238]::Register>,
/// 256x16-bit Color Palette registers
pal_239: ReadWrite<u32, PAL[239]::Register>,
/// 256x16-bit Color Palette registers
pal_240: ReadWrite<u32, PAL[240]::Register>,
/// 256x16-bit Color Palette registers
pal_241: ReadWrite<u32, PAL[241]::Register>,
/// 256x16-bit Color Palette registers
pal_242: ReadWrite<u32, PAL[242]::Register>,
/// 256x16-bit Color Palette registers
pal_243: ReadWrite<u32, PAL[243]::Register>,
/// 256x16-bit Color Palette registers
pal_244: ReadWrite<u32, PAL[244]::Register>,
/// 256x16-bit Color Palette registers
pal_245: ReadWrite<u32, PAL[245]::Register>,
/// 256x16-bit Color Palette registers
pal_246: ReadWrite<u32, PAL[246]::Register>,
/// 256x16-bit Color Palette registers
pal_247: ReadWrite<u32, PAL[247]::Register>,
/// 256x16-bit Color Palette registers
pal_248: ReadWrite<u32, PAL[248]::Register>,
/// 256x16-bit Color Palette registers
pal_249: ReadWrite<u32, PAL[249]::Register>,
/// 256x16-bit Color Palette registers
pal_250: ReadWrite<u32, PAL[250]::Register>,
/// 256x16-bit Color Palette registers
pal_251: ReadWrite<u32, PAL[251]::Register>,
/// 256x16-bit Color Palette registers
pal_252: ReadWrite<u32, PAL[252]::Register>,
/// 256x16-bit Color Palette registers
pal_253: ReadWrite<u32, PAL[253]::Register>,
/// 256x16-bit Color Palette registers
pal_254: ReadWrite<u32, PAL[254]::Register>,
/// 256x16-bit Color Palette registers
pal_255: ReadWrite<u32, PAL[255]::Register>,
_reserved1: [u8; 512],
/// Cursor Image registers
crsr_img_0: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_1: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_2: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_3: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_4: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_5: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_6: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_7: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_8: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_9: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_10: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_11: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_12: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_13: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_14: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_15: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_16: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_17: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_18: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_19: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_20: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_21: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_22: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_23: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_24: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_25: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_26: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_27: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_28: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_29: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_30: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_31: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_32: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_33: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_34: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_35: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_36: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_37: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_38: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_39: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_40: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_41: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_42: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_43: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_44: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_45: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_46: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_47: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_48: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_49: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_50: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_51: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_52: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_53: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_54: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_55: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_56: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_57: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_58: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_59: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_60: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_61: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_62: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_63: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_64: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_65: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_66: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_67: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_68: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_69: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_70: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_71: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_72: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_73: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_74: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_75: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_76: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_77: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_78: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_79: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_80: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_81: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_82: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_83: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_84: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_85: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_86: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_87: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_88: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_89: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_90: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_91: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_92: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_93: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_94: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_95: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_96: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_97: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_98: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_99: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_100: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_101: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_102: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_103: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_104: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_105: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_106: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_107: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_108: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_109: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_110: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_111: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_112: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_113: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_114: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_115: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_116: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_117: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_118: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_119: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_120: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_121: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_122: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_123: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_124: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_125: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_126: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_127: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_128: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_129: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_130: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_131: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_132: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_133: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_134: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_135: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_136: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_137: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_138: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_139: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_140: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_141: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_142: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_143: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_144: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_145: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_146: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_147: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_148: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_149: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_150: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_151: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_152: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_153: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_154: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_155: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_156: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_157: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_158: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_159: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_160: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_161: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_162: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_163: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_164: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_165: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_166: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_167: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_168: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_169: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_170: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_171: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_172: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_173: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_174: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_175: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_176: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_177: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_178: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_179: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_180: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_181: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_182: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_183: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_184: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_185: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_186: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_187: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_188: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_189: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_190: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_191: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_192: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_193: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_194: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_195: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_196: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_197: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_198: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_199: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_200: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_201: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_202: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_203: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_204: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_205: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_206: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_207: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_208: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_209: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_210: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_211: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_212: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_213: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_214: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_215: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_216: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_217: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_218: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_219: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_220: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_221: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_222: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_223: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_224: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_225: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_226: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_227: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_228: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_229: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_230: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_231: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_232: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_233: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_234: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_235: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_236: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_237: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_238: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_239: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_240: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_241: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_242: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_243: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_244: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_245: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_246: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_247: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_248: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_249: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_250: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_251: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_252: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_253: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_254: ReadWrite<u32>,
/// Cursor Image registers
crsr_img_255: ReadWrite<u32>,
/// Cursor Control register
crsr_ctrl: ReadWrite<u32, CRSR_CTRL::Register>,
/// Cursor Configuration register
crsr_cfg: ReadWrite<u32, CRSR_CFG::Register>,
/// Cursor Palette register 0
crsr_pal0: ReadWrite<u32, CRSR_PAL0::Register>,
/// Cursor Palette register 1
crsr_pal1: ReadWrite<u32, CRSR_PAL1::Register>,
/// Cursor XY Position register
crsr_xy: ReadWrite<u32, CRSR_XY::Register>,
/// Cursor Clip Position register
crsr_clip: ReadWrite<u32, CRSR_CLIP::Register>,
_reserved2: [u8; 8],
/// Cursor Interrupt Mask register
crsr_intmsk: ReadWrite<u32>,
/// Cursor Interrupt Clear register
crsr_intclr: WriteOnly<u32>,
/// Cursor Raw Interrupt Status register
crsr_intraw: ReadOnly<u32>,
/// Cursor Masked Interrupt Status register
crsr_intstat: ReadOnly<u32>,
}
register_bitfields![u32,
TIMH [
    /// Pixels-per-line. The PPL bit field specifies the number of pixels in each line o
    PPL OFFSET(2) NUMBITS(6) [],
    /// Horizontal synchronization pulse width. The 8-bit HSW field specifies the pulse
    HSW OFFSET(8) NUMBITS(8) [],
    /// Horizontal front porch. The 8-bit HFP field sets the number of pixel clock inter
    HFP OFFSET(16) NUMBITS(8) [],
    /// Horizontal back porch. The 8-bit HBP field is used to specify the number of pixe
    HBP OFFSET(24) NUMBITS(8) []
],
TIMV [
    /// Lines per panel. This is the number of active lines per screen. The LPP field sp
    LPP OFFSET(0) NUMBITS(10) [],
    /// Vertical synchronization pulse width. This is the number of horizontal synchroni
    VSW OFFSET(10) NUMBITS(6) [],
    /// Vertical front porch. This is the number of inactive lines at the end of a frame
    VFP OFFSET(16) NUMBITS(8) [],
    /// Vertical back porch. This is the number of inactive lines at the start of a fram
    VBP OFFSET(24) NUMBITS(8) []
],
POL [
    /// Lower five bits of panel clock divisor. The ten-bit PCD field, comprising PCD_HI
    PCD_LO OFFSET(0) NUMBITS(5) [],
    /// Clock Select. This bit controls the selection of the source for LCDCLK. 0 = the
    CLKSEL OFFSET(5) NUMBITS(1) [],
    /// AC bias pin frequency. The AC bias pin frequency is only applicable to STN displ
    ACB OFFSET(6) NUMBITS(5) [],
    /// Invert vertical synchronization. The IVS bit inverts the polarity of the LCDFP s
    IVS OFFSET(11) NUMBITS(1) [],
    /// Invert horizontal synchronization. The IHS bit inverts the polarity of the LCDLP
    IHS OFFSET(12) NUMBITS(1) [],
    /// Invert panel clock. The IPC bit selects the edge of the panel clock on which pix
    IPC OFFSET(13) NUMBITS(1) [],
    /// Invert output enable. This bit selects the active polarity of the output enable
    IOE OFFSET(14) NUMBITS(1) [],
    /// Clocks per line. This field specifies the number of actual LCDDCLK clocks to the
    CPL OFFSET(16) NUMBITS(10) [],
    /// Bypass pixel clock divider. Setting this to 1 bypasses the pixel clock divider l
    BCD OFFSET(26) NUMBITS(1) [],
    /// Upper five bits of panel clock divisor. See description for PCD_LO, in bits [4:0
    PCD_HI OFFSET(27) NUMBITS(5) []
],
LE [
    /// Line-end delay. Controls Line-end signal delay from the rising-edge of the last
    LED OFFSET(0) NUMBITS(7) [],
    /// LCD Line end enable. 0 = LCDLE disabled (held LOW). 1 = LCDLE signal active.
    LEE OFFSET(16) NUMBITS(1) []
],
CTRL [
    /// LCD enable control bit. 0 = LCD disabled. Signals LCDLP, LCDDCLK, LCDFP, LCDENAB
    LCDEN OFFSET(0) NUMBITS(1) [],
    /// LCD bits per pixel: Selects the number of bits per LCD pixel: 000 = 1 bpp. 001 =
    LCDBPP OFFSET(1) NUMBITS(3) [],
    /// STN LCD monochrome/color selection. 0 = STN LCD is color. 1 = STN LCD is monochr
    LCDBW OFFSET(4) NUMBITS(1) [],
    /// LCD panel TFT type selection. 0 = LCD is an STN display. Use gray scaler. 1 = LC
    LCDTFT OFFSET(5) NUMBITS(1) [],
    /// Monochrome LCD interface width. This bit controls whether a monochrome STN LCD u
    LCDMONO8 OFFSET(6) NUMBITS(1) [],
    /// Single or Dual LCD panel selection. STN LCD interface is: 0 = single-panel. 1 =
    LCDDUAL OFFSET(7) NUMBITS(1) [],
    /// Color format selection. 0 = RGB: normal output. 1 = BGR: red and blue swapped.
    BGR OFFSET(8) NUMBITS(1) [],
    /// Big-endian Byte Order. Controls byte ordering in memory: 0 = little-endian byte
    BEBO OFFSET(9) NUMBITS(1) [],
    /// Big-Endian Pixel Ordering. Controls pixel ordering within a byte: 0 = little-end
    BEPO OFFSET(10) NUMBITS(1) [],
    /// LCD power enable. 0 = power not gated through to LCD panel and LCDV[23:0] signal
    LCDPWR OFFSET(11) NUMBITS(1) [],
    /// LCD Vertical Compare Interrupt. Generate VComp interrupt at: 00 = start of verti
    LCDVCOMP OFFSET(12) NUMBITS(2) [],
    /// LCD DMA FIFO watermark level. Controls when DMA requests are generated: 0 = An L
    WATERMARK OFFSET(16) NUMBITS(1) []
],
INTMSK [
    /// FIFO underflow interrupt enable. 0: The FIFO underflow interrupt is disabled. 1:
    FUFIM OFFSET(1) NUMBITS(1) [],
    /// LCD next base address update interrupt enable. 0: The base address update interr
    LNBUIM OFFSET(2) NUMBITS(1) [],
    /// Vertical compare interrupt enable. 0: The vertical compare time interrupt is dis
    VCOMPIM OFFSET(3) NUMBITS(1) [],
    /// AHB master error interrupt enable. 0: The AHB Master error interrupt is disabled
    BERIM OFFSET(4) NUMBITS(1) []
],
INTRAW [
    /// FIFO underflow raw interrupt status. Set when either the upper or lower DMA FIFO
    FUFRIS OFFSET(1) NUMBITS(1) [],
    /// LCD next address base update raw interrupt status. Mode dependent. Set when the
    LNBURIS OFFSET(2) NUMBITS(1) [],
    /// Vertical compare raw interrupt status. Set when one of the four vertical regions
    VCOMPRIS OFFSET(3) NUMBITS(1) [],
    /// AHB master bus error raw interrupt status. Set when the AHB master interface rec
    BERRAW OFFSET(4) NUMBITS(1) []
],
INTSTAT [
    /// FIFO underflow masked interrupt status. Set when the both the FUFRIS bit in the
    FUFMIS OFFSET(1) NUMBITS(1) [],
    /// LCD next address base update masked interrupt status. Set when the both the LNBU
    LNBUMIS OFFSET(2) NUMBITS(1) [],
    /// Vertical compare masked interrupt status. Set when the both the VCompRIS bit in
    VCOMPMIS OFFSET(3) NUMBITS(1) [],
    /// AHB master bus error masked interrupt status. Set when the both the BERRAW bit i
    BERMIS OFFSET(4) NUMBITS(1) []
],
INTCLR [
    /// FIFO underflow interrupt clear. Writing a 1 to this bit clears the FIFO underflo
    FUFIC OFFSET(1) NUMBITS(1) [],
    /// LCD next address base update interrupt clear. Writing a 1 to this bit clears the
    LNBUIC OFFSET(2) NUMBITS(1) [],
    /// Vertical compare interrupt clear. Writing a 1 to this bit clears the vertical co
    VCOMPIC OFFSET(3) NUMBITS(1) [],
    /// AHB master error interrupt clear. Writing a 1 to this bit clears the AHB master
    BERIC OFFSET(4) NUMBITS(1) []
],
CRSR_CTRL [
    /// Cursor enable. 0 = Cursor is not displayed. 1 = Cursor is displayed.
    CrsrOn OFFSET(0) NUMBITS(1) [],
    /// Cursor image number. If the selected cursor size is 6x64, this field has no effe
    CRSRNUM1_0 OFFSET(4) NUMBITS(2) []
],
CRSR_CFG [
    /// Cursor size selection. 0 = 32x32 pixel cursor. Allows for 4 defined cursors. 1 =
    CrsrSize OFFSET(0) NUMBITS(1) [],
    /// Cursor frame synchronization type. 0 = Cursor coordinates are asynchronous. 1 =
    FRAMESYNC OFFSET(1) NUMBITS(1) []
],
CRSR_PAL0 [
    /// Red color component
    RED OFFSET(0) NUMBITS(8) [],
    /// Green color component
    GREEN OFFSET(8) NUMBITS(8) [],
    /// Blue color component.
    BLUE OFFSET(16) NUMBITS(8) []
],
CRSR_PAL1 [
    /// Red color component
    RED OFFSET(0) NUMBITS(8) [],
    /// Green color component
    GREEN OFFSET(8) NUMBITS(8) [],
    /// Blue color component.
    BLUE OFFSET(16) NUMBITS(8) []
],
CRSR_XY [
    /// X ordinate of the cursor origin measured in pixels. When 0, the left edge of the
    CRSRX OFFSET(0) NUMBITS(10) [],
    /// Y ordinate of the cursor origin measured in pixels. When 0, the top edge of the
    CRSRY OFFSET(16) NUMBITS(10) []
],
CRSR_CLIP [
    /// Cursor clip position for X direction. Distance from the left edge of the cursor
    CRSRCLIPX OFFSET(0) NUMBITS(6) [],
    /// Cursor clip position for Y direction. Distance from the top of the cursor image
    CRSRCLIPY OFFSET(8) NUMBITS(6) []
],
PAL[0] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[1] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[2] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[3] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[4] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[5] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[6] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[7] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[8] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[9] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[10] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[11] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[12] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[13] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[14] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[15] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[16] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[17] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[18] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[19] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[20] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[21] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[22] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[23] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[24] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[25] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[26] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[27] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[28] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[29] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[30] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[31] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[32] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[33] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[34] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[35] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[36] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[37] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[38] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[39] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[40] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[41] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[42] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[43] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[44] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[45] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[46] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[47] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[48] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[49] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[50] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[51] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[52] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[53] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[54] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[55] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[56] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[57] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[58] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[59] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[60] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[61] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[62] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[63] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[64] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[65] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[66] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[67] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[68] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[69] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[70] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[71] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[72] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[73] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[74] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[75] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[76] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[77] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[78] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[79] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[80] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[81] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[82] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[83] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[84] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[85] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[86] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[87] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[88] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[89] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[90] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[91] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[92] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[93] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[94] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[95] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[96] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[97] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[98] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[99] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[100] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[101] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[102] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[103] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[104] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[105] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[106] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[107] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[108] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[109] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[110] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[111] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[112] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[113] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[114] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[115] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[116] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[117] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[118] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[119] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[120] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[121] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[122] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[123] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[124] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[125] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[126] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[127] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[128] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[129] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[130] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[131] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[132] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[133] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[134] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[135] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[136] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[137] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[138] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[139] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[140] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[141] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[142] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[143] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[144] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[145] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[146] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[147] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[148] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[149] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[150] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[151] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[152] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[153] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[154] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[155] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[156] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[157] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[158] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[159] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[160] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[161] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[162] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[163] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[164] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[165] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[166] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[167] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[168] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[169] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[170] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[171] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[172] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[173] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[174] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[175] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[176] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[177] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[178] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[179] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[180] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[181] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[182] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[183] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[184] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[185] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[186] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[187] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[188] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[189] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[190] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[191] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[192] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[193] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[194] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[195] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[196] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[197] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[198] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[199] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[200] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[201] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[202] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[203] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[204] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[205] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[206] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[207] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[208] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[209] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[210] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[211] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[212] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[213] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[214] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[215] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[216] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[217] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[218] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[219] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[220] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[221] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[222] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[223] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[224] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[225] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[226] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[227] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[228] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[229] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[230] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[231] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[232] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[233] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[234] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[235] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[236] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[237] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[238] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[239] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[240] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[241] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[242] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[243] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[244] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[245] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[246] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[247] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[248] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[249] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[250] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[251] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[252] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[253] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[254] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
],
PAL[255] [
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R04_0 OFFSET(0) NUMBITS(5) [],
    /// Green palette data.
    G04_0 OFFSET(5) NUMBITS(5) [],
    /// Blue palette data.
    B04_0 OFFSET(10) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I0 OFFSET(15) NUMBITS(1) [],
    /// Red palette data. For STN displays, only the four MSBs, bits [4:1], are used. Fo
    R14_0 OFFSET(16) NUMBITS(5) [],
    /// Green palette data.
    G14_0 OFFSET(21) NUMBITS(5) [],
    /// Blue palette data.
    B14_0 OFFSET(26) NUMBITS(5) [],
    /// Intensity / unused bit. Can be used as the LSB of the R, G, and B inputs to a 6:
    I1 OFFSET(31) NUMBITS(1) []
]
];
const LCD_BASE: StaticRef<LcdRegisters> =
    unsafe { StaticRef::new(0x40008000 as *const LcdRegisters) };
