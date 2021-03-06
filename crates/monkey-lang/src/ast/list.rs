#[derive(Debug, Clone, PartialEq)]
pub struct List<T, DO, DC>
where
    T: Parse,
    DO: Parse,
    DC: Peek + Parse,
{
    pub open_delimit: DO,
    pub inner: Vec<T>,
    pub close_delimit: DC,
}

impl<T, DO, DC> Parse for List<T, DO, DC>
where
    T: Parse,
    DO: Parse,
    DC: Peek + Parse,
{
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        let open_delimit = p.parse()?;
        let mut inner: Vec<T> = Vec::new();

        let close_delimit = loop {
            inner.push(p.parse()?);

            #[allow(clippy::if_same_then_else)]
            if p.parse::<Option<T![,]>>()?.is_none() {
                break p.parse()?;
            } else if p.peek::<DC>() {
                break p.parse()?;
            }
        };

        Ok(List {
            open_delimit,
            inner,
            close_delimit,
        })
    }
}
