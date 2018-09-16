
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// Ethernet
#[repr(C)]
struct EthernetRegisters {
/// MAC configuration register
mac_config: ReadWrite<u32, MAC_CONFIG::Register>,
/// MAC frame filter
mac_frame_filter: ReadWrite<u32, MAC_FRAME_FILTER::Register>,
/// Hash table high register
mac_hashtable_high: ReadWrite<u32>,
/// Hash table low register
mac_hashtable_low: ReadWrite<u32>,
/// MII address register
mac_mii_addr: ReadWrite<u32, MAC_MII_ADDR::Register>,
/// MII data register
mac_mii_data: ReadWrite<u32>,
/// Flow control register
mac_flow_ctrl: ReadWrite<u32, MAC_FLOW_CTRL::Register>,
/// VLAN tag register
mac_vlan_tag: ReadWrite<u32, MAC_VLAN_TAG::Register>,
_reserved0: [u8; 4],
/// Debug register
mac_debug: ReadOnly<u32, MAC_DEBUG::Register>,
/// Remote wake-up frame filter
mac_rwake_frflt: ReadWrite<u32>,
/// PMT control and status
mac_pmt_ctrl_stat: ReadWrite<u32, MAC_PMT_CTRL_STAT::Register>,
_reserved1: [u8; 8],
/// Interrupt status register
mac_intr: ReadOnly<u32, MAC_INTR::Register>,
/// Interrupt mask register
mac_intr_mask: ReadWrite<u32, MAC_INTR_MASK::Register>,
/// MAC address 0 high register
mac_addr0_high: ReadWrite<u32, MAC_ADDR0_HIGH::Register>,
/// MAC address 0 low register
mac_addr0_low: ReadWrite<u32>,
_reserved2: [u8; 1720],
/// Time stamp control register
mac_timestp_ctrl: ReadWrite<u32, MAC_TIMESTP_CTRL::Register>,
/// Sub-second increment register
subsecond_incr: ReadWrite<u32>,
/// System time seconds register
seconds: ReadOnly<u32>,
/// System time nanoseconds register
nanoseconds: ReadOnly<u32, NANOSECONDS::Register>,
/// System time seconds update register
secondsupdate: ReadWrite<u32>,
/// System time nanoseconds update register
nanosecondsupdate: ReadWrite<u32, NANOSECONDSUPDATE::Register>,
/// Time stamp addend register
addend: ReadWrite<u32>,
/// Target time seconds register
targetseconds: ReadWrite<u32>,
/// Target time nanoseconds register
targetnanoseconds: ReadWrite<u32>,
/// System time higher word seconds register
highword: ReadWrite<u32>,
/// Time stamp status register
timestampstat: ReadOnly<u32, TIMESTAMPSTAT::Register>,
_reserved3: [u8; 2260],
/// Bus Mode Register
dma_bus_mode: ReadWrite<u32, DMA_BUS_MODE::Register>,
/// Transmit poll demand register
dma_trans_poll_demand: ReadWrite<u32>,
/// Receive poll demand register
dma_rec_poll_demand: ReadWrite<u32>,
/// Receive descriptor list address register
dma_rec_des_addr: ReadWrite<u32>,
/// Transmit descriptor list address register
dma_trans_des_addr: ReadWrite<u32>,
/// Status register
dma_stat: ReadWrite<u32, DMA_STAT::Register>,
/// Operation mode register
dma_op_mode: ReadWrite<u32, DMA_OP_MODE::Register>,
/// Interrupt enable register
dma_int_en: ReadWrite<u32, DMA_INT_EN::Register>,
/// Missed frame and buffer overflow register
dma_mfrm_bufof: ReadOnly<u32, DMA_MFRM_BUFOF::Register>,
/// Receive interrupt watchdog timer register
dma_rec_int_wdt: ReadWrite<u32>,
_reserved4: [u8; 32],
/// Current host transmit descriptor register
dma_curhost_trans_des: ReadOnly<u32>,
/// Current host receive descriptor register
dma_curhost_rec_des: ReadOnly<u32>,
/// Current host transmit buffer address register
dma_curhost_trans_buf: ReadOnly<u32>,
/// Current host receive buffer address register
dma_curhost_rec_buf: ReadOnly<u32>,
}
register_bitfields![u32,
MAC_CONFIG [
    /// Receiver enable When this bit is set, the receiver state machine of the MAC is e
    RE OFFSET(2) NUMBITS(1) [],
    /// Transmitter Enable  When this bit is set, the transmit state machine of the MAC
    TE OFFSET(3) NUMBITS(1) [],
    /// Deferral Check  When this bit is set, the deferral check function is enabled in
    DF OFFSET(4) NUMBITS(1) [],
    /// Back-Off Limit  The Back-Off limit determines the random integer number (r) of s
    BL OFFSET(5) NUMBITS(2) [],
    /// Automatic Pad/CRC Stripping  When this bit is set, the MAC strips the Pad/FCS fi
    ACS OFFSET(7) NUMBITS(1) [],
    /// Disable Retry  When this bit is set, the MAC will attempt only 1 transmission. W
    DR OFFSET(9) NUMBITS(1) [],
    /// Duplex Mode  When this bit is set, the MAC operates in a Full-Duplex mode where
    DM OFFSET(11) NUMBITS(1) [],
    /// Loopback Mode  When this bit is set, the MAC operates in loopback mode at MII. T
    LM OFFSET(12) NUMBITS(1) [],
    /// Disable Receive Own  When this bit is set, the MAC disables the reception of fra
    DO OFFSET(13) NUMBITS(1) [],
    /// Speed Indicates the speed in Fast Ethernet (MII) mode:  0 = 10 Mbps 1 = 100 Mbps
    FES OFFSET(14) NUMBITS(1) [],
    /// Port select   1 = MII (100 Mbp) - this is the only allowed value.
    PS OFFSET(15) NUMBITS(1) [],
    /// Disable carrier sense during transmission When set high, this bit makes the MAC
    DCRS OFFSET(16) NUMBITS(1) [],
    /// Inter-frame gap These bits control the minimum IFG between frames during transmi
    IFG OFFSET(17) NUMBITS(3) [],
    /// Jumbo Frame Enable  When this bit is set, MAC allows Jumbo frames of 9,018 bytes
    JE OFFSET(20) NUMBITS(1) [],
    /// Jabber Disable  When this bit is set, the MAC disables the jabber timer on the t
    JD OFFSET(22) NUMBITS(1) [],
    /// Watchdog Disable  When this bit is set, the MAC disables the watchdog timer on t
    WD OFFSET(23) NUMBITS(1) []
],
MAC_FRAME_FILTER [
    /// Promiscuous Mode  When this bit is set, the Address Filter module passes all inc
    PR OFFSET(0) NUMBITS(1) [],
    /// Hash Unicast When set, MAC performs destination address filtering of unicast fra
    HUC OFFSET(1) NUMBITS(1) [],
    /// Hash Multicast When set, MAC performs destination address filtering of received
    HMC OFFSET(2) NUMBITS(1) [],
    /// DA Inverse Filtering  When this bit is set, the Address Check block operates in
    DAIF OFFSET(3) NUMBITS(1) [],
    /// Pass All Multicast  When set, this bit indicates that all received frames with a
    PM OFFSET(4) NUMBITS(1) [],
    /// Disable Broadcast Frames  When this bit is set, the AFM module filters all incom
    DBF OFFSET(5) NUMBITS(1) [],
    /// Pass Control Frames  These bits control the forwarding of all control frames (in
    PCF OFFSET(6) NUMBITS(2) [],
    /// Hash or perfect filter When set, this bit configures the address filter to pass
    HPF OFFSET(10) NUMBITS(1) [],
    /// Receive all When this bit is set, the MAC Receiver module passes to the Applicat
    RA OFFSET(31) NUMBITS(1) []
],
MAC_MII_ADDR [
    /// MII busy This register field can be read by the application (Read), can be set t
    GB OFFSET(0) NUMBITS(1) [],
    /// MII write When set, this bit tells the PHY that this will be a Write operation u
    W OFFSET(1) NUMBITS(1) [],
    /// CSR clock range The CSR Clock Range selection determines the frequency of the MD
    CR OFFSET(2) NUMBITS(4) [],
    /// MII register These bits select the desired MII register in the selected PHY devi
    GR OFFSET(6) NUMBITS(5) [],
    /// Physical layer address This field tells which of the 32 possible PHY devices are
    PA OFFSET(11) NUMBITS(5) []
],
MAC_FLOW_CTRL [
    /// Flow Control Busy/Backpressure Activate  This register field can be read by the
    FCB OFFSET(0) NUMBITS(1) [],
    /// Transmit Flow Control Enable  In Full-Duplex mode, when this bit is set, the MAC
    TFE OFFSET(1) NUMBITS(1) [],
    /// Receive Flow Control Enable  When this bit is set, the MAC will decode the recei
    RFE OFFSET(2) NUMBITS(1) [],
    /// Unicast Pause Frame Detect  When this bit is set, the MAC will detect the Pause
    UP OFFSET(3) NUMBITS(1) [],
    /// Pause Low Threshold  This field configures the threshold of the PAUSE timer at w
    PLT OFFSET(4) NUMBITS(2) [],
    /// Disable Zero-Quanta Pause  When set, this bit disables the automatic generation
    DZPQ OFFSET(7) NUMBITS(1) [],
    /// Pause time This field holds the value to be used in the Pause Time field in the
    PT OFFSET(16) NUMBITS(16) []
],
MAC_VLAN_TAG [
    /// VLAN Tag Identifier for Receive Frames  This contains the 802.1Q VLAN tag to ide
    VL OFFSET(0) NUMBITS(16) [],
    /// Enable 12-Bit VLAN Tag Comparison  When this bit is set, a 12-bit VLAN identifie
    ETV OFFSET(16) NUMBITS(1) []
],
MAC_DEBUG [
    /// When high, it indicates that the MAC MII receive protocol engine is actively rec
    RXIDLESTAT OFFSET(0) NUMBITS(1) [],
    /// When high, it indicates the active state of the small FIFO Read and Write contro
    FIFOSTAT0 OFFSET(1) NUMBITS(2) [],
    /// When high, it indicates that the MTL RxFIFO Write Controller is active and trans
    RXFIFOSTAT1 OFFSET(4) NUMBITS(1) [],
    /// State of the RxFIFO read Controller:  00 = idle state 01 = reading frame data 10
    RXFIFOSTAT OFFSET(5) NUMBITS(2) [],
    /// Status of the RxFIFO Fill-level  00 = RxFIFO Empty  01 = RxFIFO fill-level below
    RXFIFOLVL OFFSET(8) NUMBITS(2) [],
    /// When high, it indicates that the MAC MII transmit protocol engine is actively tr
    TXIDLESTAT OFFSET(16) NUMBITS(1) [],
    /// State of the MAC Transmit Frame Controller module:  00 = idle 01 = Waiting for S
    TXSTAT OFFSET(17) NUMBITS(2) [],
    /// When high, it indicates that the MAC transmitter is in PAUSE condition (in full-
    PAUSE OFFSET(19) NUMBITS(1) [],
    /// State of the TxFIFO read Controller  00 = idle state 01 = READ state (transferri
    TXFIFOSTAT OFFSET(20) NUMBITS(2) [],
    /// When high, it indicates that the TxFIFO Write Controller is active and transferr
    TXFIFOSTAT1 OFFSET(22) NUMBITS(1) [],
    /// When high, it indicates that the TxFIFO is not empty and has some data left for
    TXFIFOLVL OFFSET(24) NUMBITS(1) [],
    /// When high, it indicates that the TxStatus FIFO is full and hence the controller
    TXFIFOFULL OFFSET(25) NUMBITS(1) []
],
MAC_PMT_CTRL_STAT [
    /// Power-down This register field can be read by the application (Read), can be set
    PD OFFSET(0) NUMBITS(1) [],
    /// Magic packet enable When set, enables generation of a power management event due
    MPE OFFSET(1) NUMBITS(1) [],
    /// Wake-up frame enable When set, enables generation of a power management event du
    WFE OFFSET(2) NUMBITS(1) [],
    /// Magic Packet Received This register field can be read by the application (Read),
    MPR OFFSET(5) NUMBITS(1) [],
    /// Wake-up Frame Received This register field can be read by the application (Read)
    WFR OFFSET(6) NUMBITS(1) [],
    /// Global Unicast When set, enables any unicast packet filtered by the MAC (DAF) ad
    GU OFFSET(9) NUMBITS(1) [],
    /// Wake-up Frame Filter Register Pointer Reset This register field can be read by t
    WFFRPR OFFSET(31) NUMBITS(1) []
],
MAC_INTR [
    /// PMT Interrupt Status  This bit is set whenever a Magic packet or Wake-on-LAN fra
    PMT OFFSET(3) NUMBITS(1) [],
    /// Timestamp interrupt status When Advanced Timestamp feature is enabled, this bit
    TS OFFSET(9) NUMBITS(1) []
],
MAC_INTR_MASK [
    /// PMT Interrupt Mask  This bit when set, will disable the assertion of the interru
    PMTIM OFFSET(3) NUMBITS(1) [],
    /// Timestamp interrupt mask When set, this bit disables the assertion of the interr
    TSIM OFFSET(9) NUMBITS(1) []
],
MAC_ADDR0_HIGH [
    /// MAC Address0 [47:32]  This field contains the upper 16 bits (47:32) of the 6-byt
    A47_32 OFFSET(0) NUMBITS(16) [],
    /// Always 1
    MO OFFSET(31) NUMBITS(1) []
],
MAC_TIMESTP_CTRL [
    /// Time stamp Enable  When this bit, is set the timestamping is enabled for transmi
    TSENA OFFSET(0) NUMBITS(1) [],
    /// Time stamp Fine or Coarse Update  When set, indicates that the system times upda
    TSCFUPDT OFFSET(1) NUMBITS(1) [],
    /// Time stamp Initialize This register field can be read and written by the applica
    TSINIT OFFSET(2) NUMBITS(1) [],
    /// Time stamp Update  This register field can be read and written by the applicatio
    TSUPDT OFFSET(3) NUMBITS(1) [],
    /// Time stamp Interrupt Trigger Enable  This register field can be read and written
    TSTRIG OFFSET(4) NUMBITS(1) [],
    /// Addend Reg Update  When set, the contents of the Time stamp Addend register is u
    TSADDREG OFFSET(5) NUMBITS(1) [],
    /// Enable Time stamp for All Frames  When set, the Time stamp snapshot is enabled f
    TSENALL OFFSET(8) NUMBITS(1) [],
    /// Time stamp Digital or Binary rollover control  When set, the Time stamp Low regi
    TSCTRLSSR OFFSET(9) NUMBITS(1) [],
    /// Enable PTP packet snooping for version 2 format  When set, the PTP packets are s
    TSVER2ENA OFFSET(10) NUMBITS(1) [],
    /// Enable Time stamp Snapshot for PTP over Ethernet frames  When set, the Time stam
    TSIPENA OFFSET(11) NUMBITS(1) [],
    /// Enable Time stamp Snapshot for IPv6 frames  When set, the Time stamp snapshot is
    TSIPV6ENA OFFSET(12) NUMBITS(1) [],
    /// Enable Time stamp Snapshot for IPv4 frames  When set, the Time stamp snapshot is
    TSIPV4ENA OFFSET(13) NUMBITS(1) [],
    /// Enable Time stamp Snapshot for Event Messages  When set, the Time stamp snapshot
    TSEVNTENA OFFSET(14) NUMBITS(1) [],
    /// Enable Snapshot for Messages Relevant to Master  When set, the snapshot is taken
    TSMSTRENA OFFSET(15) NUMBITS(1) [],
    /// Select the type of clock node  The following are the options to select the type
    TSCLKTYPE OFFSET(16) NUMBITS(2) [],
    /// Enable MAC address for PTP frame filtering  When set, uses the DA MAC address (t
    TSENMACADDR OFFSET(18) NUMBITS(1) []
],
NANOSECONDS [
    /// Time stamp sub seconds The value in this field has the sub second representation
    TSSS OFFSET(0) NUMBITS(31) [],
    /// Positive or negative time This bit indicates positive or negative time value. If
    PSNT OFFSET(31) NUMBITS(1) []
],
NANOSECONDSUPDATE [
    /// Time stamp sub seconds The value in this field has the sub second representation
    TSSS OFFSET(0) NUMBITS(31) [],
    /// Add or subtract time When this bit is set, the time value is subtracted with the
    ADDSUB OFFSET(31) NUMBITS(1) []
],
TIMESTAMPSTAT [
    /// Time stamp seconds overflow When set, indicates that the seconds value of the Ti
    TSSOVF OFFSET(0) NUMBITS(1) [],
    /// Time stamp target reached When set, indicates the value of system time is greate
    TSTARGT OFFSET(1) NUMBITS(1) []
],
DMA_BUS_MODE [
    /// Software reset This register field can be read by the application (Read), can be
    SWR OFFSET(0) NUMBITS(1) [],
    /// DMA arbitration scheme 0 = Round-robin with Rx:Tx priority given in bits [15:14]
    DA OFFSET(1) NUMBITS(1) [],
    /// Descriptor skip length This bit specifies the number of Word to skip between two
    DSL OFFSET(2) NUMBITS(5) [],
    /// Alternate descriptor size When set, the alternate descriptor (see Section 26.7.6
    ATDS OFFSET(7) NUMBITS(1) [],
    /// Programmable burst length These bits indicate the maximum number of beats to be
    PBL OFFSET(8) NUMBITS(6) [],
    /// Rx-to-Tx priority ratio RxDMA requests given priority over TxDMA requests in the
    PR OFFSET(14) NUMBITS(2) [],
    /// Fixed burst This bit controls whether the AHB Master interface performs fixed bu
    FB OFFSET(16) NUMBITS(1) [],
    /// RxDMA PBL These bits indicate the maximum number of beats to be transferred in o
    RPBL OFFSET(17) NUMBITS(6) [],
    /// Use separate PBL When set high, it configures the RxDMA to use the value configu
    USP OFFSET(23) NUMBITS(1) [],
    /// 8 x PBL mode When set high, this bit multiplies the PBL value programmed (bits [
    PBL8X OFFSET(24) NUMBITS(1) [],
    /// Address-aligned beats When this bit is set high and the FB bit equals 1, the AHB
    AAL OFFSET(25) NUMBITS(1) [],
    /// Mixed burst When this bit is set high and FB bit is low, the AHB master interfac
    MB OFFSET(26) NUMBITS(1) [],
    /// When set, this bit indicates that the transmit DMA has higher priority than the
    TXPR OFFSET(27) NUMBITS(1) []
],
DMA_STAT [
    /// Transmit interrupt This bit indicates that frame transmission is finished and TD
    TI OFFSET(0) NUMBITS(1) [],
    /// Transmit process stopped This bit is set when the transmission is stopped.
    TPS OFFSET(1) NUMBITS(1) [],
    /// Transmit buffer unavailable  This bit indicates that the Next Descriptor in the
    TU OFFSET(2) NUMBITS(1) [],
    /// Transmit jabber timeout  This bit indicates that the Transmit Jabber Timer expir
    TJT OFFSET(3) NUMBITS(1) [],
    /// Receive overflow This bit indicates that the Receive Buffer had an Overflow duri
    OVF OFFSET(4) NUMBITS(1) [],
    /// Transmit underflow This bit indicates that the Transmit Buffer had an Underflow
    UNF OFFSET(5) NUMBITS(1) [],
    /// Receive interrupt  This bit indicates the completion of frame reception. Specifi
    RI OFFSET(6) NUMBITS(1) [],
    /// Receive buffer unavailable  This bit indicates that the Next Descriptor in the R
    RU OFFSET(7) NUMBITS(1) [],
    /// Received process stopped This bit is asserted when the Receive Process enters th
    RPS OFFSET(8) NUMBITS(1) [],
    /// Receive watchdog timeout  This bit is asserted when a frame with a length greate
    RWT OFFSET(9) NUMBITS(1) [],
    /// Early transmit interrupt  This bit indicates that the frame to be transmitted wa
    ETI OFFSET(10) NUMBITS(1) [],
    /// Fatal bus error interrupt This bit indicates that a bus error occurred, as detai
    FBI OFFSET(13) NUMBITS(1) [],
    /// Early receive interrupt  This bit indicates that the DMA had filled the first da
    ERI OFFSET(14) NUMBITS(1) [],
    /// Abnormal interrupt summary  Abnormal Interrupt Summary bit value is the logical
    AIE OFFSET(15) NUMBITS(1) [],
    /// Normal interrupt summary Normal Interrupt Summary bit value is the logical OR of
    NIS OFFSET(16) NUMBITS(1) [],
    /// Receive Process State These bits indicate the receive DMA state machine state. T
    RS OFFSET(17) NUMBITS(3) [],
    /// Transmit Process State These bits indicate the transmit DMA state machine state.
    TS OFFSET(20) NUMBITS(3) [],
    /// Error bit 1 This bit indicates the type of error that caused a Bus Error (e.g.,
    EB1 OFFSET(23) NUMBITS(1) [],
    /// Error bit 2 This bit indicates the type of error that caused a Bus Error (e.g.,
    EB2 OFFSET(24) NUMBITS(1) [],
    /// Error bit 3 This bit indicates the type of error that caused a Bus Error (e.g.,
    EB3 OFFSET(25) NUMBITS(1) []
],
DMA_OP_MODE [
    /// Start/stop receive When this bit is set, the Receive process is placed in the Ru
    SR OFFSET(1) NUMBITS(1) [],
    /// Operate on second frame When this bit is set, this bit instructs the DMA to proc
    OSF OFFSET(2) NUMBITS(1) [],
    /// Receive threshold control These two bits control the threshold level of the MTL
    RTC OFFSET(3) NUMBITS(2) [],
    /// Forward undersized good frames When set, the Rx FIFO will forward Undersized fra
    FUF OFFSET(6) NUMBITS(1) [],
    /// Forward error frames When this bit is reset, the Rx FIFO drops frames with error
    FEF OFFSET(7) NUMBITS(1) [],
    /// Start/Stop Transmission Command  When this bit is set, transmission is placed in
    ST OFFSET(13) NUMBITS(1) [],
    /// Transmit threshold control These three bits control the threshold level of the M
    TTC OFFSET(14) NUMBITS(3) [],
    /// Flush transmit FIFO This register field can be read by the application (Read), c
    FTF OFFSET(20) NUMBITS(1) [],
    /// Disable flushing of received frames When this bit is set, the RxDMA does not flu
    DFF OFFSET(24) NUMBITS(1) []
],
DMA_INT_EN [
    /// Transmit interrupt enable When this bit is set with Normal Interrupt Summary Ena
    TIE OFFSET(0) NUMBITS(1) [],
    /// Transmit stopped enable When this bit is set with Abnormal Interrupt Summary Ena
    TSE OFFSET(1) NUMBITS(1) [],
    /// Transmit buffer unavailable enable When this bit is set with Normal Interrupt Su
    TUE OFFSET(2) NUMBITS(1) [],
    /// Transmit jabber timeout enable When this bit is set with Abnormal Interrupt Summ
    TJE OFFSET(3) NUMBITS(1) [],
    /// Overflow interrupt enable When this bit is set with Abnormal Interrupt Summary E
    OVE OFFSET(4) NUMBITS(1) [],
    /// Underflow interrupt enable When this bit is set with Abnormal Interrupt Summary
    UNE OFFSET(5) NUMBITS(1) [],
    /// Receive interrupt enable When this bit is set with Normal Interrupt Summary Enab
    RIE OFFSET(6) NUMBITS(1) [],
    /// Receive buffer unavailable enable When this bit is set with Abnormal Interrupt S
    RUE OFFSET(7) NUMBITS(1) [],
    /// Received stopped enable When this bit is set with Abnormal Interrupt Summary Ena
    RSE OFFSET(8) NUMBITS(1) [],
    /// Receive watchdog timeout enable When this bit is set with Abnormal Interrupt Sum
    RWE OFFSET(9) NUMBITS(1) [],
    /// Early transmit interrupt enable When this bit is set with an Abnormal Interrupt
    ETE OFFSET(10) NUMBITS(1) [],
    /// Fatal bus error enable When this bit is set with Abnormal Interrupt Summary Enab
    FBE OFFSET(13) NUMBITS(1) [],
    /// Early receive interrupt enable When this bit is set with Normal Interrupt Summar
    ERE OFFSET(14) NUMBITS(1) [],
    /// Abnormal interrupt summary enable When this bit is set, an Abnormal Interrupt is
    AIE OFFSET(15) NUMBITS(1) [],
    /// Normal interrupt summary enable When this bit is set, a normal interrupt is enab
    NIE OFFSET(16) NUMBITS(1) []
],
DMA_MFRM_BUFOF [
    /// Number of frames missed This register field can be read by the application (Read
    FMC OFFSET(0) NUMBITS(16) [],
    /// Overflow bit for missed frame counter This register field can be read by the app
    OC OFFSET(16) NUMBITS(1) [],
    /// Number of frames missed by the application This register field can be read by th
    FMA OFFSET(17) NUMBITS(11) [],
    /// Overflow bit for FIFO overflow counter This register field can be read by the ap
    OF OFFSET(28) NUMBITS(1) []
]
];
const ETHERNET_BASE: StaticRef<EthernetRegisters> =
    unsafe { StaticRef::new(0x40010000 as *const EthernetRegisters) };
