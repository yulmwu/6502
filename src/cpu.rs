#![allow(dead_code)]

use std::fmt;

use crate::{
    addressing_mode::AddressingMode,
    memory::{MemoryBus, STACK_BASE},
    registers::Registers,
};

#[doc=include_str!("../README.md")]
#[derive(Debug, Default)]
pub struct Cpu<T>
where
    T: MemoryBus<Data = u8, Addr = u16>,
{
    pub registers: Registers,
    pub memory: T,
}

impl<T> fmt::Display for Cpu<T>
where
    T: MemoryBus<Data = u8, Addr = u16>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.registers)
    }
}

impl<T> Cpu<T>
where
    T: MemoryBus<Data = u8, Addr = u16>,
{
    pub fn new(memory: T) -> Cpu<T> {
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

    pub fn load(&mut self, program: &[T::Data]) {
        self.memory.rom(program);
    }

    pub fn execute(&mut self) {
        loop {
            let opcode = self.memory.read(self.registers.pc);
            self.registers.pc += 1;

            match opcode {
                // ADC
                0x69 => self.adc(AddressingMode::Immediate),
                0x65 => self.adc(AddressingMode::ZeroPage),
                0x75 => self.adc(AddressingMode::ZeroPageX),
                0x6D => self.adc(AddressingMode::Absolute),
                0x7D => self.adc(AddressingMode::AbsoluteX),
                0x79 => self.adc(AddressingMode::AbsoluteY),
                0x61 => self.adc(AddressingMode::IndirectX),
                0x71 => self.adc(AddressingMode::IndirectY),

                // AND
                0x29 => self.and(AddressingMode::Immediate),
                0x25 => self.and(AddressingMode::ZeroPage),
                0x35 => self.and(AddressingMode::ZeroPageX),
                0x2D => self.and(AddressingMode::Absolute),
                0x3D => self.and(AddressingMode::AbsoluteX),
                0x39 => self.and(AddressingMode::AbsoluteY),
                0x21 => self.and(AddressingMode::IndirectX),
                0x31 => self.and(AddressingMode::IndirectY),

                // BRK
                0x00 => break,
                _ => todo!("opcode {:02X} not implemented", opcode),
            }
        }
    }

    fn stack_push(&mut self, data: T::Data) {
        self.memory
            .write(STACK_BASE + self.registers.sp as T::Addr, data);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
    }

    fn stack_pop(&mut self) -> T::Data {
        self.registers.sp = self.registers.sp.wrapping_add(1);
        self.memory.read(STACK_BASE + self.registers.sp as T::Addr)
    }

    fn stack_push_addr(&mut self, data: T::Addr) {
        let [lsb, msb] = data.to_le_bytes();

        self.stack_push(msb);
        self.stack_push(lsb);
    }

    fn stack_pop_addr(&mut self) -> T::Addr {
        let lsb = self.stack_pop();
        let msb = self.stack_pop();

        T::Addr::from_le_bytes([lsb, msb])
    }

    fn get_address_from_mode(&mut self, mode: AddressingMode) -> T::Addr {
        match mode {
            AddressingMode::Immediate => {
                let data = self.registers.pc;
                self.registers.pc += 1;

                data
            }
            AddressingMode::Absolute => {
                let data = self.memory.read_addr(self.registers.pc);
                self.registers.pc += 2;

                data
            }
            AddressingMode::AbsoluteX => {
                let base = self.memory.read_addr(self.registers.pc);
                self.registers.pc += 2;

                base + self.registers.x as T::Addr
            }
            AddressingMode::AbsoluteY => {
                let base = self.memory.read_addr(self.registers.pc);
                self.registers.pc += 2;

                base + self.registers.y as T::Addr
            }
            AddressingMode::IndirectX => {
                let base = self.memory.read(self.registers.pc);
                self.registers.pc += 1;

                let ptr = base.wrapping_add(self.registers.x);
                let data = self.memory.read_addr(ptr as T::Addr);
                self.registers.pc += 2;

                data
            }
            AddressingMode::IndirectY => {
                let ptr = self.memory.read(self.registers.pc);
                self.registers.pc += 1;

                let data = self.memory.read_addr(ptr as T::Addr);
                self.registers.pc += 2;

                data + self.registers.y as T::Addr
            }
            AddressingMode::ZeroPage => {
                let data = self.memory.read(self.registers.pc);
                self.registers.pc += 1;

                data as T::Addr
            }
            AddressingMode::ZeroPageX => {
                let data = self.memory.read(self.registers.pc);
                self.registers.pc += 1;

                data.wrapping_add(self.registers.x) as T::Addr
            }
            AddressingMode::ZeroPageY => {
                let data = self.memory.read(self.registers.pc);
                self.registers.pc += 1;

                data.wrapping_add(self.registers.y) as T::Addr
            }
        }
    }

    fn add_to_accumulator_with_carry(&mut self, data: T::Data) {
        let sum = if self.registers.get_flag_carry() {
            self.registers.a as u16 + data as u16 + 1
        } else {
            self.registers.a as u16 + data as u16
        };

        // Carry flag
        self.registers.set_flag_carry(sum > 0xFF);

        let sum = sum as u8;

        // Overflow flag
        self.registers
            .set_flag_overflow((self.registers.a ^ sum) & (data ^ sum) & 0x80 != 0);

        self.registers.set_zero_negative_flags(sum);

        self.registers.a = sum;
    }

    /// ## ADC (Add with Carry)
    ///
    /// Add Memory to Accumulator with Carry
    ///
    /// `A + M + C -> A, C`, Flags affected: `N` `V` `Z` `C`
    fn adc(&mut self, mode: AddressingMode) {
        let address = self.get_address_from_mode(mode);
        let data = self.memory.read(address);
        self.add_to_accumulator_with_carry(data);
    }

    /// ## AND (Logical AND)
    ///
    /// AND Memory with Accumulator
    ///
    /// `A AND M -> A`, Flags affected: `N` `Z`
    fn and(&mut self, mode: AddressingMode) {
        let address = self.get_address_from_mode(mode);
        let data = self.memory.read(address);
        self.registers.a &= data;

        self.registers.set_zero_negative_flags(self.registers.a);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::Memory;

    fn setup() -> Cpu<Memory> {
        Cpu::default()
    }

    #[cfg(test)]
    mod stack {
        use super::*;

        #[test]
        fn test_stack() {
            let mut cpu = setup();
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

            cpu.stack_push_addr(0x0203);
            /*
            Stack Push: [0x02, 0x03]
            SP: 0xFE (0x00 - 0x02 = 0xFE)

            Stack Pop: [0x02, 0x03]
            SP: 0x00
            */
            assert_eq!(cpu.registers.sp, 0xFE);
            assert_eq!(cpu.stack_pop_addr(), 0x0203);
            assert_eq!(cpu.registers.sp, 0x00);
        }
    }

    #[cfg(test)]
    mod memory_addressing_mode {
        use super::*;

        #[test]
        fn addressing_mode_immidiate() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x8000, 0x01);

            assert_eq!(cpu.get_address_from_mode(AddressingMode::Immediate), 0x8000);
            assert_eq!(cpu.registers.pc, 0x8001);
        }

        #[test]
        fn addressing_mode_absolute() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x8000, 0x01);
            cpu.memory.write(0x8001, 0x02);

            assert_eq!(cpu.get_address_from_mode(AddressingMode::Absolute), 0x0201);
            assert_eq!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn addressing_mode_absolute_x() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x8000, 0x01);
            cpu.memory.write(0x8001, 0x02);
            cpu.registers.x = 0x03;

            assert_eq!(cpu.get_address_from_mode(AddressingMode::AbsoluteX), 0x0204);
            assert_eq!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn addressing_mode_absolute_y() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x8000, 0x01);
            cpu.memory.write(0x8001, 0x02);
            cpu.registers.y = 0x03;

            assert_eq!(cpu.get_address_from_mode(AddressingMode::AbsoluteY), 0x0204);
            assert_eq!(cpu.registers.pc, 0x8002);
        }

        #[test]
        fn addressing_mode_indirect_x() {
            let mut cpu = setup();
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
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x8000, 0x04);
            cpu.memory.write(0x8001, 0x02);
            cpu.memory.write(0x0004, 0x03);
            cpu.memory.write(0x0005, 0x04);
            cpu.registers.y = 0x02;

            assert_eq!(cpu.get_address_from_mode(AddressingMode::IndirectY), 0x0405);
            assert_eq!(cpu.registers.pc, 0x8003);
        }

        #[test]
        fn addressing_mode_zero_page() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x8000, 0x01);

            assert_eq!(cpu.get_address_from_mode(AddressingMode::ZeroPage), 0x01);
            assert_eq!(cpu.registers.pc, 0x8001);
        }

        #[test]
        fn addressing_mode_zero_page_x() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x8000, 0x01);
            cpu.registers.x = 0x03;

            assert_eq!(cpu.get_address_from_mode(AddressingMode::ZeroPageX), 0x04);
            assert_eq!(cpu.registers.pc, 0x8001);
        }

        #[test]
        fn addressing_mode_zero_page_y() {
            let mut cpu = setup();
            cpu.reset();
            cpu.memory.write(0x8000, 0x01);
            cpu.registers.y = 0x03;

            assert_eq!(cpu.get_address_from_mode(AddressingMode::ZeroPageY), 0x04);
            assert_eq!(cpu.registers.pc, 0x8001);
        }
    }

    #[cfg(test)]
    mod instruction {
        use super::*;
        #[test]
        fn adc() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.a = 0x78;
            cpu.registers.set_flag_carry(true);
            cpu.load(&[
                0x69, 0x07, // ADC #$07
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.a, 0x80);
            assert_eq!(cpu.registers.pc, 0x8003);
            assert_eq!(cpu.registers.get_flag_carry(), false);
            assert_eq!(cpu.registers.get_flag_zero(), false);
            assert_eq!(cpu.registers.get_flag_overflow(), true);
            assert_eq!(cpu.registers.get_flag_negative(), true);
        }

        #[test]
        fn and() {
            let mut cpu = setup();
            cpu.reset();
            cpu.registers.a = 0x78; // 0111 1000
            cpu.load(&[
                0x29, 0x07, // AND #$07 ; 0000 0111
                0x00,
            ]);

            cpu.execute();

            assert_eq!(cpu.registers.a, 0x00);
            assert_eq!(cpu.registers.pc, 0x8003);
            assert_eq!(cpu.registers.get_flag_zero(), true);
            assert_eq!(cpu.registers.get_flag_negative(), false);
        }
    }
}
