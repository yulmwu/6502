use emulator::{
    cpu::Cpu,
    memory::{memory_hexdump, Memory, MemoryBus},
};

fn main() {
    let memory = Memory::default();
    let mut cpu = Cpu::new(memory);
    cpu.reset();
    cpu.registers.x = 0x01;
    cpu.load(&[
        /* $8000 */ 0x4C, 0x04, 0x80, // JMP $8004
        /* $8003 */ 0xE8, // INX
        /* $8004 */ 0xCA, // DEX
        /* $8005 */ 0x00,
    ]);

    cpu.execute();

    println!("PC: {:04X}, X: {:02X}", cpu.registers.pc, cpu.registers.x);
}
