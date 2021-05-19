use monkey_lang::{ast, Parse, T};

struct DoesNotImplement;

#[derive(Parse)]
struct Fail {
    hello: DoesNotImplement,
}

fn main() {}
