use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub const CONFIG_FILE_NAME: &str = "dotfiles.toml";
pub const DEFAULT_DOTFILES_DIR: &str = "dotfiles";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub dotfiles_dir: PathBuf,
    pub files: HashMap<String, PathBuf>,
}

impl Config {
    pub fn load(dotfiles_dir: &Path) -> Result<Self> {
        let config_path = dotfiles_dir.join(CONFIG_FILE_NAME);
        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file at {}", config_path.display()))?;
        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file at {}", config_path.display()))?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = self.dotfiles_dir.join(CONFIG_FILE_NAME);
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config to TOML format.")?;
        fs::write(&config_path, content)
            .with_context(|| format!("Failed to write config file to {}", config_path.display()))?;
        Ok(())
    }
}