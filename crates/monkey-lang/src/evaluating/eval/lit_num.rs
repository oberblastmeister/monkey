pub use crate::{ast, Eval, EvalResult, Value};

impl Eval for ast::LitNum {
    fn eval(self) -> EvalResult<Value> {
        Ok(Value::Num(self.value))
    }
}
