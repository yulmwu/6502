/// # Addressing Mode
///
/// Addressing mode is a way to access memory. There are several addressing modes in MOS 6502.
///
///
///
/// | Addressing Mode | Description | Example |
/// | --- | --- | --- |
/// | Implicit(Implied) | The instruction has no operands. | `CLC`, `INX` ... |
/// | Accumulator | It is computed on an accumulator. | `LSR A`, `ROR A` ... |
/// | Immediate | The instruction contains operands. | `LDA #$01`, `LDX #$02` ... |
/// | Absolute | The instructions contain 16-bits of memory address. | `LDA $1234`, `LDX $5678` ... |
/// | Indirect | The instructions contain 16-bits of pointer address. | `JMP ($1234)` ... |
/// | Zero Page | The instructions contain 8-bits of memory address. | `LDA $12`, `LDX $34` ... |
///
/// # Indexed Addressing Mode
///
/// Indexed addressing mode is a way to access memory by adding an offset to a base address.
///
/// | Addressing Mode | Description | Example |
/// | --- | --- | --- |
/// | Absolute, X or Y | Adding the `X` or `Y` register to the address. | `LDA $1234, X`, `LDX $5678, Y` ... |
/// | Indirect, X | Adding the `X` register to the address. | `JMP ($12, X) ; Address = $12 + X` ... |
/// | Indirect, Y | Adding the `Y` register to the address. | `JMP ($12), Y ; Address = M[$12] + Y` ... |
/// | Zero Page, X or Y | Adding the `X` or `Y` register to the address. | `LDA $12, X`, `LDX $34, Y` ... |
///
/// In this MOS 6502 emulator, `Implicit`, `Accumulator` and `Indirect` are not implemented.
pub enum AddressingMode {
    // Implicit,
    // Accumulator,
    Immediate,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    // Indirect,
    IndirectX,
    IndirectY,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
}
