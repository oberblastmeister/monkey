use std::collections::VecDeque;

use super::{Lexer, Parse, ParseError, ParseResult, Peek};
use crate::ast::{Token, TokenKind};
use crate::Span;

/// Construct used to peek a parser.
#[derive(Debug)]
pub struct Peeker<'a> {
    pub(crate) lexer: Lexer<'a>,

    buf: VecDeque<Token>,

    last: Option<Span>,
}

impl<'a> Peeker<'a> {
    /// Peek the token kind at the given position.
    pub fn nth(&mut self, n: usize) -> TokenKind {
        match self.at(n) {
            Ok(t) => match t {
                Some(t) => t.kind,
                None => TokenKind::Eof,
            },
            Err(error) => {
                TokenKind::Error
            }
        }
    }

    /// Make sure there are at least `n` items in the buffer, and return the
    /// item at that point.
    fn at(&mut self, n: usize) -> ParseResult<Option<&Token>> {
        while self.buf.len() <= n {
            let token = match self.lexer.next_token() {
                Ok(token) => token,
                Err(ParseError::Eof) => break,
                Err(e) => return Err(e),
            };

            self.last = Some(token.span);
            self.buf.push_back(token);
        }

        Ok(self.buf.get(n))
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
