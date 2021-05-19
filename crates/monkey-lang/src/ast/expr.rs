use crate::{ast, Parse, ParseError, ParseResult, Parser, Peek, Peeker};
use monkey_util::{Precedence, PrecedenceType};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Infix(ast::ExprInfix),
    Prefix(ast::ExprPrefix),
    Postfix(ast::ExprPostfix),
    Lit(ast::Lit),
    Named(T![ident]),
    Call(ast::ExprCall),
}

impl Parse for Expr {
    fn parse(p: &mut Parser) -> ParseResult<Self> {
        Expr::parse_binding_power(p, Precedence::min())
    }
}

impl Expr {
    fn parse_binding_power(p: &mut Parser, min_bp: Precedence) -> ParseResult<ast::Expr> {
        let mut lhs = Expr::parse_lhs(p)?;

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

                    let expr_id = p.alloc_expr(lhs);
                    lhs = Expr::parse_infix(p, r_bp, token, op, expr_id)?;
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

    fn parse_lhs(p: &mut Parser) -> ParseResult<Expr> {
        Ok(match p.nth(0)? {
            K!['('] => Expr::parse_paren(p)?,
            // TODO: fix
            K![ident] => Expr::parse_ident(p)?,
            _ if p.peek::<ast::Lit>() => Expr::Lit(p.parse()?),
            k => {
                let next = p.next()?;

                let op = ast::Op::from_kind(k)
                    .ok_or_else(|| ParseError::expected(&next, "Expected operator"))?;

                match op.precedence() {
                    PrecedenceType::Prefix((), r_bp) => Expr::parse_prefix(p, r_bp, op, next)?,
                    _ => return Err(ParseError::expected(&next, "Expeced prefix operator")),
                }
            }
        })
    }

    fn parse_ident(p: &mut Parser) -> ParseResult<Expr> {
        let curr = p.nth(0)?;
        assert_eq!(curr, K![ident]);

        Ok(match p.nth(1)? {
            K!['('] => Expr::Call(p.parse()?),
            _ => Expr::Named(p.parse()?),
        })
    }

    fn parse_paren(p: &mut Parser) -> ParseResult<ast::Expr> {
        assert_eq!(p.nth(0).unwrap(), K!['(']);

        p.next().unwrap();
        let lhs = Expr::parse_binding_power(p, Precedence::min())?;
        p.parse::<T![')']>()?;
        Ok(lhs)
    }

    fn parse_prefix(
        p: &mut Parser,
        r_bp: Precedence,
        op: ast::Op,
        token: ast::Token,
    ) -> ParseResult<Expr> {
        let rhs = Expr::parse_binding_power(p, r_bp)?;
        let expr = Expr::Prefix(ast::ExprPrefix { op, token, rhs: p.alloc_expr(rhs) });
        Ok(expr)
    }

    fn parse_infix(
        p: &mut Parser,
        r_bp: Precedence,
        token: ast::Token,
        op: ast::Op,
        lhs: ast::ExprId,
    ) -> ParseResult<Expr> {
        let rhs = Expr::parse_binding_power(p, r_bp)?;

        Ok(Expr::Infix(ast::ExprInfix { lhs, op, rhs: p.alloc_expr(rhs), token }))
    }
}

struct ExprEnd;

impl Peek for ExprEnd {
    fn peek(p: &mut Peeker) -> bool {
        matches!(p.nth(0), K![eof] | K![;] | K![')'] | K!['{'] | K![,] | K![']'])
    }
}
