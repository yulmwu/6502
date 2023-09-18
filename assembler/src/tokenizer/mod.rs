pub mod lexer;
use std::fmt;

use crate::Position;

#[derive(Debug, PartialEq, Clone, Copy)]
#[rustfmt::skip]
pub enum TokenKind<'a> {
    LParen,
    RParen,
    Comma,
    Colon,
    Hash,
    Newline,
    X,
    Y,
    Decimal(u16),
    Hexadecimal8Bit(u8),
    Hexadecimal16Bit(u16),
    Identifier(&'a str),
    Comment,
    EOF,
}

impl fmt::Display for TokenKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenKind::*;

        match self {
            LParen => write!(f, "("),
            RParen => write!(f, ")"),
            Comma => write!(f, ","),
            Colon => write!(f, ":"),
            Hash => write!(f, "#"),
            Newline => write!(f, "\\n"),
            X => write!(f, "X"),
            Y => write!(f, "Y"),
            Decimal(n) => write!(f, "{}", n),
            Hexadecimal8Bit(n) => write!(f, "${:02X}", n),
            Hexadecimal16Bit(n) => write!(f, "${:04X}", n),
            Identifier(s) => write!(f, "{}", s),
            Comment => write!(f, ";"),
            EOF => write!(f, "EOF"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub position: Position,
}

impl Default for Token<'_> {
    fn default() -> Self {
        Self {
            kind: TokenKind::EOF,
            position: Position::default(),
        }
    }
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind<'a>, position: Position) -> Self {
        Self { kind, position }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    struct IteratorLexer<'a>(Lexer<'a>);

    impl<'a> Iterator for IteratorLexer<'a> {
        type Item = Token<'a>;

        fn next(&mut self) -> Option<Self::Item> {
            let token = self.0.next_token().unwrap();
            if token.kind == TokenKind::EOF {
                None
            } else {
                Some(token)
            }
        }
    }

    fn test_tokenizer(input: &str, expected: &[TokenKind]) {
        let mut iterator_lexer = IteratorLexer(Lexer::new(input)).enumerate();
        while let Some((i, token)) = iterator_lexer.next() {
            // println!("{}: {:?}", i, token);
            assert_eq!(token.kind, expected[i]);
        }
        assert_eq!(iterator_lexer.next(), None);
    }

    #[test]
    fn test_tokenizer_decimal() {
        test_tokenizer(
            "1234 5678 9012",
            &[
                TokenKind::Decimal(1234),
                TokenKind::Decimal(5678),
                TokenKind::Decimal(9012),
            ],
        );
    }

    #[test]
    fn test_tokenizer_hexadecimal() {
        test_tokenizer(
            "$1234 0x56",
            &[
                TokenKind::Hexadecimal16Bit(0x1234),
                TokenKind::Hexadecimal8Bit(0x56),
            ],
        );
    }

    #[test]
    fn test_tokenizer_identifier() {
        test_tokenizer(
            "abc def",
            &[TokenKind::Identifier("abc"), TokenKind::Identifier("def")],
        );
    }

    #[test]
    fn test_tokenizer_symbols() {
        test_tokenizer(
            "( ) , test: #\n1",
            &[
                TokenKind::LParen,
                TokenKind::RParen,
                TokenKind::Comma,
                TokenKind::Identifier("test"),
                TokenKind::Colon,
                TokenKind::Hash,
                TokenKind::Newline,
                TokenKind::Decimal(1),
            ],
        );
    }
}
