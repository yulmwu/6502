use emulator::{
    cpu::Cpu,
    memory::{Memory, MemoryBus},
};

fn main() {
    let memory = Memory::default();
    let mut cpu = Cpu::new(memory);
    cpu.reset();
    cpu.registers.a = 0x01;
    cpu.registers.set_flag_carry(true);
    cpu.memory.write(0x8000, 0x69); // ADC, Immediate
    cpu.memory.write(0x8001, 0xFE);
    cpu.memory.write(0x8002, 0x00);

    cpu.execute();

    println!(
        "{:?} {} {}",
        cpu.registers.a,
        cpu.registers.get_flag_carry(),
        cpu.registers.get_flag_overflow()
    );
}
