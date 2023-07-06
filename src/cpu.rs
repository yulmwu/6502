use crate::{
    addressing_mode::AddressingMode,
    memory::{Memory, STACK_BASE},
    registers::Registers,
};

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
                0x00 => {}
                _ => {}
            }
        }
    }

    fn stack_push(&mut self, data: u8) {
        self.memory
            .write(STACK_BASE + self.registers.sp as u16, data);
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

    fn get_address_from_mode(&mut self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => {
                let data = self.registers.pc;
                self.registers.pc += 1;

                data
            }
            AddressingMode::Absolute => {
                let data = self.memory.read_16(self.registers.pc);
                self.registers.pc += 2;

                data
            }
            AddressingMode::AbsoluteX => {
                let base = self.memory.read_16(self.registers.pc);
                self.registers.pc += 2;

                base + self.registers.x as u16
            }
            AddressingMode::AbsoluteY => {
                let base = self.memory.read_16(self.registers.pc);
                self.registers.pc += 2;

                base + self.registers.y as u16
            }
            AddressingMode::IndirectX => {
                let base = self.memory.read(self.registers.pc);
                self.registers.pc += 1;

                let ptr = base.wrapping_add(self.registers.x);
                let data = self.memory.read_16(ptr as u16);
                self.registers.pc += 2;

                data
            }
            AddressingMode::IndirectY => {
                let base = self.memory.read(self.registers.pc);
                self.registers.pc += 1;

                let ptr = base.wrapping_add(self.registers.y);
                let data = self.memory.read_16(ptr as u16);
                self.registers.pc += 2;

                data
            }
            AddressingMode::ZeroPage => {
                let data = self.memory.read(self.registers.pc);
                self.registers.pc += 1;

                data as u16
            }
            AddressingMode::ZeroPageX => {
                let data = self.memory.read(self.registers.pc);
                self.registers.pc += 1;

                data.wrapping_add(self.registers.x) as u16
            }
            AddressingMode::ZeroPageY => {
                let data = self.memory.read(self.registers.pc);
                self.registers.pc += 1;

                data.wrapping_add(self.registers.y) as u16
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut cpu = Cpu::default();
        cpu.stack_push(0x01);
        /*
        Stack Push: [0x01]
        SP: 0xFF (0x00 - 0x01 = 0xFF)

        Stack Pop: [0x01]
        SP: 0x00
        */
        assert_eq!(cpu.registers.sp, 0xFF);
        assert_eq!(cpu.stack_pop(), 0x01);
        assert_eq!(cpu.registers.sp, 0x00);

        cpu.stack_push_16(0x0203);
        /*
        Stack Push: [0x02, 0x03]
        SP: 0xFE (0x00 - 0x02 = 0xFE)

        Stack Pop: [0x02, 0x03]
        SP: 0x00
        */
        assert_eq!(cpu.registers.sp, 0xFE);
        assert_eq!(cpu.stack_pop_16(), 0x0203);
        assert_eq!(cpu.registers.sp, 0x00);
    }

    #[test]
    fn addressing_mode_immidiate() {
        let mut cpu = Cpu::default();
        cpu.reset();
        cpu.memory.write(0x8000, 0x01);

        assert_eq!(cpu.get_address_from_mode(AddressingMode::Immediate), 0x8000);
        assert_eq!(cpu.registers.pc, 0x8001);
    }

    #[test]
    fn addressing_mode_absolute() {
        let mut cpu = Cpu::default();
        cpu.reset();
        cpu.memory.write(0x8000, 0x01);
        cpu.memory.write(0x8001, 0x02);

        assert_eq!(cpu.get_address_from_mode(AddressingMode::Absolute), 0x0201);
        assert_eq!(cpu.registers.pc, 0x8002);
    }

    #[test]
    fn addressing_mode_absolute_x() {
        let mut cpu = Cpu::default();
        cpu.reset();
        cpu.memory.write(0x8000, 0x01);
        cpu.memory.write(0x8001, 0x02);
        cpu.registers.x = 0x03;

        assert_eq!(cpu.get_address_from_mode(AddressingMode::AbsoluteX), 0x0204);
        assert_eq!(cpu.registers.pc, 0x8002);
    }

    #[test]
    fn addressing_mode_absolute_y() {
        let mut cpu = Cpu::default();
        cpu.reset();
        cpu.memory.write(0x8000, 0x01);
        cpu.memory.write(0x8001, 0x02);
        cpu.registers.y = 0x03;

        assert_eq!(cpu.get_address_from_mode(AddressingMode::AbsoluteY), 0x0204);
        assert_eq!(cpu.registers.pc, 0x8002);
    }

    #[test]
    fn addressing_mode_indirect_x() {
        let mut cpu = Cpu::default();
        cpu.reset();
        cpu.memory.write(0x8000, 0x01); // `0x01` + RegX (0x03) = 0x04
        cpu.memory.write(0x8001, 0x02);
        cpu.memory.write(0x0004, 0x03);
        cpu.memory.write(0x0005, 0x04);
        cpu.registers.x = 0x03;

        assert_eq!(cpu.get_address_from_mode(AddressingMode::IndirectX), 0x0403);
        assert_eq!(cpu.registers.pc, 0x8003);
    }

    #[test]
    fn addressing_mode_indirect_y() {
        let mut cpu = Cpu::default();
        cpu.reset();
        cpu.memory.write(0x8000, 0x01); // `0x01` + RegY (0x03) = 0x04
        cpu.memory.write(0x8001, 0x02);
        cpu.memory.write(0x0004, 0x03);
        cpu.memory.write(0x0005, 0x04);
        cpu.registers.y = 0x03;

        assert_eq!(cpu.get_address_from_mode(AddressingMode::IndirectY), 0x0403);
        assert_eq!(cpu.registers.pc, 0x8003);
    }

    #[test]
    fn addressing_mode_zero_page() {
        let mut cpu = Cpu::default();
        cpu.reset();
        cpu.memory.write(0x8000, 0x01);

        assert_eq!(cpu.get_address_from_mode(AddressingMode::ZeroPage), 0x01);
        assert_eq!(cpu.registers.pc, 0x8001);
    }

    #[test]
    fn addressing_mode_zero_page_x() {
        let mut cpu = Cpu::default();
        cpu.reset();
        cpu.memory.write(0x8000, 0x01);
        cpu.registers.x = 0x03;

        assert_eq!(cpu.get_address_from_mode(AddressingMode::ZeroPageX), 0x04);
        assert_eq!(cpu.registers.pc, 0x8001);
    }

    #[test]
    fn addressing_mode_zero_page_y() {
        let mut cpu = Cpu::default();
        cpu.reset();
        cpu.memory.write(0x8000, 0x01);
        cpu.registers.y = 0x03;

        assert_eq!(cpu.get_address_from_mode(AddressingMode::ZeroPageY), 0x04);
        assert_eq!(cpu.registers.pc, 0x8001);
    }
}
