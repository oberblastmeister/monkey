use serde::{Serialize, Deserialize};
use xshell::read_file;
use eyre::Result;

use crate::utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenDefs {
    pub tokens: Vec<TokenDef>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenDef {
    pub variant: Option<String>,
    pub text: Option<String>,
    pub ttype: TokenType,
    pub doc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType {
    Keyword,
    Literal,
    Punct,
    Token,
}

impl TokenDefs {
    pub fn get() -> Result<TokenDefs> {
        let path = utils::xtask_root().join("assets/token_def.toml");
        let contents = read_file(path)?;

        Ok(toml::from_str(&contents)?)
    }
}
