use clap::Args;

use super::CfgSyncCommand;

#[derive(Args, Debug)]
pub struct List;

impl CfgSyncCommand for List {
    fn run(&self, config: &crate::config::Config) -> crate::error::Result<()> {
        for f in &config.syncset {
            println!("{}", f);
        }

        Ok(())
    }
}
