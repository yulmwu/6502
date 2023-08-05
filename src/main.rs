use emulator::{Parser, TokenKind};
use logos::Logos;

fn main() {
    let lexer = TokenKind::lexer(
        r#"
LDA #$FF
LDA $FFFF
LDA $FFFF,X
LDA $FFFF,Y
LDA ($FFFF)
LDA ($FF, X)
LDA ($FF), Y
LDA $FF
LDA $FF,X
LDA $FF,Y
CLC
"#,
    );
    let mut parser = Parser::new(lexer);

    let p = parser.parse();

    println!("{:#?}", p.0);
}
