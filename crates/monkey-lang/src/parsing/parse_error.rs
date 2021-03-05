use std::borrow::Borrow;

use thiserror::Error;

use crate::ast::Token;
use crate::{Span, Spanned};

pub type ParseResult<T, E = ParseError> = Result<T, E>;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorKind {
    #[error("Unexpected end of file")]
    Eof,

    #[error("Failed to parse: {message}")]
    Message { message: &'static str },

    #[error("Expected eof")]
    ExpectedEof { actual: &'static str },

    #[error("Unknown token")]
    Unknown,

    #[error("expected {expected}, but got `{actual}`")]
    Expected {
        actual: &'static str,
        expected: &'static str,
    },

    #[error("Invalid number literal: {0}")]
    InvalidLitNum(#[from] std::num::ParseFloatError),
}

/// None is Eof
pub trait ToEof {
    fn eof(&self, span: impl FnOnce() -> Span) -> ParseResult<char>;
}

impl ToEof for Option<char> {
    fn eof(&self, span: impl FnOnce() -> Span) -> ParseResult<char> {
        self.ok_or_else(|| ParseError::eof(span()))
    }
}

#[derive(Error, Debug, Clone)]
#[error("Need to implement span")]
pub struct ParseError {
    span: Span,
    #[source]
    kind: ParseErrorKind,
}

impl ParseError {
    pub fn new<S, Q>(spanned: &Q, kind: ParseErrorKind) -> ParseError
    where
        S: Spanned,
        Q: Borrow<S>,
    {
        ParseError {
            span: spanned.borrow().span(),
            kind,
        }
    }

    pub fn msg<S, Q>(spanned: &Q, msg: &'static str) -> ParseError
    where
        S: Spanned,
        Q: Borrow<S>,
    {
        ParseError {
            span: spanned.borrow().span(),
            kind: ParseErrorKind::Message { message: msg },
        }
    }

    pub fn kind(&self) -> &ParseErrorKind {
        &self.kind
    }

    pub(crate) fn eof(span: Span) -> Self {
        ParseError::new(&span, ParseErrorKind::Eof)
    }

    /// Construct an expectation error.
    pub(crate) fn expected(actual: &Token, expected: &'static str) -> ParseError
    {
        Self::new(
            actual,
            ParseErrorKind::Expected {
                actual: actual.kind.as_str(),
                expected,
            },
        )
    }
}
