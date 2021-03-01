use crate::{ast, Parse, ParseError, ParseResult, Parser, Peek, Peeker, Spanned};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lit {
    Bool(ast::LitBool),
}

impl Parse for Lit {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        match p.nth(0)? {
            K![true] | K![false] => return Ok(Lit::Bool(p.parse()?)),
            _ => (),
        }

        Err(ParseError::expected(&p.next()?, "expected literal"))
    }
}
