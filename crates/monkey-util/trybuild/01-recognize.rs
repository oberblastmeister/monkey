use monkey_macros::Precedent;

#[derive(Precedent)]
pub enum Op {
    #[prec(infix, left)]
    Add,
    #[prec(infix, left)]
    Sub,
    #[prec(infix, left)]
    Mul,
    #[prec(infix, left)]
    Not,
}

fn main() {

}
