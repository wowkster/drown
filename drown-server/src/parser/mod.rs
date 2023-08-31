use thiserror::Error;

use crate::{statement::Statement, parser::{lexer::Lexer, token::TokenKind}};

mod lexer;
mod parser;
mod token;

pub struct StatementParser;

#[derive(Debug, Error)]
pub enum StatementParseError {
    #[error("Encountered Lexer Error: {0}")]
    LexerError(#[from] lexer::LexerError),
}

impl StatementParser {
    pub fn parse(statement: &str) -> Result<Vec<Statement>, StatementParseError> {
        
        let mut lexer = Lexer::new(statement);

        let mut token = lexer.next_token()?;

        while token.kind() != TokenKind::Eof {
            println!("{:?}", token);
            token = lexer.next_token()?;
        }
        
        todo!()
    }
}