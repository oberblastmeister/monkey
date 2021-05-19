use monkey_lang::{ast, Parse, T};

#[derive(Parse)]
struct ReturnLit {
    return_token: T![return],
    lit: ast::Lit,
}

fn main() {}
