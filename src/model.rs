use core::fmt;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(from = "String")]
#[serde(into = "String")]
pub struct ConfigFile {
    rel_path: PathBuf,
}

impl fmt::Display for ConfigFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "~/{}", self.rel_path.to_string_lossy())
    }
}

impl ConfigFile {
    pub fn try_from_path(rel_path: PathBuf) -> Result<Self> {
        if rel_path.is_absolute() {
            if let Some(home) = dirs::home_dir() {
                if rel_path.starts_with(&home) {
                    Ok(ConfigFile {
                        rel_path: rel_path.strip_prefix(&home)?.to_owned(),
                    })
                } else {
                    Err(anyhow!(
                        "Absolute path outside of home directory not supported"
                    ))
                }
            } else {
                Err(anyhow!("Absolute path not supported"))
            }
        } else {
            Ok(ConfigFile { rel_path })
        }
    }
}

impl From<String> for ConfigFile {
    fn from(value: String) -> Self {
        ConfigFile {
            rel_path: value.into(),
        }
    }
}

impl From<ConfigFile> for String {
    fn from(value: ConfigFile) -> Self {
        value.rel_path.to_string_lossy().into_owned()
    }
}
