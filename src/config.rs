use serde::Deserialize;

use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    pub repo_count: usize,
    pub exclude_chars: Vec<char>,
    pub languages: Vec<Language>,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(file: &P) -> crate::Result<Self> {
        let raw = fs::read_to_string(file)?;
        let config = toml::de::from_str(&raw)?;
        Ok(config)
    }
}

#[derive(Deserialize)]
pub struct Language {
    pub id: String,
    pub extensions: Vec<String>,
    pub exclude_repos: Vec<String>,
}
