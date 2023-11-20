use crate::{AssemblerError, AssemblerErrorKind, AssemblerResult, Position, Token};

#[derive(Debug, Default, Clone, Copy)]
pub struct Lexer<'a> {
    pub input: &'a str,
    pub position: usize,
    pub read_position: usize,
    pub current_char: char,
    pub current_position: Position,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            current_position: Position(1, 0),
            ..Default::default()
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.current_char = '\0';
        } else {
            self.current_char = self.input.chars().nth(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;

        self.current_position.1 += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.is_whitespace() {
            if self.current_char == '\n' {
                self.current_position.0 += 1;
                self.current_position.1 = 0;
                break;
            }
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> &'a str {
        let position = self.position;
        while self.current_char.is_alphanumeric() || self.current_char == '_' {
            self.read_char();
        }

        &self.input[position..self.position]
    }

    fn read_number(&mut self) -> AssemblerResult<usize> /* Negative number not supported yet */ {
        let position = self.position;
        while self.current_char.is_ascii_digit() {
            self.read_char();
        }

        Ok(match self.input[position..self.position].parse() {
            Ok(number) => number,
            Err(_) => {
                return Err(AssemblerError::new(
                    AssemblerErrorKind::InvalidNumber,
                    self.current_position,
                ))
            }
        })
    }

    fn read_comment(&mut self) {
        self.read_char();

        while self.current_char != '\0' && self.current_char != '\n' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> AssemblerResult<Token<'a>> {
        use super::TokenKind::*;

        self.skip_whitespace();

        let position = self.current_position;

        let token = match self.current_char {
            '\n' => Some(Newline),
            '(' => Some(LParen),
            ')' => Some(RParen),
            ',' => Some(Comma),
            ':' => Some(Colon),
            'x' | 'X' => Some(X),
            'y' | 'Y' => Some(Y),
            '#' => Some(Hash),
            ';' => {
                self.read_comment();
                return self.next_token();
            }
            '\0' => Some(EOF),
            _ => None,
        };

        match token {
            Some(token) => {
                self.read_char();
                Ok(Token::new(token, position))
            }
            None => match self.current_char {
                '$' => {
                    self.read_char();

                    let _position = self.position;
                    while self.current_char.is_ascii_hexdigit() {
                        self.read_char();
                    }

                    let number = &self.input[_position..self.position];
                    match number.len() {
                        1 | 2 => Ok(Token::new(
                            Hexadecimal8Bit(u8::from_str_radix(number, 16).unwrap()),
                            position,
                        )),
                        3 | 4 => Ok(Token::new(
                            Hexadecimal16Bit(u16::from_str_radix(number, 16).unwrap()),
                            position,
                        )),
                        _ => Err(AssemblerError::new(
                            AssemblerErrorKind::IllegalCharacter(self.current_char),
                            self.current_position,
                        )),
                    }
                }
                '0' if self.peek_char() == 'x' => {
                    self.read_char();
                    self.read_char();

                    let _position = self.position;
                    while self.current_char.is_ascii_hexdigit() {
                        self.read_char();
                    }

                    let number = &self.input[_position..self.position];
                    match number.len() {
                        2 => Ok(Token::new(
                            Hexadecimal8Bit(u8::from_str_radix(number, 16).unwrap()),
                            position,
                        )),
                        4 => Ok(Token::new(
                            Hexadecimal16Bit(u16::from_str_radix(number, 16).unwrap()),
                            position,
                        )),
                        _ => Err(AssemblerError::new(
                            AssemblerErrorKind::IllegalCharacter(self.current_char),
                            self.current_position,
                        )),
                    }
                }
                c if c.is_alphabetic() => {
                    let identifier = self.read_identifier();
                    Ok(match identifier {
                        "define" => Token::new(Define, position),
                        identifier => Token::new(Identifier(identifier), position),
                    })
                }
                c if c.is_numeric() => {
                    Ok(Token::new(Decimal(self.read_number()? as u16), position))
                }
                _ => Err(AssemblerError::new(
                    AssemblerErrorKind::IllegalCharacter(self.current_char),
                    self.current_position,
                )),
            },
        }
    }
}
