pub use crate::{ast, Eval, EvalResult, Value};

impl Eval for ast::Stmt {
    fn eval(self) -> EvalResult<Value> {
        match self {
            ast::Stmt::Expr(stmt_expr) => stmt_expr.eval(),
            _ => todo!("Only expression statements are supported"),
        }
    }
}
