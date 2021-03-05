pub use monkey_macros::Precedent;

pub enum PrecedenceType {
    Prefix((), Precedence),
    Postfix(Precedence, ()),
    Infix(Precedence, Precedence)
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Precedence(u8);

#[doc(hidden)]
pub fn __precedence(n: u8) -> Precedence {
    Precedence(n)
}

impl Precedence {
    pub fn min() -> Precedence {
        Precedence(u8::MIN)
    }

    pub fn max() -> Precedence {
        Precedence(u8::MAX)
    }
    
    pub fn inner(self) -> u8 {
        self.0
    }
}
