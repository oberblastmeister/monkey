#[macro_use]
pub mod generated;

pub use generated::TokenKind;

// macro_rules! expr_parse {
//     ($ty:ident, $local:ty, $expected:literal) => {
//         impl crate::Parse for $local {
//             fn parse(p: &mut crate::Parser<'_>) -> Result<Self, crate::ParseError> {
//                 let t = p.nth(0)?;

//                 match crate::ast::Expr::parse(p)? {
//                     crate::ast::Expr::$ty(expr) => Ok(*expr),
//                     _ => Err(crate::ParseError::expected(&t, $expected)),
//                 }
//             }
//         }
//     };
// }

mod token;

mod expr;
mod expr_lit;
mod expr_binary;
mod expr_prefix;

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
pub use expr_binary::ExprBinary;
pub use expr_prefix::ExprPrefix;

pub use op::Op;

pub use lit::Lit;
pub use lit_bool::LitBool;
pub use lit_num::LitNum;
pub use lit_str::LitStr;

pub use stmt::Stmt;
pub use file::File;
