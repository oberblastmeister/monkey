use crate::{ast, Parse, ParseError, ParseResult, Parser, Peek, Peeker};

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Bool(ast::LitBool),
    Num(ast::LitNum),
    Str(ast::LitStr),
    Fn(Box<ast::LitFn>),
}

impl Parse for Lit {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        Ok(match p.nth(0)? {
            K![true] | K![false] => Lit::Bool(p.parse()?),
            K![number] => Lit::Num(p.parse()?),
            K![string] => Lit::Str(p.parse()?),
            K![fn] => Lit::Fn(p.parse()?),
            _ => return Err(ParseError::expected(&p.next()?, "expected literal")),
        })
    }
}

impl Peek for Lit {
    fn peek(p: &mut Peeker) -> bool {
        p.peek::<ast::LitFn>()
            | p.peek::<ast::LitBool>()
            | p.peek::<ast::LitStr>()
            | p.peek::<ast::LitNum>()
    }
}
