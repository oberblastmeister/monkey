use crate::{ast, Parse, ParseError, ParseResult, Parser, Peek, Peeker, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub open_bracket: T!['{'],
    pub stmts: Vec<ast::Stmt>,
    pub close_bracket: T!['}'],
}

impl Parse for Block {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        let open_bracket = p.parse()?;

        let mut stmts = Vec::new();

        loop {
            stmts.push(p.parse()?);
            if p.is_eof()? {
                break;
            }
        }

        let close_bracket = p.parse()?;

        Ok(Block {
            open_bracket,
            stmts,
            close_bracket,
        })
    }
}
