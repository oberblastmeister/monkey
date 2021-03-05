#[macro_use]
pub mod generated;

pub use generated::TokenKind;

macro_rules! expr_parse {
    ($ty:ident, $local:ty, $expected:literal) => {
        impl crate::Parse for $local {
            fn parse(p: &mut crate::Parser<'_>) -> Result<Self, crate::ParseError> {
                let span = p.tok_at(0)?.span;

                match crate::ast::Expr::parse(p)? {
                    crate::ast::Expr::$ty(expr) => Ok(*expr),
                    _ => Err(crate::ParseError::msg(&span, $expected)),
                }
            }
        }
    };
}

mod token;

mod expr;
mod expr_lit;
mod expr_infix;
mod expr_prefix;
mod expr_postfix;

mod lit;
mod lit_bool;
mod lit_num;
mod lit_str;

mod stmt;

mod file;

mod op;

pub use token::Token;

pub use expr::Expr;
pub use expr_lit::ExprLit;
pub use expr_infix::ExprInfix;
pub use expr_prefix::ExprPrefix;
pub use expr_postfix::ExprPostfix;

pub use op::Op;

pub use lit::Lit;
pub use lit_bool::LitBool;
pub use lit_num::LitNum;
pub use lit_str::LitStr;

pub use stmt::Stmt;
pub use file::File;
