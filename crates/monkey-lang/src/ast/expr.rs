use crate::{ast, Parse, ParseError, ParseResult, Parser, Peek, Peeker, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Lit(ast::Lit)
}

impl Parse for Expr {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        let lit: ast::Lit = p.parse()?;
        Ok(Expr::Lit(lit))
    }
}
