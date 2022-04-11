pub mod commands;

use clap::{Parser, Subcommand};

use crate::config::Config;
use crate::error::*;
use commands::CfgSyncCommand;

///Synchronize/Backup your configuration files
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Name of the person to greet
    #[clap(subcommand)]
    pub command: Command,
}

/// Command to execute
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Add file to synchronization set
    Add(commands::add::Add),
    /// List synchronization set contents
    List(commands::list::List),
    /// Remove file from synchronization set
    Rm(commands::rm::Rm),
    /// Override cfgsync own configuration param
    Configure(commands::configure::Configure),
    /// Push current config files to remote repository
    Push(commands::push::Push),
    /// Stage changes for commit, useful for debugging
    Stage(commands::stage::Stage),
}

impl Command {
    pub fn run(&self, config: &Config) -> Result<()> {
        match self {
            Command::Add(c) => c.run(config),
            Command::List(c) => c.run(config),
            Command::Rm(c) => c.run(config),
            Command::Configure(c) => c.run(config),
            Command::Push(c) => c.run(config),
            Command::Stage(c) => c.run(config),
        }
    }
}
