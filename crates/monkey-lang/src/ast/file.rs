use crate::{ast, Parse, ParseError, ParseResult, Parser, Peek, Peeker, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub stmts: Vec<ast::Stmt>,
}

impl Parse for File {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        let mut stmts: Vec<ast::Stmt> = Vec::new();

        loop {
            stmts.push(p.parse()?);
            if p.is_eof()? {
                break;
            }
        }

        Ok(File { stmts })
    }
}
