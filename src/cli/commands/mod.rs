use crate::config::Config;
use crate::error::*;

pub mod add;
pub mod list;
pub mod rm;

pub trait CfgSyncCommand {
    fn run(&self, config: &Config) -> Result<()>;
}
