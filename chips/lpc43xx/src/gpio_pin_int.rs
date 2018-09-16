
use kernel::common::StaticRef;
use kernel::common::registers::{self, ReadOnly, ReadWrite, WriteOnly};
    /// GPIO pin interrupt
#[repr(C)]
struct Gpio_Pin_IntRegisters {
/// Pin Interrupt Mode register
isel: ReadWrite<u32, ISEL::Register>,
/// Pin Interrupt Enable (Rising) register
ienr: ReadWrite<u32, IENR::Register>,
/// Set Pin Interrupt Enable (Rising) register
sienr: WriteOnly<u32, SIENR::Register>,
/// Clear Pin Interrupt Enable (Rising) register
cienr: WriteOnly<u32, CIENR::Register>,
/// Pin Interrupt Enable Falling Edge / Active Level register
ienf: ReadWrite<u32, IENF::Register>,
/// Set Pin Interrupt Enable Falling Edge / Active Level register
sienf: WriteOnly<u32, SIENF::Register>,
/// Clear Pin Interrupt Enable Falling Edge / Active Level address
cienf: WriteOnly<u32, CIENF::Register>,
/// Pin Interrupt Rising Edge register
rise: ReadWrite<u32, RISE::Register>,
/// Pin Interrupt Falling Edge register
fall: ReadWrite<u32, FALL::Register>,
/// Pin Interrupt Status register
ist: ReadWrite<u32, IST::Register>,
}
register_bitfields![u32,
ISEL [
    /// Selects the interrupt mode for each pin interrupt. Bit n configures the pin inte
    PMODE0 OFFSET(0) NUMBITS(1) [],
    /// Selects the interrupt mode for each pin interrupt. Bit n configures the pin inte
    PMODE1 OFFSET(1) NUMBITS(1) [],
    /// Selects the interrupt mode for each pin interrupt. Bit n configures the pin inte
    PMODE2 OFFSET(2) NUMBITS(1) [],
    /// Selects the interrupt mode for each pin interrupt. Bit n configures the pin inte
    PMODE3 OFFSET(3) NUMBITS(1) [],
    /// Selects the interrupt mode for each pin interrupt. Bit n configures the pin inte
    PMODE4 OFFSET(4) NUMBITS(1) [],
    /// Selects the interrupt mode for each pin interrupt. Bit n configures the pin inte
    PMODE5 OFFSET(5) NUMBITS(1) [],
    /// Selects the interrupt mode for each pin interrupt. Bit n configures the pin inte
    PMODE6 OFFSET(6) NUMBITS(1) [],
    /// Selects the interrupt mode for each pin interrupt. Bit n configures the pin inte
    PMODE7 OFFSET(7) NUMBITS(1) []
],
IENR [
    /// Enables the rising edge or level interrupt for each pin interrupt. Bit n configu
    ENRL0 OFFSET(0) NUMBITS(1) [],
    /// Enables the rising edge or level interrupt for each pin interrupt. Bit n configu
    ENRL1 OFFSET(1) NUMBITS(1) [],
    /// Enables the rising edge or level interrupt for each pin interrupt. Bit n configu
    ENRL2 OFFSET(2) NUMBITS(1) [],
    /// Enables the rising edge or level interrupt for each pin interrupt. Bit n configu
    ENRL3 OFFSET(3) NUMBITS(1) [],
    /// Enables the rising edge or level interrupt for each pin interrupt. Bit n configu
    ENRL4 OFFSET(4) NUMBITS(1) [],
    /// Enables the rising edge or level interrupt for each pin interrupt. Bit n configu
    ENRL5 OFFSET(5) NUMBITS(1) [],
    /// Enables the rising edge or level interrupt for each pin interrupt. Bit n configu
    ENRL6 OFFSET(6) NUMBITS(1) [],
    /// Enables the rising edge or level interrupt for each pin interrupt. Bit n configu
    ENRL7 OFFSET(7) NUMBITS(1) []
],
SIENR [
    /// Ones written to this address set bits in the PINTEN_R, thus enabling interrupts.
    SETENRL0 OFFSET(0) NUMBITS(1) [],
    /// Ones written to this address set bits in the PINTEN_R, thus enabling interrupts.
    SETENRL1 OFFSET(1) NUMBITS(1) [],
    /// Ones written to this address set bits in the PINTEN_R, thus enabling interrupts.
    SETENRL2 OFFSET(2) NUMBITS(1) [],
    /// Ones written to this address set bits in the PINTEN_R, thus enabling interrupts.
    SETENRL3 OFFSET(3) NUMBITS(1) [],
    /// Ones written to this address set bits in the PINTEN_R, thus enabling interrupts.
    SETENRL4 OFFSET(4) NUMBITS(1) [],
    /// Ones written to this address set bits in the PINTEN_R, thus enabling interrupts.
    SETENRL5 OFFSET(5) NUMBITS(1) [],
    /// Ones written to this address set bits in the PINTEN_R, thus enabling interrupts.
    SETENRL6 OFFSET(6) NUMBITS(1) [],
    /// Ones written to this address set bits in the PINTEN_R, thus enabling interrupts.
    SETENRL7 OFFSET(7) NUMBITS(1) []
],
CIENR [
    /// Ones written to this address clear bits in the IENR, thus disabling the interrup
    CENRL0 OFFSET(0) NUMBITS(1) [],
    /// Ones written to this address clear bits in the IENR, thus disabling the interrup
    CENRL1 OFFSET(1) NUMBITS(1) [],
    /// Ones written to this address clear bits in the IENR, thus disabling the interrup
    CENRL2 OFFSET(2) NUMBITS(1) [],
    /// Ones written to this address clear bits in the IENR, thus disabling the interrup
    CENRL3 OFFSET(3) NUMBITS(1) [],
    /// Ones written to this address clear bits in the IENR, thus disabling the interrup
    CENRL4 OFFSET(4) NUMBITS(1) [],
    /// Ones written to this address clear bits in the IENR, thus disabling the interrup
    CENRL5 OFFSET(5) NUMBITS(1) [],
    /// Ones written to this address clear bits in the IENR, thus disabling the interrup
    CENRL6 OFFSET(6) NUMBITS(1) [],
    /// Ones written to this address clear bits in the IENR, thus disabling the interrup
    CENRL7 OFFSET(7) NUMBITS(1) []
],
IENF [
    /// Enables the falling edge or configures the active level interrupt for each pin i
    ENAF0 OFFSET(0) NUMBITS(1) [],
    /// Enables the falling edge or configures the active level interrupt for each pin i
    ENAF1 OFFSET(1) NUMBITS(1) [],
    /// Enables the falling edge or configures the active level interrupt for each pin i
    ENAF2 OFFSET(2) NUMBITS(1) [],
    /// Enables the falling edge or configures the active level interrupt for each pin i
    ENAF3 OFFSET(3) NUMBITS(1) [],
    /// Enables the falling edge or configures the active level interrupt for each pin i
    ENAF4 OFFSET(4) NUMBITS(1) [],
    /// Enables the falling edge or configures the active level interrupt for each pin i
    ENAF5 OFFSET(5) NUMBITS(1) [],
    /// Enables the falling edge or configures the active level interrupt for each pin i
    ENAF6 OFFSET(6) NUMBITS(1) [],
    /// Enables the falling edge or configures the active level interrupt for each pin i
    ENAF7 OFFSET(7) NUMBITS(1) []
],
SIENF [
    /// Ones written to this address set bits in the IENF, thus enabling interrupts. Bit
    SETENAF0 OFFSET(0) NUMBITS(1) [],
    /// Ones written to this address set bits in the IENF, thus enabling interrupts. Bit
    SETENAF1 OFFSET(1) NUMBITS(1) [],
    /// Ones written to this address set bits in the IENF, thus enabling interrupts. Bit
    SETENAF2 OFFSET(2) NUMBITS(1) [],
    /// Ones written to this address set bits in the IENF, thus enabling interrupts. Bit
    SETENAF3 OFFSET(3) NUMBITS(1) [],
    /// Ones written to this address set bits in the IENF, thus enabling interrupts. Bit
    SETENAF4 OFFSET(4) NUMBITS(1) [],
    /// Ones written to this address set bits in the IENF, thus enabling interrupts. Bit
    SETENAF5 OFFSET(5) NUMBITS(1) [],
    /// Ones written to this address set bits in the IENF, thus enabling interrupts. Bit
    SETENAF6 OFFSET(6) NUMBITS(1) [],
    /// Ones written to this address set bits in the IENF, thus enabling interrupts. Bit
    SETENAF7 OFFSET(7) NUMBITS(1) []
],
CIENF [
    /// Ones written to this address clears bits in the IENF, thus disabling interrupts.
    CENAF0 OFFSET(0) NUMBITS(1) [],
    /// Ones written to this address clears bits in the IENF, thus disabling interrupts.
    CENAF1 OFFSET(1) NUMBITS(1) [],
    /// Ones written to this address clears bits in the IENF, thus disabling interrupts.
    CENAF2 OFFSET(2) NUMBITS(1) [],
    /// Ones written to this address clears bits in the IENF, thus disabling interrupts.
    CENAF3 OFFSET(3) NUMBITS(1) [],
    /// Ones written to this address clears bits in the IENF, thus disabling interrupts.
    CENAF4 OFFSET(4) NUMBITS(1) [],
    /// Ones written to this address clears bits in the IENF, thus disabling interrupts.
    CENAF5 OFFSET(5) NUMBITS(1) [],
    /// Ones written to this address clears bits in the IENF, thus disabling interrupts.
    CENAF6 OFFSET(6) NUMBITS(1) [],
    /// Ones written to this address clears bits in the IENF, thus disabling interrupts.
    CENAF7 OFFSET(7) NUMBITS(1) []
],
RISE [
    /// Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSEL
    RDET0 OFFSET(0) NUMBITS(1) [],
    /// Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSEL
    RDET1 OFFSET(1) NUMBITS(1) [],
    /// Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSEL
    RDET2 OFFSET(2) NUMBITS(1) [],
    /// Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSEL
    RDET3 OFFSET(3) NUMBITS(1) [],
    /// Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSEL
    RDET4 OFFSET(4) NUMBITS(1) [],
    /// Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSEL
    RDET5 OFFSET(5) NUMBITS(1) [],
    /// Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSEL
    RDET6 OFFSET(6) NUMBITS(1) [],
    /// Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSEL
    RDET7 OFFSET(7) NUMBITS(1) []
],
FALL [
    /// Falling edge detect. Bit n detects the falling edge of the pin selected in PINTS
    FDET0 OFFSET(0) NUMBITS(1) [],
    /// Falling edge detect. Bit n detects the falling edge of the pin selected in PINTS
    FDET1 OFFSET(1) NUMBITS(1) [],
    /// Falling edge detect. Bit n detects the falling edge of the pin selected in PINTS
    FDET2 OFFSET(2) NUMBITS(1) [],
    /// Falling edge detect. Bit n detects the falling edge of the pin selected in PINTS
    FDET3 OFFSET(3) NUMBITS(1) [],
    /// Falling edge detect. Bit n detects the falling edge of the pin selected in PINTS
    FDET4 OFFSET(4) NUMBITS(1) [],
    /// Falling edge detect. Bit n detects the falling edge of the pin selected in PINTS
    FDET5 OFFSET(5) NUMBITS(1) [],
    /// Falling edge detect. Bit n detects the falling edge of the pin selected in PINTS
    FDET6 OFFSET(6) NUMBITS(1) [],
    /// Falling edge detect. Bit n detects the falling edge of the pin selected in PINTS
    FDET7 OFFSET(7) NUMBITS(1) []
],
IST [
    /// Pin interrupt status. Bit n returns the status, clears the edge interrupt, or in
    PSTAT0 OFFSET(0) NUMBITS(1) [],
    /// Pin interrupt status. Bit n returns the status, clears the edge interrupt, or in
    PSTAT1 OFFSET(1) NUMBITS(1) [],
    /// Pin interrupt status. Bit n returns the status, clears the edge interrupt, or in
    PSTAT2 OFFSET(2) NUMBITS(1) [],
    /// Pin interrupt status. Bit n returns the status, clears the edge interrupt, or in
    PSTAT3 OFFSET(3) NUMBITS(1) [],
    /// Pin interrupt status. Bit n returns the status, clears the edge interrupt, or in
    PSTAT4 OFFSET(4) NUMBITS(1) [],
    /// Pin interrupt status. Bit n returns the status, clears the edge interrupt, or in
    PSTAT5 OFFSET(5) NUMBITS(1) [],
    /// Pin interrupt status. Bit n returns the status, clears the edge interrupt, or in
    PSTAT6 OFFSET(6) NUMBITS(1) [],
    /// Pin interrupt status. Bit n returns the status, clears the edge interrupt, or in
    PSTAT7 OFFSET(7) NUMBITS(1) []
]
];
const GPIO_PIN_INT_BASE: StaticRef<Gpio_Pin_IntRegisters> =
    unsafe { StaticRef::new(0x40087000 as *const Gpio_Pin_IntRegisters) };
