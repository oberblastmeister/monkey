pub use crate::{ast, Eval, EvalResult, Value};

impl Eval for ast::Expr {
    fn eval(self) -> EvalResult<Value> {
        match self {
            ast::Expr::Lit(lit) => lit.eval(),
            _ => todo!("Only literal expressions are supported"),
        }
    }
}
