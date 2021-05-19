use monkey_lang::Parse;

#[derive(Parse)]
pub enum Enum {
    First,
    Second(u32),
}

fn main() {}
