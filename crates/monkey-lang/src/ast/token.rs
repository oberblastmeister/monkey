use smol_str::SmolStr;

use crate::{Spanned, Span};
use crate::ast::TokenKind;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
    pub text: SmolStr,
}

impl Spanned for Token {
    fn span(&self) -> Span {
        self.span
    }
}
