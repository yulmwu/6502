use crate::{Mnemonics, Operand};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Program(pub Vec<Statement>);

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Instruction(Instruction),
    Label(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub opcode: Mnemonics,
    pub operand: Operand,
}

impl Instruction {
    pub fn new(opcode: Mnemonics, operand: Operand) -> Self {
        Self { opcode, operand }
    }
}
