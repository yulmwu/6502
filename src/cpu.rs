use crate::{memory::{Memory, STACK_BASE}, registers::Registers};

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
        self.registers = Registers {
            pc: 0x8000,
            ..Registers::default()
        }
    }

    pub fn load(&mut self, program: &[u8]) {
        self.memory.rom(program);
    }

    pub fn execute(&mut self) {
        loop {
            let opcode = self.memory.read(self.registers.pc);
            self.registers.pc += 1;

            match opcode {
                0x00 => {},
                _ => {}
            }
        }
    }

    fn stack_push(&mut self, data: u8) {
        self.memory.write(STACK_BASE + self.registers.sp as u16, data);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
    }

    fn stack_pop(&mut self) -> u8 {
        self.registers.sp = self.registers.sp.wrapping_add(1);
        self.memory.read(STACK_BASE + self.registers.sp as u16)
    }

    fn stack_push_16(&mut self, data: u16) {
        let [lsb, msb] = data.to_le_bytes();

        self.stack_push(msb);
        self.stack_push(lsb);
    }

    fn stack_pop_16(&mut self) -> u16 {
        let lsb = self.stack_pop();
        let msb = self.stack_pop();

        u16::from_le_bytes([lsb, msb])
    }
}
