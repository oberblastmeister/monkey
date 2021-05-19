use text_size::TextRange;

pub type Span = TextRange;

/// Types for which we can get a span.
pub trait Spanned {
    /// Get the span of the type.
    fn span(&self) -> Span;
}

impl Spanned for Span {
    fn span(&self) -> Span {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Spanned;

    #[test]
    fn derive_spanned() {
        #[derive(Spanned)]
        struct It {
            first: Span,
            second: Span,
        }

        let it =
            It { first: Span::new(0.into(), 3.into()), second: Span::new(10.into(), 15.into()) };

        assert_eq!(it.span(), Span::new(0.into(), 15.into()));
    }
}
