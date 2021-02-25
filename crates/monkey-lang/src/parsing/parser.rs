use std::collections::VecDeque;

use crate::{Span};
use crate::ast::Token;
use super::{Lexer, ParseError};

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

#[derive(Debug)]
pub struct Parser {

}
