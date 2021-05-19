pub use crate::{ast, Eval, EvalResult, Value};

impl Eval for ast::LitStr {
    fn eval(self) -> EvalResult<Value> {
        todo!()
    }
}
