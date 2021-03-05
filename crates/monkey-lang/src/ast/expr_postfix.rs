use crate::{ast, Parse, ParseError, ParseResult, Parser, Peek, Peeker, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub struct ExprPostfix {
    /// The right hand side of the expression
    pub lhs: ast::Expr,
    /// the token of the operator
    pub token: ast::Token,
    /// The operator type
    pub op: ast::Op,
}

expr_parse!(Postfix, ExprPostfix, "postfix expression");
