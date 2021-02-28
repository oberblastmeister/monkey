use std::collections::VecDeque;

use super::{Lexer, Parse, ParseError, ParseResult, Peek};
use crate::ast::{Token, TokenKind};
use crate::Span;

/// Construct used to peek a parser.
#[derive(Debug)]
pub struct Peeker<'a> {
    pub(crate) lexer: Lexer<'a>,
    buf: VecDeque<Token>,
    // NB: parse errors encountered during peeking.
    error: Option<ParseError>,
    /// The last span we encountered. Used to provide better EOF diagnostics.
    last: Option<Span>,
}

impl<'a> Peeker<'a> {
    pub fn nth(&mut self, n: usize) -> TokenKind {
        todo!()
    }
}

#[derive(Debug)]
pub struct Parser<'a> {
    peeker: Peeker<'a>,
}

impl<'a> Parser<'a> {
    /// Parse a specific item from the parser.
    pub fn parse<T>(&mut self) -> ParseResult<T>
    where
        T: Parse,
    {
        T::parse(self)
    }

    pub fn peek<T>(&mut self) -> ParseResult<bool>
    where
        T: Peek,
    {
        let result = T::peek(&mut self.peeker);
        Ok(result)
    }

    pub fn next(&mut self) -> ParseResult<Token> {
        todo!()
    }
}
