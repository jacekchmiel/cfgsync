use std::path::Path;

use clap::Args;

use crate::config::Config;
use crate::error::prelude::*;
use crate::git;

use super::stage::Stage;
use super::CfgSyncCommand;

#[derive(Args, Debug)]
pub struct Push;

impl CfgSyncCommand for Push {
    fn run(&self, config: &Config) -> Result<()> {
        Stage.run(config)?;

        let mut repo = git::LocalRepository::new(Path::new(&config.git_local_repo));
        repo.commit(&default_commit_message()?)?;
        repo.push()?;

        Ok(())
    }
}

fn default_commit_message() -> Result<String> {
    Ok(format!(
        "Update configs from {}",
        hostname::get()?.to_string_lossy()
    ))
}
