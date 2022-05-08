use crate::error::*;
use crate::model::ConfigFile;
use std::collections::BTreeSet;
use std::fs::File;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_git_branch")]
    pub git_branch: String,
    #[serde(default = "default_git_local_repo")]
    pub git_local_repo: String,
    #[serde(default = "default_git_repo_url")]
    pub git_repo_url: String,
    #[serde(default)]
    pub syncset: BTreeSet<ConfigFile>,
}

impl Config {
    fn default_location() -> Result<PathBuf> {
        dirs::config_dir()
            .map(|p| p.join(".cfgsync"))
            .ok_or_else(|| anyhow!("Home dir not available"))
    }

    pub fn try_from_file(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let config: Self = serde_yaml::from_reader(file)?;
        Ok(config)
    }

    pub fn load() -> Result<Config> {
        let path = Config::default_location()?;
        Self::load_with_path(&path)
    }

    pub fn load_with_path(path: &Path) -> Result<Config> {
        if path.exists() {
            Self::try_from_file(&path)
        } else {
            let config = Config::default();
            config.store()?;
            Ok(config)
        }
    }

    pub fn store_modified<F>(&self, modify: F) -> Result<()>
    where
        F: FnOnce(&mut Config),
    {
        let mut cloned = self.clone();
        modify(&mut cloned);
        cloned.store()
    }

    pub fn store(&self) -> Result<()> {
        // TODO: atomic change?
        let file = File::create(Config::default_location()?)?;
        serde_yaml::to_writer(file, self)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            git_branch: default_git_branch(),
            git_local_repo: default_git_local_repo(),
            git_repo_url: default_git_repo_url(),
            syncset: Default::default(),
        }
    }
}

fn default_git_branch() -> String {
    "main".into()
}

fn default_git_local_repo() -> String {
    "~/.cfgsync.git".into()
}

fn default_git_repo_url() -> String {
    "https://example.com/your/repo".into()
}
