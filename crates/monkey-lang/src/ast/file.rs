use crate::{ast, Parse, ParseResult, Parser};

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub stmts: Vec<ast::StmtId>,
}

impl Parse for File {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        let mut stmts = Vec::new();

        loop {
            stmts.push(p.parse()?);

            if p.is_eof()? {
                break;
            }
        }

        Ok(File { stmts })
    }
}
