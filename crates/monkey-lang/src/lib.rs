#[macro_use]
pub mod ast;
mod evaluating;
mod parsing;
mod spanned;
mod traits;

pub use self::traits::ShrinkToFit;
pub use evaluating::{Eval, EvalError, EvalResult, Value};
pub use monkey_macros::{Parse, ShrinkToFit, Spanned};
pub use parsing::{
    parse, tokenize, Parse, ParseError, ParseErrorKind, ParseResult, Parser, Peek, Peeker,
};
pub(crate) use spanned::{Span, Spanned};
