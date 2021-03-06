use core::fmt;
use std::{borrow::Cow, collections::VecDeque};

use la_arena::Arena;

use super::{Lexer, Parse, ParseError, ParseErrorKind, ParseResult, Peek};
use crate::ast::{self, Token, TokenKind};
use crate::Span;
use crate::ShrinkToFit;

/// Construct used to peek a parser.
#[derive(Debug)]
pub struct Peeker<'a> {
    pub(crate) lexer: Lexer<'a>,
    buf: VecDeque<Token>,
    last: Option<Span>,
}

impl<'a> Peeker<'a> {
    pub fn new(input: &'a str) -> Peeker<'a> {
        let lexer = Lexer::new(input);
        let buf = VecDeque::new();
        let last = None;
        Peeker { lexer, buf, last }
    }

    /// Peek the token kind at the given position.
    pub fn nth(&mut self, n: usize) -> TokenKind {
        match self.at(n) {
            Ok(t) => match t {
                Some(t) => t.kind,
                None => TokenKind::Eof,
            },
            Err(_) => TokenKind::Error,
        }
    }

    /// Make sure there are at least `n` items in the buffer, and return the
    /// item at that point.
    fn at(&mut self, n: usize) -> ParseResult<Option<&Token>> {
        while self.buf.len() <= n {
            let token = match self.lexer.next_token() {
                Ok(token) => token,
                Err(e) if e.kind() == &ParseErrorKind::Eof => break,
                Err(e) => return Err(e),
            };

            self.last = Some(token.span);
            self.buf.push_back(token);
        }

        Ok(self.buf.get(n))
    }

    /// Peek something else
    pub fn peek<T>(&mut self) -> bool
    where
        T: Peek,
    {
        T::peek(self)
    }
}

#[derive(Debug)]
pub struct Parser<'a> {
    peeker: Peeker<'a>,
    errors: Vec<ParseError>,
    ast_arena: ast::Arena,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { peeker: Peeker::new(input), errors: Vec::new(), ast_arena: ast::Arena::default() }
    }

    #[inline]
    pub fn peeker(&mut self) -> &mut Peeker<'a> {
        &mut self.peeker
    }

    /// Parse a specific item from the parser.
    pub fn parse<T>(&mut self) -> ParseResult<T>
    where
        T: Parse,
    {
        T::parse(self)
    }

    pub fn peek<T>(&mut self) -> bool
    where
        T: Peek,
    {
        T::peek(&mut self.peeker)
    }

    /// Try to consume a single thing matching `T`, returns `true` if any tokens
    /// were consumed.
    pub fn try_parse<T>(&mut self) -> ParseResult<bool>
    where
        T: Parse + Peek,
    {
        Ok(if self.peek::<T>() {
            self.parse::<T>()?;
            true
        } else {
            false
        })
    }

    /// Try to consume all things matching `T`, returns `true` if any tokens
    /// were consumed.
    pub fn try_parse_all<T>(&mut self) -> ParseResult<bool>
    where
        T: Parse + Peek,
    {
        let mut consumed = false;

        while self.peek::<T>() {
            self.parse::<T>()?;
            consumed = true;
        }

        Ok(consumed)
    }

    /// Consume the next token from the parser.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> ParseResult<Token> {
        if let Some(t) = self.peeker.buf.pop_front() {
            return Ok(t);
        }

        match self.peeker.lexer.next_token() {
            Err(e) if e.kind() == &ParseErrorKind::Eof => {
                Ok(Token { span: Span::new(0.into(), 0.into()), kind: TokenKind::Eof })
            }
            Err(e) => Err(e),
            Ok(t) => Ok(t),
        }
    }

    /// Get the span at the given position.
    pub fn tok_at(&mut self, n: usize) -> Result<Cow<'_, Token>, ParseError> {
        Ok(if let Some(t) = self.peeker.at(n)? {
            Cow::Borrowed(t)
        } else {
            let t = Token { kind: TokenKind::Eof, span: Span::new(0.into(), 0.into()) };
            Cow::Owned(t)
        })
    }

    /// Peek the token kind at the given position.
    pub fn nth(&mut self, n: usize) -> ParseResult<TokenKind> {
        if let Some(t) = self.peeker.at(n)? {
            Ok(t.kind)
        } else {
            Ok(TokenKind::Eof)
        }
    }

    /// Test if the parser is at end-of-file, after which there is no more input
    /// to parse.
    pub fn is_eof(&mut self) -> Result<bool, ParseError> {
        Ok(self.peeker.at(0)?.is_none())
    }

    /// Assert that the parser has reached its end-of-file.
    pub fn eof(&mut self) -> Result<(), ParseError> {
        if let Some(token) = self.peeker.at(0)? {
            return Err(ParseError::new(
                token,
                ParseErrorKind::ExpectedEof { actual: token.kind.as_str() },
            ));
        }

        Ok(())
    }

    pub fn alloc_stmt(&mut self, stmt: ast::Stmt) -> ast::StmtId {
        self.ast_arena.stmts.alloc(stmt)
    }

    pub fn alloc_expr(&mut self, expr: ast::Expr) -> ast::ExprId {
        self.ast_arena.exprs.alloc(expr)
    }
}

pub fn parse<T: Parse>(input: &str) -> ParseResult<T> {
    Parser::new(input).parse::<T>()
}
