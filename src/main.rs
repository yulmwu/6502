use crate::{memory::Memory, cpu::Cpu};

mod cpu;
mod memory;
mod registers;

fn main() {
    let memory = Memory::default();
    let mut cpu = Cpu::new(memory);

    cpu.execute();

    println!("Hello, world!");
}
