mod ast;
mod instruction;
mod parser;
mod tokenizer;

pub use ast::*;
pub use instruction::*;
pub use parser::*;
use std::{collections::HashMap, fmt};
use tokenizer::lexer::Lexer;
pub use tokenizer::*;

#[derive(Debug)]
pub enum AssemblerErrorKind {
    IllegalCharacter(char),
    InvalidNumber,
    UnexpectedToken { expected: String, found: String },
    UnexpectedToken2,
    InvalidOperand(String),
    InvalidLabel(String),
    InvalidInstruction(String, AddressingMode),
    InvalidMnemonic(String),
}

impl fmt::Display for AssemblerErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblerErrorKind::IllegalCharacter(c) => write!(f, "Illegal character: {}", c),
            AssemblerErrorKind::InvalidNumber => write!(f, "Invalid number"),
            AssemblerErrorKind::UnexpectedToken { expected, found } => write!(f, "Unexpected token: expected {expected:?}, found {found:?}"),
            AssemblerErrorKind::UnexpectedToken2 => write!(f, "Unexpected token"),
            AssemblerErrorKind::InvalidOperand(operand) => write!(f, "Invalid operand: {operand}"),
            AssemblerErrorKind::InvalidLabel(label) => write!(f, "Invalid label: {label}",),
            AssemblerErrorKind::InvalidInstruction(mnemonic, addressing_mode) => write!(f, "Invalid instruction: mnemonic {mnemonic:?} does not support {addressing_mode:?} addressing mode"),
            AssemblerErrorKind::InvalidMnemonic(mnemonic) => write!(f, "Invalid mnemonic: {mnemonic:?}")
        }
    }
}

#[derive(Debug)]
pub struct AssemblerError {
    pub kind: AssemblerErrorKind,
    pub position: Position,
}

impl AssemblerError {
    pub fn new(kind: AssemblerErrorKind, position: Position) -> Self {
        Self { kind, position }
    }
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)?;
        let Position(line, column) = self.position;
        write!(f, " at line {}, column {}", line, column)?;
        Ok(())
    }
}

pub type AssemblerResult<T> = Result<T, AssemblerError>;

pub struct Assembler<'a> {
    pub source: &'a str,
    pointer: usize,
    labels: HashMap<String, u16>,
}

impl<'a> Assembler<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            pointer: 0,
            labels: HashMap::new(),
        }
    }

    pub fn assemble(&mut self) -> AssemblerResult<Vec<u8>> {
        let lexer = Lexer::new(self.source);
        let mut parser = Parser::new(lexer);
        let p = parser.parse()?;

        let mut bytes = Vec::new();

        for statement in p.0.clone() {
            match statement {
                Statement::Label(label) => self.assemble_label(label),
                Statement::Instruction(instruction) => {
                    self.pointer += 1;

                    let Instruction {
                        opcode: _,
                        operand: Operand { value, .. },
                        position: _,
                    } = instruction;

                    if let Some(value) = value {
                        match value {
                            OperandData::Number(number_type) => match number_type {
                                NumberType::Decimal8(_) | NumberType::Hexadecimal8(_) => {
                                    self.pointer += 1
                                }
                                NumberType::Decimal16(_) | NumberType::Hexadecimal16(_) => {
                                    self.pointer += 2
                                }
                            },
                            OperandData::Label(_) => self.pointer += 1,
                        }
                    }
                }
            }
        }

        self.pointer = 0;

        for statement in p.0 {
            if let Statement::Instruction(instruction) = statement {
                bytes.extend(self.assemble_instruction(instruction)?)
            }
        }

        Ok(bytes)
    }

    fn assemble_instruction(&mut self, instruction: Instruction) -> AssemblerResult<Vec<u8>> {
        let Instruction {
            opcode,
            operand: Operand {
                addressing_mode,
                value,
            },
            position,
        } = instruction;
        let mut bytes = vec![instruction_to_byte(opcode, addressing_mode, position)?];

        if let Some(value) = value {
            match value {
                OperandData::Number(number_type) => match number_type {
                    NumberType::Decimal8(value) => bytes.extend(value.to_le_bytes()),
                    NumberType::Decimal16(value) => bytes.extend(value.to_le_bytes()),
                    NumberType::Hexadecimal8(value) => bytes.extend(value.to_le_bytes()),
                    NumberType::Hexadecimal16(value) => bytes.extend(value.to_le_bytes()),
                },
                OperandData::Label(label) => {
                    let label_address = self.labels.get(&label).ok_or(AssemblerError::new(
                        AssemblerErrorKind::InvalidLabel(label),
                        position,
                    ))?;

                    // Absolute addressing
                    if opcode == Mnemonics::JMP {
                        let absolute_address: u16 = *label_address + 0x8000;
                        bytes.extend(absolute_address.to_le_bytes());
                    } else {
                        let relative_address: u8 =
                            (*label_address as i16 - self.pointer as i16 - 2) as u8;
                        bytes.extend(relative_address.to_le_bytes());
                    }
                }
            }
        }

        self.pointer += bytes.len();
        Ok(bytes)
    }

    fn assemble_label(&mut self, label: String) {
        self.labels.insert(label, self.pointer as u16);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assemble_instruction() {
        let s = r#"
LDX #$01
STX $0000
"#;

        let src = Assembler::new(s).assemble().unwrap();
        assert_eq!(src, vec![0xA2, 0x01, 0x8E, 0x00, 0x00]);
    }

    #[test]
    fn test_assemble_label() {
        let s = r#"
LDA #$02
CMP #$01
BNE FOO
LDA #$01
STA $00
BRK

FOO:
    LDA #$01
    STA $01
    BRK
        "#;

        let src = Assembler::new(s).assemble().unwrap();
        assert_eq!(
            src,
            vec![
                0xA9, 0x02, // LDA #$02
                0xC9, 0x01, // CMP #$01
                0xD0, 0x05, // BNE FOO
                0xA9, 0x01, // LDA #$01
                0x85, 0x00, // STA $00
                0x00, // BRK
                0xA9, 0x01, // LDA #$01
                0x85, 0x01, // STA $01
                0x00, // BRK
            ]
        );
    }
}
