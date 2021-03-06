#[macro_use]
pub mod generated;

use std::ops::{Index, IndexMut};

pub use generated::TokenKind;

macro_rules! expr_parse {
    ($ty:ident, $local:ty, $expected:literal) => {
        impl crate::Parse for $local {
            fn parse(p: &mut crate::Parser<'_>) -> Result<Self, crate::ParseError> {
                let span = p.tok_at(0)?.span;

                match crate::ast::Expr::parse(p)? {
                    crate::ast::Expr::$ty(expr) => Ok(expr),
                    _ => Err(crate::ParseError::msg(&span, $expected)),
                }
            }
        }
    };
}

mod token;

mod expr;
mod expr_call;
mod expr_infix;
mod expr_lit;
mod expr_postfix;
mod expr_prefix;

mod lit;
mod lit_bool;
mod lit_fn;
mod lit_num;
mod lit_str;

mod stmt;
mod stmt_expr;
mod stmt_if;
mod stmt_let;
mod stmt_return;

mod block;

mod file;

mod op;

mod list;

mod ids;

pub use token::Token;

pub use expr::Expr;
pub use expr_call::ExprCall;
pub use expr_infix::ExprInfix;
pub use expr_lit::ExprLit;
pub use expr_postfix::ExprPostfix;
pub use expr_prefix::ExprPrefix;

pub use op::Op;

pub use lit::Lit;
pub use lit_bool::LitBool;
pub use lit_fn::LitFn;
pub use lit_num::LitNum;
pub use lit_str::LitStr;

pub use stmt::Stmt;
pub use stmt_expr::StmtExpr;
pub use stmt_if::StmtIf;
pub use stmt_let::StmtLet;
pub use stmt_return::StmtReturn;

pub use block::Block;

pub use file::File;

pub use list::List;

pub use ids::{Arena, ExprId, StmtId};

use crate::{Parse, ParseError, ParseResult};

pub struct Ast<P> {
    arena: Arena,
    errors: Vec<ParseError>,
    node: P,
}

impl<I, P> Index<I> for Ast<P>
where
    Arena: Index<I>,
{
    type Output = <Arena as Index<I>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.arena[index]
    }
}

impl<I, P> IndexMut<I> for Ast<P>
where
    Arena: IndexMut<I>,
{
    fn index_mut(&mut self, index: I) -> &mut <Arena as Index<I>>::Output {
        &mut self.arena[index]
    }
}
