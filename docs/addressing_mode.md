# Addressing Mode

Addressing mode is a way to access memory. There are several addressing modes in MOS 6502.

## Implicit(Implied) Addressing Mode, Accumulator Addressing Mode

The instruction has no operands. Example: `CLC`, `INX` ...

Accumulator Addressing Mode is computed on an accumulator. Example: `LSR A`, `ROR A` ...
in assembly language, the `A` is omitted like `LSR`, `ROR` ...

## Immediate Addressing Mode

The instruction contains operands. Example: `LDA #$01`, `LDX #$02` ...

```
Mnemonic        Instruction (Hex)
LDA #7          A9 01
                   |
                   +--> Register A
```

## Absolute Addressing Mode

The instructions contain 16-bits of memory address. Example: `LDA $1234`, `LDX $5678` ...

```
Mnemonic        Instruction (Hex)
LDA $1234       AD 34 12
                   |
                   +--> M[1234] -> Register A
```

## Indirect Addressing Mode

The instructions contain 16-bits of pointer address. Example: `JMP ($1234)` ...

```
Mnemonic        Instruction (Hex)
JMP ($1234)     6C 34 12
                   |
                   +--> M[1234] -> PC
```

## Zero Page Addressing Mode

The instructions contain 8-bits of memory address. Example: `LDA $12`, `LDX $34` ...

```
Mnemonic        Instruction (Hex)
LDA $12         A5 12
                   |
                   +--> M[0012] -> Register A
```

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
