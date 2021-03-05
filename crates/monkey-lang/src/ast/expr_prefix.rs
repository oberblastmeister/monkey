use crate::{ast, Parse, ParseError, ParseResult, Parser, Peek, Peeker, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub struct ExprPrefix {
    /// the token of the operator
    pub token: ast::Token,
    /// The operator type
    pub op: ast::Op,
    /// The right hand side of the expression
    pub rhs: ast::Expr,
}
