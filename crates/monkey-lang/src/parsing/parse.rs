use crate::parsing::{ParseError, Parser, Peek};

/// The parse trait, implemented by items that can be parsed.
pub trait Parse
where
    Self: Sized,
{
    /// Parse the current item from the parser.
    fn parse(p: &mut Parser) -> Result<Self, ParseError>;
}
