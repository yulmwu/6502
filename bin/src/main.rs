use assembler::Assembler;
use emulator::{
    cpu::Cpu,
    memory::{memory_hexdump, Memory},
};

fn main() {
    let s = r#"
LDA #$02
CMP #$01
BNE FOO
LDA #$01
STA $00
BRK

FOO:
    LDA #$02
    STA $01
    BRK
    "#;
    let src = Assembler::new(s.to_string()).assemble().unwrap();
    println!("{:?}", src);

    let mut memory = Memory::new();
    memory.set_debug_callback(Box::new(|msg| println!("Memory Debug      : {msg}")));

    let mut emulator = Cpu::<Memory>::new(memory);
    emulator.set_debug_callback(Box::new(|msg| println!("CPU Debug         : {msg}")));
    emulator
        .registers
        .set_debug_callback(Box::new(|msg| println!("Register Debug    : {msg}")));

    emulator.reset();
    emulator.load(&src);
    emulator.execute();

    println!("{}", memory_hexdump(emulator.memory.mem, 0x0000, 0x0020));
    // println!("{}", memory_hexdump(&emulator.memory, 0x8000, 0x800F));
}
