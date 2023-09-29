use crate::{AssemblerError, AssemblerErrorKind, AssemblerResult, Instruction, Position};
use std::fmt;

macro_rules! enum_mnemonics {
    ($($ident:ident),*) => {
        #[repr(u8)]
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub enum Mnemonics {
            $($ident,)*
        }

        impl Mnemonics {
            pub fn to_mnemonics(s: &str, position: Position) -> AssemblerResult<Self> {
                Ok(match s.to_uppercase().as_str() {
                    $(stringify!($ident) => Mnemonics::$ident,)*
                    _ => return Err(AssemblerError::new(AssemblerErrorKind::InvalidMnemonic(s.to_string()), position)),
                })
            }
        }

        impl std::fmt::Display for Mnemonics {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Mnemonics::$ident => write!(f, "{}", stringify!($ident)),)*
                }
            }
        }
    };
}

enum_mnemonics! {
    ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI,
    BNE, BPL, BRK, BVC, BVS, CLC, CLD, CLI,
    CLV, CMP, CPX, CPY, DEC, DEX, DEY, EOR,
    INC, INX, INY, JMP, JSR, LDA, LDX, LDY,
    LSR, NOP, ORA, PHA, PHP, PLA, PLP, ROL,
    ROR, RTI, RTS, SBC, SEC, SED, SEI, STA,
    STX, STY, TAX, TAY, TSX, TXA, TXS, TYA
}

#[rustfmt::skip]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AddressingMode {
    /* IMP Implicit */  /* ACC Accumulator */  IMM /* Immediate */,
    ABS /* Absolute */, ABX   /* AbsoluteX */, ABY /* AbsoluteY */,
    IND /* Indirect */, IDX   /* IndirectX */, IDY /* IndirectY */,
    /* ZPG ZeroPage */  ZPX   /* ZeroPageX */, ZPY /* ZeroPageY */,
    /* REL Relative */  RELZPG /* Relative or ZeroPage */,
                        IMPACC /* Implicit or Accumulator */
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operand {
    pub addressing_mode: AddressingMode,
    pub value: Option<OperandData>,
}

impl Operand {
    pub fn new(addressing_mode: AddressingMode, value: Option<OperandData>) -> Self {
        Self {
            addressing_mode,
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperandData {
    Number(NumberType),
    Label(String),
}

impl fmt::Display for OperandData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperandData::Number(number) => write!(f, "{number}"),
            OperandData::Label(s) => write!(f, "{s}"),
        }
    }
}

impl OperandData {
    pub fn is_number(&self) -> bool {
        matches!(self, OperandData::Number(_))
    }

    pub fn is_label(&self) -> bool {
        matches!(self, OperandData::Label(_))
    }

    pub fn is_dec_8(&self) -> bool {
        matches!(self, OperandData::Number(NumberType::Decimal8(_)))
    }

    pub fn is_dec_16(&self) -> bool {
        matches!(self, OperandData::Number(NumberType::Decimal16(_)))
    }

    pub fn is_hex_8(&self) -> bool {
        matches!(self, OperandData::Number(NumberType::Hexadecimal8(_)))
    }

    pub fn is_hex_16(&self) -> bool {
        matches!(self, OperandData::Number(NumberType::Hexadecimal16(_)))
    }

    pub fn is_8(&self) -> bool {
        self.is_dec_8() || self.is_hex_8()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumberType {
    Decimal8(u8),
    Decimal16(u16),
    Hexadecimal8(u8),
    Hexadecimal16(u16),
}

impl fmt::Display for NumberType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumberType::Decimal8(n) => write!(f, "{n}"),
            NumberType::Decimal16(n) => write!(f, "{n}"),
            NumberType::Hexadecimal8(n) => write!(f, "${:02X}", n),
            NumberType::Hexadecimal16(n) => write!(f, "${:04X}", n),
        }
    }
}

pub fn instruction_to_byte(instruction: Instruction) -> AssemblerResult<u8> {
    let Instruction {
        opcode,
        operand: Operand {
            addressing_mode, ..
        },
        position,
    } = instruction.clone();

    use AddressingMode::*;
    use Mnemonics::*;

    Ok(match (opcode, addressing_mode) {
        // ADC
        (ADC, IMM) => 0x69,
        (ADC, RELZPG) => 0x65,
        (ADC, ZPX) => 0x75,
        (ADC, ABS) => 0x6D,
        (ADC, ABX) => 0x7D,
        (ADC, ABY) => 0x79,
        (ADC, IDX) => 0x61,
        (ADC, IDY) => 0x71,
        // AND
        (AND, IMM) => 0x29,
        (AND, RELZPG) => 0x25,
        (AND, ZPX) => 0x35,
        (AND, ABS) => 0x2D,
        (AND, ABX) => 0x3D,
        (AND, ABY) => 0x39,
        (AND, IDX) => 0x21,
        (AND, IDY) => 0x31,
        // ASL
        (ASL, IMPACC) => 0x0A,
        (ASL, RELZPG) => 0x06,
        (ASL, ZPX) => 0x16,
        (ASL, ABS) => 0x0E,
        (ASL, ABX) => 0x1E,
        // BCC
        (BCC, RELZPG) => 0x90,
        // BCS
        (BCS, RELZPG) => 0xB0,
        // BEQ
        (BEQ, RELZPG) => 0xF0,
        // BIT
        (BIT, RELZPG) => 0x24,
        (BIT, ABS) => 0x2C,
        // BMI
        (BMI, RELZPG) => 0x30,
        // BNE
        (BNE, RELZPG) => 0xD0,
        // BPL
        (BPL, RELZPG) => 0x10,
        // BRK
        (BRK, IMPACC) => 0x00,
        // BVC
        (BVC, RELZPG) => 0x50,
        // BVS
        (BVS, RELZPG) => 0x70,
        // CLC
        (CLC, IMPACC) => 0x18,
        // CLD
        (CLD, IMPACC) => 0xD8,
        // CLI
        (CLI, IMPACC) => 0x58,
        // CLV
        (CLV, IMPACC) => 0xB8,
        // CMP
        (CMP, IMM) => 0xC9,
        (CMP, RELZPG) => 0xC5,
        (CMP, ZPX) => 0xD5,
        (CMP, ABS) => 0xCD,
        (CMP, ABX) => 0xDD,
        (CMP, ABY) => 0xD9,
        (CMP, IDX) => 0xC1,
        (CMP, IDY) => 0xD1,
        // CPX
        (CPX, IMM) => 0xE0,
        (CPX, RELZPG) => 0xE4,
        (CPX, ABS) => 0xEC,
        // CPY
        (CPY, IMM) => 0xC0,
        (CPY, RELZPG) => 0xC4,
        (CPY, ABS) => 0xCC,
        // DEC
        (DEC, RELZPG) => 0xC6,
        (DEC, ZPX) => 0xD6,
        (DEC, ABS) => 0xCE,
        (DEC, ABX) => 0xDE,
        // DEX
        (DEX, IMPACC) => 0xCA,
        // DEY
        (DEY, IMPACC) => 0x88,
        // EOR
        (EOR, IMM) => 0x49,
        (EOR, RELZPG) => 0x45,
        (EOR, ZPX) => 0x55,
        (EOR, ABS) => 0x4D,
        (EOR, ABX) => 0x5D,
        (EOR, ABY) => 0x59,
        (EOR, IDX) => 0x41,
        (EOR, IDY) => 0x51,
        // INC
        (INC, RELZPG) => 0xE6,
        (INC, ZPX) => 0xF6,
        (INC, ABS) => 0xEE,
        (INC, ABX) => 0xFE,
        // INX
        (INX, IMPACC) => 0xE8,
        // INY
        (INY, IMPACC) => 0xC8,
        // JMP
        (JMP, RELZPG) => 0x4C, // for label
        (JMP, ABS) => 0x4C,
        (JMP, IND) => 0x6C,
        // JSR
        (JSR, RELZPG) => 0x20, // for label
        (JSR, ABS) => 0x20,
        // LDA
        (LDA, IMM) => 0xA9,
        (LDA, RELZPG) => 0xA5,
        (LDA, ZPX) => 0xB5,
        (LDA, ABS) => 0xAD,
        (LDA, ABX) => 0xBD,
        (LDA, ABY) => 0xB9,
        (LDA, IDX) => 0xA1,
        (LDA, IDY) => 0xB1,
        // LDX
        (LDX, IMM) => 0xA2,
        (LDX, RELZPG) => 0xA6,
        (LDX, ZPY) => 0xB6,
        (LDX, ABS) => 0xAE,
        (LDX, ABY) => 0xBE,
        // LDY
        (LDY, IMM) => 0xA0,
        (LDY, RELZPG) => 0xA4,
        (LDY, ZPX) => 0xB4,
        (LDY, ABS) => 0xAC,
        (LDY, ABX) => 0xBC,
        // LSR
        (LSR, IMPACC) => 0x4A,
        (LSR, RELZPG) => 0x46,
        (LSR, ZPX) => 0x56,
        (LSR, ABS) => 0x4E,
        (LSR, ABX) => 0x5E,
        // NOP
        (NOP, IMPACC) => 0xEA,
        // ORA
        (ORA, IMM) => 0x09,
        (ORA, RELZPG) => 0x05,
        (ORA, ZPX) => 0x15,
        (ORA, ABS) => 0x0D,
        (ORA, ABX) => 0x1D,
        (ORA, ABY) => 0x19,
        (ORA, IDX) => 0x01,
        (ORA, IDY) => 0x11,
        // PHA
        (PHA, IMPACC) => 0x48,
        // PHP
        (PHP, IMPACC) => 0x08,
        // PLA
        (PLA, IMPACC) => 0x68,
        // PLP
        (PLP, IMPACC) => 0x28,
        // ROL
        (ROL, IMPACC) => 0x2A,
        (ROL, RELZPG) => 0x26,
        (ROL, ZPX) => 0x36,
        (ROL, ABS) => 0x2E,
        (ROL, ABX) => 0x3E,
        // ROR
        (ROR, IMPACC) => 0x6A,
        (ROR, RELZPG) => 0x66,
        (ROR, ZPX) => 0x76,
        (ROR, ABS) => 0x6E,
        (ROR, ABX) => 0x7E,
        // RTI
        (RTI, IMPACC) => 0x40,
        // RTS
        (RTS, IMPACC) => 0x60,
        // SBC
        (SBC, IMM) => 0xE9,
        (SBC, RELZPG) => 0xE5,
        (SBC, ZPX) => 0xF5,
        (SBC, ABS) => 0xED,
        (SBC, ABX) => 0xFD,
        (SBC, ABY) => 0xF9,
        (SBC, IDX) => 0xE1,
        (SBC, IDY) => 0xF1,
        // SEC
        (SEC, IMPACC) => 0x38,
        // SED
        (SED, IMPACC) => 0xF8,
        // SEI
        (SEI, IMPACC) => 0x78,
        // STA
        (STA, RELZPG) => 0x85,
        (STA, ZPX) => 0x95,
        (STA, ABS) => 0x8D,
        (STA, ABX) => 0x9D,
        (STA, ABY) => 0x99,
        (STA, IDX) => 0x81,
        (STA, IDY) => 0x91,
        // STX
        (STX, RELZPG) => 0x86,
        (STX, ZPY) => 0x96,
        (STX, ABS) => 0x8E,
        // STY
        (STY, RELZPG) => 0x84,
        (STY, ZPX) => 0x94,
        (STY, ABS) => 0x8C,
        // TAX
        (TAX, IMPACC) => 0xAA,
        // TAY
        (TAY, IMPACC) => 0xA8,
        // TSX
        (TSX, IMPACC) => 0xBA,
        // TXA
        (TXA, IMPACC) => 0x8A,
        // TXS
        (TXS, IMPACC) => 0x9A,
        // TYA
        (TYA, IMPACC) => 0x98,
        _ => {
            return Err(AssemblerError::new(
                AssemblerErrorKind::InvalidInstruction(opcode.to_string(), addressing_mode),
                position,
            ))
        }
    })
}

pub fn byte_to_opcode(opcode: u8) -> AssemblerResult<(Mnemonics, AddressingMode)> {
    use AddressingMode::*;
    use Mnemonics::*;

    Ok(match opcode {
        0x69 => (ADC, IMM),
        0x65 => (ADC, RELZPG),
        0x75 => (ADC, ZPX),
        0x6D => (ADC, ABS),
        0x7D => (ADC, ABX),
        0x79 => (ADC, ABY),
        0x61 => (ADC, IDX),
        0x71 => (ADC, IDY),
        0x29 => (AND, IMM),
        0x25 => (AND, RELZPG),
        0x35 => (AND, ZPX),
        0x2D => (AND, ABS),
        0x3D => (AND, ABX),
        0x39 => (AND, ABY),
        0x21 => (AND, IDX),
        0x31 => (AND, IDY),
        0x0A => (ASL, IMPACC),
        0x06 => (ASL, RELZPG),
        0x16 => (ASL, ZPX),
        0x0E => (ASL, ABS),
        0x1E => (ASL, ABX),
        0x90 => (BCC, RELZPG),
        0xB0 => (BCS, RELZPG),
        0xF0 => (BEQ, RELZPG),
        0x24 => (BIT, RELZPG),
        0x2C => (BIT, ABS),
        0x30 => (BMI, RELZPG),
        0xD0 => (BNE, RELZPG),
        0x10 => (BPL, RELZPG),
        0x00 => (BRK, IMPACC),
        0x50 => (BVC, RELZPG),
        0x70 => (BVS, RELZPG),
        0x18 => (CLC, IMPACC),
        0xD8 => (CLD, IMPACC),
        0x58 => (CLI, IMPACC),
        0xB8 => (CLV, IMPACC),
        0xC9 => (CMP, IMM),
        0xC5 => (CMP, RELZPG),
        0xD5 => (CMP, ZPX),
        0xCD => (CMP, ABS),
        0xDD => (CMP, ABX),
        0xD9 => (CMP, ABY),
        0xC1 => (CMP, IDX),
        0xD1 => (CMP, IDY),
        0xE0 => (CPX, IMM),
        0xE4 => (CPX, RELZPG),
        0xEC => (CPX, ABS),
        0xC0 => (CPY, IMM),
        0xC4 => (CPY, RELZPG),
        0xCC => (CPY, ABS),
        0xC6 => (DEC, RELZPG),
        0xD6 => (DEC, ZPX),
        0xCE => (DEC, ABS),
        0xDE => (DEC, ABX),
        0xCA => (DEX, IMPACC),
        0x88 => (DEY, IMPACC),
        0x49 => (EOR, IMM),
        0x45 => (EOR, RELZPG),
        0x55 => (EOR, ZPX),
        0x4D => (EOR, ABS),
        0x5D => (EOR, ABX),
        0x59 => (EOR, ABY),
        0x41 => (EOR, IDX),
        0x51 => (EOR, IDY),
        0xE6 => (INC, RELZPG),
        0xF6 => (INC, ZPX),
        0xEE => (INC, ABS),
        0xFE => (INC, ABX),
        0xE8 => (INX, IMPACC),
        0xC8 => (INY, IMPACC),
        0x4C => (JMP, ABS),
        0x6C => (JMP, IND),
        0x20 => (JSR, ABS),
        0xA9 => (LDA, IMM),
        0xA5 => (LDA, RELZPG),
        0xB5 => (LDA, ZPX),
        0xAD => (LDA, ABS),
        0xBD => (LDA, ABX),
        0xB9 => (LDA, ABY),
        0xA1 => (LDA, IDX),
        0xB1 => (LDA, IDY),
        0xA2 => (LDX, IMM),
        0xA6 => (LDX, RELZPG),
        0xB6 => (LDX, ZPY),
        0xAE => (LDX, ABS),
        0xBE => (LDX, ABY),
        0xA0 => (LDY, IMM),
        0xA4 => (LDY, RELZPG),
        0xB4 => (LDY, ZPX),
        0xAC => (LDY, ABS),
        0xBC => (LDY, ABX),
        0x4A => (LSR, IMPACC),
        0x46 => (LSR, RELZPG),
        0x56 => (LSR, ZPX),
        0x4E => (LSR, ABS),
        0x5E => (LSR, ABX),
        0xEA => (NOP, IMPACC),
        0x09 => (ORA, IMM),
        0x05 => (ORA, RELZPG),
        0x15 => (ORA, ZPX),
        0x0D => (ORA, ABS),
        0x1D => (ORA, ABX),
        0x19 => (ORA, ABY),
        0x01 => (ORA, IDX),
        0x11 => (ORA, IDY),
        0x48 => (PHA, IMPACC),
        0x08 => (PHP, IMPACC),
        0x68 => (PLA, IMPACC),
        0x28 => (PLP, IMPACC),
        0x2A => (ROL, IMPACC),
        0x26 => (ROL, RELZPG),
        0x36 => (ROL, ZPX),
        0x2E => (ROL, ABS),
        0x3E => (ROL, ABX),
        0x6A => (ROR, IMPACC),
        0x66 => (ROR, RELZPG),
        0x76 => (ROR, ZPX),
        0x6E => (ROR, ABS),
        0x7E => (ROR, ABX),
        0x40 => (RTI, IMPACC),
        0x60 => (RTS, IMPACC),
        0xE9 => (SBC, IMM),
        0xE5 => (SBC, RELZPG),
        0xF5 => (SBC, ZPX),
        0xED => (SBC, ABS),
        0xFD => (SBC, ABX),
        0xF9 => (SBC, ABY),
        0xE1 => (SBC, IDX),
        0xF1 => (SBC, IDY),
        0x38 => (SEC, IMPACC),
        0xF8 => (SED, IMPACC),
        0x78 => (SEI, IMPACC),
        0x85 => (STA, RELZPG),
        0x95 => (STA, ZPX),
        0x8D => (STA, ABS),
        0x9D => (STA, ABX),
        0x99 => (STA, ABY),
        0x81 => (STA, IDX),
        0x91 => (STA, IDY),
        0x86 => (STX, RELZPG),
        0x96 => (STX, ZPY),
        0x8E => (STX, ABS),
        0x84 => (STY, RELZPG),
        0x94 => (STY, ZPX),
        0x8C => (STY, ABS),
        0xAA => (TAX, IMPACC),
        0xA8 => (TAY, IMPACC),
        0xBA => (TSX, IMPACC),
        0x8A => (TXA, IMPACC),
        0x9A => (TXS, IMPACC),
        0x98 => (TYA, IMPACC),
        _ => {
            return Err(AssemblerError::new(
                AssemblerErrorKind::InvalidOpcode(opcode),
                Position(0, 0),
            ))
        }
    })
}
