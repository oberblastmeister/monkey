use crate::{ast, Parse, ParseResult, Parser};

#[derive(Debug, Clone, PartialEq)]
pub struct StmtLet {
    let_token: T![let],
    ident: T![ident],
    eq: T![=],
    expr: ast::ExprId,
    semi: T![;],
}

impl Parse for StmtLet {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        Ok(StmtLet {
            let_token: p.parse()?,
            ident: p.parse()?,
            eq: p.parse()?,
            expr: p.parse()?,
            semi: p.parse()?,
        })
    }
}
