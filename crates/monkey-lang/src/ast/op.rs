use crate::{ast, Parse, ParseError, ParseResult, Parser, Peek, Peeker, Spanned};

pub enum Op {
    Prefix(PrefixOp),
    Bin(BinOp),
}

impl Op {
    pub fn from_peeker(p: &mut Peeker<'_>) -> Option<Op> {
        BinOp::from_peeker(p)
            .map(Op::Bin)
            .or_else(|| PrefixOp::from_peeker(p).map(Op::Prefix))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
}

use BinOp::*;

impl BinOp {
    pub fn from_peeker(p: &mut Peeker<'_>) -> Option<BinOp> {
        Some(match p.nth(0) {
            K![+] => Add,
            K![-] => Sub,
            _ => return None,
        })
    }

    fn binding_power(self) -> (u8, u8) {
        match self {
            Add | Sub => (1, 2),
        }
    }
}

impl Peek for BinOp {
    fn peek(p: &mut Peeker) -> bool {
        BinOp::from_peeker(p).is_some()
    }
}

pub enum PrefixOp {
    Bang,
}

use PrefixOp::*;

impl PrefixOp {
    fn from_peeker(p: &mut Peeker<'_>) -> Option<PrefixOp> {
        Some(match p.nth(0) {
            K![!] => Bang,
            _ => return None,
        })
    }
}
