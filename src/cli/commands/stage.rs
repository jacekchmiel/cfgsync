use std::path::Path;

use clap::Args;

use crate::config::Config;
use crate::error::*;
use crate::git;

use super::CfgSyncCommand;

#[derive(Args, Debug)]
pub struct Stage;

impl CfgSyncCommand for Stage {
    fn run(&self, config: &crate::config::Config) -> crate::error::prelude::Result<()> {
        let mut repo = init_local_repo(config)?;
        repo.checkout(&config.git_branch)?;
        repo.reset_hard()?;
        repo.pull()?;

        copy_tracked_files_to_local_repo(config)?;
        for file in &config.syncset {
            repo.add(file.as_path())?;
        }

        Ok(())
    }
}

fn init_local_repo(config: &Config) -> Result<git::LocalRepository> {
    let url = &config.git_repo_url;
    let path = Path::new(&config.git_local_repo);
    if path.exists() {
        Ok(git::LocalRepository::new(path))
    } else {
        info!("Cloning {} into {}, &config", url, path.to_string_lossy());
        git::LocalRepository::clone(url, path)
    }
}

fn copy_tracked_files_to_local_repo(config: &Config) -> Result<()> {
    let home = dirs::home_dir().expect("Home directory not present");
    let local_repo = Path::new(&config.git_local_repo);

    for file in &config.syncset {
        let src = home.join(&file.rel_path);
        let dst = local_repo.join(&file.rel_path);
        if let Some(dst_dir) = dst.parent() {
            std::fs::create_dir_all(&dst_dir)?;
        }
        info!("Saving {}", src.to_string_lossy());
        std::fs::copy(&src, &dst)?;
    }

    Ok(())
}
