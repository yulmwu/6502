#[doc = include_str!("../docs/addressing_mode.md")]
#[derive(Clone, Copy)]
pub enum AddressingMode {
    // Implicit,
    // Accumulator,
    Immediate,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    // Relative,
}
