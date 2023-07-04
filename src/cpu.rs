use crate::{memory::Memory, registers::Registers};

#[doc=include_str!("../README.md")]
#[derive(Debug, Default)]
pub struct Cpu {
    pub registers: Registers,
    pub memory: Memory,
}

impl Cpu {
    pub fn new(memory: Memory) -> Cpu {
        Cpu {
            registers: Registers::default(),
            memory,
        }
    }

    pub fn reset(&mut self) {
        self.registers.pc = 0xFFFC;
    }

    pub fn execute(&mut self) {
        loop {
            let opcode = self.memory.read(self.registers.pc);
            self.registers.pc += 1;

            if self.execute_instruction(opcode) {
                break;
            }
        }
    }

    fn execute_instruction(&mut self, opcode: u8) -> bool /* Break */ {
        match opcode {
            0x00 => return true,
            _ => {}
        }

        false
    }
}
