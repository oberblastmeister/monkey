//! Borrowed from rune

use crate::{ParseError, ParseResult, Parser, Peek};

/// The parse trait, implemented by items that can be parsed.
pub trait Parse
where
    Self: Sized,
{
    /// Parse the current item from the parser.
    fn parse(p: &mut Parser) -> ParseResult<Self>;
}

macro_rules! tuple_impls {
    { $( ($( $name:ident ),+ $(,)?) ),+ $(,)? } => {
        $(
            impl<$($name),+> Parse for ($($name),+)
            where
                $($name: Parse,)+
                {
                    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
                        Ok(($( parser.parse::<$name>()? ),+))
                    }
                }
        )+
    };
}

tuple_impls! {
    (T0, T1),
    (T0, T1, T2),
    (T0, T1, T2, T3),
    (T0, T1, T2, T3, T4),
    (T0, T1, T2, T3, T4, T5),
    (T0, T1, T2, T3, T4, T5, T6),
    (T0, T1, T2, T3, T4, T5, T6, T7),
    (T0, T1, T2, T3, T4, T5, T6, T7, T8),
}

/// Parse implementation for something that can be optionally parsed.
impl<T> Parse for Option<T>
where
    T: Parse + Peek,
{
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        Ok(if parser.peek::<T>() {
            Some(parser.parse()?)
        } else {
            None
        })
    }
}

/// Parse implementation for something that is boxed.
impl<T> Parse for Box<T>
where
    T: Parse,
{
    #[inline]
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        Ok(Box::new(parser.parse()?))
    }
}

/// Parser implementation for a vector.
impl<T> Parse for Vec<T>
where
    T: Parse + Peek,
{
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut output = Vec::new();

        while parser.peek::<T>() {
            output.push(parser.parse()?);
        }

        Ok(output)
    }
}
