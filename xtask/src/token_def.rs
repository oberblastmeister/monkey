use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};
use xshell::read_file;
use eyre::Result;

use crate::utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenDef {
    pub keywords: Vec<String>,
    pub literals: Vec<String>,
    pub punct: BTreeMap<String, String>,
}

impl TokenDef {
    pub fn get() -> Result<TokenDef> {
        let path = utils::xtask_root().join("assets/token_def.toml");
        let contents = read_file(path)?;

        Ok(toml::from_str(&contents)?)
    }
}
