use crate::{ast, Parse, ParseError, ParseErrorKind, ParseResult, Parser, Peek, Peeker, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub struct LitNum {
    pub num: T![number],
    pub value: f64,
}

impl Parse for LitNum {
    fn parse(p: &mut Parser) -> ParseResult<LitNum> {
        let num: T![number] = p.parse()?;
        let token = &num.token;

        let value = match token.kind {
            K![number] => token
                .text
                .parse::<f64>()
                .map_err(|e| ParseError::new(token, e.into()))?,
            _ => {
                return Err(ParseError::expected(
                    &num.token,
                    "boolean literal must be true or false",
                ))
            }
        };

        Ok(LitNum { value, num })
    }
}

impl Peek for LitNum {
    fn peek(p: &mut Peeker) -> bool {
        matches!(p.nth(0), K![number])
    }
}
