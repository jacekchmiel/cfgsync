#[macro_use]
extern crate log;

#[macro_use]
extern crate anyhow;

pub mod cli;
pub mod config;
pub mod error;
pub mod git;
pub mod model;

use std::path::{Path, PathBuf};
use std::str::FromStr;

pub use cli::Cli;
pub use config::Config;
pub use error::Result;

use crate::model::ConfigFile;

#[derive(Debug)]
pub struct CfgSync {
    config: Config,
}

impl CfgSync {
    pub fn new(config: Config) -> CfgSync {
        CfgSync { config }
    }

    pub fn add<S: AsRef<str>>(&mut self, filename: S) -> Result<()> {
        let file_path = shellexpand::tilde(filename.as_ref());
        let file_path = ConfigFile::try_from_path(&PathBuf::from_str(file_path.as_ref()).unwrap())?;
        self.config.syncset.insert(file_path);

        Ok(())
    }

    pub fn list(&self) -> Vec<String> {
        self.config
            .syncset
            .iter()
            .map(ToString::to_string)
            .collect()
    }
}
