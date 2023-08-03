use logos::{Lexer, Logos};

fn decimal(text: &mut Lexer<TokenKind>) -> Option<u64> {
    text.slice().parse().ok()
}

fn hexadecimal(text: &mut Lexer<TokenKind>, slice: usize) -> Option<u64> {
    u64::from_str_radix(&text.slice()[slice..], 16).ok()
}

fn identifier(text: &mut Lexer<TokenKind>) -> Option<String> {
    Some(text.slice().to_string())
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\f]+")]
#[rustfmt::skip]
pub enum TokenKind {
    #[token("(")] LParen,
    #[token(")")] RParen,
    #[token(",")] Comma,
    #[token(":")] Colon,
    #[token("#")] Hash,
    #[token("\n")] Newline,
    #[regex(r"[1-9][0-9]*", decimal)] Decimal(u64),
    #[regex(r"0x[0-9a-fA-F]+", |lexer| hexadecimal(lexer, 2))]
    #[regex(r"\$[0-9a-fA-F]+", |lexer| hexadecimal(lexer, 1))] Hexadecimal(u64),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", identifier)] Identifier(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_tokenizer(input: &str, expected: &[TokenKind]) {
        let mut lexer = TokenKind::lexer(input).enumerate();
        while let Some((i, token)) = lexer.next() {
            assert_eq!(token.clone().unwrap(), expected[i]);
        }
        assert_eq!(lexer.next(), None);
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
            "$1234 0x5678",
            &[
                TokenKind::Hexadecimal(0x1234),
                TokenKind::Hexadecimal(0x5678),
            ],
        );
    }

    #[test]
    fn test_tokenizer_identifier() {
        test_tokenizer(
            "abc def",
            &[
                TokenKind::Identifier("abc".to_string()),
                TokenKind::Identifier("def".to_string()),
            ],
        );
    }

    #[test]
    fn test_tokenizer_symbols() {
        test_tokenizer(
            "( ) , : # \n",
            &[
                TokenKind::LParen,
                TokenKind::RParen,
                TokenKind::Comma,
                TokenKind::Colon,
                TokenKind::Hash,
                TokenKind::Newline,
            ],
        );
    }
}
