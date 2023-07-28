macro_rules! enum_mnemonics {
    ($($ident:ident),*) => {
        #[repr(u8)]
        pub enum Mnemonics {
            $($ident,)*
        }

        impl From<&str> for Mnemonics {
            fn from(s: &str) -> Self {
                match s {
                    $(stringify!($ident) => Mnemonics::$ident,)*
                    _ => panic!("Invalid mnemonic"),
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
pub enum AddressingMode {
    IMP /* Implicit */, ACC /* Accumulator */, IMM /* Immediate */,
    ABS /* Absolute */, ABX   /* AbsoluteX */, ABY /* AbsoluteY */,
    IND /* Indirect */, IDX   /* IndirectX */, IDY /* IndirectY */,
    ZPG /* ZeroPage */, ZPX   /* ZeroPageX */, ZPY /* ZeroPageY */,
    REL /* Relative */,
}

pub struct Operand {
    pub addressing_mode: AddressingMode,
    pub value: OperandData,
}

pub enum OperandData {
    Number(NumberType),
    Label(String),
}

pub enum NumberType {
    Decimal(u8),
    Hexadecimal(u8),
    Binary(u8),
}

pub fn instruction_to_byte(mnemonic: Mnemonics, addressing_mode: AddressingMode) -> u8 {
    use AddressingMode::*;
    use Mnemonics::*;

    match (mnemonic, addressing_mode) {
        // ADC
        (ADC, IMM) => 0x69,
        (ADC, ZPG) => 0x65,
        (ADC, ZPX) => 0x75,
        (ADC, ABS) => 0x6D,
        (ADC, ABX) => 0x7D,
        (ADC, ABY) => 0x79,
        (ADC, IDX) => 0x61,
        (ADC, IDY) => 0x71,
        // AND
        (AND, IMM) => 0x29,
        (AND, ZPG) => 0x25,
        (AND, ZPX) => 0x35,
        (AND, ABS) => 0x2D,
        (AND, ABX) => 0x3D,
        (AND, ABY) => 0x39,
        (AND, IDX) => 0x21,
        (AND, IDY) => 0x31,
        // ASL
        (ASL, ACC) => 0x0A,
        (ASL, ZPG) => 0x06,
        (ASL, ZPX) => 0x16,
        (ASL, ABS) => 0x0E,
        (ASL, ABX) => 0x1E,
        // BCC
        (BCC, REL) => 0x90,
        // BCS
        (BCS, REL) => 0xB0,
        // BEQ
        (BEQ, REL) => 0xF0,
        // BIT
        (BIT, ZPG) => 0x24,
        (BIT, ABS) => 0x2C,
        // BMI
        (BMI, REL) => 0x30,
        // BNE
        (BNE, REL) => 0xD0,
        // BPL
        (BPL, REL) => 0x10,
        // BRK
        (BRK, IMP) => 0x00,
        // BVC
        (BVC, REL) => 0x50,
        // BVS
        (BVS, REL) => 0x70,
        // CLC
        (CLC, IMP) => 0x18,
        // CLD
        (CLD, IMP) => 0xD8,
        // CLI
        (CLI, IMP) => 0x58,
        // CLV
        (CLV, IMP) => 0xB8,
        // CMP
        (CMP, IMM) => 0xC9,
        (CMP, ZPG) => 0xC5,
        (CMP, ZPX) => 0xD5,
        (CMP, ABS) => 0xCD,
        (CMP, ABX) => 0xDD,
        (CMP, ABY) => 0xD9,
        (CMP, IDX) => 0xC1,
        (CMP, IDY) => 0xD1,
        // CPX
        (CPX, IMM) => 0xE0,
        (CPX, ZPG) => 0xE4,
        (CPX, ABS) => 0xEC,
        // CPY
        (CPY, IMM) => 0xC0,
        (CPY, ZPG) => 0xC4,
        (CPY, ABS) => 0xCC,
        // DEC
        (DEC, ZPG) => 0xC6,
        (DEC, ZPX) => 0xD6,
        (DEC, ABS) => 0xCE,
        (DEC, ABX) => 0xDE,
        // DEX
        (DEX, IMP) => 0xCA,
        // DEY
        (DEY, IMP) => 0x88,
        // EOR
        (EOR, IMM) => 0x49,
        (EOR, ZPG) => 0x45,
        (EOR, ZPX) => 0x55,
        (EOR, ABS) => 0x4D,
        (EOR, ABX) => 0x5D,
        (EOR, ABY) => 0x59,
        (EOR, IDX) => 0x41,
        (EOR, IDY) => 0x51,
        // INC
        (INC, ZPG) => 0xE6,
        (INC, ZPX) => 0xF6,
        (INC, ABS) => 0xEE,
        (INC, ABX) => 0xFE,
        // INX
        (INX, IMP) => 0xE8,
        // INY
        (INY, IMP) => 0xC8,
        // JMP
        (JMP, ABS) => 0x4C,
        (JMP, IND) => 0x6C,
        // JSR
        (JSR, ABS) => 0x20,
        // LDA
        (LDA, IMM) => 0xA9,
        (LDA, ZPG) => 0xA5,
        (LDA, ZPX) => 0xB5,
        (LDA, ABS) => 0xAD,
        (LDA, ABX) => 0xBD,
        (LDA, ABY) => 0xB9,
        (LDA, IDX) => 0xA1,
        (LDA, IDY) => 0xB1,
        // LDX
        (LDX, IMM) => 0xA2,
        (LDX, ZPG) => 0xA6,
        (LDX, ZPY) => 0xB6,
        (LDX, ABS) => 0xAE,
        (LDX, ABY) => 0xBE,
        // LDY
        (LDY, IMM) => 0xA0,
        (LDY, ZPG) => 0xA4,
        (LDY, ZPX) => 0xB4,
        (LDY, ABS) => 0xAC,
        (LDY, ABX) => 0xBC,
        // LSR
        (LSR, ACC) => 0x4A,
        (LSR, ZPG) => 0x46,
        (LSR, ZPX) => 0x56,
        (LSR, ABS) => 0x4E,
        (LSR, ABX) => 0x5E,
        // NOP
        (NOP, IMP) => 0xEA,
        // ORA
        (ORA, IMM) => 0x09,
        (ORA, ZPG) => 0x05,
        (ORA, ZPX) => 0x15,
        (ORA, ABS) => 0x0D,
        (ORA, ABX) => 0x1D,
        (ORA, ABY) => 0x19,
        (ORA, IDX) => 0x01,
        (ORA, IDY) => 0x11,
        // PHA
        (PHA, IMP) => 0x48,
        // PHP
        (PHP, IMP) => 0x08,
        // PLA
        (PLA, IMP) => 0x68,
        // PLP
        (PLP, IMP) => 0x28,
        // ROL
        (ROL, ACC) => 0x2A,
        (ROL, ZPG) => 0x26,
        (ROL, ZPX) => 0x36,
        (ROL, ABS) => 0x2E,
        (ROL, ABX) => 0x3E,
        // ROR
        (ROR, ACC) => 0x6A,
        (ROR, ZPG) => 0x66,
        (ROR, ZPX) => 0x76,
        (ROR, ABS) => 0x6E,
        (ROR, ABX) => 0x7E,
        // RTI
        (RTI, IMP) => 0x40,
        // RTS
        (RTS, IMP) => 0x60,
        // SBC
        (SBC, IMM) => 0xE9,
        (SBC, ZPG) => 0xE5,
        (SBC, ZPX) => 0xF5,
        (SBC, ABS) => 0xED,
        (SBC, ABX) => 0xFD,
        (SBC, ABY) => 0xF9,
        (SBC, IDX) => 0xE1,
        (SBC, IDY) => 0xF1,
        // SEC
        (SEC, IMP) => 0x38,
        // SED
        (SED, IMP) => 0xF8,
        // SEI
        (SEI, IMP) => 0x78,
        // STA
        (STA, ZPG) => 0x85,
        (STA, ZPX) => 0x95,
        (STA, ABS) => 0x8D,
        (STA, ABX) => 0x9D,
        (STA, ABY) => 0x99,
        (STA, IDX) => 0x81,
        (STA, IDY) => 0x91,
        // STX
        (STX, ZPG) => 0x86,
        (STX, ZPY) => 0x96,
        (STX, ABS) => 0x8E,
        // STY
        (STY, ZPG) => 0x84,
        (STY, ZPX) => 0x94,
        (STY, ABS) => 0x8C,
        // TAX
        (TAX, IMP) => 0xAA,
        // TAY
        (TAY, IMP) => 0xA8,
        // TSX
        (TSX, IMP) => 0xBA,
        // TXA
        (TXA, IMP) => 0x8A,
        // TXS
        (TXS, IMP) => 0x9A,
        // TYA
        (TYA, IMP) => 0x98,
        _ => panic!("Invalid instruction and addressing mode combination"),
    }
}
