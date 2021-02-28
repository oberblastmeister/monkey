use thiserror::Error;

use crate::ast::Token;

pub type ParseResult<T, E = ParseError> = Result<T, E>;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum ParseError {
    #[error("BUG: Should not be displayed, this is just a marker error")]
    Eof,

    #[error("Unknown type")]
    Unknown,
}

/// None is Eof
pub trait ToEof {
    fn eof(&self) -> ParseResult<char>;  
}

impl ToEof for Option<char> {
    fn eof(&self) -> ParseResult<char> {
        self.ok_or(ParseError::Eof)
    }
}

impl ParseError {
    pub fn expected(first: &Token, second: &str) -> Self {
        todo!()
    }
}
