use crate::config::Config;
use crate::error::*;

pub mod add;
pub mod configure;
pub mod list;
pub mod push;
pub mod rm;
pub mod stage;

pub trait CfgSyncCommand {
    fn run(&self, config: &Config) -> Result<()>;
}
