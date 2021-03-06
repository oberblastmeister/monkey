use crate::{ast, Parse, ParseError, ParseResult, Parser, Peek, Peeker, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Box<ast::StmtExpr>),
    Return(Box<ast::StmtReturn>),
    Let(Box<ast::StmtLet>),
    If(Box<ast::StmtIf>),
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
