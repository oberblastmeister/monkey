use crate::{ast, Parse, ParseError, ParseResult, Parser, Peek, Peeker, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Bool(ast::LitBool),
    Num(ast::LitNum),
    Str(ast::LitStr),
}

impl Parse for Lit {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        match p.nth(0)? {
            K![true] | K![false] => return Ok(Lit::Bool(p.parse()?)),
            K![number] => return Ok(Lit::Num(p.parse()?)),
            K![string] => return Ok(Lit::Str(p.parse()?)),
            _ => (),
        }

        Err(ParseError::expected(&p.next()?, "expected literal"))
    }
}

impl Peek for Lit {
    fn peek(p: &mut Peeker) -> bool {
        p.peek::<ast::LitBool>() | p.peek::<ast::LitStr>() | p.peek::<ast::LitNum>()
    }
}
