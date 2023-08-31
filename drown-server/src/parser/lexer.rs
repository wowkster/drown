use thiserror::Error;

use super::token::{Span, Token, TokenKind};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Encountered Non-ASCII Character: `{0}` ({})", .0.escape_unicode())]
    NonAsciiCharacter(char),
    #[error("Encountered Unexpected ASCII Control Character: `{0}` ({})", .0.escape_debug())]
    AsciiControlCharacter(char),
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.input.chars().nth(self.position);
        self.position += 1;
        c
    }

    fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if !c.is_whitespace() {
                break;
            }
            self.next_char();
        }
    }

    fn parse_single_char_token(&mut self, kind: TokenKind) -> Token {
        let token = Token::new(
            kind,
            &self.input[self.position..self.position + 1],
            Span::single(self.position),
        );

        self.position += 1;

        token
    }

    fn parse_keyword_or_identifier(&mut self) -> Token {
        let starting_position = self.position;

        while let Some(c) = self.peek_char() {
            if !c.is_ascii_alphanumeric() && c != '_' {
                break;
            }
            self.next_char().unwrap();
        }

        let literal = &self.input[starting_position..self.position];

        Token::new(
            TokenKind::from_keyword_or_identifier(literal),
            literal,
            Span::new(starting_position, self.position),
        )
    }

    fn parse_number(&mut self) -> Token {
        let starting_position = self.position;
        let mut token_kind = TokenKind::IntegerLiteral;

        // Integer or first portion of float
        while let Some(c) = self.peek_char() {
            if !c.is_ascii_digit() {
                break;
            }

            self.next_char().unwrap();
        }

        // Parse Float Literal if applicable
        if let Some('.') = self.peek_char() {
            self.next_char().unwrap();
            token_kind = TokenKind::FloatLiteral;

            while let Some(c) = self.peek_char() {
                if !c.is_ascii_digit() {
                    break;
                }

                self.next_char().unwrap();
            }
        }

        let literal = &self.input[starting_position..self.position];

        Token::new(
            token_kind,
            literal,
            Span::new(starting_position, self.position),
        )
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();

        let Some(c) = self.peek_char() else {
            return Ok(Token::eof(self.position));
        };

        let token = match c {
            'a'..='z' | 'A'..='Z' => self.parse_keyword_or_identifier(),
            '0'..='9' => self.parse_number(),
            '"' | '\'' | '`' => {
                todo!("Parse string")
            }
            ';' => self.parse_single_char_token(TokenKind::Semicolon),
            '+' => self.parse_single_char_token(TokenKind::Plus),
            '-' => self.parse_single_char_token(TokenKind::Minus),
            '*' => self.parse_single_char_token(TokenKind::Asterisk),
            '/' => self.parse_single_char_token(TokenKind::ForwardSlash),
            '=' => self.parse_single_char_token(TokenKind::Equals),
            ',' => self.parse_single_char_token(TokenKind::Comma),
            '.' => self.parse_single_char_token(TokenKind::Period),
            '[' => self.parse_single_char_token(TokenKind::OpeningBracket),
            ']' => self.parse_single_char_token(TokenKind::ClosingBracket),
            '{' => self.parse_single_char_token(TokenKind::OpeningBrace),
            '}' => self.parse_single_char_token(TokenKind::ClosingBrace),
            '<' => todo!("Parse <, <="),
            '>' => todo!("Parse >, >="),
            x if !x.is_ascii() => {
                return Err(LexerError::NonAsciiCharacter(x));
            }
            x if x.is_ascii_control() => {
                return Err(LexerError::AsciiControlCharacter(x));
            }
            _ => {
                todo!()
            }
        };

        Ok(token)
    }
}
