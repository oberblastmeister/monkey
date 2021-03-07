pub use crate::{ast, Eval, EvalResult, Value};

impl Eval for ast::Lit {
    fn eval(self) -> EvalResult<Value> {
        match self {
            ast::Lit::Bool(lit_bool) => lit_bool.eval(),
            ast::Lit::Num(lit_num) => lit_num.eval(),
            ast::Lit::Str(lit_str) => lit_str.eval(),
            ast::Lit::Fn(_lit_fn) => todo!("Function literals are not yet supported"),
        }
    }
}
