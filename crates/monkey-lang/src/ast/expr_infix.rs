use crate::ast;

#[derive(Debug, Clone, PartialEq)]
pub struct ExprInfix {
    /// The left hand side of the expression
    pub lhs: ast::ExprId,
    /// the token of the operator
    pub token: ast::Token,
    /// The operator type
    pub op: ast::Op,
    /// The right hand side of the expression
    pub rhs: ast::ExprId,
}

expr_parse!(Infix, ExprInfix, "binary expression");
