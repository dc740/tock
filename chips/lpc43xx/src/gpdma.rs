
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// General Purpose DMA (GPDMA)
#[repr(C)]
struct GpdmaRegisters {
/// DMA Interrupt Status Register
intstat: ReadOnly<u32, INTSTAT::Register>,
/// DMA Interrupt Terminal Count Request Status Register
inttcstat: ReadOnly<u32, INTTCSTAT::Register>,
/// DMA Interrupt Terminal Count Request Clear Register
inttcclear: WriteOnly<u32, INTTCCLEAR::Register>,
/// DMA Interrupt Error Status Register
interrstat: ReadOnly<u32, INTERRSTAT::Register>,
/// DMA Interrupt Error Clear Register
interrclr: WriteOnly<u32, INTERRCLR::Register>,
/// DMA Raw Interrupt Terminal Count Status Register
rawinttcstat: ReadOnly<u32, RAWINTTCSTAT::Register>,
/// DMA Raw Error Interrupt Status Register
rawinterrstat: ReadOnly<u32, RAWINTERRSTAT::Register>,
/// DMA Enabled Channel Register
enbldchns: ReadOnly<u32, ENBLDCHNS::Register>,
/// DMA Software Burst Request Register
softbreq: ReadWrite<u32, SOFTBREQ::Register>,
/// DMA Software Single Request Register
softsreq: ReadWrite<u32, SOFTSREQ::Register>,
/// DMA Software Last Burst Request Register
softlbreq: ReadWrite<u32, SOFTLBREQ::Register>,
/// DMA Software Last Single Request Register
softlsreq: ReadWrite<u32, SOFTLSREQ::Register>,
/// DMA Configuration Register
config: ReadWrite<u32, CONFIG::Register>,
/// DMA Synchronization Register
sync: ReadWrite<u32, SYNC::Register>,
_reserved0: [u8; 200],
/// DMA Channel  Source Address Register
c0srcaddr: ReadWrite<u32>,
/// DMA Channel  Destination Address Register
c0destaddr: ReadWrite<u32>,
/// DMA Channel Linked List Item Register
c0lli: ReadWrite<u32, C0LLI::Register>,
/// DMA Channel  Control Register
c0control: ReadWrite<u32, C0CONTROL::Register>,
/// DMA Channel Configuration Register
c0config: ReadWrite<u32, C0CONFIG::Register>,
_reserved1: [u8; 12],
/// DMA Channel  Source Address Register
c1srcaddr: ReadWrite<u32>,
/// DMA Channel  Destination Address Register
c1destaddr: ReadWrite<u32>,
/// DMA Channel Linked List Item Register
c1lli: ReadWrite<u32, C1LLI::Register>,
/// DMA Channel  Control Register
c1control: ReadWrite<u32, C1CONTROL::Register>,
/// DMA Channel Configuration Register
c1config: ReadWrite<u32, C1CONFIG::Register>,
_reserved2: [u8; 12],
/// DMA Channel  Source Address Register
c2srcaddr: ReadWrite<u32>,
/// DMA Channel  Destination Address Register
c2destaddr: ReadWrite<u32>,
/// DMA Channel Linked List Item Register
c2lli: ReadWrite<u32, C2LLI::Register>,
/// DMA Channel  Control Register
c2control: ReadWrite<u32, C2CONTROL::Register>,
/// DMA Channel Configuration Register
c2config: ReadWrite<u32, C2CONFIG::Register>,
_reserved3: [u8; 12],
/// DMA Channel  Source Address Register
c3srcaddr: ReadWrite<u32>,
/// DMA Channel  Destination Address Register
c3destaddr: ReadWrite<u32>,
/// DMA Channel Linked List Item Register
c3lli: ReadWrite<u32, C3LLI::Register>,
/// DMA Channel  Control Register
c3control: ReadWrite<u32, C3CONTROL::Register>,
/// DMA Channel Configuration Register
c3config: ReadWrite<u32, C3CONFIG::Register>,
_reserved4: [u8; 12],
/// DMA Channel  Source Address Register
c4srcaddr: ReadWrite<u32>,
/// DMA Channel  Destination Address Register
c4destaddr: ReadWrite<u32>,
/// DMA Channel Linked List Item Register
c4lli: ReadWrite<u32, C4LLI::Register>,
/// DMA Channel  Control Register
c4control: ReadWrite<u32, C4CONTROL::Register>,
/// DMA Channel Configuration Register
c4config: ReadWrite<u32, C4CONFIG::Register>,
_reserved5: [u8; 12],
/// DMA Channel  Source Address Register
c5srcaddr: ReadWrite<u32>,
/// DMA Channel  Destination Address Register
c5destaddr: ReadWrite<u32>,
/// DMA Channel Linked List Item Register
c5lli: ReadWrite<u32, C5LLI::Register>,
/// DMA Channel  Control Register
c5control: ReadWrite<u32, C5CONTROL::Register>,
/// DMA Channel Configuration Register
c5config: ReadWrite<u32, C5CONFIG::Register>,
_reserved6: [u8; 12],
/// DMA Channel  Source Address Register
c6srcaddr: ReadWrite<u32>,
/// DMA Channel  Destination Address Register
c6destaddr: ReadWrite<u32>,
/// DMA Channel Linked List Item Register
c6lli: ReadWrite<u32, C6LLI::Register>,
/// DMA Channel  Control Register
c6control: ReadWrite<u32, C6CONTROL::Register>,
/// DMA Channel Configuration Register
c6config: ReadWrite<u32, C6CONFIG::Register>,
_reserved7: [u8; 12],
/// DMA Channel  Source Address Register
c7srcaddr: ReadWrite<u32>,
/// DMA Channel  Destination Address Register
c7destaddr: ReadWrite<u32>,
/// DMA Channel Linked List Item Register
c7lli: ReadWrite<u32, C7LLI::Register>,
/// DMA Channel  Control Register
c7control: ReadWrite<u32, C7CONTROL::Register>,
/// DMA Channel Configuration Register
c7config: ReadWrite<u32, C7CONFIG::Register>,
}
register_bitfields![u32,
INTSTAT [
    /// Status of DMA channel interrupts after masking. Each bit represents one channel:
    INTSTAT0 OFFSET(0) NUMBITS(1) [],
    /// Status of DMA channel interrupts after masking. Each bit represents one channel:
    INTSTAT1 OFFSET(1) NUMBITS(1) [],
    /// Status of DMA channel interrupts after masking. Each bit represents one channel:
    INTSTAT2 OFFSET(2) NUMBITS(1) [],
    /// Status of DMA channel interrupts after masking. Each bit represents one channel:
    INTSTAT3 OFFSET(3) NUMBITS(1) [],
    /// Status of DMA channel interrupts after masking. Each bit represents one channel:
    INTSTAT4 OFFSET(4) NUMBITS(1) [],
    /// Status of DMA channel interrupts after masking. Each bit represents one channel:
    INTSTAT5 OFFSET(5) NUMBITS(1) [],
    /// Status of DMA channel interrupts after masking. Each bit represents one channel:
    INTSTAT6 OFFSET(6) NUMBITS(1) [],
    /// Status of DMA channel interrupts after masking. Each bit represents one channel:
    INTSTAT7 OFFSET(7) NUMBITS(1) []
],
INTTCSTAT [
    /// Terminal count interrupt request status for DMA channels. Each bit represents on
    INTTCSTAT0 OFFSET(0) NUMBITS(1) [],
    /// Terminal count interrupt request status for DMA channels. Each bit represents on
    INTTCSTAT1 OFFSET(1) NUMBITS(1) [],
    /// Terminal count interrupt request status for DMA channels. Each bit represents on
    INTTCSTAT2 OFFSET(2) NUMBITS(1) [],
    /// Terminal count interrupt request status for DMA channels. Each bit represents on
    INTTCSTAT3 OFFSET(3) NUMBITS(1) [],
    /// Terminal count interrupt request status for DMA channels. Each bit represents on
    INTTCSTAT4 OFFSET(4) NUMBITS(1) [],
    /// Terminal count interrupt request status for DMA channels. Each bit represents on
    INTTCSTAT5 OFFSET(5) NUMBITS(1) [],
    /// Terminal count interrupt request status for DMA channels. Each bit represents on
    INTTCSTAT6 OFFSET(6) NUMBITS(1) [],
    /// Terminal count interrupt request status for DMA channels. Each bit represents on
    INTTCSTAT7 OFFSET(7) NUMBITS(1) []
],
INTTCCLEAR [
    /// Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channel
    INTTCCLEAR0 OFFSET(0) NUMBITS(1) [],
    /// Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channel
    INTTCCLEAR1 OFFSET(1) NUMBITS(1) [],
    /// Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channel
    INTTCCLEAR2 OFFSET(2) NUMBITS(1) [],
    /// Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channel
    INTTCCLEAR3 OFFSET(3) NUMBITS(1) [],
    /// Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channel
    INTTCCLEAR4 OFFSET(4) NUMBITS(1) [],
    /// Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channel
    INTTCCLEAR5 OFFSET(5) NUMBITS(1) [],
    /// Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channel
    INTTCCLEAR6 OFFSET(6) NUMBITS(1) [],
    /// Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channel
    INTTCCLEAR7 OFFSET(7) NUMBITS(1) []
],
INTERRSTAT [
    /// Interrupt error status for DMA channels. Each bit represents one channel: 0 - th
    INTERRSTAT0 OFFSET(0) NUMBITS(1) [],
    /// Interrupt error status for DMA channels. Each bit represents one channel: 0 - th
    INTERRSTAT1 OFFSET(1) NUMBITS(1) [],
    /// Interrupt error status for DMA channels. Each bit represents one channel: 0 - th
    INTERRSTAT2 OFFSET(2) NUMBITS(1) [],
    /// Interrupt error status for DMA channels. Each bit represents one channel: 0 - th
    INTERRSTAT3 OFFSET(3) NUMBITS(1) [],
    /// Interrupt error status for DMA channels. Each bit represents one channel: 0 - th
    INTERRSTAT4 OFFSET(4) NUMBITS(1) [],
    /// Interrupt error status for DMA channels. Each bit represents one channel: 0 - th
    INTERRSTAT5 OFFSET(5) NUMBITS(1) [],
    /// Interrupt error status for DMA channels. Each bit represents one channel: 0 - th
    INTERRSTAT6 OFFSET(6) NUMBITS(1) [],
    /// Interrupt error status for DMA channels. Each bit represents one channel: 0 - th
    INTERRSTAT7 OFFSET(7) NUMBITS(1) []
],
INTERRCLR [
    /// Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Ea
    INTERRCLR0 OFFSET(0) NUMBITS(1) [],
    /// Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Ea
    INTERRCLR1 OFFSET(1) NUMBITS(1) [],
    /// Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Ea
    INTERRCLR2 OFFSET(2) NUMBITS(1) [],
    /// Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Ea
    INTERRCLR3 OFFSET(3) NUMBITS(1) [],
    /// Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Ea
    INTERRCLR4 OFFSET(4) NUMBITS(1) [],
    /// Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Ea
    INTERRCLR5 OFFSET(5) NUMBITS(1) [],
    /// Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Ea
    INTERRCLR6 OFFSET(6) NUMBITS(1) [],
    /// Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Ea
    INTERRCLR7 OFFSET(7) NUMBITS(1) []
],
RAWINTTCSTAT [
    /// Status of the terminal count interrupt for DMA channels prior to masking. Each b
    RAWINTTCSTAT0 OFFSET(0) NUMBITS(1) [],
    /// Status of the terminal count interrupt for DMA channels prior to masking. Each b
    RAWINTTCSTAT1 OFFSET(1) NUMBITS(1) [],
    /// Status of the terminal count interrupt for DMA channels prior to masking. Each b
    RAWINTTCSTAT2 OFFSET(2) NUMBITS(1) [],
    /// Status of the terminal count interrupt for DMA channels prior to masking. Each b
    RAWINTTCSTAT3 OFFSET(3) NUMBITS(1) [],
    /// Status of the terminal count interrupt for DMA channels prior to masking. Each b
    RAWINTTCSTAT4 OFFSET(4) NUMBITS(1) [],
    /// Status of the terminal count interrupt for DMA channels prior to masking. Each b
    RAWINTTCSTAT5 OFFSET(5) NUMBITS(1) [],
    /// Status of the terminal count interrupt for DMA channels prior to masking. Each b
    RAWINTTCSTAT6 OFFSET(6) NUMBITS(1) [],
    /// Status of the terminal count interrupt for DMA channels prior to masking. Each b
    RAWINTTCSTAT7 OFFSET(7) NUMBITS(1) []
],
RAWINTERRSTAT [
    /// Status of the error interrupt for DMA channels prior to masking. Each bit repres
    RAWINTERRSTAT0 OFFSET(0) NUMBITS(1) [],
    /// Status of the error interrupt for DMA channels prior to masking. Each bit repres
    RAWINTERRSTAT1 OFFSET(1) NUMBITS(1) [],
    /// Status of the error interrupt for DMA channels prior to masking. Each bit repres
    RAWINTERRSTAT2 OFFSET(2) NUMBITS(1) [],
    /// Status of the error interrupt for DMA channels prior to masking. Each bit repres
    RAWINTERRSTAT3 OFFSET(3) NUMBITS(1) [],
    /// Status of the error interrupt for DMA channels prior to masking. Each bit repres
    RAWINTERRSTAT4 OFFSET(4) NUMBITS(1) [],
    /// Status of the error interrupt for DMA channels prior to masking. Each bit repres
    RAWINTERRSTAT5 OFFSET(5) NUMBITS(1) [],
    /// Status of the error interrupt for DMA channels prior to masking. Each bit repres
    RAWINTERRSTAT6 OFFSET(6) NUMBITS(1) [],
    /// Status of the error interrupt for DMA channels prior to masking. Each bit repres
    RAWINTERRSTAT7 OFFSET(7) NUMBITS(1) []
],
ENBLDCHNS [
    /// Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel
    ENABLEDCHANNELS0 OFFSET(0) NUMBITS(1) [],
    /// Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel
    ENABLEDCHANNELS1 OFFSET(1) NUMBITS(1) [],
    /// Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel
    ENABLEDCHANNELS2 OFFSET(2) NUMBITS(1) [],
    /// Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel
    ENABLEDCHANNELS3 OFFSET(3) NUMBITS(1) [],
    /// Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel
    ENABLEDCHANNELS4 OFFSET(4) NUMBITS(1) [],
    /// Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel
    ENABLEDCHANNELS5 OFFSET(5) NUMBITS(1) [],
    /// Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel
    ENABLEDCHANNELS6 OFFSET(6) NUMBITS(1) [],
    /// Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel
    ENABLEDCHANNELS7 OFFSET(7) NUMBITS(1) []
],
SOFTBREQ [
    /// Software burst request flags for each of 16 possible sources. Each bit represent
    SOFTBREQ0 OFFSET(0) NUMBITS(1) [],
    /// Software burst request flags for each of 16 possible sources. Each bit represent
    SOFTBREQ1 OFFSET(1) NUMBITS(1) [],
    /// Software burst request flags for each of 16 possible sources. Each bit represent
    SOFTBREQ2 OFFSET(2) NUMBITS(1) [],
    /// Software burst request flags for each of 16 possible sources. Each bit represent
    SOFTBREQ3 OFFSET(3) NUMBITS(1) [],
    /// Software burst request flags for each of 16 possible sources. Each bit represent
    SOFTBREQ4 OFFSET(4) NUMBITS(1) [],
    /// Software burst request flags for each of 16 possible sources. Each bit represent
    SOFTBREQ5 OFFSET(5) NUMBITS(1) [],
    /// Software burst request flags for each of 16 possible sources. Each bit represent
    SOFTBREQ6 OFFSET(6) NUMBITS(1) [],
    /// Software burst request flags for each of 16 possible sources. Each bit represent
    SOFTBREQ7 OFFSET(7) NUMBITS(1) [],
    /// Software burst request flags for each of 16 possible sources. Each bit represent
    SOFTBREQ8 OFFSET(8) NUMBITS(1) [],
    /// Software burst request flags for each of 16 possible sources. Each bit represent
    SOFTBREQ9 OFFSET(9) NUMBITS(1) [],
    /// Software burst request flags for each of 16 possible sources. Each bit represent
    SOFTBREQ10 OFFSET(10) NUMBITS(1) [],
    /// Software burst request flags for each of 16 possible sources. Each bit represent
    SOFTBREQ11 OFFSET(11) NUMBITS(1) [],
    /// Software burst request flags for each of 16 possible sources. Each bit represent
    SOFTBREQ12 OFFSET(12) NUMBITS(1) [],
    /// Software burst request flags for each of 16 possible sources. Each bit represent
    SOFTBREQ13 OFFSET(13) NUMBITS(1) [],
    /// Software burst request flags for each of 16 possible sources. Each bit represent
    SOFTBREQ14 OFFSET(14) NUMBITS(1) [],
    /// Software burst request flags for each of 16 possible sources. Each bit represent
    SOFTBREQ15 OFFSET(15) NUMBITS(1) []
],
SOFTSREQ [
    /// Software single transfer request flags for each of 16 possible sources. Each bit
    SOFTSREQ0 OFFSET(0) NUMBITS(1) [],
    /// Software single transfer request flags for each of 16 possible sources. Each bit
    SOFTSREQ1 OFFSET(1) NUMBITS(1) [],
    /// Software single transfer request flags for each of 16 possible sources. Each bit
    SOFTSREQ2 OFFSET(2) NUMBITS(1) [],
    /// Software single transfer request flags for each of 16 possible sources. Each bit
    SOFTSREQ3 OFFSET(3) NUMBITS(1) [],
    /// Software single transfer request flags for each of 16 possible sources. Each bit
    SOFTSREQ4 OFFSET(4) NUMBITS(1) [],
    /// Software single transfer request flags for each of 16 possible sources. Each bit
    SOFTSREQ5 OFFSET(5) NUMBITS(1) [],
    /// Software single transfer request flags for each of 16 possible sources. Each bit
    SOFTSREQ6 OFFSET(6) NUMBITS(1) [],
    /// Software single transfer request flags for each of 16 possible sources. Each bit
    SOFTSREQ7 OFFSET(7) NUMBITS(1) [],
    /// Software single transfer request flags for each of 16 possible sources. Each bit
    SOFTSREQ8 OFFSET(8) NUMBITS(1) [],
    /// Software single transfer request flags for each of 16 possible sources. Each bit
    SOFTSREQ9 OFFSET(9) NUMBITS(1) [],
    /// Software single transfer request flags for each of 16 possible sources. Each bit
    SOFTSREQ10 OFFSET(10) NUMBITS(1) [],
    /// Software single transfer request flags for each of 16 possible sources. Each bit
    SOFTSREQ11 OFFSET(11) NUMBITS(1) [],
    /// Software single transfer request flags for each of 16 possible sources. Each bit
    SOFTSREQ12 OFFSET(12) NUMBITS(1) [],
    /// Software single transfer request flags for each of 16 possible sources. Each bit
    SOFTSREQ13 OFFSET(13) NUMBITS(1) [],
    /// Software single transfer request flags for each of 16 possible sources. Each bit
    SOFTSREQ14 OFFSET(14) NUMBITS(1) [],
    /// Software single transfer request flags for each of 16 possible sources. Each bit
    SOFTSREQ15 OFFSET(15) NUMBITS(1) []
],
SOFTLBREQ [
    /// Software last burst request flags for each of 16 possible sources. Each bit repr
    SOFTLBREQ0 OFFSET(0) NUMBITS(1) [],
    /// Software last burst request flags for each of 16 possible sources. Each bit repr
    SOFTLBREQ1 OFFSET(1) NUMBITS(1) [],
    /// Software last burst request flags for each of 16 possible sources. Each bit repr
    SOFTLBREQ2 OFFSET(2) NUMBITS(1) [],
    /// Software last burst request flags for each of 16 possible sources. Each bit repr
    SOFTLBREQ3 OFFSET(3) NUMBITS(1) [],
    /// Software last burst request flags for each of 16 possible sources. Each bit repr
    SOFTLBREQ4 OFFSET(4) NUMBITS(1) [],
    /// Software last burst request flags for each of 16 possible sources. Each bit repr
    SOFTLBREQ5 OFFSET(5) NUMBITS(1) [],
    /// Software last burst request flags for each of 16 possible sources. Each bit repr
    SOFTLBREQ6 OFFSET(6) NUMBITS(1) [],
    /// Software last burst request flags for each of 16 possible sources. Each bit repr
    SOFTLBREQ7 OFFSET(7) NUMBITS(1) [],
    /// Software last burst request flags for each of 16 possible sources. Each bit repr
    SOFTLBREQ8 OFFSET(8) NUMBITS(1) [],
    /// Software last burst request flags for each of 16 possible sources. Each bit repr
    SOFTLBREQ9 OFFSET(9) NUMBITS(1) [],
    /// Software last burst request flags for each of 16 possible sources. Each bit repr
    SOFTLBREQ10 OFFSET(10) NUMBITS(1) [],
    /// Software last burst request flags for each of 16 possible sources. Each bit repr
    SOFTLBREQ11 OFFSET(11) NUMBITS(1) [],
    /// Software last burst request flags for each of 16 possible sources. Each bit repr
    SOFTLBREQ12 OFFSET(12) NUMBITS(1) [],
    /// Software last burst request flags for each of 16 possible sources. Each bit repr
    SOFTLBREQ13 OFFSET(13) NUMBITS(1) [],
    /// Software last burst request flags for each of 16 possible sources. Each bit repr
    SOFTLBREQ14 OFFSET(14) NUMBITS(1) [],
    /// Software last burst request flags for each of 16 possible sources. Each bit repr
    SOFTLBREQ15 OFFSET(15) NUMBITS(1) []
],
SOFTLSREQ [
    /// Software last single transfer request flags for each of 16 possible sources. Eac
    SOFTLSREQ0 OFFSET(0) NUMBITS(1) [],
    /// Software last single transfer request flags for each of 16 possible sources. Eac
    SOFTLSREQ1 OFFSET(1) NUMBITS(1) [],
    /// Software last single transfer request flags for each of 16 possible sources. Eac
    SOFTLSREQ2 OFFSET(2) NUMBITS(1) [],
    /// Software last single transfer request flags for each of 16 possible sources. Eac
    SOFTLSREQ3 OFFSET(3) NUMBITS(1) [],
    /// Software last single transfer request flags for each of 16 possible sources. Eac
    SOFTLSREQ4 OFFSET(4) NUMBITS(1) [],
    /// Software last single transfer request flags for each of 16 possible sources. Eac
    SOFTLSREQ5 OFFSET(5) NUMBITS(1) [],
    /// Software last single transfer request flags for each of 16 possible sources. Eac
    SOFTLSREQ6 OFFSET(6) NUMBITS(1) [],
    /// Software last single transfer request flags for each of 16 possible sources. Eac
    SOFTLSREQ7 OFFSET(7) NUMBITS(1) [],
    /// Software last single transfer request flags for each of 16 possible sources. Eac
    SOFTLSREQ8 OFFSET(8) NUMBITS(1) [],
    /// Software last single transfer request flags for each of 16 possible sources. Eac
    SOFTLSREQ9 OFFSET(9) NUMBITS(1) [],
    /// Software last single transfer request flags for each of 16 possible sources. Eac
    SOFTLSREQ10 OFFSET(10) NUMBITS(1) [],
    /// Software last single transfer request flags for each of 16 possible sources. Eac
    SOFTLSREQ11 OFFSET(11) NUMBITS(1) [],
    /// Software last single transfer request flags for each of 16 possible sources. Eac
    SOFTLSREQ12 OFFSET(12) NUMBITS(1) [],
    /// Software last single transfer request flags for each of 16 possible sources. Eac
    SOFTLSREQ13 OFFSET(13) NUMBITS(1) [],
    /// Software last single transfer request flags for each of 16 possible sources. Eac
    SOFTLSREQ14 OFFSET(14) NUMBITS(1) [],
    /// Software last single transfer request flags for each of 16 possible sources. Eac
    SOFTLSREQ15 OFFSET(15) NUMBITS(1) []
],
CONFIG [
    /// DMA Controller enable:
    E OFFSET(0) NUMBITS(1) [
        /// Disabled (default). Disabling the DMA Controller reduces power consumption.
        DisabledDefaultDisablingTheDMAControllerReducesPowerConsumption = 0,
        /// Enabled
        Enabled = 1
    ],
    /// AHB Master 0 endianness configuration:
    M0 OFFSET(1) NUMBITS(1) [
        /// Little-endian mode (default).
        LittleEndianModeDefault = 0,
        /// Big-endian mode.
        BigEndianMode = 1
    ],
    /// AHB Master 1 endianness configuration:
    M1 OFFSET(2) NUMBITS(1) [
        /// Little-endian mode (default).
        LittleEndianModeDefault = 0,
        /// Big-endian mode.
        BigEndianMode = 1
    ]
],
SYNC [
    /// Controls the synchronization logic for DMA request signals. Each bit represents
    DMACSYNC0 OFFSET(0) NUMBITS(1) [],
    /// Controls the synchronization logic for DMA request signals. Each bit represents
    DMACSYNC1 OFFSET(1) NUMBITS(1) [],
    /// Controls the synchronization logic for DMA request signals. Each bit represents
    DMACSYNC2 OFFSET(2) NUMBITS(1) [],
    /// Controls the synchronization logic for DMA request signals. Each bit represents
    DMACSYNC3 OFFSET(3) NUMBITS(1) [],
    /// Controls the synchronization logic for DMA request signals. Each bit represents
    DMACSYNC4 OFFSET(4) NUMBITS(1) [],
    /// Controls the synchronization logic for DMA request signals. Each bit represents
    DMACSYNC5 OFFSET(5) NUMBITS(1) [],
    /// Controls the synchronization logic for DMA request signals. Each bit represents
    DMACSYNC6 OFFSET(6) NUMBITS(1) [],
    /// Controls the synchronization logic for DMA request signals. Each bit represents
    DMACSYNC7 OFFSET(7) NUMBITS(1) [],
    /// Controls the synchronization logic for DMA request signals. Each bit represents
    DMACSYNC8 OFFSET(8) NUMBITS(1) [],
    /// Controls the synchronization logic for DMA request signals. Each bit represents
    DMACSYNC9 OFFSET(9) NUMBITS(1) [],
    /// Controls the synchronization logic for DMA request signals. Each bit represents
    DMACSYNC10 OFFSET(10) NUMBITS(1) [],
    /// Controls the synchronization logic for DMA request signals. Each bit represents
    DMACSYNC11 OFFSET(11) NUMBITS(1) [],
    /// Controls the synchronization logic for DMA request signals. Each bit represents
    DMACSYNC12 OFFSET(12) NUMBITS(1) [],
    /// Controls the synchronization logic for DMA request signals. Each bit represents
    DMACSYNC13 OFFSET(13) NUMBITS(1) [],
    /// Controls the synchronization logic for DMA request signals. Each bit represents
    DMACSYNC14 OFFSET(14) NUMBITS(1) [],
    /// Controls the synchronization logic for DMA request signals. Each bit represents
    DMACSYNC15 OFFSET(15) NUMBITS(1) []
],
C0LLI [
    /// AHB master select for loading the next LLI:
    LM OFFSET(0) NUMBITS(1) [
        /// AHB Master 0.
        AHBMaster0 = 0,
        /// AHB Master 1.
        AHBMaster1 = 1
    ],
    /// Reserved, and must be written as 0, masked on read.
    R OFFSET(1) NUMBITS(1) [],
    /// Linked list item. Bits [31:2] of the address for the next LLI. Address bits [1:0
    LLI OFFSET(2) NUMBITS(30) []
],
C1LLI [
    /// AHB master select for loading the next LLI:
    LM OFFSET(0) NUMBITS(1) [
        /// AHB Master 0.
        AHBMaster0 = 0,
        /// AHB Master 1.
        AHBMaster1 = 1
    ],
    /// Reserved, and must be written as 0, masked on read.
    R OFFSET(1) NUMBITS(1) [],
    /// Linked list item. Bits [31:2] of the address for the next LLI. Address bits [1:0
    LLI OFFSET(2) NUMBITS(30) []
],
C2LLI [
    /// AHB master select for loading the next LLI:
    LM OFFSET(0) NUMBITS(1) [
        /// AHB Master 0.
        AHBMaster0 = 0,
        /// AHB Master 1.
        AHBMaster1 = 1
    ],
    /// Reserved, and must be written as 0, masked on read.
    R OFFSET(1) NUMBITS(1) [],
    /// Linked list item. Bits [31:2] of the address for the next LLI. Address bits [1:0
    LLI OFFSET(2) NUMBITS(30) []
],
C3LLI [
    /// AHB master select for loading the next LLI:
    LM OFFSET(0) NUMBITS(1) [
        /// AHB Master 0.
        AHBMaster0 = 0,
        /// AHB Master 1.
        AHBMaster1 = 1
    ],
    /// Reserved, and must be written as 0, masked on read.
    R OFFSET(1) NUMBITS(1) [],
    /// Linked list item. Bits [31:2] of the address for the next LLI. Address bits [1:0
    LLI OFFSET(2) NUMBITS(30) []
],
C4LLI [
    /// AHB master select for loading the next LLI:
    LM OFFSET(0) NUMBITS(1) [
        /// AHB Master 0.
        AHBMaster0 = 0,
        /// AHB Master 1.
        AHBMaster1 = 1
    ],
    /// Reserved, and must be written as 0, masked on read.
    R OFFSET(1) NUMBITS(1) [],
    /// Linked list item. Bits [31:2] of the address for the next LLI. Address bits [1:0
    LLI OFFSET(2) NUMBITS(30) []
],
C5LLI [
    /// AHB master select for loading the next LLI:
    LM OFFSET(0) NUMBITS(1) [
        /// AHB Master 0.
        AHBMaster0 = 0,
        /// AHB Master 1.
        AHBMaster1 = 1
    ],
    /// Reserved, and must be written as 0, masked on read.
    R OFFSET(1) NUMBITS(1) [],
    /// Linked list item. Bits [31:2] of the address for the next LLI. Address bits [1:0
    LLI OFFSET(2) NUMBITS(30) []
],
C6LLI [
    /// AHB master select for loading the next LLI:
    LM OFFSET(0) NUMBITS(1) [
        /// AHB Master 0.
        AHBMaster0 = 0,
        /// AHB Master 1.
        AHBMaster1 = 1
    ],
    /// Reserved, and must be written as 0, masked on read.
    R OFFSET(1) NUMBITS(1) [],
    /// Linked list item. Bits [31:2] of the address for the next LLI. Address bits [1:0
    LLI OFFSET(2) NUMBITS(30) []
],
C7LLI [
    /// AHB master select for loading the next LLI:
    LM OFFSET(0) NUMBITS(1) [
        /// AHB Master 0.
        AHBMaster0 = 0,
        /// AHB Master 1.
        AHBMaster1 = 1
    ],
    /// Reserved, and must be written as 0, masked on read.
    R OFFSET(1) NUMBITS(1) [],
    /// Linked list item. Bits [31:2] of the address for the next LLI. Address bits [1:0
    LLI OFFSET(2) NUMBITS(30) []
],
C0CONTROL [
    /// Transfer size in byte. A write to this field sets the size of the transfer when
    TRANSFERSIZE OFFSET(0) NUMBITS(12) [],
    /// Source burst size. Indicates the number of transfers that make up a source burst
    SBSIZE OFFSET(12) NUMBITS(3) [
        /// Source burst size = 1
        SourceBurstSize1 = 0,
        /// Source burst size = 4
        SourceBurstSize4 = 1,
        /// Source burst size = 8
        SourceBurstSize8 = 2,
        /// Source burst size = 16
        SourceBurstSize16 = 3,
        /// Source burst size = 32
        SourceBurstSize32 = 4,
        /// Source burst size = 64
        SourceBurstSize64 = 5,
        /// Source burst size = 128
        SourceBurstSize128 = 6,
        /// Source burst size = 256
        SourceBurstSize256 = 7
    ],
    /// Destination burst size. Indicates the number of transfers that make up a destina
    DBSIZE OFFSET(15) NUMBITS(3) [
        /// Destination burst size = 1
        DestinationBurstSize1 = 0,
        /// Destination burst size = 4
        DestinationBurstSize4 = 1,
        /// Destination burst size = 8
        DestinationBurstSize8 = 2,
        /// Destination burst size = 16
        DestinationBurstSize16 = 3,
        /// Destination burst size = 32
        DestinationBurstSize32 = 4,
        /// Destination burst size = 64
        DestinationBurstSize64 = 5,
        /// Destination burst size = 128
        DestinationBurstSize128 = 6,
        /// Destination burst size = 256
        DestinationBurstSize256 = 7
    ],
    /// Source transfer width. Transfers wider than the AHB master bus width are illegal
    SWIDTH OFFSET(18) NUMBITS(3) [
        /// Byte (8-bit)
        Byte8Bit = 0,
        /// Halfword (16-bit)
        Halfword16Bit = 1,
        /// Word (32-bit)
        Word32Bit = 2
    ],
    /// Destination transfer width. Transfers wider than the AHB master bus width are no
    DWIDTH OFFSET(21) NUMBITS(3) [
        /// Byte (8-bit)
        Byte8Bit = 0,
        /// Halfword (16-bit)
        Halfword16Bit = 1,
        /// Word (32-bit)
        Word32Bit = 2
    ],
    /// Source AHB master select:
    S OFFSET(24) NUMBITS(1) [
        /// AHB Master 0 selected for source transfer.
        AHBMaster0SelectedForSourceTransfer = 0,
        /// AHB Master 1 selected for source transfer.
        AHBMaster1SelectedForSourceTransfer = 1
    ],
    /// Destination AHB master select: Only Master1 can access a peripheral. Master0 can
    D OFFSET(25) NUMBITS(1) [
        /// AHB Master 0 selected for destination transfer.
        AHBMaster0SelectedForDestinationTransfer = 0,
        /// AHB Master 1 selected for destination transfer.
        AHBMaster1SelectedForDestinationTransfer = 1
    ],
    /// Source increment:
    SI OFFSET(26) NUMBITS(1) [
        /// The source address is not incremented after each transfer.
        TheSourceAddressIsNotIncrementedAfterEachTransfer = 0,
        /// The source address is incremented after each transfer.
        TheSourceAddressIsIncrementedAfterEachTransfer = 1
    ],
    /// Destination increment:
    DI OFFSET(27) NUMBITS(1) [
        /// The destination address is not incremented after each transfer.
        TheDestinationAddressIsNotIncrementedAfterEachTransfer = 0,
        /// The destination address is incremented after each transfer.
        TheDestinationAddressIsIncrementedAfterEachTransfer = 1
    ],
    /// Indicates that the access is in user mode or privileged mode:
    PROT1 OFFSET(28) NUMBITS(1) [
        /// Access is in user mode
        AccessIsInUserMode = 0,
        /// Access is in privileged mode.
        AccessIsInPrivilegedMode = 1
    ],
    /// Indicates that the access is bufferable or not bufferable:
    PROT2 OFFSET(29) NUMBITS(1) [
        /// Access is not bufferable.
        AccessIsNotBufferable = 0,
        /// Access is bufferable.
        AccessIsBufferable = 1
    ],
    /// Indicates that the access is cacheable or not cacheable:
    PROT3 OFFSET(30) NUMBITS(1) [
        /// Access is not cacheable.
        AccessIsNotCacheable = 0,
        /// Access is cacheable.
        AccessIsCacheable = 1
    ],
    /// Terminal count interrupt enable bit.
    I OFFSET(31) NUMBITS(1) [
        /// The terminal count interrupt is disabled.
        TheTerminalCountInterruptIsDisabled = 0,
        /// The terminal count interrupt is enabled.
        TheTerminalCountInterruptIsEnabled = 1
    ]
],
C1CONTROL [
    /// Transfer size in byte. A write to this field sets the size of the transfer when
    TRANSFERSIZE OFFSET(0) NUMBITS(12) [],
    /// Source burst size. Indicates the number of transfers that make up a source burst
    SBSIZE OFFSET(12) NUMBITS(3) [
        /// Source burst size = 1
        SourceBurstSize1 = 0,
        /// Source burst size = 4
        SourceBurstSize4 = 1,
        /// Source burst size = 8
        SourceBurstSize8 = 2,
        /// Source burst size = 16
        SourceBurstSize16 = 3,
        /// Source burst size = 32
        SourceBurstSize32 = 4,
        /// Source burst size = 64
        SourceBurstSize64 = 5,
        /// Source burst size = 128
        SourceBurstSize128 = 6,
        /// Source burst size = 256
        SourceBurstSize256 = 7
    ],
    /// Destination burst size. Indicates the number of transfers that make up a destina
    DBSIZE OFFSET(15) NUMBITS(3) [
        /// Destination burst size = 1
        DestinationBurstSize1 = 0,
        /// Destination burst size = 4
        DestinationBurstSize4 = 1,
        /// Destination burst size = 8
        DestinationBurstSize8 = 2,
        /// Destination burst size = 16
        DestinationBurstSize16 = 3,
        /// Destination burst size = 32
        DestinationBurstSize32 = 4,
        /// Destination burst size = 64
        DestinationBurstSize64 = 5,
        /// Destination burst size = 128
        DestinationBurstSize128 = 6,
        /// Destination burst size = 256
        DestinationBurstSize256 = 7
    ],
    /// Source transfer width. Transfers wider than the AHB master bus width are illegal
    SWIDTH OFFSET(18) NUMBITS(3) [
        /// Byte (8-bit)
        Byte8Bit = 0,
        /// Halfword (16-bit)
        Halfword16Bit = 1,
        /// Word (32-bit)
        Word32Bit = 2
    ],
    /// Destination transfer width. Transfers wider than the AHB master bus width are no
    DWIDTH OFFSET(21) NUMBITS(3) [
        /// Byte (8-bit)
        Byte8Bit = 0,
        /// Halfword (16-bit)
        Halfword16Bit = 1,
        /// Word (32-bit)
        Word32Bit = 2
    ],
    /// Source AHB master select:
    S OFFSET(24) NUMBITS(1) [
        /// AHB Master 0 selected for source transfer.
        AHBMaster0SelectedForSourceTransfer = 0,
        /// AHB Master 1 selected for source transfer.
        AHBMaster1SelectedForSourceTransfer = 1
    ],
    /// Destination AHB master select: Only Master1 can access a peripheral. Master0 can
    D OFFSET(25) NUMBITS(1) [
        /// AHB Master 0 selected for destination transfer.
        AHBMaster0SelectedForDestinationTransfer = 0,
        /// AHB Master 1 selected for destination transfer.
        AHBMaster1SelectedForDestinationTransfer = 1
    ],
    /// Source increment:
    SI OFFSET(26) NUMBITS(1) [
        /// The source address is not incremented after each transfer.
        TheSourceAddressIsNotIncrementedAfterEachTransfer = 0,
        /// The source address is incremented after each transfer.
        TheSourceAddressIsIncrementedAfterEachTransfer = 1
    ],
    /// Destination increment:
    DI OFFSET(27) NUMBITS(1) [
        /// The destination address is not incremented after each transfer.
        TheDestinationAddressIsNotIncrementedAfterEachTransfer = 0,
        /// The destination address is incremented after each transfer.
        TheDestinationAddressIsIncrementedAfterEachTransfer = 1
    ],
    /// Indicates that the access is in user mode or privileged mode:
    PROT1 OFFSET(28) NUMBITS(1) [
        /// Access is in user mode
        AccessIsInUserMode = 0,
        /// Access is in privileged mode.
        AccessIsInPrivilegedMode = 1
    ],
    /// Indicates that the access is bufferable or not bufferable:
    PROT2 OFFSET(29) NUMBITS(1) [
        /// Access is not bufferable.
        AccessIsNotBufferable = 0,
        /// Access is bufferable.
        AccessIsBufferable = 1
    ],
    /// Indicates that the access is cacheable or not cacheable:
    PROT3 OFFSET(30) NUMBITS(1) [
        /// Access is not cacheable.
        AccessIsNotCacheable = 0,
        /// Access is cacheable.
        AccessIsCacheable = 1
    ],
    /// Terminal count interrupt enable bit.
    I OFFSET(31) NUMBITS(1) [
        /// The terminal count interrupt is disabled.
        TheTerminalCountInterruptIsDisabled = 0,
        /// The terminal count interrupt is enabled.
        TheTerminalCountInterruptIsEnabled = 1
    ]
],
C2CONTROL [
    /// Transfer size in byte. A write to this field sets the size of the transfer when
    TRANSFERSIZE OFFSET(0) NUMBITS(12) [],
    /// Source burst size. Indicates the number of transfers that make up a source burst
    SBSIZE OFFSET(12) NUMBITS(3) [
        /// Source burst size = 1
        SourceBurstSize1 = 0,
        /// Source burst size = 4
        SourceBurstSize4 = 1,
        /// Source burst size = 8
        SourceBurstSize8 = 2,
        /// Source burst size = 16
        SourceBurstSize16 = 3,
        /// Source burst size = 32
        SourceBurstSize32 = 4,
        /// Source burst size = 64
        SourceBurstSize64 = 5,
        /// Source burst size = 128
        SourceBurstSize128 = 6,
        /// Source burst size = 256
        SourceBurstSize256 = 7
    ],
    /// Destination burst size. Indicates the number of transfers that make up a destina
    DBSIZE OFFSET(15) NUMBITS(3) [
        /// Destination burst size = 1
        DestinationBurstSize1 = 0,
        /// Destination burst size = 4
        DestinationBurstSize4 = 1,
        /// Destination burst size = 8
        DestinationBurstSize8 = 2,
        /// Destination burst size = 16
        DestinationBurstSize16 = 3,
        /// Destination burst size = 32
        DestinationBurstSize32 = 4,
        /// Destination burst size = 64
        DestinationBurstSize64 = 5,
        /// Destination burst size = 128
        DestinationBurstSize128 = 6,
        /// Destination burst size = 256
        DestinationBurstSize256 = 7
    ],
    /// Source transfer width. Transfers wider than the AHB master bus width are illegal
    SWIDTH OFFSET(18) NUMBITS(3) [
        /// Byte (8-bit)
        Byte8Bit = 0,
        /// Halfword (16-bit)
        Halfword16Bit = 1,
        /// Word (32-bit)
        Word32Bit = 2
    ],
    /// Destination transfer width. Transfers wider than the AHB master bus width are no
    DWIDTH OFFSET(21) NUMBITS(3) [
        /// Byte (8-bit)
        Byte8Bit = 0,
        /// Halfword (16-bit)
        Halfword16Bit = 1,
        /// Word (32-bit)
        Word32Bit = 2
    ],
    /// Source AHB master select:
    S OFFSET(24) NUMBITS(1) [
        /// AHB Master 0 selected for source transfer.
        AHBMaster0SelectedForSourceTransfer = 0,
        /// AHB Master 1 selected for source transfer.
        AHBMaster1SelectedForSourceTransfer = 1
    ],
    /// Destination AHB master select: Only Master1 can access a peripheral. Master0 can
    D OFFSET(25) NUMBITS(1) [
        /// AHB Master 0 selected for destination transfer.
        AHBMaster0SelectedForDestinationTransfer = 0,
        /// AHB Master 1 selected for destination transfer.
        AHBMaster1SelectedForDestinationTransfer = 1
    ],
    /// Source increment:
    SI OFFSET(26) NUMBITS(1) [
        /// The source address is not incremented after each transfer.
        TheSourceAddressIsNotIncrementedAfterEachTransfer = 0,
        /// The source address is incremented after each transfer.
        TheSourceAddressIsIncrementedAfterEachTransfer = 1
    ],
    /// Destination increment:
    DI OFFSET(27) NUMBITS(1) [
        /// The destination address is not incremented after each transfer.
        TheDestinationAddressIsNotIncrementedAfterEachTransfer = 0,
        /// The destination address is incremented after each transfer.
        TheDestinationAddressIsIncrementedAfterEachTransfer = 1
    ],
    /// Indicates that the access is in user mode or privileged mode:
    PROT1 OFFSET(28) NUMBITS(1) [
        /// Access is in user mode
        AccessIsInUserMode = 0,
        /// Access is in privileged mode.
        AccessIsInPrivilegedMode = 1
    ],
    /// Indicates that the access is bufferable or not bufferable:
    PROT2 OFFSET(29) NUMBITS(1) [
        /// Access is not bufferable.
        AccessIsNotBufferable = 0,
        /// Access is bufferable.
        AccessIsBufferable = 1
    ],
    /// Indicates that the access is cacheable or not cacheable:
    PROT3 OFFSET(30) NUMBITS(1) [
        /// Access is not cacheable.
        AccessIsNotCacheable = 0,
        /// Access is cacheable.
        AccessIsCacheable = 1
    ],
    /// Terminal count interrupt enable bit.
    I OFFSET(31) NUMBITS(1) [
        /// The terminal count interrupt is disabled.
        TheTerminalCountInterruptIsDisabled = 0,
        /// The terminal count interrupt is enabled.
        TheTerminalCountInterruptIsEnabled = 1
    ]
],
C3CONTROL [
    /// Transfer size in byte. A write to this field sets the size of the transfer when
    TRANSFERSIZE OFFSET(0) NUMBITS(12) [],
    /// Source burst size. Indicates the number of transfers that make up a source burst
    SBSIZE OFFSET(12) NUMBITS(3) [
        /// Source burst size = 1
        SourceBurstSize1 = 0,
        /// Source burst size = 4
        SourceBurstSize4 = 1,
        /// Source burst size = 8
        SourceBurstSize8 = 2,
        /// Source burst size = 16
        SourceBurstSize16 = 3,
        /// Source burst size = 32
        SourceBurstSize32 = 4,
        /// Source burst size = 64
        SourceBurstSize64 = 5,
        /// Source burst size = 128
        SourceBurstSize128 = 6,
        /// Source burst size = 256
        SourceBurstSize256 = 7
    ],
    /// Destination burst size. Indicates the number of transfers that make up a destina
    DBSIZE OFFSET(15) NUMBITS(3) [
        /// Destination burst size = 1
        DestinationBurstSize1 = 0,
        /// Destination burst size = 4
        DestinationBurstSize4 = 1,
        /// Destination burst size = 8
        DestinationBurstSize8 = 2,
        /// Destination burst size = 16
        DestinationBurstSize16 = 3,
        /// Destination burst size = 32
        DestinationBurstSize32 = 4,
        /// Destination burst size = 64
        DestinationBurstSize64 = 5,
        /// Destination burst size = 128
        DestinationBurstSize128 = 6,
        /// Destination burst size = 256
        DestinationBurstSize256 = 7
    ],
    /// Source transfer width. Transfers wider than the AHB master bus width are illegal
    SWIDTH OFFSET(18) NUMBITS(3) [
        /// Byte (8-bit)
        Byte8Bit = 0,
        /// Halfword (16-bit)
        Halfword16Bit = 1,
        /// Word (32-bit)
        Word32Bit = 2
    ],
    /// Destination transfer width. Transfers wider than the AHB master bus width are no
    DWIDTH OFFSET(21) NUMBITS(3) [
        /// Byte (8-bit)
        Byte8Bit = 0,
        /// Halfword (16-bit)
        Halfword16Bit = 1,
        /// Word (32-bit)
        Word32Bit = 2
    ],
    /// Source AHB master select:
    S OFFSET(24) NUMBITS(1) [
        /// AHB Master 0 selected for source transfer.
        AHBMaster0SelectedForSourceTransfer = 0,
        /// AHB Master 1 selected for source transfer.
        AHBMaster1SelectedForSourceTransfer = 1
    ],
    /// Destination AHB master select: Only Master1 can access a peripheral. Master0 can
    D OFFSET(25) NUMBITS(1) [
        /// AHB Master 0 selected for destination transfer.
        AHBMaster0SelectedForDestinationTransfer = 0,
        /// AHB Master 1 selected for destination transfer.
        AHBMaster1SelectedForDestinationTransfer = 1
    ],
    /// Source increment:
    SI OFFSET(26) NUMBITS(1) [
        /// The source address is not incremented after each transfer.
        TheSourceAddressIsNotIncrementedAfterEachTransfer = 0,
        /// The source address is incremented after each transfer.
        TheSourceAddressIsIncrementedAfterEachTransfer = 1
    ],
    /// Destination increment:
    DI OFFSET(27) NUMBITS(1) [
        /// The destination address is not incremented after each transfer.
        TheDestinationAddressIsNotIncrementedAfterEachTransfer = 0,
        /// The destination address is incremented after each transfer.
        TheDestinationAddressIsIncrementedAfterEachTransfer = 1
    ],
    /// Indicates that the access is in user mode or privileged mode:
    PROT1 OFFSET(28) NUMBITS(1) [
        /// Access is in user mode
        AccessIsInUserMode = 0,
        /// Access is in privileged mode.
        AccessIsInPrivilegedMode = 1
    ],
    /// Indicates that the access is bufferable or not bufferable:
    PROT2 OFFSET(29) NUMBITS(1) [
        /// Access is not bufferable.
        AccessIsNotBufferable = 0,
        /// Access is bufferable.
        AccessIsBufferable = 1
    ],
    /// Indicates that the access is cacheable or not cacheable:
    PROT3 OFFSET(30) NUMBITS(1) [
        /// Access is not cacheable.
        AccessIsNotCacheable = 0,
        /// Access is cacheable.
        AccessIsCacheable = 1
    ],
    /// Terminal count interrupt enable bit.
    I OFFSET(31) NUMBITS(1) [
        /// The terminal count interrupt is disabled.
        TheTerminalCountInterruptIsDisabled = 0,
        /// The terminal count interrupt is enabled.
        TheTerminalCountInterruptIsEnabled = 1
    ]
],
C4CONTROL [
    /// Transfer size in byte. A write to this field sets the size of the transfer when
    TRANSFERSIZE OFFSET(0) NUMBITS(12) [],
    /// Source burst size. Indicates the number of transfers that make up a source burst
    SBSIZE OFFSET(12) NUMBITS(3) [
        /// Source burst size = 1
        SourceBurstSize1 = 0,
        /// Source burst size = 4
        SourceBurstSize4 = 1,
        /// Source burst size = 8
        SourceBurstSize8 = 2,
        /// Source burst size = 16
        SourceBurstSize16 = 3,
        /// Source burst size = 32
        SourceBurstSize32 = 4,
        /// Source burst size = 64
        SourceBurstSize64 = 5,
        /// Source burst size = 128
        SourceBurstSize128 = 6,
        /// Source burst size = 256
        SourceBurstSize256 = 7
    ],
    /// Destination burst size. Indicates the number of transfers that make up a destina
    DBSIZE OFFSET(15) NUMBITS(3) [
        /// Destination burst size = 1
        DestinationBurstSize1 = 0,
        /// Destination burst size = 4
        DestinationBurstSize4 = 1,
        /// Destination burst size = 8
        DestinationBurstSize8 = 2,
        /// Destination burst size = 16
        DestinationBurstSize16 = 3,
        /// Destination burst size = 32
        DestinationBurstSize32 = 4,
        /// Destination burst size = 64
        DestinationBurstSize64 = 5,
        /// Destination burst size = 128
        DestinationBurstSize128 = 6,
        /// Destination burst size = 256
        DestinationBurstSize256 = 7
    ],
    /// Source transfer width. Transfers wider than the AHB master bus width are illegal
    SWIDTH OFFSET(18) NUMBITS(3) [
        /// Byte (8-bit)
        Byte8Bit = 0,
        /// Halfword (16-bit)
        Halfword16Bit = 1,
        /// Word (32-bit)
        Word32Bit = 2
    ],
    /// Destination transfer width. Transfers wider than the AHB master bus width are no
    DWIDTH OFFSET(21) NUMBITS(3) [
        /// Byte (8-bit)
        Byte8Bit = 0,
        /// Halfword (16-bit)
        Halfword16Bit = 1,
        /// Word (32-bit)
        Word32Bit = 2
    ],
    /// Source AHB master select:
    S OFFSET(24) NUMBITS(1) [
        /// AHB Master 0 selected for source transfer.
        AHBMaster0SelectedForSourceTransfer = 0,
        /// AHB Master 1 selected for source transfer.
        AHBMaster1SelectedForSourceTransfer = 1
    ],
    /// Destination AHB master select: Only Master1 can access a peripheral. Master0 can
    D OFFSET(25) NUMBITS(1) [
        /// AHB Master 0 selected for destination transfer.
        AHBMaster0SelectedForDestinationTransfer = 0,
        /// AHB Master 1 selected for destination transfer.
        AHBMaster1SelectedForDestinationTransfer = 1
    ],
    /// Source increment:
    SI OFFSET(26) NUMBITS(1) [
        /// The source address is not incremented after each transfer.
        TheSourceAddressIsNotIncrementedAfterEachTransfer = 0,
        /// The source address is incremented after each transfer.
        TheSourceAddressIsIncrementedAfterEachTransfer = 1
    ],
    /// Destination increment:
    DI OFFSET(27) NUMBITS(1) [
        /// The destination address is not incremented after each transfer.
        TheDestinationAddressIsNotIncrementedAfterEachTransfer = 0,
        /// The destination address is incremented after each transfer.
        TheDestinationAddressIsIncrementedAfterEachTransfer = 1
    ],
    /// Indicates that the access is in user mode or privileged mode:
    PROT1 OFFSET(28) NUMBITS(1) [
        /// Access is in user mode
        AccessIsInUserMode = 0,
        /// Access is in privileged mode.
        AccessIsInPrivilegedMode = 1
    ],
    /// Indicates that the access is bufferable or not bufferable:
    PROT2 OFFSET(29) NUMBITS(1) [
        /// Access is not bufferable.
        AccessIsNotBufferable = 0,
        /// Access is bufferable.
        AccessIsBufferable = 1
    ],
    /// Indicates that the access is cacheable or not cacheable:
    PROT3 OFFSET(30) NUMBITS(1) [
        /// Access is not cacheable.
        AccessIsNotCacheable = 0,
        /// Access is cacheable.
        AccessIsCacheable = 1
    ],
    /// Terminal count interrupt enable bit.
    I OFFSET(31) NUMBITS(1) [
        /// The terminal count interrupt is disabled.
        TheTerminalCountInterruptIsDisabled = 0,
        /// The terminal count interrupt is enabled.
        TheTerminalCountInterruptIsEnabled = 1
    ]
],
C5CONTROL [
    /// Transfer size in byte. A write to this field sets the size of the transfer when
    TRANSFERSIZE OFFSET(0) NUMBITS(12) [],
    /// Source burst size. Indicates the number of transfers that make up a source burst
    SBSIZE OFFSET(12) NUMBITS(3) [
        /// Source burst size = 1
        SourceBurstSize1 = 0,
        /// Source burst size = 4
        SourceBurstSize4 = 1,
        /// Source burst size = 8
        SourceBurstSize8 = 2,
        /// Source burst size = 16
        SourceBurstSize16 = 3,
        /// Source burst size = 32
        SourceBurstSize32 = 4,
        /// Source burst size = 64
        SourceBurstSize64 = 5,
        /// Source burst size = 128
        SourceBurstSize128 = 6,
        /// Source burst size = 256
        SourceBurstSize256 = 7
    ],
    /// Destination burst size. Indicates the number of transfers that make up a destina
    DBSIZE OFFSET(15) NUMBITS(3) [
        /// Destination burst size = 1
        DestinationBurstSize1 = 0,
        /// Destination burst size = 4
        DestinationBurstSize4 = 1,
        /// Destination burst size = 8
        DestinationBurstSize8 = 2,
        /// Destination burst size = 16
        DestinationBurstSize16 = 3,
        /// Destination burst size = 32
        DestinationBurstSize32 = 4,
        /// Destination burst size = 64
        DestinationBurstSize64 = 5,
        /// Destination burst size = 128
        DestinationBurstSize128 = 6,
        /// Destination burst size = 256
        DestinationBurstSize256 = 7
    ],
    /// Source transfer width. Transfers wider than the AHB master bus width are illegal
    SWIDTH OFFSET(18) NUMBITS(3) [
        /// Byte (8-bit)
        Byte8Bit = 0,
        /// Halfword (16-bit)
        Halfword16Bit = 1,
        /// Word (32-bit)
        Word32Bit = 2
    ],
    /// Destination transfer width. Transfers wider than the AHB master bus width are no
    DWIDTH OFFSET(21) NUMBITS(3) [
        /// Byte (8-bit)
        Byte8Bit = 0,
        /// Halfword (16-bit)
        Halfword16Bit = 1,
        /// Word (32-bit)
        Word32Bit = 2
    ],
    /// Source AHB master select:
    S OFFSET(24) NUMBITS(1) [
        /// AHB Master 0 selected for source transfer.
        AHBMaster0SelectedForSourceTransfer = 0,
        /// AHB Master 1 selected for source transfer.
        AHBMaster1SelectedForSourceTransfer = 1
    ],
    /// Destination AHB master select: Only Master1 can access a peripheral. Master0 can
    D OFFSET(25) NUMBITS(1) [
        /// AHB Master 0 selected for destination transfer.
        AHBMaster0SelectedForDestinationTransfer = 0,
        /// AHB Master 1 selected for destination transfer.
        AHBMaster1SelectedForDestinationTransfer = 1
    ],
    /// Source increment:
    SI OFFSET(26) NUMBITS(1) [
        /// The source address is not incremented after each transfer.
        TheSourceAddressIsNotIncrementedAfterEachTransfer = 0,
        /// The source address is incremented after each transfer.
        TheSourceAddressIsIncrementedAfterEachTransfer = 1
    ],
    /// Destination increment:
    DI OFFSET(27) NUMBITS(1) [
        /// The destination address is not incremented after each transfer.
        TheDestinationAddressIsNotIncrementedAfterEachTransfer = 0,
        /// The destination address is incremented after each transfer.
        TheDestinationAddressIsIncrementedAfterEachTransfer = 1
    ],
    /// Indicates that the access is in user mode or privileged mode:
    PROT1 OFFSET(28) NUMBITS(1) [
        /// Access is in user mode
        AccessIsInUserMode = 0,
        /// Access is in privileged mode.
        AccessIsInPrivilegedMode = 1
    ],
    /// Indicates that the access is bufferable or not bufferable:
    PROT2 OFFSET(29) NUMBITS(1) [
        /// Access is not bufferable.
        AccessIsNotBufferable = 0,
        /// Access is bufferable.
        AccessIsBufferable = 1
    ],
    /// Indicates that the access is cacheable or not cacheable:
    PROT3 OFFSET(30) NUMBITS(1) [
        /// Access is not cacheable.
        AccessIsNotCacheable = 0,
        /// Access is cacheable.
        AccessIsCacheable = 1
    ],
    /// Terminal count interrupt enable bit.
    I OFFSET(31) NUMBITS(1) [
        /// The terminal count interrupt is disabled.
        TheTerminalCountInterruptIsDisabled = 0,
        /// The terminal count interrupt is enabled.
        TheTerminalCountInterruptIsEnabled = 1
    ]
],
C6CONTROL [
    /// Transfer size in byte. A write to this field sets the size of the transfer when
    TRANSFERSIZE OFFSET(0) NUMBITS(12) [],
    /// Source burst size. Indicates the number of transfers that make up a source burst
    SBSIZE OFFSET(12) NUMBITS(3) [
        /// Source burst size = 1
        SourceBurstSize1 = 0,
        /// Source burst size = 4
        SourceBurstSize4 = 1,
        /// Source burst size = 8
        SourceBurstSize8 = 2,
        /// Source burst size = 16
        SourceBurstSize16 = 3,
        /// Source burst size = 32
        SourceBurstSize32 = 4,
        /// Source burst size = 64
        SourceBurstSize64 = 5,
        /// Source burst size = 128
        SourceBurstSize128 = 6,
        /// Source burst size = 256
        SourceBurstSize256 = 7
    ],
    /// Destination burst size. Indicates the number of transfers that make up a destina
    DBSIZE OFFSET(15) NUMBITS(3) [
        /// Destination burst size = 1
        DestinationBurstSize1 = 0,
        /// Destination burst size = 4
        DestinationBurstSize4 = 1,
        /// Destination burst size = 8
        DestinationBurstSize8 = 2,
        /// Destination burst size = 16
        DestinationBurstSize16 = 3,
        /// Destination burst size = 32
        DestinationBurstSize32 = 4,
        /// Destination burst size = 64
        DestinationBurstSize64 = 5,
        /// Destination burst size = 128
        DestinationBurstSize128 = 6,
        /// Destination burst size = 256
        DestinationBurstSize256 = 7
    ],
    /// Source transfer width. Transfers wider than the AHB master bus width are illegal
    SWIDTH OFFSET(18) NUMBITS(3) [
        /// Byte (8-bit)
        Byte8Bit = 0,
        /// Halfword (16-bit)
        Halfword16Bit = 1,
        /// Word (32-bit)
        Word32Bit = 2
    ],
    /// Destination transfer width. Transfers wider than the AHB master bus width are no
    DWIDTH OFFSET(21) NUMBITS(3) [
        /// Byte (8-bit)
        Byte8Bit = 0,
        /// Halfword (16-bit)
        Halfword16Bit = 1,
        /// Word (32-bit)
        Word32Bit = 2
    ],
    /// Source AHB master select:
    S OFFSET(24) NUMBITS(1) [
        /// AHB Master 0 selected for source transfer.
        AHBMaster0SelectedForSourceTransfer = 0,
        /// AHB Master 1 selected for source transfer.
        AHBMaster1SelectedForSourceTransfer = 1
    ],
    /// Destination AHB master select: Only Master1 can access a peripheral. Master0 can
    D OFFSET(25) NUMBITS(1) [
        /// AHB Master 0 selected for destination transfer.
        AHBMaster0SelectedForDestinationTransfer = 0,
        /// AHB Master 1 selected for destination transfer.
        AHBMaster1SelectedForDestinationTransfer = 1
    ],
    /// Source increment:
    SI OFFSET(26) NUMBITS(1) [
        /// The source address is not incremented after each transfer.
        TheSourceAddressIsNotIncrementedAfterEachTransfer = 0,
        /// The source address is incremented after each transfer.
        TheSourceAddressIsIncrementedAfterEachTransfer = 1
    ],
    /// Destination increment:
    DI OFFSET(27) NUMBITS(1) [
        /// The destination address is not incremented after each transfer.
        TheDestinationAddressIsNotIncrementedAfterEachTransfer = 0,
        /// The destination address is incremented after each transfer.
        TheDestinationAddressIsIncrementedAfterEachTransfer = 1
    ],
    /// Indicates that the access is in user mode or privileged mode:
    PROT1 OFFSET(28) NUMBITS(1) [
        /// Access is in user mode
        AccessIsInUserMode = 0,
        /// Access is in privileged mode.
        AccessIsInPrivilegedMode = 1
    ],
    /// Indicates that the access is bufferable or not bufferable:
    PROT2 OFFSET(29) NUMBITS(1) [
        /// Access is not bufferable.
        AccessIsNotBufferable = 0,
        /// Access is bufferable.
        AccessIsBufferable = 1
    ],
    /// Indicates that the access is cacheable or not cacheable:
    PROT3 OFFSET(30) NUMBITS(1) [
        /// Access is not cacheable.
        AccessIsNotCacheable = 0,
        /// Access is cacheable.
        AccessIsCacheable = 1
    ],
    /// Terminal count interrupt enable bit.
    I OFFSET(31) NUMBITS(1) [
        /// The terminal count interrupt is disabled.
        TheTerminalCountInterruptIsDisabled = 0,
        /// The terminal count interrupt is enabled.
        TheTerminalCountInterruptIsEnabled = 1
    ]
],
C7CONTROL [
    /// Transfer size in byte. A write to this field sets the size of the transfer when
    TRANSFERSIZE OFFSET(0) NUMBITS(12) [],
    /// Source burst size. Indicates the number of transfers that make up a source burst
    SBSIZE OFFSET(12) NUMBITS(3) [
        /// Source burst size = 1
        SourceBurstSize1 = 0,
        /// Source burst size = 4
        SourceBurstSize4 = 1,
        /// Source burst size = 8
        SourceBurstSize8 = 2,
        /// Source burst size = 16
        SourceBurstSize16 = 3,
        /// Source burst size = 32
        SourceBurstSize32 = 4,
        /// Source burst size = 64
        SourceBurstSize64 = 5,
        /// Source burst size = 128
        SourceBurstSize128 = 6,
        /// Source burst size = 256
        SourceBurstSize256 = 7
    ],
    /// Destination burst size. Indicates the number of transfers that make up a destina
    DBSIZE OFFSET(15) NUMBITS(3) [
        /// Destination burst size = 1
        DestinationBurstSize1 = 0,
        /// Destination burst size = 4
        DestinationBurstSize4 = 1,
        /// Destination burst size = 8
        DestinationBurstSize8 = 2,
        /// Destination burst size = 16
        DestinationBurstSize16 = 3,
        /// Destination burst size = 32
        DestinationBurstSize32 = 4,
        /// Destination burst size = 64
        DestinationBurstSize64 = 5,
        /// Destination burst size = 128
        DestinationBurstSize128 = 6,
        /// Destination burst size = 256
        DestinationBurstSize256 = 7
    ],
    /// Source transfer width. Transfers wider than the AHB master bus width are illegal
    SWIDTH OFFSET(18) NUMBITS(3) [
        /// Byte (8-bit)
        Byte8Bit = 0,
        /// Halfword (16-bit)
        Halfword16Bit = 1,
        /// Word (32-bit)
        Word32Bit = 2
    ],
    /// Destination transfer width. Transfers wider than the AHB master bus width are no
    DWIDTH OFFSET(21) NUMBITS(3) [
        /// Byte (8-bit)
        Byte8Bit = 0,
        /// Halfword (16-bit)
        Halfword16Bit = 1,
        /// Word (32-bit)
        Word32Bit = 2
    ],
    /// Source AHB master select:
    S OFFSET(24) NUMBITS(1) [
        /// AHB Master 0 selected for source transfer.
        AHBMaster0SelectedForSourceTransfer = 0,
        /// AHB Master 1 selected for source transfer.
        AHBMaster1SelectedForSourceTransfer = 1
    ],
    /// Destination AHB master select: Only Master1 can access a peripheral. Master0 can
    D OFFSET(25) NUMBITS(1) [
        /// AHB Master 0 selected for destination transfer.
        AHBMaster0SelectedForDestinationTransfer = 0,
        /// AHB Master 1 selected for destination transfer.
        AHBMaster1SelectedForDestinationTransfer = 1
    ],
    /// Source increment:
    SI OFFSET(26) NUMBITS(1) [
        /// The source address is not incremented after each transfer.
        TheSourceAddressIsNotIncrementedAfterEachTransfer = 0,
        /// The source address is incremented after each transfer.
        TheSourceAddressIsIncrementedAfterEachTransfer = 1
    ],
    /// Destination increment:
    DI OFFSET(27) NUMBITS(1) [
        /// The destination address is not incremented after each transfer.
        TheDestinationAddressIsNotIncrementedAfterEachTransfer = 0,
        /// The destination address is incremented after each transfer.
        TheDestinationAddressIsIncrementedAfterEachTransfer = 1
    ],
    /// Indicates that the access is in user mode or privileged mode:
    PROT1 OFFSET(28) NUMBITS(1) [
        /// Access is in user mode
        AccessIsInUserMode = 0,
        /// Access is in privileged mode.
        AccessIsInPrivilegedMode = 1
    ],
    /// Indicates that the access is bufferable or not bufferable:
    PROT2 OFFSET(29) NUMBITS(1) [
        /// Access is not bufferable.
        AccessIsNotBufferable = 0,
        /// Access is bufferable.
        AccessIsBufferable = 1
    ],
    /// Indicates that the access is cacheable or not cacheable:
    PROT3 OFFSET(30) NUMBITS(1) [
        /// Access is not cacheable.
        AccessIsNotCacheable = 0,
        /// Access is cacheable.
        AccessIsCacheable = 1
    ],
    /// Terminal count interrupt enable bit.
    I OFFSET(31) NUMBITS(1) [
        /// The terminal count interrupt is disabled.
        TheTerminalCountInterruptIsDisabled = 0,
        /// The terminal count interrupt is enabled.
        TheTerminalCountInterruptIsEnabled = 1
    ]
],
C0CONFIG [
    /// Channel enable. Reading this bit indicates whether a channel is currently enable
    E OFFSET(0) NUMBITS(1) [
        /// Channel disabled.
        ChannelDisabled = 0,
        /// Channel enabled.
        ChannelEnabled = 1
    ],
    /// Source peripheral. This value selects the DMA source request peripheral. This fi
    SRCPERIPHERAL OFFSET(1) NUMBITS(5) [
        /// Source = SPIFI
        SourceSPIFI = 0,
        /// Source = Timer 0 match 0/UART0 transmit
        SourceTimer0Match0UART0Transmit = 1,
        /// Source = Timer 0 match 1/UART0 receive
        SourceTimer0Match1UART0Receive = 2,
        /// Source = Timer 1 match 0/UART1 transmit
        SourceTimer1Match0UART1Transmit = 3,
        /// Source = Timer 1 match 1/UART 1 receive
        SourceTimer1Match1UART1Receive = 4,
        /// Source = Timer 2 match 0/UART 2 transmit
        SourceTimer2Match0UART2Transmit = 5,
        /// Source = Timer 2 match 1/UART 2 receive
        SourceTimer2Match1UART2Receive = 6,
        /// Source = Timer 3 match 0/UART3 transmit/SCT DMA request 0
        SourceTimer3Match0UART3TransmitSCTDMARequest0 = 7,
        /// Source = Timer 3 match 1/UART3 receive/SCT DMA request 1
        SourceTimer3Match1UART3ReceiveSCTDMARequest1 = 8,
        /// Source = SSP0 receive/I2S channel 0
        SourceSSP0ReceiveI2SChannel0 = 9,
        /// Source = SSP0 transmit/I2S channel 1
        SourceSSP0TransmitI2SChannel1 = 10,
        /// Source = SSP1 receive
        SourceSSP1Receive = 11,
        /// Source = SSP1 transmit
        SourceSSP1Transmit = 12,
        /// Source = ADC0
        SourceADC0 = 13,
        /// Source = ADC1
        SourceADC1 = 14,
        /// Source = DAC
        SourceDAC = 15
    ],
    /// Destination peripheral. This value selects the DMA destination request periphera
    DESTPERIPHERAL OFFSET(6) NUMBITS(5) [
        /// Destination = SPIFI
        DestinationSPIFI = 0,
        /// Destination = Timer 0 match 0/UART0 transmit
        DestinationTimer0Match0UART0Transmit = 1,
        /// Destination = Timer 0 match 1/UART0 receive
        DestinationTimer0Match1UART0Receive = 2,
        /// Destination = Timer 1 match 0/UART1 transmit
        DestinationTimer1Match0UART1Transmit = 3,
        /// Destination = Timer 1 match 1/UART 1 receive
        DestinationTimer1Match1UART1Receive = 4,
        /// Destination = Timer 2 match 0/UART 2 transmit
        DestinationTimer2Match0UART2Transmit = 5,
        /// Destination = Timer 2 match 1/UART 2 receive
        DestinationTimer2Match1UART2Receive = 6,
        /// Destination = Timer 3 match 0/UART3 transmit/SCT DMA request 0
        DestinationTimer3Match0UART3TransmitSCTDMARequest0 = 7,
        /// Destination = Timer 3 match 1/UART3 receive/SCT DMA request 1
        DestinationTimer3Match1UART3ReceiveSCTDMARequest1 = 8,
        /// Destination = SSP0 receive/I2S channel 0
        DestinationSSP0ReceiveI2SChannel0 = 9,
        /// Destination = SSP0 transmit/I2S channel 1
        DestinationSSP0TransmitI2SChannel1 = 10,
        /// Destination = SSP1 receive
        DestinationSSP1Receive = 11,
        /// Destination = SSP1 transmit
        DestinationSSP1Transmit = 12,
        /// Destination = ADC0
        DestinationADC0 = 13,
        /// Destination = ADC1
        DestinationADC1 = 14,
        /// Destination = DAC
        DestinationDAC = 15
    ],
    /// Flow control and transfer type. This value indicates the flow controller and tra
    FLOWCNTRL OFFSET(11) NUMBITS(3) [
        /// Memory to memory (DMA control)
        MemoryToMemoryDMAControl = 0,
        /// Memory to peripheral (DMA control)
        MemoryToPeripheralDMAControl = 1,
        /// Peripheral to memory (DMA control)
        PeripheralToMemoryDMAControl = 2,
        /// Source peripheral to destination peripheral (DMA control)
        SourcePeripheralToDestinationPeripheralDMAControl = 3,
        /// Source peripheral to destination peripheral (destination control)
        SourcePeripheralToDestinationPeripheralDestinationControl = 4,
        /// Memory to peripheral (peripheral control)
        MemoryToPeripheralPeripheralControl = 5,
        /// Peripheral to memory (peripheral control)
        PeripheralToMemoryPeripheralControl = 6,
        /// Source peripheral to destination peripheral (source control)
        SourcePeripheralToDestinationPeripheralSourceControl = 7
    ],
    /// Interrupt error mask. When cleared, this bit masks out the error interrupt of th
    IE OFFSET(14) NUMBITS(1) [],
    /// Terminal count interrupt mask. When cleared, this bit masks out the terminal cou
    ITC OFFSET(15) NUMBITS(1) [],
    /// Lock. When set, this bit enables locked transfers.
    L OFFSET(16) NUMBITS(1) [],
    /// Active: 0 = there is no data in the FIFO of the channel. 1 = the channel FIFO ha
    A OFFSET(17) NUMBITS(1) [],
    /// Halt: 0 = enable DMA requests. 1 = ignore further source DMA requests. The conte
    H OFFSET(18) NUMBITS(1) [
        /// Enable DMA requests.
        EnableDMARequests = 0,
        /// Ignore further source DMA requests.
        IgnoreFurtherSourceDMARequests = 1
    ]
],
C1CONFIG [
    /// Channel enable. Reading this bit indicates whether a channel is currently enable
    E OFFSET(0) NUMBITS(1) [
        /// Channel disabled.
        ChannelDisabled = 0,
        /// Channel enabled.
        ChannelEnabled = 1
    ],
    /// Source peripheral. This value selects the DMA source request peripheral. This fi
    SRCPERIPHERAL OFFSET(1) NUMBITS(5) [
        /// Source = SPIFI
        SourceSPIFI = 0,
        /// Source = Timer 0 match 0/UART0 transmit
        SourceTimer0Match0UART0Transmit = 1,
        /// Source = Timer 0 match 1/UART0 receive
        SourceTimer0Match1UART0Receive = 2,
        /// Source = Timer 1 match 0/UART1 transmit
        SourceTimer1Match0UART1Transmit = 3,
        /// Source = Timer 1 match 1/UART 1 receive
        SourceTimer1Match1UART1Receive = 4,
        /// Source = Timer 2 match 0/UART 2 transmit
        SourceTimer2Match0UART2Transmit = 5,
        /// Source = Timer 2 match 1/UART 2 receive
        SourceTimer2Match1UART2Receive = 6,
        /// Source = Timer 3 match 0/UART3 transmit/SCT DMA request 0
        SourceTimer3Match0UART3TransmitSCTDMARequest0 = 7,
        /// Source = Timer 3 match 1/UART3 receive/SCT DMA request 1
        SourceTimer3Match1UART3ReceiveSCTDMARequest1 = 8,
        /// Source = SSP0 receive/I2S channel 0
        SourceSSP0ReceiveI2SChannel0 = 9,
        /// Source = SSP0 transmit/I2S channel 1
        SourceSSP0TransmitI2SChannel1 = 10,
        /// Source = SSP1 receive
        SourceSSP1Receive = 11,
        /// Source = SSP1 transmit
        SourceSSP1Transmit = 12,
        /// Source = ADC0
        SourceADC0 = 13,
        /// Source = ADC1
        SourceADC1 = 14,
        /// Source = DAC
        SourceDAC = 15
    ],
    /// Destination peripheral. This value selects the DMA destination request periphera
    DESTPERIPHERAL OFFSET(6) NUMBITS(5) [
        /// Destination = SPIFI
        DestinationSPIFI = 0,
        /// Destination = Timer 0 match 0/UART0 transmit
        DestinationTimer0Match0UART0Transmit = 1,
        /// Destination = Timer 0 match 1/UART0 receive
        DestinationTimer0Match1UART0Receive = 2,
        /// Destination = Timer 1 match 0/UART1 transmit
        DestinationTimer1Match0UART1Transmit = 3,
        /// Destination = Timer 1 match 1/UART 1 receive
        DestinationTimer1Match1UART1Receive = 4,
        /// Destination = Timer 2 match 0/UART 2 transmit
        DestinationTimer2Match0UART2Transmit = 5,
        /// Destination = Timer 2 match 1/UART 2 receive
        DestinationTimer2Match1UART2Receive = 6,
        /// Destination = Timer 3 match 0/UART3 transmit/SCT DMA request 0
        DestinationTimer3Match0UART3TransmitSCTDMARequest0 = 7,
        /// Destination = Timer 3 match 1/UART3 receive/SCT DMA request 1
        DestinationTimer3Match1UART3ReceiveSCTDMARequest1 = 8,
        /// Destination = SSP0 receive/I2S channel 0
        DestinationSSP0ReceiveI2SChannel0 = 9,
        /// Destination = SSP0 transmit/I2S channel 1
        DestinationSSP0TransmitI2SChannel1 = 10,
        /// Destination = SSP1 receive
        DestinationSSP1Receive = 11,
        /// Destination = SSP1 transmit
        DestinationSSP1Transmit = 12,
        /// Destination = ADC0
        DestinationADC0 = 13,
        /// Destination = ADC1
        DestinationADC1 = 14,
        /// Destination = DAC
        DestinationDAC = 15
    ],
    /// Flow control and transfer type. This value indicates the flow controller and tra
    FLOWCNTRL OFFSET(11) NUMBITS(3) [
        /// Memory to memory (DMA control)
        MemoryToMemoryDMAControl = 0,
        /// Memory to peripheral (DMA control)
        MemoryToPeripheralDMAControl = 1,
        /// Peripheral to memory (DMA control)
        PeripheralToMemoryDMAControl = 2,
        /// Source peripheral to destination peripheral (DMA control)
        SourcePeripheralToDestinationPeripheralDMAControl = 3,
        /// Source peripheral to destination peripheral (destination control)
        SourcePeripheralToDestinationPeripheralDestinationControl = 4,
        /// Memory to peripheral (peripheral control)
        MemoryToPeripheralPeripheralControl = 5,
        /// Peripheral to memory (peripheral control)
        PeripheralToMemoryPeripheralControl = 6,
        /// Source peripheral to destination peripheral (source control)
        SourcePeripheralToDestinationPeripheralSourceControl = 7
    ],
    /// Interrupt error mask. When cleared, this bit masks out the error interrupt of th
    IE OFFSET(14) NUMBITS(1) [],
    /// Terminal count interrupt mask. When cleared, this bit masks out the terminal cou
    ITC OFFSET(15) NUMBITS(1) [],
    /// Lock. When set, this bit enables locked transfers.
    L OFFSET(16) NUMBITS(1) [],
    /// Active: 0 = there is no data in the FIFO of the channel. 1 = the channel FIFO ha
    A OFFSET(17) NUMBITS(1) [],
    /// Halt: 0 = enable DMA requests. 1 = ignore further source DMA requests. The conte
    H OFFSET(18) NUMBITS(1) [
        /// Enable DMA requests.
        EnableDMARequests = 0,
        /// Ignore further source DMA requests.
        IgnoreFurtherSourceDMARequests = 1
    ]
],
C2CONFIG [
    /// Channel enable. Reading this bit indicates whether a channel is currently enable
    E OFFSET(0) NUMBITS(1) [
        /// Channel disabled.
        ChannelDisabled = 0,
        /// Channel enabled.
        ChannelEnabled = 1
    ],
    /// Source peripheral. This value selects the DMA source request peripheral. This fi
    SRCPERIPHERAL OFFSET(1) NUMBITS(5) [
        /// Source = SPIFI
        SourceSPIFI = 0,
        /// Source = Timer 0 match 0/UART0 transmit
        SourceTimer0Match0UART0Transmit = 1,
        /// Source = Timer 0 match 1/UART0 receive
        SourceTimer0Match1UART0Receive = 2,
        /// Source = Timer 1 match 0/UART1 transmit
        SourceTimer1Match0UART1Transmit = 3,
        /// Source = Timer 1 match 1/UART 1 receive
        SourceTimer1Match1UART1Receive = 4,
        /// Source = Timer 2 match 0/UART 2 transmit
        SourceTimer2Match0UART2Transmit = 5,
        /// Source = Timer 2 match 1/UART 2 receive
        SourceTimer2Match1UART2Receive = 6,
        /// Source = Timer 3 match 0/UART3 transmit/SCT DMA request 0
        SourceTimer3Match0UART3TransmitSCTDMARequest0 = 7,
        /// Source = Timer 3 match 1/UART3 receive/SCT DMA request 1
        SourceTimer3Match1UART3ReceiveSCTDMARequest1 = 8,
        /// Source = SSP0 receive/I2S channel 0
        SourceSSP0ReceiveI2SChannel0 = 9,
        /// Source = SSP0 transmit/I2S channel 1
        SourceSSP0TransmitI2SChannel1 = 10,
        /// Source = SSP1 receive
        SourceSSP1Receive = 11,
        /// Source = SSP1 transmit
        SourceSSP1Transmit = 12,
        /// Source = ADC0
        SourceADC0 = 13,
        /// Source = ADC1
        SourceADC1 = 14,
        /// Source = DAC
        SourceDAC = 15
    ],
    /// Destination peripheral. This value selects the DMA destination request periphera
    DESTPERIPHERAL OFFSET(6) NUMBITS(5) [
        /// Destination = SPIFI
        DestinationSPIFI = 0,
        /// Destination = Timer 0 match 0/UART0 transmit
        DestinationTimer0Match0UART0Transmit = 1,
        /// Destination = Timer 0 match 1/UART0 receive
        DestinationTimer0Match1UART0Receive = 2,
        /// Destination = Timer 1 match 0/UART1 transmit
        DestinationTimer1Match0UART1Transmit = 3,
        /// Destination = Timer 1 match 1/UART 1 receive
        DestinationTimer1Match1UART1Receive = 4,
        /// Destination = Timer 2 match 0/UART 2 transmit
        DestinationTimer2Match0UART2Transmit = 5,
        /// Destination = Timer 2 match 1/UART 2 receive
        DestinationTimer2Match1UART2Receive = 6,
        /// Destination = Timer 3 match 0/UART3 transmit/SCT DMA request 0
        DestinationTimer3Match0UART3TransmitSCTDMARequest0 = 7,
        /// Destination = Timer 3 match 1/UART3 receive/SCT DMA request 1
        DestinationTimer3Match1UART3ReceiveSCTDMARequest1 = 8,
        /// Destination = SSP0 receive/I2S channel 0
        DestinationSSP0ReceiveI2SChannel0 = 9,
        /// Destination = SSP0 transmit/I2S channel 1
        DestinationSSP0TransmitI2SChannel1 = 10,
        /// Destination = SSP1 receive
        DestinationSSP1Receive = 11,
        /// Destination = SSP1 transmit
        DestinationSSP1Transmit = 12,
        /// Destination = ADC0
        DestinationADC0 = 13,
        /// Destination = ADC1
        DestinationADC1 = 14,
        /// Destination = DAC
        DestinationDAC = 15
    ],
    /// Flow control and transfer type. This value indicates the flow controller and tra
    FLOWCNTRL OFFSET(11) NUMBITS(3) [
        /// Memory to memory (DMA control)
        MemoryToMemoryDMAControl = 0,
        /// Memory to peripheral (DMA control)
        MemoryToPeripheralDMAControl = 1,
        /// Peripheral to memory (DMA control)
        PeripheralToMemoryDMAControl = 2,
        /// Source peripheral to destination peripheral (DMA control)
        SourcePeripheralToDestinationPeripheralDMAControl = 3,
        /// Source peripheral to destination peripheral (destination control)
        SourcePeripheralToDestinationPeripheralDestinationControl = 4,
        /// Memory to peripheral (peripheral control)
        MemoryToPeripheralPeripheralControl = 5,
        /// Peripheral to memory (peripheral control)
        PeripheralToMemoryPeripheralControl = 6,
        /// Source peripheral to destination peripheral (source control)
        SourcePeripheralToDestinationPeripheralSourceControl = 7
    ],
    /// Interrupt error mask. When cleared, this bit masks out the error interrupt of th
    IE OFFSET(14) NUMBITS(1) [],
    /// Terminal count interrupt mask. When cleared, this bit masks out the terminal cou
    ITC OFFSET(15) NUMBITS(1) [],
    /// Lock. When set, this bit enables locked transfers.
    L OFFSET(16) NUMBITS(1) [],
    /// Active: 0 = there is no data in the FIFO of the channel. 1 = the channel FIFO ha
    A OFFSET(17) NUMBITS(1) [],
    /// Halt: 0 = enable DMA requests. 1 = ignore further source DMA requests. The conte
    H OFFSET(18) NUMBITS(1) [
        /// Enable DMA requests.
        EnableDMARequests = 0,
        /// Ignore further source DMA requests.
        IgnoreFurtherSourceDMARequests = 1
    ]
],
C3CONFIG [
    /// Channel enable. Reading this bit indicates whether a channel is currently enable
    E OFFSET(0) NUMBITS(1) [
        /// Channel disabled.
        ChannelDisabled = 0,
        /// Channel enabled.
        ChannelEnabled = 1
    ],
    /// Source peripheral. This value selects the DMA source request peripheral. This fi
    SRCPERIPHERAL OFFSET(1) NUMBITS(5) [
        /// Source = SPIFI
        SourceSPIFI = 0,
        /// Source = Timer 0 match 0/UART0 transmit
        SourceTimer0Match0UART0Transmit = 1,
        /// Source = Timer 0 match 1/UART0 receive
        SourceTimer0Match1UART0Receive = 2,
        /// Source = Timer 1 match 0/UART1 transmit
        SourceTimer1Match0UART1Transmit = 3,
        /// Source = Timer 1 match 1/UART 1 receive
        SourceTimer1Match1UART1Receive = 4,
        /// Source = Timer 2 match 0/UART 2 transmit
        SourceTimer2Match0UART2Transmit = 5,
        /// Source = Timer 2 match 1/UART 2 receive
        SourceTimer2Match1UART2Receive = 6,
        /// Source = Timer 3 match 0/UART3 transmit/SCT DMA request 0
        SourceTimer3Match0UART3TransmitSCTDMARequest0 = 7,
        /// Source = Timer 3 match 1/UART3 receive/SCT DMA request 1
        SourceTimer3Match1UART3ReceiveSCTDMARequest1 = 8,
        /// Source = SSP0 receive/I2S channel 0
        SourceSSP0ReceiveI2SChannel0 = 9,
        /// Source = SSP0 transmit/I2S channel 1
        SourceSSP0TransmitI2SChannel1 = 10,
        /// Source = SSP1 receive
        SourceSSP1Receive = 11,
        /// Source = SSP1 transmit
        SourceSSP1Transmit = 12,
        /// Source = ADC0
        SourceADC0 = 13,
        /// Source = ADC1
        SourceADC1 = 14,
        /// Source = DAC
        SourceDAC = 15
    ],
    /// Destination peripheral. This value selects the DMA destination request periphera
    DESTPERIPHERAL OFFSET(6) NUMBITS(5) [
        /// Destination = SPIFI
        DestinationSPIFI = 0,
        /// Destination = Timer 0 match 0/UART0 transmit
        DestinationTimer0Match0UART0Transmit = 1,
        /// Destination = Timer 0 match 1/UART0 receive
        DestinationTimer0Match1UART0Receive = 2,
        /// Destination = Timer 1 match 0/UART1 transmit
        DestinationTimer1Match0UART1Transmit = 3,
        /// Destination = Timer 1 match 1/UART 1 receive
        DestinationTimer1Match1UART1Receive = 4,
        /// Destination = Timer 2 match 0/UART 2 transmit
        DestinationTimer2Match0UART2Transmit = 5,
        /// Destination = Timer 2 match 1/UART 2 receive
        DestinationTimer2Match1UART2Receive = 6,
        /// Destination = Timer 3 match 0/UART3 transmit/SCT DMA request 0
        DestinationTimer3Match0UART3TransmitSCTDMARequest0 = 7,
        /// Destination = Timer 3 match 1/UART3 receive/SCT DMA request 1
        DestinationTimer3Match1UART3ReceiveSCTDMARequest1 = 8,
        /// Destination = SSP0 receive/I2S channel 0
        DestinationSSP0ReceiveI2SChannel0 = 9,
        /// Destination = SSP0 transmit/I2S channel 1
        DestinationSSP0TransmitI2SChannel1 = 10,
        /// Destination = SSP1 receive
        DestinationSSP1Receive = 11,
        /// Destination = SSP1 transmit
        DestinationSSP1Transmit = 12,
        /// Destination = ADC0
        DestinationADC0 = 13,
        /// Destination = ADC1
        DestinationADC1 = 14,
        /// Destination = DAC
        DestinationDAC = 15
    ],
    /// Flow control and transfer type. This value indicates the flow controller and tra
    FLOWCNTRL OFFSET(11) NUMBITS(3) [
        /// Memory to memory (DMA control)
        MemoryToMemoryDMAControl = 0,
        /// Memory to peripheral (DMA control)
        MemoryToPeripheralDMAControl = 1,
        /// Peripheral to memory (DMA control)
        PeripheralToMemoryDMAControl = 2,
        /// Source peripheral to destination peripheral (DMA control)
        SourcePeripheralToDestinationPeripheralDMAControl = 3,
        /// Source peripheral to destination peripheral (destination control)
        SourcePeripheralToDestinationPeripheralDestinationControl = 4,
        /// Memory to peripheral (peripheral control)
        MemoryToPeripheralPeripheralControl = 5,
        /// Peripheral to memory (peripheral control)
        PeripheralToMemoryPeripheralControl = 6,
        /// Source peripheral to destination peripheral (source control)
        SourcePeripheralToDestinationPeripheralSourceControl = 7
    ],
    /// Interrupt error mask. When cleared, this bit masks out the error interrupt of th
    IE OFFSET(14) NUMBITS(1) [],
    /// Terminal count interrupt mask. When cleared, this bit masks out the terminal cou
    ITC OFFSET(15) NUMBITS(1) [],
    /// Lock. When set, this bit enables locked transfers.
    L OFFSET(16) NUMBITS(1) [],
    /// Active: 0 = there is no data in the FIFO of the channel. 1 = the channel FIFO ha
    A OFFSET(17) NUMBITS(1) [],
    /// Halt: 0 = enable DMA requests. 1 = ignore further source DMA requests. The conte
    H OFFSET(18) NUMBITS(1) [
        /// Enable DMA requests.
        EnableDMARequests = 0,
        /// Ignore further source DMA requests.
        IgnoreFurtherSourceDMARequests = 1
    ]
],
C4CONFIG [
    /// Channel enable. Reading this bit indicates whether a channel is currently enable
    E OFFSET(0) NUMBITS(1) [
        /// Channel disabled.
        ChannelDisabled = 0,
        /// Channel enabled.
        ChannelEnabled = 1
    ],
    /// Source peripheral. This value selects the DMA source request peripheral. This fi
    SRCPERIPHERAL OFFSET(1) NUMBITS(5) [
        /// Source = SPIFI
        SourceSPIFI = 0,
        /// Source = Timer 0 match 0/UART0 transmit
        SourceTimer0Match0UART0Transmit = 1,
        /// Source = Timer 0 match 1/UART0 receive
        SourceTimer0Match1UART0Receive = 2,
        /// Source = Timer 1 match 0/UART1 transmit
        SourceTimer1Match0UART1Transmit = 3,
        /// Source = Timer 1 match 1/UART 1 receive
        SourceTimer1Match1UART1Receive = 4,
        /// Source = Timer 2 match 0/UART 2 transmit
        SourceTimer2Match0UART2Transmit = 5,
        /// Source = Timer 2 match 1/UART 2 receive
        SourceTimer2Match1UART2Receive = 6,
        /// Source = Timer 3 match 0/UART3 transmit/SCT DMA request 0
        SourceTimer3Match0UART3TransmitSCTDMARequest0 = 7,
        /// Source = Timer 3 match 1/UART3 receive/SCT DMA request 1
        SourceTimer3Match1UART3ReceiveSCTDMARequest1 = 8,
        /// Source = SSP0 receive/I2S channel 0
        SourceSSP0ReceiveI2SChannel0 = 9,
        /// Source = SSP0 transmit/I2S channel 1
        SourceSSP0TransmitI2SChannel1 = 10,
        /// Source = SSP1 receive
        SourceSSP1Receive = 11,
        /// Source = SSP1 transmit
        SourceSSP1Transmit = 12,
        /// Source = ADC0
        SourceADC0 = 13,
        /// Source = ADC1
        SourceADC1 = 14,
        /// Source = DAC
        SourceDAC = 15
    ],
    /// Destination peripheral. This value selects the DMA destination request periphera
    DESTPERIPHERAL OFFSET(6) NUMBITS(5) [
        /// Destination = SPIFI
        DestinationSPIFI = 0,
        /// Destination = Timer 0 match 0/UART0 transmit
        DestinationTimer0Match0UART0Transmit = 1,
        /// Destination = Timer 0 match 1/UART0 receive
        DestinationTimer0Match1UART0Receive = 2,
        /// Destination = Timer 1 match 0/UART1 transmit
        DestinationTimer1Match0UART1Transmit = 3,
        /// Destination = Timer 1 match 1/UART 1 receive
        DestinationTimer1Match1UART1Receive = 4,
        /// Destination = Timer 2 match 0/UART 2 transmit
        DestinationTimer2Match0UART2Transmit = 5,
        /// Destination = Timer 2 match 1/UART 2 receive
        DestinationTimer2Match1UART2Receive = 6,
        /// Destination = Timer 3 match 0/UART3 transmit/SCT DMA request 0
        DestinationTimer3Match0UART3TransmitSCTDMARequest0 = 7,
        /// Destination = Timer 3 match 1/UART3 receive/SCT DMA request 1
        DestinationTimer3Match1UART3ReceiveSCTDMARequest1 = 8,
        /// Destination = SSP0 receive/I2S channel 0
        DestinationSSP0ReceiveI2SChannel0 = 9,
        /// Destination = SSP0 transmit/I2S channel 1
        DestinationSSP0TransmitI2SChannel1 = 10,
        /// Destination = SSP1 receive
        DestinationSSP1Receive = 11,
        /// Destination = SSP1 transmit
        DestinationSSP1Transmit = 12,
        /// Destination = ADC0
        DestinationADC0 = 13,
        /// Destination = ADC1
        DestinationADC1 = 14,
        /// Destination = DAC
        DestinationDAC = 15
    ],
    /// Flow control and transfer type. This value indicates the flow controller and tra
    FLOWCNTRL OFFSET(11) NUMBITS(3) [
        /// Memory to memory (DMA control)
        MemoryToMemoryDMAControl = 0,
        /// Memory to peripheral (DMA control)
        MemoryToPeripheralDMAControl = 1,
        /// Peripheral to memory (DMA control)
        PeripheralToMemoryDMAControl = 2,
        /// Source peripheral to destination peripheral (DMA control)
        SourcePeripheralToDestinationPeripheralDMAControl = 3,
        /// Source peripheral to destination peripheral (destination control)
        SourcePeripheralToDestinationPeripheralDestinationControl = 4,
        /// Memory to peripheral (peripheral control)
        MemoryToPeripheralPeripheralControl = 5,
        /// Peripheral to memory (peripheral control)
        PeripheralToMemoryPeripheralControl = 6,
        /// Source peripheral to destination peripheral (source control)
        SourcePeripheralToDestinationPeripheralSourceControl = 7
    ],
    /// Interrupt error mask. When cleared, this bit masks out the error interrupt of th
    IE OFFSET(14) NUMBITS(1) [],
    /// Terminal count interrupt mask. When cleared, this bit masks out the terminal cou
    ITC OFFSET(15) NUMBITS(1) [],
    /// Lock. When set, this bit enables locked transfers.
    L OFFSET(16) NUMBITS(1) [],
    /// Active: 0 = there is no data in the FIFO of the channel. 1 = the channel FIFO ha
    A OFFSET(17) NUMBITS(1) [],
    /// Halt: 0 = enable DMA requests. 1 = ignore further source DMA requests. The conte
    H OFFSET(18) NUMBITS(1) [
        /// Enable DMA requests.
        EnableDMARequests = 0,
        /// Ignore further source DMA requests.
        IgnoreFurtherSourceDMARequests = 1
    ]
],
C5CONFIG [
    /// Channel enable. Reading this bit indicates whether a channel is currently enable
    E OFFSET(0) NUMBITS(1) [
        /// Channel disabled.
        ChannelDisabled = 0,
        /// Channel enabled.
        ChannelEnabled = 1
    ],
    /// Source peripheral. This value selects the DMA source request peripheral. This fi
    SRCPERIPHERAL OFFSET(1) NUMBITS(5) [
        /// Source = SPIFI
        SourceSPIFI = 0,
        /// Source = Timer 0 match 0/UART0 transmit
        SourceTimer0Match0UART0Transmit = 1,
        /// Source = Timer 0 match 1/UART0 receive
        SourceTimer0Match1UART0Receive = 2,
        /// Source = Timer 1 match 0/UART1 transmit
        SourceTimer1Match0UART1Transmit = 3,
        /// Source = Timer 1 match 1/UART 1 receive
        SourceTimer1Match1UART1Receive = 4,
        /// Source = Timer 2 match 0/UART 2 transmit
        SourceTimer2Match0UART2Transmit = 5,
        /// Source = Timer 2 match 1/UART 2 receive
        SourceTimer2Match1UART2Receive = 6,
        /// Source = Timer 3 match 0/UART3 transmit/SCT DMA request 0
        SourceTimer3Match0UART3TransmitSCTDMARequest0 = 7,
        /// Source = Timer 3 match 1/UART3 receive/SCT DMA request 1
        SourceTimer3Match1UART3ReceiveSCTDMARequest1 = 8,
        /// Source = SSP0 receive/I2S channel 0
        SourceSSP0ReceiveI2SChannel0 = 9,
        /// Source = SSP0 transmit/I2S channel 1
        SourceSSP0TransmitI2SChannel1 = 10,
        /// Source = SSP1 receive
        SourceSSP1Receive = 11,
        /// Source = SSP1 transmit
        SourceSSP1Transmit = 12,
        /// Source = ADC0
        SourceADC0 = 13,
        /// Source = ADC1
        SourceADC1 = 14,
        /// Source = DAC
        SourceDAC = 15
    ],
    /// Destination peripheral. This value selects the DMA destination request periphera
    DESTPERIPHERAL OFFSET(6) NUMBITS(5) [
        /// Destination = SPIFI
        DestinationSPIFI = 0,
        /// Destination = Timer 0 match 0/UART0 transmit
        DestinationTimer0Match0UART0Transmit = 1,
        /// Destination = Timer 0 match 1/UART0 receive
        DestinationTimer0Match1UART0Receive = 2,
        /// Destination = Timer 1 match 0/UART1 transmit
        DestinationTimer1Match0UART1Transmit = 3,
        /// Destination = Timer 1 match 1/UART 1 receive
        DestinationTimer1Match1UART1Receive = 4,
        /// Destination = Timer 2 match 0/UART 2 transmit
        DestinationTimer2Match0UART2Transmit = 5,
        /// Destination = Timer 2 match 1/UART 2 receive
        DestinationTimer2Match1UART2Receive = 6,
        /// Destination = Timer 3 match 0/UART3 transmit/SCT DMA request 0
        DestinationTimer3Match0UART3TransmitSCTDMARequest0 = 7,
        /// Destination = Timer 3 match 1/UART3 receive/SCT DMA request 1
        DestinationTimer3Match1UART3ReceiveSCTDMARequest1 = 8,
        /// Destination = SSP0 receive/I2S channel 0
        DestinationSSP0ReceiveI2SChannel0 = 9,
        /// Destination = SSP0 transmit/I2S channel 1
        DestinationSSP0TransmitI2SChannel1 = 10,
        /// Destination = SSP1 receive
        DestinationSSP1Receive = 11,
        /// Destination = SSP1 transmit
        DestinationSSP1Transmit = 12,
        /// Destination = ADC0
        DestinationADC0 = 13,
        /// Destination = ADC1
        DestinationADC1 = 14,
        /// Destination = DAC
        DestinationDAC = 15
    ],
    /// Flow control and transfer type. This value indicates the flow controller and tra
    FLOWCNTRL OFFSET(11) NUMBITS(3) [
        /// Memory to memory (DMA control)
        MemoryToMemoryDMAControl = 0,
        /// Memory to peripheral (DMA control)
        MemoryToPeripheralDMAControl = 1,
        /// Peripheral to memory (DMA control)
        PeripheralToMemoryDMAControl = 2,
        /// Source peripheral to destination peripheral (DMA control)
        SourcePeripheralToDestinationPeripheralDMAControl = 3,
        /// Source peripheral to destination peripheral (destination control)
        SourcePeripheralToDestinationPeripheralDestinationControl = 4,
        /// Memory to peripheral (peripheral control)
        MemoryToPeripheralPeripheralControl = 5,
        /// Peripheral to memory (peripheral control)
        PeripheralToMemoryPeripheralControl = 6,
        /// Source peripheral to destination peripheral (source control)
        SourcePeripheralToDestinationPeripheralSourceControl = 7
    ],
    /// Interrupt error mask. When cleared, this bit masks out the error interrupt of th
    IE OFFSET(14) NUMBITS(1) [],
    /// Terminal count interrupt mask. When cleared, this bit masks out the terminal cou
    ITC OFFSET(15) NUMBITS(1) [],
    /// Lock. When set, this bit enables locked transfers.
    L OFFSET(16) NUMBITS(1) [],
    /// Active: 0 = there is no data in the FIFO of the channel. 1 = the channel FIFO ha
    A OFFSET(17) NUMBITS(1) [],
    /// Halt: 0 = enable DMA requests. 1 = ignore further source DMA requests. The conte
    H OFFSET(18) NUMBITS(1) [
        /// Enable DMA requests.
        EnableDMARequests = 0,
        /// Ignore further source DMA requests.
        IgnoreFurtherSourceDMARequests = 1
    ]
],
C6CONFIG [
    /// Channel enable. Reading this bit indicates whether a channel is currently enable
    E OFFSET(0) NUMBITS(1) [
        /// Channel disabled.
        ChannelDisabled = 0,
        /// Channel enabled.
        ChannelEnabled = 1
    ],
    /// Source peripheral. This value selects the DMA source request peripheral. This fi
    SRCPERIPHERAL OFFSET(1) NUMBITS(5) [
        /// Source = SPIFI
        SourceSPIFI = 0,
        /// Source = Timer 0 match 0/UART0 transmit
        SourceTimer0Match0UART0Transmit = 1,
        /// Source = Timer 0 match 1/UART0 receive
        SourceTimer0Match1UART0Receive = 2,
        /// Source = Timer 1 match 0/UART1 transmit
        SourceTimer1Match0UART1Transmit = 3,
        /// Source = Timer 1 match 1/UART 1 receive
        SourceTimer1Match1UART1Receive = 4,
        /// Source = Timer 2 match 0/UART 2 transmit
        SourceTimer2Match0UART2Transmit = 5,
        /// Source = Timer 2 match 1/UART 2 receive
        SourceTimer2Match1UART2Receive = 6,
        /// Source = Timer 3 match 0/UART3 transmit/SCT DMA request 0
        SourceTimer3Match0UART3TransmitSCTDMARequest0 = 7,
        /// Source = Timer 3 match 1/UART3 receive/SCT DMA request 1
        SourceTimer3Match1UART3ReceiveSCTDMARequest1 = 8,
        /// Source = SSP0 receive/I2S channel 0
        SourceSSP0ReceiveI2SChannel0 = 9,
        /// Source = SSP0 transmit/I2S channel 1
        SourceSSP0TransmitI2SChannel1 = 10,
        /// Source = SSP1 receive
        SourceSSP1Receive = 11,
        /// Source = SSP1 transmit
        SourceSSP1Transmit = 12,
        /// Source = ADC0
        SourceADC0 = 13,
        /// Source = ADC1
        SourceADC1 = 14,
        /// Source = DAC
        SourceDAC = 15
    ],
    /// Destination peripheral. This value selects the DMA destination request periphera
    DESTPERIPHERAL OFFSET(6) NUMBITS(5) [
        /// Destination = SPIFI
        DestinationSPIFI = 0,
        /// Destination = Timer 0 match 0/UART0 transmit
        DestinationTimer0Match0UART0Transmit = 1,
        /// Destination = Timer 0 match 1/UART0 receive
        DestinationTimer0Match1UART0Receive = 2,
        /// Destination = Timer 1 match 0/UART1 transmit
        DestinationTimer1Match0UART1Transmit = 3,
        /// Destination = Timer 1 match 1/UART 1 receive
        DestinationTimer1Match1UART1Receive = 4,
        /// Destination = Timer 2 match 0/UART 2 transmit
        DestinationTimer2Match0UART2Transmit = 5,
        /// Destination = Timer 2 match 1/UART 2 receive
        DestinationTimer2Match1UART2Receive = 6,
        /// Destination = Timer 3 match 0/UART3 transmit/SCT DMA request 0
        DestinationTimer3Match0UART3TransmitSCTDMARequest0 = 7,
        /// Destination = Timer 3 match 1/UART3 receive/SCT DMA request 1
        DestinationTimer3Match1UART3ReceiveSCTDMARequest1 = 8,
        /// Destination = SSP0 receive/I2S channel 0
        DestinationSSP0ReceiveI2SChannel0 = 9,
        /// Destination = SSP0 transmit/I2S channel 1
        DestinationSSP0TransmitI2SChannel1 = 10,
        /// Destination = SSP1 receive
        DestinationSSP1Receive = 11,
        /// Destination = SSP1 transmit
        DestinationSSP1Transmit = 12,
        /// Destination = ADC0
        DestinationADC0 = 13,
        /// Destination = ADC1
        DestinationADC1 = 14,
        /// Destination = DAC
        DestinationDAC = 15
    ],
    /// Flow control and transfer type. This value indicates the flow controller and tra
    FLOWCNTRL OFFSET(11) NUMBITS(3) [
        /// Memory to memory (DMA control)
        MemoryToMemoryDMAControl = 0,
        /// Memory to peripheral (DMA control)
        MemoryToPeripheralDMAControl = 1,
        /// Peripheral to memory (DMA control)
        PeripheralToMemoryDMAControl = 2,
        /// Source peripheral to destination peripheral (DMA control)
        SourcePeripheralToDestinationPeripheralDMAControl = 3,
        /// Source peripheral to destination peripheral (destination control)
        SourcePeripheralToDestinationPeripheralDestinationControl = 4,
        /// Memory to peripheral (peripheral control)
        MemoryToPeripheralPeripheralControl = 5,
        /// Peripheral to memory (peripheral control)
        PeripheralToMemoryPeripheralControl = 6,
        /// Source peripheral to destination peripheral (source control)
        SourcePeripheralToDestinationPeripheralSourceControl = 7
    ],
    /// Interrupt error mask. When cleared, this bit masks out the error interrupt of th
    IE OFFSET(14) NUMBITS(1) [],
    /// Terminal count interrupt mask. When cleared, this bit masks out the terminal cou
    ITC OFFSET(15) NUMBITS(1) [],
    /// Lock. When set, this bit enables locked transfers.
    L OFFSET(16) NUMBITS(1) [],
    /// Active: 0 = there is no data in the FIFO of the channel. 1 = the channel FIFO ha
    A OFFSET(17) NUMBITS(1) [],
    /// Halt: 0 = enable DMA requests. 1 = ignore further source DMA requests. The conte
    H OFFSET(18) NUMBITS(1) [
        /// Enable DMA requests.
        EnableDMARequests = 0,
        /// Ignore further source DMA requests.
        IgnoreFurtherSourceDMARequests = 1
    ]
],
C7CONFIG [
    /// Channel enable. Reading this bit indicates whether a channel is currently enable
    E OFFSET(0) NUMBITS(1) [
        /// Channel disabled.
        ChannelDisabled = 0,
        /// Channel enabled.
        ChannelEnabled = 1
    ],
    /// Source peripheral. This value selects the DMA source request peripheral. This fi
    SRCPERIPHERAL OFFSET(1) NUMBITS(5) [
        /// Source = SPIFI
        SourceSPIFI = 0,
        /// Source = Timer 0 match 0/UART0 transmit
        SourceTimer0Match0UART0Transmit = 1,
        /// Source = Timer 0 match 1/UART0 receive
        SourceTimer0Match1UART0Receive = 2,
        /// Source = Timer 1 match 0/UART1 transmit
        SourceTimer1Match0UART1Transmit = 3,
        /// Source = Timer 1 match 1/UART 1 receive
        SourceTimer1Match1UART1Receive = 4,
        /// Source = Timer 2 match 0/UART 2 transmit
        SourceTimer2Match0UART2Transmit = 5,
        /// Source = Timer 2 match 1/UART 2 receive
        SourceTimer2Match1UART2Receive = 6,
        /// Source = Timer 3 match 0/UART3 transmit/SCT DMA request 0
        SourceTimer3Match0UART3TransmitSCTDMARequest0 = 7,
        /// Source = Timer 3 match 1/UART3 receive/SCT DMA request 1
        SourceTimer3Match1UART3ReceiveSCTDMARequest1 = 8,
        /// Source = SSP0 receive/I2S channel 0
        SourceSSP0ReceiveI2SChannel0 = 9,
        /// Source = SSP0 transmit/I2S channel 1
        SourceSSP0TransmitI2SChannel1 = 10,
        /// Source = SSP1 receive
        SourceSSP1Receive = 11,
        /// Source = SSP1 transmit
        SourceSSP1Transmit = 12,
        /// Source = ADC0
        SourceADC0 = 13,
        /// Source = ADC1
        SourceADC1 = 14,
        /// Source = DAC
        SourceDAC = 15
    ],
    /// Destination peripheral. This value selects the DMA destination request periphera
    DESTPERIPHERAL OFFSET(6) NUMBITS(5) [
        /// Destination = SPIFI
        DestinationSPIFI = 0,
        /// Destination = Timer 0 match 0/UART0 transmit
        DestinationTimer0Match0UART0Transmit = 1,
        /// Destination = Timer 0 match 1/UART0 receive
        DestinationTimer0Match1UART0Receive = 2,
        /// Destination = Timer 1 match 0/UART1 transmit
        DestinationTimer1Match0UART1Transmit = 3,
        /// Destination = Timer 1 match 1/UART 1 receive
        DestinationTimer1Match1UART1Receive = 4,
        /// Destination = Timer 2 match 0/UART 2 transmit
        DestinationTimer2Match0UART2Transmit = 5,
        /// Destination = Timer 2 match 1/UART 2 receive
        DestinationTimer2Match1UART2Receive = 6,
        /// Destination = Timer 3 match 0/UART3 transmit/SCT DMA request 0
        DestinationTimer3Match0UART3TransmitSCTDMARequest0 = 7,
        /// Destination = Timer 3 match 1/UART3 receive/SCT DMA request 1
        DestinationTimer3Match1UART3ReceiveSCTDMARequest1 = 8,
        /// Destination = SSP0 receive/I2S channel 0
        DestinationSSP0ReceiveI2SChannel0 = 9,
        /// Destination = SSP0 transmit/I2S channel 1
        DestinationSSP0TransmitI2SChannel1 = 10,
        /// Destination = SSP1 receive
        DestinationSSP1Receive = 11,
        /// Destination = SSP1 transmit
        DestinationSSP1Transmit = 12,
        /// Destination = ADC0
        DestinationADC0 = 13,
        /// Destination = ADC1
        DestinationADC1 = 14,
        /// Destination = DAC
        DestinationDAC = 15
    ],
    /// Flow control and transfer type. This value indicates the flow controller and tra
    FLOWCNTRL OFFSET(11) NUMBITS(3) [
        /// Memory to memory (DMA control)
        MemoryToMemoryDMAControl = 0,
        /// Memory to peripheral (DMA control)
        MemoryToPeripheralDMAControl = 1,
        /// Peripheral to memory (DMA control)
        PeripheralToMemoryDMAControl = 2,
        /// Source peripheral to destination peripheral (DMA control)
        SourcePeripheralToDestinationPeripheralDMAControl = 3,
        /// Source peripheral to destination peripheral (destination control)
        SourcePeripheralToDestinationPeripheralDestinationControl = 4,
        /// Memory to peripheral (peripheral control)
        MemoryToPeripheralPeripheralControl = 5,
        /// Peripheral to memory (peripheral control)
        PeripheralToMemoryPeripheralControl = 6,
        /// Source peripheral to destination peripheral (source control)
        SourcePeripheralToDestinationPeripheralSourceControl = 7
    ],
    /// Interrupt error mask. When cleared, this bit masks out the error interrupt of th
    IE OFFSET(14) NUMBITS(1) [],
    /// Terminal count interrupt mask. When cleared, this bit masks out the terminal cou
    ITC OFFSET(15) NUMBITS(1) [],
    /// Lock. When set, this bit enables locked transfers.
    L OFFSET(16) NUMBITS(1) [],
    /// Active: 0 = there is no data in the FIFO of the channel. 1 = the channel FIFO ha
    A OFFSET(17) NUMBITS(1) [],
    /// Halt: 0 = enable DMA requests. 1 = ignore further source DMA requests. The conte
    H OFFSET(18) NUMBITS(1) [
        /// Enable DMA requests.
        EnableDMARequests = 0,
        /// Ignore further source DMA requests.
        IgnoreFurtherSourceDMARequests = 1
    ]
]
];
const GPDMA_BASE: StaticRef<GpdmaRegisters> =
    unsafe { StaticRef::new(0x40002000 as *const GpdmaRegisters) };
