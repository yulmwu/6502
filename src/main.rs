use emulator::{cpu::Cpu, memory::Memory};

fn main() {
    let memory = Memory::default();
    let mut cpu = Cpu::new(memory);
    cpu.load(&[0x00]);
    cpu.reset();
    cpu.execute();

    println!("{:?}", cpu.memory[0]);
}
