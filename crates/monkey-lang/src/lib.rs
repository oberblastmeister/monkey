#[macro_use]
mod ast;
mod spanned;
mod parsing;

pub(crate) use spanned::{Spanned, Span};

#[cfg(test)]
mod tests {
    use std::{
        env,
        path::{Path, PathBuf},
    };

    pub fn crate_root() -> PathBuf {
        Path::new(
            &env::var("CARGO_MANIFEST_DIR")
                .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
        )
        .to_path_buf()
    }

    pub fn data_path() -> PathBuf {
        crate_root().join("src/test_data")
    }
}
