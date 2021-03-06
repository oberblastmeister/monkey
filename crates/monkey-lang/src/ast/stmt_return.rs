use crate::{Parse, ParseResult, Parser, ast};

#[derive(Debug, Clone, PartialEq)]
pub struct StmtReturn {
    return_token: T![return],
    expr: ast::ExprId,
    semi: T![;],
}

impl Parse for StmtReturn {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        Ok(StmtReturn {
            return_token: p.parse()?,
            expr: p.parse()?,
            semi: p.parse()?,
        })
    }
}
