use emulator::{
    cpu::Cpu,
    memory::{memory_hexdump, Memory, MemoryBus},
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

    // H e l l o   W o r l d !
    cpu.memory.write(0x8020, 0x48); // H
    cpu.memory.write(0x8021, 0x65); // e
    cpu.memory.write(0x8022, 0x6C); // l
    cpu.memory.write(0x8023, 0x6C); // l
    cpu.memory.write(0x8024, 0x6F); // o
    cpu.memory.write(0x8025, 0x20); // <space>
    cpu.memory.write(0x8026, 0x57); // W
    cpu.memory.write(0x8027, 0x6F); // o
    cpu.memory.write(0x8028, 0x72); // r
    cpu.memory.write(0x8029, 0x6C); // l
    cpu.memory.write(0x802A, 0x64); // d
    cpu.memory.write(0x802B, 0x21); // !

    println!("{cpu}");
    println!("{}", memory_hexdump(&cpu.memory, 0x0000, 0x0020));
    println!("{}", memory_hexdump(&cpu.memory, 0x8000, 0x8030));
}
