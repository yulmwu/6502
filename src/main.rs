use emulator::{
    cpu::Cpu,
    memory::{memory_hexdump, Memory},
    Assembler,
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
    let src = Assembler::new(s.to_string()).assemble();
    println!("{:?}", src);
    let mut emulator = Cpu::<Memory>::default();
    emulator.reset();
    emulator.load(&src);
    emulator.execute();
    println!("{}", memory_hexdump(&emulator.memory, 0x0000, 0x0020));
    // println!("{}", memory_hexdump(&emulator.memory, 0x8000, 0x800F));
}
