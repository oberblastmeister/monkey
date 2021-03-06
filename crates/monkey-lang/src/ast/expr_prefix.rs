use crate::ast;

#[derive(Debug, Clone, PartialEq)]
pub struct ExprPrefix {
    /// the token of the operator
    pub token: ast::Token,
    /// The operator type
    pub op: ast::Op,
    /// The right hand side of the expression
    pub rhs: ast::Expr,
}

expr_parse!(Prefix, ExprPrefix, "prefix expression");
