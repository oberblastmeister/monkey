pub use crate::{ast, Eval, EvalResult, Value};

impl Eval for ast::StmtExpr {
    fn eval(self) -> EvalResult<Value> {
        todo!()
        // self.expr.eval()
    }
}
