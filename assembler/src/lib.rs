/*
TODO

define 파서에서 다 처리하기
=> OperandData::Ident -> OperandData::Label
*/

mod ast;
mod instruction;
mod parser;
mod tokenizer;

pub use ast::*;
pub use instruction::*;
pub use parser::*;
pub use tokenizer::*;

use std::{collections::HashMap, fmt};
use tokenizer::lexer::Lexer;

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
            self.preprocess_statement(statement);
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
        let operand = self.assemble_operand_data(instruction.clone())?;
        let instruction = Instruction {
            operand: Operand {
                addressing_mode: operand.1,
                ..instruction.operand
            },
            ..instruction
        };
        let bytes = [instruction_to_byte(instruction.clone())?]
            .iter()
            .chain(&operand.0)
            .copied()
            .collect::<Vec<_>>();

        self.pointer += bytes.len();

        Ok(bytes)
    }

    fn preprocess_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Instruction(instruction) => {
                self.pointer += 1;

                self.preprocess_operand(instruction);
            }
            Statement::Label(label) => {
                self.labels.insert(label, self.pointer as u16);
            }
        }
    }

    fn preprocess_operand(&mut self, instruction: Instruction) {
        let Instruction {
            opcode,
            operand: Operand { value, .. },
            ..
        } = instruction;

        if let Some(value) = value {
            match value {
                OperandData::Number(number_type) => match number_type {
                    NumberType::Decimal8(_) | NumberType::Hexadecimal8(_) => self.pointer += 1,
                    NumberType::Decimal16(_) | NumberType::Hexadecimal16(_) => self.pointer += 2,
                },
                OperandData::Label(_) => match opcode {
                    Mnemonics::BCC
                    | Mnemonics::BCS
                    | Mnemonics::BEQ
                    | Mnemonics::BMI
                    | Mnemonics::BNE
                    | Mnemonics::BPL
                    | Mnemonics::BVC
                    | Mnemonics::BVS => self.pointer += 1,
                    _ => self.pointer += 2,
                },
            }
        }
    }

    fn assemble_operand_data(
        &mut self,
        instruction: Instruction,
    ) -> AssemblerResult<(Vec<u8>, AddressingMode)> {
        let Instruction {
            opcode,
            operand: Operand {
                value,
                addressing_mode,
            },
            position,
        } = instruction;

        let value = if let Some(value) = value {
            value
        } else {
            return Ok((vec![], AddressingMode::IMPACC));
        };

        let mut bytes = Vec::new();

        match value {
            OperandData::Number(number_type) => match number_type {
                NumberType::Decimal8(value) => bytes.extend(value.to_le_bytes()),
                NumberType::Decimal16(value) => bytes.extend(value.to_le_bytes()),
                NumberType::Hexadecimal8(value) => bytes.extend(value.to_le_bytes()),
                NumberType::Hexadecimal16(value) => bytes.extend(value.to_le_bytes()),
            },
            OperandData::Label(label) => match self.labels.get(&label) {
                Some(address) => match opcode {
                    Mnemonics::BCC
                    | Mnemonics::BCS
                    | Mnemonics::BEQ
                    | Mnemonics::BMI
                    | Mnemonics::BNE
                    | Mnemonics::BPL
                    | Mnemonics::BVC
                    | Mnemonics::BVS => {
                        let relative_address = (*address as i16 - self.pointer as i16 - 2) as u8;
                        bytes.extend(relative_address.to_le_bytes());
                    }
                    _ => {
                        let absolute_address = *address + 0x8000;
                        bytes.extend(absolute_address.to_le_bytes());
                    }
                },
                None => {
                    return Err(AssemblerError::new(
                        AssemblerErrorKind::InvalidLabel(label),
                        position,
                    ))
                }
            },
        }

        Ok((bytes, addressing_mode))
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
