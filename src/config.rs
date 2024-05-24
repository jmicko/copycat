use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub output: String,
    pub exclude: Vec<String>,
    pub include: Vec<String>,
    pub ignore_hidden: bool,
}

impl Config {
    pub fn default() -> Self {
        Config {
            output: "copycat/concatenated_codebase.md".to_string(),
            exclude: vec!["*.lock".to_string(), "*.md".to_string()],
            include: vec![],
            ignore_hidden: true,
        }
    }

    pub fn load_from_file(file_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file(&self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string(self)?;
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}
