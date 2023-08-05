mod ast;
mod instruction;
mod parser;
mod tokenizer;

pub use ast::*;
pub use instruction::*;
pub use parser::*;
pub use tokenizer::*;

use logos::Logos;

pub fn assemble(source: &str) -> Vec<u8> {
    let lexer = TokenKind::lexer(source);
    let mut parser = Parser::new(lexer);
    let p = parser.parse();

    let mut bytes = Vec::new();

    for statement in p.0 {
        match statement {
            Statement::Instruction(instruction) => bytes.extend(assemble_instruction(instruction)),
            Statement::Label(_) => todo!(),
        }
    }

    bytes
}

fn assemble_instruction(instruction: Instruction) -> Vec<u8> {
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
            OperandData::Label(_) => todo!(),
        }
    }

    bytes
}
