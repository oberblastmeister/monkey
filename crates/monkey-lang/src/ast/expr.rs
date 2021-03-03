use crate::{ast, Parse, ParseError, ParseResult, Parser, Peek, Peeker, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary(Box<ast::ExprBinary>),
    Lit(ast::Lit),
}

impl Parse for Expr {
    #[inline]
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        expr(p)
    }
}

#[inline]
fn expr(p: &mut Parser) -> ParseResult<Expr> {
    expr_bp(p, 0)
}

fn expr_bp(p: &mut Parser, min_bp: u8) -> ParseResult<Expr> {
    let mut lhs = match p.nth(0)? {
        K!['('] => expr_paren(p)?,
        _ if p.peek::<ast::Lit>() => Expr::Lit(p.parse()?),
        t => panic!("Bad token: {:?}", t),
    };

    loop {
        if p.peek::<ExprEnd>() {
            break;
        };

        let op = ast::BinOp::from_peeker(&mut p.peeker).ok_or_else(|| {
            ParseError::expected(&p.next().unwrap(), "Expected a binary operator")
        })?;

        let (l_bp, r_bp) = op.binding_power();

        if l_bp < min_bp {
            break;
        }

        lhs = {
            let token = p.next()?;

            let rhs = expr_bp(p, r_bp)?;

            Expr::Binary(Box::new(ast::ExprBinary {
                lhs,
                op,
                rhs,
                token,
            }))
        };

        continue;
    }

    Ok(lhs)
}

struct ExprEnd;

impl Peek for ExprEnd {
    fn peek(p: &mut Peeker) -> bool {
        matches!(p.nth(0), K![eof] | K![;] | K![')'])
    }
}

fn expr_paren(p: &mut Parser) -> ParseResult<Expr> {
    assert_eq!(p.nth(0).unwrap(), K!['(']);

    p.next().unwrap();
    let lhs = expr_bp(p, 0)?;
    p.parse::<T![')']>()?;
    Ok(lhs)
}
