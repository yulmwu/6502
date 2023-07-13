# Addressing Mode

Addressing mode is a way to access memory. There are several addressing modes in MOS 6502.

## Implicit(Implied) Addressing Mode, Accumulator Addressing Mode

The instruction has no operands. Example: `CLC`, `INX` ...

Accumulator Addressing Mode is computed on an accumulator. Example: `LSR A`, `ROR A` ...
in assembly language, the `A` is omitted like `LSR`, `ROR` ...

## Immediate Addressing Mode

The instruction contains 8-bit operands. Example: `LDA #$01`, `LDX #$02` ...

```
Mnemonic        Instruction (Hex)
LDA #7          A9 01
                   |
                   +--> Register A
```

## Absolute(Direct) Addressing Mode

The instructions contain 16-bit operands. Example: `LDA $1234`, `LDX $5678` ...

```
Mnemonic        Instruction (Hex)
LDA $1234       AD 34 12
                   |
                   +--> Register A
```

## Indirect Addressing Mode

The instructions contain 16-bit operand and the memory at the address of the operand is the effective address.

only `JMP` instruction can use this addressing mode and in assembly language, the label is used instead of the operand.

Example: `JMP ($1234)` ...

```
Mnemonic        Instruction (Hex)
JMP ($1234)     6C 34 12
                   |
                   +--> M[1234] -> PC
```

## Zero Page Addressing Mode

Zero Page that indicates only range from `0x0000` to `0x00FF` for memory section and fast access Example: `LDA $12`, `LDX $34` ...

```
Mnemonic        Instruction (Hex)
LDA $12         A5 12
                   |
                   +--> M[0012] -> Register A
```

## Relative Addressing Mode

The instruction contains signed 8-bit operand and the operand is added to the program counter(PC) to get the effective address.

Example: `BNE $01` ...

```
Mnemonic        Instruction (Hex)
BNE $12         D0 12
                   |
                   +--> PC + 12 + 1 -> PC
```

last `+ 1` is because the program counter is incremented by 1 after the instruction is fetched.

# Indexed Addressing Mode

Indexed addressing mode is a way to access memory by adding an offset to a base address.

## Absolute Indexed Addressing Mode

Adding the `X` or `Y` register to the address. Example: `LDA $1234, X`, `LDX $5678, Y` ...

```
Mnemonic        Instruction (Hex)
LDA $1234, X    BD 34 12
                   |
                   +--> M[1234 + X] -> Register A
```

## Indirect Indexed Addressing Mode

### Indirect, X (Pre-Indexed Indirect)

Adding the `X` register to the address. (Address is Zero Page) Example: `JMP ($12, X) ; Address = $12 + X` ...

```
Mnemonic        Instruction (Hex)
LDA ($12, X)    A1 12
                   |
                   +--> M[0012 + X] -> Register A
```

### Indirect, Y (Post-Indexed Indirect)

Adding the `Y` register to the address. (Address is Zero Page) Example: `JMP ($12), Y ; Address = M[$12] + Y` ...

```
Mnemonic        Instruction (Hex)
LDA ($12), Y    B1 12
                   |
                   +--> M[M[0012] + Y] -> Register A
```

## Zero Page Indexed Addressing Mode

Adding the `X` or `Y` register to the address. Example: `LDA $12, X`, `LDX $34, Y` ...

```
Mnemonic        Instruction (Hex)
LDA $12, X      B5 12
                   |
                   +--> M[0012 + X] -> Register A
```

---

In this MOS 6502 emulator, `Implicit`, `Accumulator` and `Indirect` are not implemented.
