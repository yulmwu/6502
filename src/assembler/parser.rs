use crate::{
    AddressingMode, Instruction, Mnemonics, NumberType, Operand, OperandData, Program, Statement,
    TokenKind,
};
use logos::Lexer;

#[derive(Debug)]
pub struct Parser<'a> {
    pub lexer: Lexer<'a, TokenKind>,
    current_token: Option<TokenKind>,
    peek_token: Option<TokenKind>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a, TokenKind>) -> Self {
        let mut parser = Self {
            lexer,
            current_token: None,
            peek_token: None,
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = self.lexer.next().transpose().unwrap();
    }

    fn expected(&mut self, expected: TokenKind) -> ! {
        panic!("Expected {:?} but got {:?}", expected, self.current_token);
    }

    fn expect_token(&mut self, token: TokenKind) {
        if self.current_token == Some(token.clone()) {
            self.next_token();
        } else {
            self.expected(token);
        }
    }

    pub fn parse(&mut self) -> Program {
        let mut program = Program::default();

        while let Some(token) = self.current_token.clone() {
            match token {
                TokenKind::Identifier(identifier) => {
                    let statement = self.parse_identifier(identifier);
                    program.0.push(statement);

                    if self.current_token == Some(TokenKind::Newline) {
                        self.next_token();
                    }
                }
                TokenKind::Newline => {
                    self.next_token();
                }
                _ => {
                    panic!("Unexpected token {:?}", token);
                }
            }
        }

        program
    }

    fn parse_identifier(&mut self, identifier: String) -> Statement {
        if self.peek_token == Some(TokenKind::Colon) {
            self.next_token();
            self.next_token();
            Statement::Label(identifier)
        } else {
            let instruction = Mnemonics::from(identifier.as_str());
            self.next_token();
            let operand = self.parse_operand();

            Statement::Instruction(Instruction {
                opcode: instruction,
                operand,
            })
        }
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
    fn parse_operand(&mut self) -> Operand {
        match self.current_token.clone() {
            Some(TokenKind::Hash) => {
                self.next_token();
                let operand_data = self.parse_operand_data();

                if !operand_data.is_8() {
                    panic!("Invalid operand");
                }

                self.next_token();

                Operand::new(AddressingMode::IMM, Some(operand_data))
            }
            Some(TokenKind::Hexadecimal8Bit(number)) => {
                // $00
                self.next_token();
                let operand_data = OperandData::Number(NumberType::Hexadecimal8(number));

                self.parse_8bit_operand_comma(operand_data)
            }
            Some(TokenKind::Hexadecimal16Bit(number)) => {
                // $0000
                self.next_token();
                let operand_data = OperandData::Number(NumberType::Hexadecimal16(number));

                self.parse_16bit_operand_comma(operand_data)
            }
            Some(TokenKind::LParen) => {
                // (
                self.next_token();
                let operand_data = self.parse_operand_data(); // ($0000
                self.next_token();

                self.parse_operand_lparen(operand_data)
            }
            Some(TokenKind::Decimal(number)) => {
                // 00
                self.next_token();
                if number > 255 {
                    let operand_data = OperandData::Number(NumberType::Decimal16(number));
                    self.parse_16bit_operand_comma(operand_data)
                } else {
                    let operand_data = OperandData::Number(NumberType::Decimal8(number as u8));
                    self.parse_8bit_operand_comma(operand_data)
                }
            }
            None | Some(TokenKind::Newline) => Operand::new(AddressingMode::IMPACC, None),
            _ => panic!("Invalid operand"),
        }
    }

    fn parse_operand_data(&mut self) -> OperandData {
        match self.current_token.clone() {
            Some(TokenKind::Decimal(number)) => {
                if number > 255 {
                    OperandData::Number(NumberType::Decimal16(number))
                } else {
                    OperandData::Number(NumberType::Decimal8(number as u8))
                }
            }
            Some(TokenKind::Hexadecimal8Bit(number)) => {
                OperandData::Number(NumberType::Hexadecimal8(number))
            }
            Some(TokenKind::Hexadecimal16Bit(number)) => {
                OperandData::Number(NumberType::Hexadecimal16(number))
            }
            Some(TokenKind::Identifier(identifier)) => OperandData::Label(identifier),
            _ => panic!("Invalid operand data"),
        }
    }

    fn parse_8bit_operand_comma(&mut self, operand_data: OperandData) -> Operand {
        if let Some(TokenKind::Comma) = self.current_token.clone() {
            self.next_token();
            match self.current_token.clone() {
                Some(TokenKind::X) => {
                    self.next_token();
                    // $00,X
                    Operand::new(AddressingMode::ZPX, Some(operand_data))
                }
                Some(TokenKind::Y) => {
                    self.next_token();
                    // $00,Y
                    Operand::new(AddressingMode::ZPY, Some(operand_data))
                }
                _ => panic!("Invalid operand"),
            }
        } else {
            Operand::new(AddressingMode::RELZPG, Some(operand_data))
        }
    }

    fn parse_16bit_operand_comma(&mut self, operand_data: OperandData) -> Operand {
        if let Some(TokenKind::Comma) = self.current_token.clone() {
            self.next_token();
            match self.current_token.clone() {
                Some(TokenKind::X) => {
                    self.next_token();
                    // $0000,X
                    Operand::new(AddressingMode::ABX, Some(operand_data))
                }
                Some(TokenKind::Y) => {
                    self.next_token();
                    // $0000,Y
                    Operand::new(AddressingMode::ABY, Some(operand_data))
                }
                _ => panic!("Invalid operand"),
            }
        } else {
            Operand::new(AddressingMode::ABS, Some(operand_data))
        }
    }

    fn parse_operand_lparen(&mut self, operand_data: OperandData) -> Operand {
        match self.current_token.clone() {
            Some(TokenKind::Comma) => {
                // ($0000,
                self.next_token();

                self.expect_token(TokenKind::X); // ($00,X

                if let Some(TokenKind::RParen) = self.current_token.clone() {
                    self.next_token();
                    Operand::new(AddressingMode::IDX, Some(operand_data))
                    // TODO: later check if the operand is 8 bit
                } else {
                    self.expected(TokenKind::RParen);
                }
            }
            Some(TokenKind::RParen) => {
                self.next_token();

                // ($0000),
                if let Some(TokenKind::Comma) = self.current_token.clone() {
                    self.next_token();
                    if let Some(TokenKind::Y) = self.current_token.clone() {
                        self.next_token();
                        // ($00),Y
                        Operand::new(AddressingMode::IDY, Some(operand_data))
                        // TODO: later check if the operand is 8 bit
                    } else {
                        self.expected(TokenKind::Y);
                    }
                } else {
                    Operand::new(AddressingMode::IND, Some(operand_data))
                }
            }
            _ => panic!("Invalid operand"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        AddressingMode::{self, *},
        Instruction,
        Mnemonics::{self, *},
        NumberType, Operand, OperandData, Parser, Statement,
    };
    use logos::Lexer;

    fn test_parse_instruction(input: &str, expected: Instruction) {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let instruction = parser.parse().0;

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
