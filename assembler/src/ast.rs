use crate::{Mnemonics, Operand};

#[derive(Debug, Clone, Copy, Default)]
pub struct Position(pub usize, pub usize);

impl PartialEq for Position {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Program(pub Vec<Statement>);

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Instruction(Instruction),
    Label(String),
    Define(String, Operand),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub opcode: Mnemonics,
    pub operand: Operand,
    pub position: Position,
}

impl Instruction {
    pub fn new(opcode: Mnemonics, operand: Operand, position: Position) -> Self {
        Self {
            opcode,
            operand,
            position,
        }
    }
}
