use crate::{ast, Parse, ParseResult, Parser};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(ast::StmtExpr),
    Return(ast::StmtReturn),
    Let(ast::StmtLet),
    If(ast::StmtIf),
}

impl Parse for Stmt {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        let parsed = match p.nth(0)? {
            K![return] => Stmt::Return(p.parse()?),
            K![let] => Stmt::Let(p.parse()?),
            K![if] => Stmt::If(p.parse()?),
            _ => Stmt::Expr(p.parse()?),
        };

        Ok(parsed)
    }
}
