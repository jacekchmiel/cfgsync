use crate::error::*;
use std::fs::File;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub git_branch: String,
    pub items: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            git_branch: String::from("main"),
            items: Default::default(),
        }
    }
}

impl Config {
    fn default_location() -> Result<PathBuf> {
        dirs::config_dir()
            .map(|p| p.join(".cfgsync"))
            .ok_or_else(|| anyhow!("Home dir not available"))
    }

    fn try_from_file(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let config: Self = serde_yaml::from_reader(file)?;
        Ok(config)
    }

    pub fn load() -> Result<Config> {
        let path = Config::default_location()?;
        if path.exists() {
            Self::try_from_file(&path)
        } else {
            let config = Config::default();
            config.store()?;
            Ok(config)
        }
    }

    pub fn store(&self) -> Result<()> {
        // TODO: atomic change?
        let file = File::create(Config::default_location()?)?;
        serde_yaml::to_writer(file, self)?;
        Ok(())
    }
}
