
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly, register_bitfields};
    /// External Memory Controller (EMC)
#[repr(C)]
struct EmcRegisters {
/// Controls operation of the memory controller.
control: ReadWrite<u32, CONTROL::Register>,
/// Provides EMC status information.
status: ReadOnly<u32, STATUS::Register>,
/// Configures operation of the memory controller.
config: ReadWrite<u32>,
_reserved0: [u8; 20],
/// Controls dynamic memory operation.
dynamiccontrol: ReadWrite<u32, DYNAMICCONTROL::Register>,
/// Configures dynamic memory refresh operation.
dynamicrefresh: ReadWrite<u32>,
/// Configures the dynamic memory read strategy.
dynamicreadconfig: ReadWrite<u32>,
_reserved1: [u8; 4],
/// Selects the precharge command period.
dynamicrp: ReadWrite<u32>,
/// Selects the active to precharge command period.
dynamicras: ReadWrite<u32>,
/// Selects the self-refresh exit time.
dynamicsrex: ReadWrite<u32>,
/// Selects the last-data-out to active command time.
dynamicapr: ReadWrite<u32>,
/// Selects the data-in to active command time.
dynamicdal: ReadWrite<u32>,
/// Selects the write recovery time.
dynamicwr: ReadWrite<u32>,
/// Selects the active to active command period.
dynamicrc: ReadWrite<u32>,
/// Selects the auto-refresh period.
dynamicrfc: ReadWrite<u32>,
/// Selects the exit self-refresh to active command time.
dynamicxsr: ReadWrite<u32>,
/// Selects the active bank A to active bank B latency.
dynamicrrd: ReadWrite<u32>,
/// Selects the load mode register to active command time.
dynamicmrd: ReadWrite<u32>,
_reserved2: [u8; 36],
/// Selects time for long static memory read and write transfers.
staticextendedwait: ReadWrite<u32>,
_reserved3: [u8; 124],
/// Selects the configuration information for dynamic memory chip select 0.
dynamicconfig0: ReadWrite<u32, DYNAMICCONFIG0::Register>,
/// Selects the RAS and CAS latencies for dynamic memory chip select 0.
dynamicrascas0: ReadWrite<u32, DYNAMICRASCAS0::Register>,
_reserved4: [u8; 24],
/// Selects the configuration information for dynamic memory chip select 0.
dynamicconfig1: ReadWrite<u32, DYNAMICCONFIG1::Register>,
/// Selects the RAS and CAS latencies for dynamic memory chip select 0.
dynamicrascas1: ReadWrite<u32, DYNAMICRASCAS1::Register>,
_reserved5: [u8; 24],
/// Selects the configuration information for dynamic memory chip select 0.
dynamicconfig2: ReadWrite<u32, DYNAMICCONFIG2::Register>,
/// Selects the RAS and CAS latencies for dynamic memory chip select 0.
dynamicrascas2: ReadWrite<u32, DYNAMICRASCAS2::Register>,
_reserved6: [u8; 24],
/// Selects the configuration information for dynamic memory chip select 0.
dynamicconfig3: ReadWrite<u32, DYNAMICCONFIG3::Register>,
/// Selects the RAS and CAS latencies for dynamic memory chip select 0.
dynamicrascas3: ReadWrite<u32, DYNAMICRASCAS3::Register>,
_reserved7: [u8; 152],
/// Selects the memory configuration for static chip select 0.
staticconfig0: ReadWrite<u32, STATICCONFIG0::Register>,
/// Selects the delay from chip select 0 to write enable.
staticwaitwen0: ReadWrite<u32>,
/// Selects the delay from chip select 0 or address change, whichever is later, to o
staticwaitoen0: ReadWrite<u32>,
/// Selects the delay from chip select 0 to a read access.
staticwaitrd0: ReadWrite<u32>,
/// Selects the delay for asynchronous page mode sequential accesses for chip select
staticwaitpage0: ReadWrite<u32>,
/// Selects the delay from chip select 0 to a write access.
staticwaitwr0: ReadWrite<u32>,
/// Selects the number of bus turnaround cycles for chip select 0.
staticwaitturn0: ReadWrite<u32>,
_reserved8: [u8; 4],
/// Selects the memory configuration for static chip select 0.
staticconfig1: ReadWrite<u32, STATICCONFIG1::Register>,
/// Selects the delay from chip select 0 to write enable.
staticwaitwen1: ReadWrite<u32>,
/// Selects the delay from chip select 0 or address change, whichever is later, to o
staticwaitoen1: ReadWrite<u32>,
/// Selects the delay from chip select 0 to a read access.
staticwaitrd1: ReadWrite<u32>,
/// Selects the delay for asynchronous page mode sequential accesses for chip select
staticwaitpage1: ReadWrite<u32>,
/// Selects the delay from chip select 0 to a write access.
staticwaitwr1: ReadWrite<u32>,
/// Selects the number of bus turnaround cycles for chip select 0.
staticwaitturn1: ReadWrite<u32>,
_reserved9: [u8; 4],
/// Selects the memory configuration for static chip select 0.
staticconfig2: ReadWrite<u32, STATICCONFIG2::Register>,
/// Selects the delay from chip select 0 to write enable.
staticwaitwen2: ReadWrite<u32>,
/// Selects the delay from chip select 0 or address change, whichever is later, to o
staticwaitoen2: ReadWrite<u32>,
/// Selects the delay from chip select 0 to a read access.
staticwaitrd2: ReadWrite<u32>,
/// Selects the delay for asynchronous page mode sequential accesses for chip select
staticwaitpage2: ReadWrite<u32>,
/// Selects the delay from chip select 0 to a write access.
staticwaitwr2: ReadWrite<u32>,
/// Selects the number of bus turnaround cycles for chip select 0.
staticwaitturn2: ReadWrite<u32>,
_reserved10: [u8; 4],
/// Selects the memory configuration for static chip select 0.
staticconfig3: ReadWrite<u32, STATICCONFIG3::Register>,
/// Selects the delay from chip select 0 to write enable.
staticwaitwen3: ReadWrite<u32>,
/// Selects the delay from chip select 0 or address change, whichever is later, to o
staticwaitoen3: ReadWrite<u32>,
/// Selects the delay from chip select 0 to a read access.
staticwaitrd3: ReadWrite<u32>,
/// Selects the delay for asynchronous page mode sequential accesses for chip select
staticwaitpage3: ReadWrite<u32>,
/// Selects the delay from chip select 0 to a write access.
staticwaitwr3: ReadWrite<u32>,
/// Selects the number of bus turnaround cycles for chip select 0.
staticwaitturn3: ReadWrite<u32>,
}
register_bitfields![u32,
CONTROL [
    /// EMC Enable. Indicates if the EMC is enabled or disabled.Disabling the EMC reduce
    E OFFSET(0) NUMBITS(1) [
        /// Disabled
        Disabled = 0,
        /// Enabled. (POR and warm reset value).
        EnabledPORAndWarmResetValue = 1
    ],
    /// Address mirror. Indicates normal or reset memory map. On POR, CS1 is mirrored to
    M OFFSET(1) NUMBITS(1) [
        /// Normal. Normal memory map.
        NormalNormalMemoryMap = 0,
        /// Reset. Reset memory map. Static memory CS1 is mirrored onto CS0 and DYCS0 (POR r
        ResetResetMemoryMapStaticMemoryCS1IsMirroredOntoCS0AndDYCS0PORResetValue = 1
    ],
    /// Low-power mode. Indicates normal, or low-power mode. Entering low-power mode red
    L OFFSET(2) NUMBITS(1) [
        /// Normal. Normal mode (warm reset value).
        NormalNormalModeWarmResetValue = 0,
        /// Low-power mode.
        LowPowerMode = 1
    ]
],
STATUS [
    /// Busy indicator. This bit is used to ensure that the memory controller enters the
    B OFFSET(0) NUMBITS(1) [
        /// Idle. EMC is idle (warm reset value).
        IdleEMCIsIdleWarmResetValue = 0,
        /// Busy. EMC is busy performing memory transactions, commands, auto-refresh cycles,
        BUSY = 1
    ],
    /// Write buffer status. This bit enables the EMC to enter low-power mode or disable
    S OFFSET(1) NUMBITS(1) [
        /// Empty. Write buffers empty (POR reset value)
        EmptyWriteBuffersEmptyPORResetValue = 0,
        /// Data. Write buffers contain data.
        DataWriteBuffersContainData = 1
    ],
    /// Self-refresh acknowledge. This bit indicates the operating mode of the EMC:
    SA OFFSET(2) NUMBITS(1) [
        /// Normal mode.
        NormalMode = 0,
        /// Self-refresh mode. (POR reset value.)
        SelfRefreshModePORResetValue = 1
    ]
],
DYNAMICCONTROL [
    /// Dynamic memory clock enable.
    CE OFFSET(0) NUMBITS(1) [
        /// Disabled. Clock enable of idle devices are deasserted to save power (POR reset v
        DisabledClockEnableOfIdleDevicesAreDeassertedToSavePowerPORResetValue = 0,
        /// Enabled. All clock enables are driven HIGH continuously.[1]
        EnabledAllClockEnablesAreDrivenHIGHContinuously1 = 1
    ],
    /// Dynamic memory clock control. When clock control is LOW the output clock CLKOUT
    CS OFFSET(1) NUMBITS(1) [
        /// Stop. CLKOUT stops when all SDRAMs are idle and during self-refresh mode.
        StopCLKOUTStopsWhenAllSDRAMsAreIdleAndDuringSelfRefreshMode = 0,
        /// Run. CLKOUT runs continuously (POR reset value).
        RunCLKOUTRunsContinuouslyPORResetValue = 1
    ],
    /// Self-refresh request, EMC SREFREQ. By writing 1 to this bit self-refresh can be
    SR OFFSET(2) NUMBITS(1) [
        /// Normal mode.
        NormalMode = 0,
        /// Self-refresh. Enter self-refresh mode (POR reset value).
        SelfRefreshEnterSelfRefreshModePORResetValue = 1
    ],
    /// Memory clock control.
    MMC OFFSET(5) NUMBITS(1) [
        /// Enabled. CLKOUT enabled (POR reset value).
        EnabledCLKOUTEnabledPORResetValue = 0,
        /// Disabled. CLKOUT disabled.[3]
        DisabledCLKOUTDisabled3 = 1
    ],
    /// SDRAM initialization.
    I OFFSET(7) NUMBITS(2) [
        /// Normal. Issue SDRAM NORMAL operation command (POR reset value).
        NormalIssueSDRAMNORMALOperationCommandPORResetValue = 0,
        /// Mode. Issue SDRAM MODE command.
        ModeIssueSDRAMMODECommand = 1,
        /// PALL. Issue SDRAM PALL (precharge all) command.
        PALLIssueSDRAMPALLPrechargeAllCommand = 2,
        /// NOP. Issue SDRAM NOP (no operation) command)
        NOPIssueSDRAMNOPNoOperationCommand = 3
    ]
],
DYNAMICCONFIG0 [
    /// Memory device.
    MD OFFSET(3) NUMBITS(2) [
        /// SDRAM (POR reset value).
        SDRAMPORResetValue = 0,
        /// Reserved.
        Reserved = 1
    ],
    /// Address mapping. See Table 382. 000000 = reset value.[1]
    AM0 OFFSET(7) NUMBITS(6) [],
    /// Address mapping See Table 382. 0 = reset value.
    AM1 OFFSET(14) NUMBITS(1) [],
    /// Buffer enable.
    B OFFSET(19) NUMBITS(1) [
        /// Disabled. Buffer disabled for accesses to this chip select (POR reset value).
        DisabledBufferDisabledForAccessesToThisChipSelectPORResetValue = 0,
        /// Enabled. Buffer enabled for accesses to this chip select. After configuration of
        ENABLED = 1
    ],
    /// Write protect.
    P OFFSET(20) NUMBITS(1) [
        /// None. Writes not protected (POR reset value).
        NoneWritesNotProtectedPORResetValue = 0,
        /// Protected. Writes protected.
        ProtectedWritesProtected = 1
    ]
],
DYNAMICCONFIG1 [
    /// Memory device.
    MD OFFSET(3) NUMBITS(2) [
        /// SDRAM (POR reset value).
        SDRAMPORResetValue = 0,
        /// Reserved.
        Reserved = 1
    ],
    /// Address mapping. See Table 382. 000000 = reset value.[1]
    AM0 OFFSET(7) NUMBITS(6) [],
    /// Address mapping See Table 382. 0 = reset value.
    AM1 OFFSET(14) NUMBITS(1) [],
    /// Buffer enable.
    B OFFSET(19) NUMBITS(1) [
        /// Disabled. Buffer disabled for accesses to this chip select (POR reset value).
        DisabledBufferDisabledForAccessesToThisChipSelectPORResetValue = 0,
        /// Enabled. Buffer enabled for accesses to this chip select. After configuration of
        ENABLED = 1
    ],
    /// Write protect.
    P OFFSET(20) NUMBITS(1) [
        /// None. Writes not protected (POR reset value).
        NoneWritesNotProtectedPORResetValue = 0,
        /// Protected. Writes protected.
        ProtectedWritesProtected = 1
    ]
],
DYNAMICCONFIG2 [
    /// Memory device.
    MD OFFSET(3) NUMBITS(2) [
        /// SDRAM (POR reset value).
        SDRAMPORResetValue = 0,
        /// Reserved.
        Reserved = 1
    ],
    /// Address mapping. See Table 382. 000000 = reset value.[1]
    AM0 OFFSET(7) NUMBITS(6) [],
    /// Address mapping See Table 382. 0 = reset value.
    AM1 OFFSET(14) NUMBITS(1) [],
    /// Buffer enable.
    B OFFSET(19) NUMBITS(1) [
        /// Disabled. Buffer disabled for accesses to this chip select (POR reset value).
        DisabledBufferDisabledForAccessesToThisChipSelectPORResetValue = 0,
        /// Enabled. Buffer enabled for accesses to this chip select. After configuration of
        ENABLED = 1
    ],
    /// Write protect.
    P OFFSET(20) NUMBITS(1) [
        /// None. Writes not protected (POR reset value).
        NoneWritesNotProtectedPORResetValue = 0,
        /// Protected. Writes protected.
        ProtectedWritesProtected = 1
    ]
],
DYNAMICCONFIG3 [
    /// Memory device.
    MD OFFSET(3) NUMBITS(2) [
        /// SDRAM (POR reset value).
        SDRAMPORResetValue = 0,
        /// Reserved.
        Reserved = 1
    ],
    /// Address mapping. See Table 382. 000000 = reset value.[1]
    AM0 OFFSET(7) NUMBITS(6) [],
    /// Address mapping See Table 382. 0 = reset value.
    AM1 OFFSET(14) NUMBITS(1) [],
    /// Buffer enable.
    B OFFSET(19) NUMBITS(1) [
        /// Disabled. Buffer disabled for accesses to this chip select (POR reset value).
        DisabledBufferDisabledForAccessesToThisChipSelectPORResetValue = 0,
        /// Enabled. Buffer enabled for accesses to this chip select. After configuration of
        ENABLED = 1
    ],
    /// Write protect.
    P OFFSET(20) NUMBITS(1) [
        /// None. Writes not protected (POR reset value).
        NoneWritesNotProtectedPORResetValue = 0,
        /// Protected. Writes protected.
        ProtectedWritesProtected = 1
    ]
],
DYNAMICRASCAS0 [
    /// RAS latency (active to read/write delay).
    RAS OFFSET(0) NUMBITS(2) [
        /// Reserved.
        Reserved = 0,
        /// One EMC_CCLK cycle.
        OneEMC_CCLKCycle = 1,
        /// Two EMC_CCLK cycles.
        TwoEMC_CCLKCycles = 2,
        /// Three EMC_CCLK cycles (POR reset value).
        ThreeEMC_CCLKCyclesPORResetValue = 3
    ],
    /// CAS latency.
    CAS OFFSET(8) NUMBITS(2) [
        /// Reserved.
        Reserved = 0,
        /// One EMC_CCLK cycle.
        OneEMC_CCLKCycle = 1,
        /// Two EMC_CCLK cycles.
        TwoEMC_CCLKCycles = 2,
        /// Three EMC_CCLK cycles (POR reset value).
        ThreeEMC_CCLKCyclesPORResetValue = 3
    ]
],
DYNAMICRASCAS1 [
    /// RAS latency (active to read/write delay).
    RAS OFFSET(0) NUMBITS(2) [
        /// Reserved.
        Reserved = 0,
        /// One EMC_CCLK cycle.
        OneEMC_CCLKCycle = 1,
        /// Two EMC_CCLK cycles.
        TwoEMC_CCLKCycles = 2,
        /// Three EMC_CCLK cycles (POR reset value).
        ThreeEMC_CCLKCyclesPORResetValue = 3
    ],
    /// CAS latency.
    CAS OFFSET(8) NUMBITS(2) [
        /// Reserved.
        Reserved = 0,
        /// One EMC_CCLK cycle.
        OneEMC_CCLKCycle = 1,
        /// Two EMC_CCLK cycles.
        TwoEMC_CCLKCycles = 2,
        /// Three EMC_CCLK cycles (POR reset value).
        ThreeEMC_CCLKCyclesPORResetValue = 3
    ]
],
DYNAMICRASCAS2 [
    /// RAS latency (active to read/write delay).
    RAS OFFSET(0) NUMBITS(2) [
        /// Reserved.
        Reserved = 0,
        /// One EMC_CCLK cycle.
        OneEMC_CCLKCycle = 1,
        /// Two EMC_CCLK cycles.
        TwoEMC_CCLKCycles = 2,
        /// Three EMC_CCLK cycles (POR reset value).
        ThreeEMC_CCLKCyclesPORResetValue = 3
    ],
    /// CAS latency.
    CAS OFFSET(8) NUMBITS(2) [
        /// Reserved.
        Reserved = 0,
        /// One EMC_CCLK cycle.
        OneEMC_CCLKCycle = 1,
        /// Two EMC_CCLK cycles.
        TwoEMC_CCLKCycles = 2,
        /// Three EMC_CCLK cycles (POR reset value).
        ThreeEMC_CCLKCyclesPORResetValue = 3
    ]
],
DYNAMICRASCAS3 [
    /// RAS latency (active to read/write delay).
    RAS OFFSET(0) NUMBITS(2) [
        /// Reserved.
        Reserved = 0,
        /// One EMC_CCLK cycle.
        OneEMC_CCLKCycle = 1,
        /// Two EMC_CCLK cycles.
        TwoEMC_CCLKCycles = 2,
        /// Three EMC_CCLK cycles (POR reset value).
        ThreeEMC_CCLKCyclesPORResetValue = 3
    ],
    /// CAS latency.
    CAS OFFSET(8) NUMBITS(2) [
        /// Reserved.
        Reserved = 0,
        /// One EMC_CCLK cycle.
        OneEMC_CCLKCycle = 1,
        /// Two EMC_CCLK cycles.
        TwoEMC_CCLKCycles = 2,
        /// Three EMC_CCLK cycles (POR reset value).
        ThreeEMC_CCLKCyclesPORResetValue = 3
    ]
],
STATICCONFIG0 [
    /// Memory width.
    MW OFFSET(0) NUMBITS(2) [
        /// 8 bit (POR reset value).
        _8BitPORResetValue = 0,
        /// 16 bit.
        _16Bit = 1,
        /// 32 bit.
        _32Bit = 2,
        /// Reserved.
        Reserved = 3
    ],
    /// Page mode. In page mode the EMC can burst up to four external accesses. Therefor
    PM OFFSET(3) NUMBITS(1) [
        /// Disabled. (POR reset value.)
        DisabledPORResetValue = 0,
        /// Enabled. Async page mode enabled (page length four).
        EnabledAsyncPageModeEnabledPageLengthFour = 1
    ],
    /// Chip select polarity. The value of the chip select polarity on power-on reset is
    PC OFFSET(6) NUMBITS(1) [
        /// Active LOW chip select.
        ActiveLOWChipSelect = 0,
        /// Active HIGH chip select.
        ActiveHIGHChipSelect = 1
    ],
    /// Byte lane state. The byte lane state bit, PB, enables different types of memory
    PB OFFSET(7) NUMBITS(1) [
        /// High. For reads all the bits in BLSn[3:0] are HIGH. For writes the respective ac
        HIGH = 0,
        /// Low. For reads the respective active bits in BLSn[3:0] are LOW. For writes the r
        LOW = 1
    ],
    /// Extended wait. Extended wait (EW) uses the StaticExtendedWait register to time b
    EW OFFSET(8) NUMBITS(1) [
        /// Disabled. Extended wait disabled (POR reset value).
        DisabledExtendedWaitDisabledPORResetValue = 0,
        /// Enabled. Extended wait enabled.
        EnabledExtendedWaitEnabled = 1
    ],
    /// Buffer enable [2].
    B OFFSET(19) NUMBITS(1) [
        /// Disabled. Buffer disabled (POR reset value).
        DisabledBufferDisabledPORResetValue = 0,
        /// Enabled. Buffer enabled.
        EnabledBufferEnabled = 1
    ],
    /// Write protect.
    P OFFSET(20) NUMBITS(1) [
        /// None. Writes not protected (POR reset value).
        NoneWritesNotProtectedPORResetValue = 0,
        /// Protect. Write protected.
        ProtectWriteProtected = 1
    ]
],
STATICCONFIG1 [
    /// Memory width.
    MW OFFSET(0) NUMBITS(2) [
        /// 8 bit (POR reset value).
        _8BitPORResetValue = 0,
        /// 16 bit.
        _16Bit = 1,
        /// 32 bit.
        _32Bit = 2,
        /// Reserved.
        Reserved = 3
    ],
    /// Page mode. In page mode the EMC can burst up to four external accesses. Therefor
    PM OFFSET(3) NUMBITS(1) [
        /// Disabled. (POR reset value.)
        DisabledPORResetValue = 0,
        /// Enabled. Async page mode enabled (page length four).
        EnabledAsyncPageModeEnabledPageLengthFour = 1
    ],
    /// Chip select polarity. The value of the chip select polarity on power-on reset is
    PC OFFSET(6) NUMBITS(1) [
        /// Active LOW chip select.
        ActiveLOWChipSelect = 0,
        /// Active HIGH chip select.
        ActiveHIGHChipSelect = 1
    ],
    /// Byte lane state. The byte lane state bit, PB, enables different types of memory
    PB OFFSET(7) NUMBITS(1) [
        /// High. For reads all the bits in BLSn[3:0] are HIGH. For writes the respective ac
        HIGH = 0,
        /// Low. For reads the respective active bits in BLSn[3:0] are LOW. For writes the r
        LOW = 1
    ],
    /// Extended wait. Extended wait (EW) uses the StaticExtendedWait register to time b
    EW OFFSET(8) NUMBITS(1) [
        /// Disabled. Extended wait disabled (POR reset value).
        DisabledExtendedWaitDisabledPORResetValue = 0,
        /// Enabled. Extended wait enabled.
        EnabledExtendedWaitEnabled = 1
    ],
    /// Buffer enable [2].
    B OFFSET(19) NUMBITS(1) [
        /// Disabled. Buffer disabled (POR reset value).
        DisabledBufferDisabledPORResetValue = 0,
        /// Enabled. Buffer enabled.
        EnabledBufferEnabled = 1
    ],
    /// Write protect.
    P OFFSET(20) NUMBITS(1) [
        /// None. Writes not protected (POR reset value).
        NoneWritesNotProtectedPORResetValue = 0,
        /// Protect. Write protected.
        ProtectWriteProtected = 1
    ]
],
STATICCONFIG2 [
    /// Memory width.
    MW OFFSET(0) NUMBITS(2) [
        /// 8 bit (POR reset value).
        _8BitPORResetValue = 0,
        /// 16 bit.
        _16Bit = 1,
        /// 32 bit.
        _32Bit = 2,
        /// Reserved.
        Reserved = 3
    ],
    /// Page mode. In page mode the EMC can burst up to four external accesses. Therefor
    PM OFFSET(3) NUMBITS(1) [
        /// Disabled. (POR reset value.)
        DisabledPORResetValue = 0,
        /// Enabled. Async page mode enabled (page length four).
        EnabledAsyncPageModeEnabledPageLengthFour = 1
    ],
    /// Chip select polarity. The value of the chip select polarity on power-on reset is
    PC OFFSET(6) NUMBITS(1) [
        /// Active LOW chip select.
        ActiveLOWChipSelect = 0,
        /// Active HIGH chip select.
        ActiveHIGHChipSelect = 1
    ],
    /// Byte lane state. The byte lane state bit, PB, enables different types of memory
    PB OFFSET(7) NUMBITS(1) [
        /// High. For reads all the bits in BLSn[3:0] are HIGH. For writes the respective ac
        HIGH = 0,
        /// Low. For reads the respective active bits in BLSn[3:0] are LOW. For writes the r
        LOW = 1
    ],
    /// Extended wait. Extended wait (EW) uses the StaticExtendedWait register to time b
    EW OFFSET(8) NUMBITS(1) [
        /// Disabled. Extended wait disabled (POR reset value).
        DisabledExtendedWaitDisabledPORResetValue = 0,
        /// Enabled. Extended wait enabled.
        EnabledExtendedWaitEnabled = 1
    ],
    /// Buffer enable [2].
    B OFFSET(19) NUMBITS(1) [
        /// Disabled. Buffer disabled (POR reset value).
        DisabledBufferDisabledPORResetValue = 0,
        /// Enabled. Buffer enabled.
        EnabledBufferEnabled = 1
    ],
    /// Write protect.
    P OFFSET(20) NUMBITS(1) [
        /// None. Writes not protected (POR reset value).
        NoneWritesNotProtectedPORResetValue = 0,
        /// Protect. Write protected.
        ProtectWriteProtected = 1
    ]
],
STATICCONFIG3 [
    /// Memory width.
    MW OFFSET(0) NUMBITS(2) [
        /// 8 bit (POR reset value).
        _8BitPORResetValue = 0,
        /// 16 bit.
        _16Bit = 1,
        /// 32 bit.
        _32Bit = 2,
        /// Reserved.
        Reserved = 3
    ],
    /// Page mode. In page mode the EMC can burst up to four external accesses. Therefor
    PM OFFSET(3) NUMBITS(1) [
        /// Disabled. (POR reset value.)
        DisabledPORResetValue = 0,
        /// Enabled. Async page mode enabled (page length four).
        EnabledAsyncPageModeEnabledPageLengthFour = 1
    ],
    /// Chip select polarity. The value of the chip select polarity on power-on reset is
    PC OFFSET(6) NUMBITS(1) [
        /// Active LOW chip select.
        ActiveLOWChipSelect = 0,
        /// Active HIGH chip select.
        ActiveHIGHChipSelect = 1
    ],
    /// Byte lane state. The byte lane state bit, PB, enables different types of memory
    PB OFFSET(7) NUMBITS(1) [
        /// High. For reads all the bits in BLSn[3:0] are HIGH. For writes the respective ac
        HIGH = 0,
        /// Low. For reads the respective active bits in BLSn[3:0] are LOW. For writes the r
        LOW = 1
    ],
    /// Extended wait. Extended wait (EW) uses the StaticExtendedWait register to time b
    EW OFFSET(8) NUMBITS(1) [
        /// Disabled. Extended wait disabled (POR reset value).
        DisabledExtendedWaitDisabledPORResetValue = 0,
        /// Enabled. Extended wait enabled.
        EnabledExtendedWaitEnabled = 1
    ],
    /// Buffer enable [2].
    B OFFSET(19) NUMBITS(1) [
        /// Disabled. Buffer disabled (POR reset value).
        DisabledBufferDisabledPORResetValue = 0,
        /// Enabled. Buffer enabled.
        EnabledBufferEnabled = 1
    ],
    /// Write protect.
    P OFFSET(20) NUMBITS(1) [
        /// None. Writes not protected (POR reset value).
        NoneWritesNotProtectedPORResetValue = 0,
        /// Protect. Write protected.
        ProtectWriteProtected = 1
    ]
]
];
const EMC_BASE: StaticRef<EmcRegisters> =
    unsafe { StaticRef::new(0x40005000 as *const EmcRegisters) };
