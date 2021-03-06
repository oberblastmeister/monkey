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
            if p.is_eof()? || p.peek::<BlockEnd>() {
                break;
            }

            stmts.push(p.parse()?);
        }

        let close_bracket = p.parse()?;

        Ok(Block {
            open_bracket,
            stmts,
            close_bracket,
        })
    }
}

struct BlockEnd;

impl Peek for BlockEnd {
    fn peek(p: &mut Peeker) -> bool {
        matches!(p.nth(0), K!['}'])
    }
}
