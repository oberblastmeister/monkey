use crate::{ast, Parse, ParseError, ParseResult, Parser, Peek, Peeker};
use monkey_util::{Precedence, PrecedenceType};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Infix(Box<ast::ExprInfix>),
    Prefix(Box<ast::ExprPrefix>),
    Postfix(Box<ast::ExprPostfix>),
    Lit(ast::Lit),
    Named(T![ident]),
}

impl Parse for Expr {
    #[inline]
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        expr(p)
    }
}

#[inline]
fn expr(p: &mut Parser) -> ParseResult<Expr> {
    expr_bp(p, Precedence::min())
}

fn expr_bp(p: &mut Parser, min_bp: Precedence) -> ParseResult<Expr> {
    let mut lhs = expr_lhs(p)?;

    loop {
        if p.peek::<ExprEnd>() {
            break;
        };

        let op = ast::Op::from_peeker(p.peeker())
            .ok_or_else(|| ParseError::expected(&p.next().unwrap(), "Expected an operator"))?;

        let binding_power = op.precedence();

        match binding_power {
            PrecedenceType::Infix(l_bp, r_bp) => {
                if l_bp < min_bp {
                    break;
                }

                let token = p.next()?;

                lhs = expr_infix(p, r_bp, token, op, lhs)?;
            }
            _ => {
                return Err(ParseError::expected(
                    &p.next().unwrap(),
                    "Expected an infix or postfix operator",
                ))
            }
        }

        continue;
    }

    Ok(lhs)
}

fn expr_lhs(p: &mut Parser) -> ParseResult<Expr> {
    Ok(match p.nth(0)? {
        K!['('] => expr_paren(p)?,
        K![ident] => Expr::Named(p.parse()?),
        _ if p.peek::<ast::Lit>() => Expr::Lit(p.parse()?),
        k => {
            let next = p.next()?;

            let op = ast::Op::from_kind(k)
                .ok_or_else(|| ParseError::expected(&next, "Expected operator"))?;

            match op.precedence() {
                PrecedenceType::Prefix((), r_bp) => expr_prefix(p, r_bp, op, next)?,
                _ => return Err(ParseError::expected(&next, "Expeced prefix operator")),
            }
        }
    })
}

fn expr_with_ident(p: &mut Parser) -> ParseResult<Expr> {
    let curr = p.nth(0)?;
    assert_eq!(curr, K![ident]);

    Ok(match p.nth(1)? {
        K!['('] => todo!(),
        _ => Expr::Named(p.parse()?),
    })
}

struct ExprEnd;

impl Peek for ExprEnd {
    fn peek(p: &mut Peeker) -> bool {
        matches!(p.nth(0), K![eof] | K![;] | K![')'] | K!['{'] | K![,])
    }
}

fn expr_prefix(
    p: &mut Parser,
    r_bp: Precedence,
    op: ast::Op,
    token: ast::Token,
) -> ParseResult<Expr> {
    let rhs = expr_bp(p, r_bp)?;
    let expr = Expr::Prefix(Box::new(ast::ExprPrefix { op, token, rhs }));
    Ok(expr)
}

fn expr_infix(
    p: &mut Parser,
    r_bp: Precedence,
    token: ast::Token,
    op: ast::Op,
    lhs: Expr,
) -> ParseResult<Expr> {
    let rhs = expr_bp(p, r_bp)?;

    Ok(Expr::Infix(Box::new(ast::ExprInfix {
        lhs,
        op,
        rhs,
        token,
    })))
}

fn expr_paren(p: &mut Parser) -> ParseResult<Expr> {
    assert_eq!(p.nth(0).unwrap(), K!['(']);

    p.next().unwrap();
    let lhs = expr_bp(p, Precedence::min())?;
    p.parse::<T![')']>()?;
    Ok(lhs)
}
