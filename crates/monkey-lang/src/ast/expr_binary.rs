use crate::{ast, Parse, ParseError, ParseResult, Parser, Peek, Peeker, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub struct ExprBinary {
    /// The left hand side of the expression
    pub lhs: ast::Expr,
    /// the token of the operator
    pub token: ast::Token,
    /// The operator type
    pub op: BinOp,
    /// The right hand side of the expression
    pub rhs: ast::Expr,
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
