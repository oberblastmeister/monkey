use thiserror::Error;

pub type EvalResult<T, E = EvalError> = Result<T, E>;

pub struct EvalError {}

#[derive(Debug, Error)]
pub enum EvalErrorKind {}
