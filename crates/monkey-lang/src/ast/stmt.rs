use crate::{ast, Parse, ParseError, ParseResult, Parser, Peek, Peeker, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(ast::Expr, T![;]),
}

impl Parse for Stmt {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        Ok(Stmt::Expr(p.parse()?, p.parse()?))
    }
}
