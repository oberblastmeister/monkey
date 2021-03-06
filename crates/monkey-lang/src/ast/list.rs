use std::marker::PhantomData;

use crate::{Parse, ParseResult, Parser, Peek};

#[derive(Debug, Clone, PartialEq)]
pub struct List<T, DO, DC, SEP = T![,]>
where
    T: Parse,
    DO: Parse,
    DC: Peek + Parse,
    SEP: Parse + Peek,
{
    pub open_delimit: DO,
    pub inner: Vec<T>,
    pub close_delimit: DC,
    pub sep: PhantomData<fn(SEP)>,
}

impl<T, DO, DC, SEP> Parse for List<T, DO, DC, SEP>
where
    T: Parse,
    DO: Parse,
    DC: Peek + Parse,
    SEP: Parse + Peek,
{
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        let open_delimit = p.parse()?;
        let mut inner: Vec<T> = Vec::new();

        let close_delimit = if p.peek::<DC>() {
            p.parse()?
        } else {
            loop {
                inner.push(p.parse()?);

                #[allow(clippy::if_same_then_else)]
                if p.parse::<Option<SEP>>()?.is_none() {
                    break p.parse()?;
                } else if p.peek::<DC>() {
                    break p.parse()?;
                }
            }
        };

        Ok(List {
            open_delimit,
            inner,
            close_delimit,
            sep: PhantomData,
        })
    }
}
