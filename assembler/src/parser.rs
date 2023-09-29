use crate::{
    lexer::Lexer, AddressingMode, AssemblerError, AssemblerErrorKind, AssemblerResult, Instruction,
    Mnemonics, NumberType, Operand, OperandData, Position, Program, Statement, Token, TokenKind,
};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    current_token: Token<'a>,
    peek_token: Token<'a>,
    position: Position,
    defines: HashMap<String, Operand>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Self {
            lexer,
            ..Default::default()
        };
        parser.next_token().unwrap();
        parser.next_token().unwrap();

        parser
    }

    fn next_token(&mut self) -> AssemblerResult<()> {
        self.current_token = self.peek_token;
        self.peek_token = self.lexer.next_token()?;

        self.position = self.current_token.position;

        Ok(())
    }

    fn expected(&mut self, expected: &TokenKind) -> AssemblerResult<()> {
        Err(AssemblerError::new(
            AssemblerErrorKind::UnexpectedToken {
                expected: expected.to_string(),
                found: self.current_token.kind.to_string(),
            },
            self.position,
        ))
    }

    fn expect_token(&mut self, token: &TokenKind) -> AssemblerResult<()> {
        if self.current_token.kind == *token {
            self.next_token()?;
            Ok(())
        } else {
            self.expected(token)
        }
    }

    pub fn parse(&mut self) -> AssemblerResult<Program> {
        let mut program = Program::default();

        while self.current_token.kind != TokenKind::EOF {
            match self.current_token.kind {
                TokenKind::Identifier(identifier) => {
                    let statement = self.parse_identifier(identifier)?;
                    program.0.push(statement);

                    if self.current_token.kind == TokenKind::Newline {
                        self.next_token()?;
                    }
                }
                TokenKind::Newline => self.next_token()?,
                TokenKind::Define => {
                    self.parse_define()?;

                    if self.current_token.kind == TokenKind::Newline {
                        self.next_token()?;
                    }
                }
                _ => {
                    return Err(AssemblerError::new(
                        AssemblerErrorKind::UnexpectedToken {
                            expected: TokenKind::Identifier("identifier").to_string(),
                            found: self.current_token.kind.to_string(),
                        },
                        self.position,
                    ))
                }
            }
        }

        Ok(program)
    }

    fn parse_identifier(&mut self, identifier: &'a str) -> AssemblerResult<Statement> {
        Ok(if self.peek_token.kind == TokenKind::Colon {
            self.next_token()?;
            self.next_token()?;
            Statement::Label(identifier.to_string())
        } else {
            let instruction = Mnemonics::to_mnemonics(identifier, self.position)?;
            self.next_token()?;
            let operand = self.parse_operand()?;

            Statement::Instruction(Instruction {
                opcode: instruction,
                operand,
                position: self.position,
            })
        })
    }

    /*
    IMM LDA #$00 ($00 is the operand)
    ABS LDA $0000 ($0000 is the operand)
    ABX LDA $0000,X ($0000 is the operand)
    ABY LDA $0000,Y ($0000 is the operand)
    IND JMP ($0000) ($0000 is the operand)
    IDX LDA ($00,X) ($00 is the operand)
    IDY LDA ($00),Y ($00 is the operand)
    ZPX LDA $00,X ($00 is the operand)
    ZPY LDA $00,Y ($00 is the operand)

    IMP ACC --> IMPACC

    ZPG LDA $00 ($00 is the operand) -+
    REL BNE $00 ($00 is the operand) -+--> RELZPG
    */
    fn parse_operand(&mut self) -> AssemblerResult<Operand> {
        Ok(match self.current_token.kind {
            TokenKind::Hash => {
                self.next_token()?;
                let operand_data = self.parse_operand_data()?;

                if !operand_data.is_8() {
                    return Err(AssemblerError::new(
                        AssemblerErrorKind::InvalidOperand(operand_data.to_string()),
                        self.position,
                    ));
                }

                self.next_token()?;

                Operand::new(AddressingMode::IMM, Some(operand_data))
            }
            TokenKind::Hexadecimal8Bit(number) => {
                // $00
                self.next_token()?;
                let operand_data = OperandData::Number(NumberType::Hexadecimal8(number));

                self.parse_8bit_operand_comma(operand_data)?
            }
            TokenKind::Hexadecimal16Bit(number) => {
                // $0000
                self.next_token()?;
                let operand_data = OperandData::Number(NumberType::Hexadecimal16(number));

                self.parse_16bit_operand_comma(operand_data)?
            }
            TokenKind::LParen => {
                // (
                self.next_token()?;
                let operand_data = self.parse_operand_data()?; // ($0000
                self.next_token()?;

                self.parse_operand_lparen(operand_data)?
            }
            TokenKind::Decimal(number) => {
                // 00
                self.next_token()?;
                if number > 255 {
                    let operand_data = OperandData::Number(NumberType::Decimal16(number));
                    self.parse_16bit_operand_comma(operand_data)?
                } else {
                    let operand_data = OperandData::Number(NumberType::Decimal8(number as u8));
                    self.parse_8bit_operand_comma(operand_data)?
                }
            }
            TokenKind::Identifier(identifier) => {
                self.next_token()?;
                if let Some(operand) = self.defines.get(identifier) {
                    match operand.addressing_mode {
                        AddressingMode::ZPX
                        | AddressingMode::ZPY
                        | AddressingMode::ABX
                        | AddressingMode::ABY => operand.clone(),
                        _ => match operand.value.clone().unwrap() {
                            OperandData::Number(number_type) => match number_type {
                                NumberType::Decimal8(_) | NumberType::Hexadecimal8(_) => {
                                    self.parse_8bit_operand_comma(operand.value.clone().unwrap())?
                                }
                                NumberType::Decimal16(_) | NumberType::Hexadecimal16(_) => {
                                    self.parse_16bit_operand_comma(operand.value.clone().unwrap())?
                                }
                            },
                            _ => operand.clone(),
                        },
                    }
                } else {
                    let operand_data = OperandData::Label(identifier.to_string());
                    Operand::new(AddressingMode::RELZPG, Some(operand_data))
                }
            }
            TokenKind::EOF | TokenKind::Newline => Operand::new(AddressingMode::IMPACC, None),
            _ => {
                return Err(AssemblerError::new(
                    AssemblerErrorKind::InvalidOperand(self.current_token.kind.to_string()),
                    self.position,
                ))
            }
        })
    }

    fn parse_operand_data(&mut self) -> AssemblerResult<OperandData> {
        Ok(match self.current_token.kind {
            TokenKind::Decimal(number) => {
                if number > 255 {
                    OperandData::Number(NumberType::Decimal16(number))
                } else {
                    OperandData::Number(NumberType::Decimal8(number as u8))
                }
            }
            TokenKind::Hexadecimal8Bit(number) => {
                OperandData::Number(NumberType::Hexadecimal8(number))
            }
            TokenKind::Hexadecimal16Bit(number) => {
                OperandData::Number(NumberType::Hexadecimal16(number))
            }
            TokenKind::Identifier(identifier) => {
                if let Some(operand) = self.defines.get(identifier) {
                    operand.value.clone().unwrap()
                } else {
                    OperandData::Label(identifier.to_string())
                }
            }
            _ => {
                return Err(AssemblerError::new(
                    AssemblerErrorKind::InvalidOperand(self.current_token.kind.to_string()),
                    self.position,
                ))
            }
        })
    }

    fn parse_8bit_operand_comma(&mut self, operand_data: OperandData) -> AssemblerResult<Operand> {
        Ok(if let TokenKind::Comma = self.current_token.kind {
            self.next_token()?;
            match self.current_token.kind {
                TokenKind::X => {
                    self.next_token()?;
                    // $00,X
                    Operand::new(AddressingMode::ZPX, Some(operand_data))
                }
                TokenKind::Y => {
                    self.next_token()?;
                    // $00,Y
                    Operand::new(AddressingMode::ZPY, Some(operand_data))
                }
                _ => {
                    return Err(AssemblerError::new(
                        AssemblerErrorKind::InvalidOperand(operand_data.to_string()),
                        self.position,
                    ))
                }
            }
        } else {
            Operand::new(AddressingMode::RELZPG, Some(operand_data))
        })
    }

    fn parse_16bit_operand_comma(&mut self, operand_data: OperandData) -> AssemblerResult<Operand> {
        Ok(if let TokenKind::Comma = self.current_token.kind {
            self.next_token()?;
            match self.current_token.kind {
                TokenKind::X => {
                    self.next_token()?;
                    // $0000,X
                    Operand::new(AddressingMode::ABX, Some(operand_data))
                }
                TokenKind::Y => {
                    self.next_token()?;
                    // $0000,Y
                    Operand::new(AddressingMode::ABY, Some(operand_data))
                }
                _ => {
                    return Err(AssemblerError::new(
                        AssemblerErrorKind::InvalidOperand(operand_data.to_string()),
                        self.position,
                    ))
                }
            }
        } else {
            Operand::new(AddressingMode::ABS, Some(operand_data))
        })
    }

    fn parse_operand_lparen(&mut self, operand_data: OperandData) -> AssemblerResult<Operand> {
        Ok(match self.current_token.kind {
            TokenKind::Comma => {
                // ($0000,
                self.next_token()?;

                self.expect_token(&TokenKind::X)?; // ($00,X

                if let TokenKind::RParen = self.current_token.kind {
                    self.next_token()?;
                    Operand::new(AddressingMode::IDX, Some(operand_data))
                    // TODO: later check if the operand is 8 bit
                } else {
                    return Err(AssemblerError::new(
                        AssemblerErrorKind::UnexpectedToken {
                            expected: TokenKind::RParen.to_string(),
                            found: self.current_token.kind.to_string(),
                        },
                        self.position,
                    ));
                }
            }
            TokenKind::RParen => {
                self.next_token()?;

                // ($0000),
                if let TokenKind::Comma = self.current_token.kind {
                    self.next_token()?;
                    if let TokenKind::Y = self.current_token.kind {
                        self.next_token()?;
                        // ($00),Y
                        Operand::new(AddressingMode::IDY, Some(operand_data))
                        // TODO: later check if the operand is 8 bit
                    } else {
                        return Err(AssemblerError::new(
                            AssemblerErrorKind::UnexpectedToken {
                                expected: TokenKind::Y.to_string(),
                                found: self.current_token.kind.to_string(),
                            },
                            self.position,
                        ));
                    }
                } else {
                    Operand::new(AddressingMode::IND, Some(operand_data))
                }
            }
            _ => {
                return Err(AssemblerError::new(
                    AssemblerErrorKind::InvalidOperand(operand_data.to_string()),
                    self.position,
                ))
            }
        })
    }

    fn parse_define(&mut self) -> AssemblerResult<()> {
        self.expect_token(&TokenKind::Define)?;
        let identifier = match self.current_token.kind {
            TokenKind::Identifier(identifier) => identifier,
            _ => {
                return Err(AssemblerError::new(
                    AssemblerErrorKind::UnexpectedToken {
                        expected: TokenKind::Identifier("identifier").to_string(),
                        found: self.current_token.kind.to_string(),
                    },
                    self.position,
                ))
            }
        };
        self.next_token()?;

        let operand = self.parse_operand()?;
        self.defines.insert(identifier.to_string(), operand);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Lexer,
        AddressingMode::{self, *},
        Instruction,
        Mnemonics::{self, *},
        NumberType, Operand, OperandData, Parser, Position, Statement,
    };

    fn test_parse_instruction(input: &str, expected: Instruction) {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let instruction = parser.parse().unwrap().0;

        assert_eq!(instruction[0], Statement::Instruction(expected));
    }

    fn instruction(
        mnemonic: Mnemonics,
        mode: AddressingMode,
        data: Option<NumberType>,
    ) -> Instruction {
        Instruction::new(
            mnemonic,
            Operand::new(
                mode,
                match data {
                    Some(data) => Some(OperandData::Number(data)),
                    None => None,
                },
            ),
            Position::default(),
        )
    }

    #[test]
    fn test_parse_lda_immediate() {
        test_parse_instruction(
            "LDA #$FF",
            instruction(LDA, IMM, Some(NumberType::Hexadecimal8(255))),
        );
    }

    #[test]
    fn test_parse_lda_absolute() {
        test_parse_instruction(
            "LDA $FFFF",
            instruction(LDA, ABS, Some(NumberType::Hexadecimal16(65535))),
        );
    }

    #[test]
    fn test_parse_lda_absolute_x() {
        test_parse_instruction(
            "LDA $FFFF,X",
            instruction(LDA, ABX, Some(NumberType::Hexadecimal16(65535))),
        );
    }

    #[test]
    fn test_parse_lda_absolute_y() {
        test_parse_instruction(
            "LDA $FFFF,Y",
            instruction(LDA, ABY, Some(NumberType::Hexadecimal16(65535))),
        );
    }

    #[test]
    fn test_parse_lda_indirect() {
        test_parse_instruction(
            "JMP ($FFFF)",
            instruction(JMP, IND, Some(NumberType::Hexadecimal16(65535))),
        );
    }

    #[test]
    fn test_parse_lda_indirect_x() {
        test_parse_instruction(
            "LDA ($FF, X)",
            instruction(LDA, IDX, Some(NumberType::Hexadecimal8(255))),
        );
    }

    #[test]
    fn test_parse_lda_indirect_y() {
        test_parse_instruction(
            "LDA ($FF), Y",
            instruction(LDA, IDY, Some(NumberType::Hexadecimal8(255))),
        );
    }

    #[test]
    fn test_parse_lda_zeropage() {
        test_parse_instruction(
            "LDA $FF",
            instruction(LDA, RELZPG, Some(NumberType::Hexadecimal8(255))),
        );
    }

    #[test]
    fn test_parse_ldx_zeropage_x() {
        test_parse_instruction(
            "LDX $FF,Y",
            instruction(LDX, ZPY, Some(NumberType::Hexadecimal8(255))),
        );
    }

    #[test]
    fn test_parse_ldy_zeropage_y() {
        test_parse_instruction(
            "LDY $FF,X",
            instruction(LDY, ZPX, Some(NumberType::Hexadecimal8(255))),
        );
    }

    #[test]
    fn test_parse_clc_implied_accumulator() {
        test_parse_instruction("CLC", instruction(CLC, IMPACC, None));
    }
}
