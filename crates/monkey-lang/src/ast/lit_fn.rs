use std::marker::PhantomData;

use crate::{ast, Parse, ParseResult, Parser, Peek, Peeker};

#[derive(Debug, Clone, PartialEq)]
pub struct LitFn {
    fn_token: T![fn],
    parameters: Parameters,
    body: ast::Block,
}

impl Parse for LitFn {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        Ok(LitFn {
            fn_token: p.parse()?,
            parameters: p.parse()?,
            body: p.parse()?,
        })
    }
}

impl Peek for LitFn {
    fn peek(p: &mut Peeker) -> bool {
        matches!(p.nth(0), K![fn])
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameters(pub ast::List<T![ident], T!['('], T![')']>);

impl Parse for Parameters {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        Ok(Parameters(p.parse()?))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParamList<T, DO, DC, SEP = T![,]>
where
    T: Parse,
    DO: Parse,
    DC: Peek + Parse,
{
    pub open_delimit: DO,
    pub inner: Vec<T>,
    pub close_delimit: DC,
    pub sep: PhantomData<fn(SEP)>,
}

impl<T, DO, DC, SEP> Parse for ParamList<T, DO, DC, SEP>
where
    T: Parse,
    DO: Parse,
    DC: Peek + Parse,
    SEP: Parse,
{
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        let open_delimit: DO = p.parse()?;
        let mut inner: Vec<T> = Vec::new();

        let close_delimit = if p.peek::<DC>() {
            // if there was nothing inside
            p.parse()?
        } else {
            loop {
                inner.push(p.parse()?);

                if p.peek::<DC>() {
                    break p.parse()?;
                }

                p.parse::<SEP>()?;
            }
        };

        Ok(ParamList {
            open_delimit,
            inner,
            close_delimit,
            sep: PhantomData,
        })
    }
}
