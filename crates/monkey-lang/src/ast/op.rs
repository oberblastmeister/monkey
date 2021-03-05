use monkey_util::Precedent;

use crate::{ast, Parse, ParseError, ParseResult, Parser, Peek, Peeker, Spanned};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Precedent)]
pub enum Op {
    #[prec(infix, left)]
    LogicalAnd,

    #[prec(infix, left)]
    LogicalOr,

    #[prec(infix, left)]
    BitOr,

    #[prec(infix, left)]
    BitAnd,

    #[prec(infix, left)]
    Gt,

    #[prec(infix, left)]
    GtEq,

    #[prec(infix, left)]
    Lt,

    #[prec(infix, left)]
    LtEq,

    #[prec(infix, left)]
    ShiftLeft,

    #[prec(infix, left)]
    ShiftRight,

    #[prec(infix, left)]
    Add,

    #[prec(infix, left)]
    Sub,

    #[prec(infix, left)]
    Mul,

    #[prec(infix, left)]
    Div,

    #[prec(infix, left)]
    Modulo,

    #[prec(infix, right)]
    Power,

    #[prec(prefix)]
    BitNot,

    #[prec(prefix)]
    LogicalNot,
}

use Op::*;

impl Op {
    pub fn from_kind(kind: ast::TokenKind) -> Option<Op> {
        Some(match kind {
            K![&&] => LogicalAnd,
            K![||] => LogicalOr,

            K![&] => BitAnd,
            K![|] => BitOr,

            K![>] => Gt,
            K![>=] => GtEq,
            K![<] => Lt,
            K![<=] => LtEq,

            K![<<] => ShiftLeft,
            K![>>] => ShiftRight,

            K![+] => Add,
            K![-] => Sub,
            K![*] => Mul,
            K![/] => Div,
            K![%] => Modulo,

            K![^] => Power,
            K![~] => BitNot,
            K![!] => LogicalNot,
            _ => return None,
        })
    }

    pub fn from_peeker(p: &mut Peeker<'_>) -> Option<Op> {
        Op::from_kind(p.nth(0))
    }
}
