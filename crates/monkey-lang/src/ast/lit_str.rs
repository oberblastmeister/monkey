use crate::{ast, Parse, ParseError, ParseResult, Parser, Peek, Peeker, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub struct LitStr {
    pub token: ast::Token,
}

impl Parse for LitStr {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        let token = p.next()?;

        match token.kind {
            K![string] => (),
            _ => return Err(ParseError::expected(&token, "expected a string literal")),
        };

        Ok(LitStr { token })
    }
}

impl Peek for LitStr {
    fn peek(p: &mut Peeker) -> bool {
        matches!(p.nth(0), K![string])
    }
}
