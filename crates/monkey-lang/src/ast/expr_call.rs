use crate::{ast, Parse, ParseResult, Parser};

#[derive(Debug, Clone, PartialEq)]
pub struct ExprCall {
    ident: T![ident],
    arguments: Arguments,
}

impl Parse for ExprCall {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        Ok(ExprCall {
            ident: p.parse()?,
            arguments: p.parse()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arguments(ast::List<T![ident], T!['{'], T!['}']>);

impl Parse for Arguments {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        Ok(Arguments(p.parse()?))
    }
}
