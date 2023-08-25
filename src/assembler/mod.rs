mod ast;
mod instruction;
mod parser;
mod tokenizer;

pub use ast::*;
pub use instruction::*;
pub use parser::*;
pub use tokenizer::*;

use logos::Logos;
use std::collections::HashMap;

pub struct Assembler {
    pub source: String,
    pointer: usize,
    labels: HashMap<String, u16>,
}

impl Assembler {
    pub fn new(source: String) -> Self {
        Self {
            source,
            pointer: 0,
            labels: HashMap::new(),
        }
    }

    pub fn assemble(&mut self) -> Vec<u8> {
        let lexer = TokenKind::lexer(&self.source);
        let mut parser = Parser::new(lexer);
        let p = parser.parse();

        let mut bytes = Vec::new();

        for statement in p.0.clone() {
            match statement {
                Statement::Label(label) => self.assemble_label(label),
                Statement::Instruction(instruction) => {
                    self.pointer += 1;

                    let Instruction {
                        opcode: _,
                        operand: Operand { value, .. },
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
                bytes.extend(self.assemble_instruction(instruction))
            }
        }

        bytes
    }

    fn assemble_instruction(&mut self, instruction: Instruction) -> Vec<u8> {
        let Instruction {
            opcode,
            operand: Operand {
                addressing_mode,
                value,
            },
        } = instruction;
        let mut bytes = vec![instruction_to_byte(opcode, addressing_mode)];

        if let Some(value) = value {
            match value {
                OperandData::Number(number_type) => match number_type {
                    NumberType::Decimal8(value) => bytes.extend(value.to_be_bytes()),
                    NumberType::Decimal16(value) => bytes.extend(value.to_be_bytes()),
                    NumberType::Hexadecimal8(value) => bytes.extend(value.to_be_bytes()),
                    NumberType::Hexadecimal16(value) => bytes.extend(value.to_be_bytes()),
                },
                OperandData::Label(label) => {
                    let label_address = self.labels.get(&label).unwrap();
                    let relative_address: u8 =
                        (*label_address as i16 - self.pointer as i16 - 2) as u8;
                    bytes.extend(relative_address.to_be_bytes());
                }
            }
        }

        self.pointer += bytes.len();
        bytes
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

        let src = Assembler::new(s.to_string()).assemble();
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

        let src = Assembler::new(s.to_string()).assemble();
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
