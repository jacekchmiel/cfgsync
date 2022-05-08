use std::path::PathBuf;

use super::CfgSyncCommand;
use crate::config::Config;
use crate::model::ConfigFile;
use clap::Args;

#[derive(Args, Debug)]
pub struct Add {
    /// Path to a file to be synchronized
    pub filename: String,
}

impl CfgSyncCommand for Add {
    fn run(&self, config: &Config) -> crate::error::Result<()> {
        let file_path = ConfigFile::try_from_path(self.filename.as_ref())?;
        //FIXME:
        // config.store_modified(|c| {
        //     c.syncset.insert(file_path.clone());
        // })?;

        println!("Added {}", file_path);
        Ok(())
    }
}
