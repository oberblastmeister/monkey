pub use crate::{ast, Eval, EvalResult, Value};

impl Eval for ast::LitBool {
    fn eval(self) -> EvalResult<Value> {
        Ok(Value::Bool(self.value))
    }
}
