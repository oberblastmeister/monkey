use monkey_util::Precedent;

#[derive(Precedent)]
pub enum Op {
    #[prec(infix, left)]
    Add,

    #[prec(infix, left)]
    Sub,

    #[prec(infix, left)]
    Mul,

    #[prec(postfix)]
    Combine,

    #[prec(infix, right)]
    Power,

    #[prec(prefix)]
    Not,
}

fn main() {

}
