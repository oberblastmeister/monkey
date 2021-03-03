use crate::{ast, Parse, ParseError, ParseResult, Parser, Peek, Peeker, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub struct ExprBinary {
    pub lhs: ast::Expr,
    /// the token of the operator
    pub token: ast::Token,
    pub op: BinOp,
    pub rhs: ast::Expr,
}

impl ExprBinary {
    pub fn with_lhs(
        p: &mut Parser<'_>,
        lhs: ast::Expr,
        op: BinOp,
    ) -> ParseResult<ExprBinary> {
        assert!(BinOp::from_peeker(&mut p.peeker).is_some());

        let token = p.next()?;

        Ok(ExprBinary {
            lhs,
            token,
            op,
            rhs: p.parse()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

use BinOp::*;

impl BinOp {
    pub fn from_peeker(p: &mut Peeker<'_>) -> Option<BinOp> {
        Some(match p.nth(0) {
            K![+] => Add,
            K![-] => Sub,
            K![*] => Mul,
            K![/] => Div,
            _ => return None,
        })
    }

    pub fn binding_power(self) -> (u8, u8) {
        match self {
            Add | Sub => (1, 2),
            Mul | Div => (3, 4),
        }
    }
}

impl Peek for BinOp {
    fn peek(p: &mut Peeker) -> bool {
        BinOp::from_peeker(p).is_some()
    }
}
