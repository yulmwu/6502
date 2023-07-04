use emulator::{cpu::Cpu, memory::Memory};

fn main() {
    let mut memory = Memory::default();
    memory.rom(&[0x00, 0x00, 0x00, 0x00]);

    let mut cpu = Cpu::new(memory);

    cpu.reset();
    cpu.execute();

    println!("{:?}", cpu.memory[0]);
}
