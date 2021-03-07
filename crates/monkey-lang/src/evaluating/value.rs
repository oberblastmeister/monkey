use smol_str::SmolStr;

pub enum Value {
    Bool(bool),
    Str(SmolStr),
    Num(f64),
}
