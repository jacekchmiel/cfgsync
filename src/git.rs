use std::path::{Path, PathBuf};

use duct::cmd;

use crate::error::*;

pub struct LocalRepository {
    path: PathBuf,
}

impl LocalRepository {
    pub fn new(path: &Path) -> Self {
        LocalRepository {
            path: path.to_path_buf(),
        }
    }

    pub fn clone(url: &str, path: &Path) -> Result<Self> {
        cmd!("git", "clone", url, path).run()?;
        Ok(LocalRepository::new(path))
    }

    pub fn run(&mut self, cmd: duct::Expression) -> Result<()> {
        cmd.dir(self.path.clone()).run()?;
        Ok(())
    }

    pub fn checkout(&mut self, commit: &str) -> Result<()> {
        self.run(cmd!("git", "checkout", commit))
    }

    pub fn reset_hard(&mut self) -> Result<()> {
        self.run(cmd!("git", "reset", "--hard"))
    }

    pub(crate) fn pull(&mut self) -> Result<()> {
        self.run(cmd!("git", "pull"))
    }

    pub fn add(&mut self, pathspec: &Path) -> Result<()> {
        self.run(cmd!("git", "add", pathspec))
    }

    pub fn commit(&mut self, message: &str) -> Result<()> {
        self.run(cmd!("git", "commit", "-m", message))
    }

    pub fn push(&mut self) -> Result<()> {
        self.run(cmd!("git", "push"))
    }
}
