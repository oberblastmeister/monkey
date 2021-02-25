mod codegen;
mod utils;
mod token_def;

use eyre::Result;

fn main() -> Result<()> {
    codegen::run()
}
