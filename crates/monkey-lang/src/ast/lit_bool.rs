use crate::{ast, Parse, ParseError, ParseResult, Parser, Peek, Peeker};

#[derive(Debug, Clone, PartialEq)]
pub struct LitBool {
    pub token: ast::Token,
    pub value: bool,
}

impl Parse for LitBool {
    fn parse(p: &mut Parser) -> ParseResult<LitBool> {
        let token = p.next()?;

        let value = match token.kind {
            K![true] => true,
            K![false] => false,
            _ => {
                return Err(ParseError::expected(
                    &token,
                    "boolean literal must be true or false",
                ))
            }
        };

        Ok(LitBool { value, token })
    }
}

impl Peek for LitBool {
    fn peek(p: &mut Peeker) -> bool {
        matches!(p.nth(0), K![true] | K![false])
    }
}
