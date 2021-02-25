use super::Peeker;

/// Implemented by tokens that can be peeked for.
pub trait Peek {
    /// Peek the parser for the given token.
    fn peek(peeker: &mut Peeker) -> bool;
}
