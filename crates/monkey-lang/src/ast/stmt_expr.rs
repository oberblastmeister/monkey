use crate::{ast, Parse, ParseResult, Parser};

#[derive(Debug, Clone, PartialEq)]
pub struct StmtExpr {
    pub expr: ast::Expr,
    pub semi: T![;],
}

impl Parse for StmtExpr {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        Ok(StmtExpr {
            expr: p.parse()?,
            semi: p.parse()?,
        })
    }
}
