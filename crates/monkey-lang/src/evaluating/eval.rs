mod lit;
mod lit_bool;
mod lit_num;
mod lit_str;

mod expr;

mod stmt;
mod stmt_expr;

mod file;

use crate::{EvalResult, Value};

pub trait Eval {
    fn eval(self) -> EvalResult<Value>;
}

impl<T: Eval> Eval for Box<T> {
    fn eval(self) -> EvalResult<Value> {
        T::eval(*self)
    }
}
