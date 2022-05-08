use std::path::PathBuf;

use clap::Args;

use crate::model::ConfigFile;

use super::CfgSyncCommand;

#[derive(Args, Debug)]
pub struct Rm {
    /// Path to a file to be removed from synchronization
    pub filename: String,
}

impl CfgSyncCommand for Rm {
    fn run(&self, config: &crate::config::Config) -> crate::error::Result<()> {
        let file = ConfigFile::try_from_path(self.filename.as_ref())?;
        if !config.syncset.contains(&file) {
            bail!(
                "{} not present on synchronization list. Cannot remove.",
                file
            );
        }

        config.store_modified(|c| {
            c.syncset.remove(&file);
        })
    }
}
