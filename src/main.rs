use emulator::assemble;

fn main() {
    let src = r#"
LDA #$FF
LDA $FFFF
LDA $FFFF,X
LDA $FFFF,Y
JMP ($FFFF)
LDA ($FF, X)
LDA ($FF), Y
LDA $FF
LDX $FF,Y
LDY $FF,X
CLC
"#;
    println!("{:?}", assemble(src));
}
