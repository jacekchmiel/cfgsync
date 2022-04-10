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
// #[clap(PARENT APP ATTRIBUTE)]
pub enum Command {
    /// Add file to synchronization
    Add(commands::add::Add),
    List(commands::list::List),
    Rm(commands::rm::Rm),
}

impl Command {
    pub fn run(&self, config: &Config) -> Result<()> {
        match self {
            Command::Add(c) => c.run(config),
            Command::List(c) => c.run(config),
            Command::Rm(c) => c.run(config),
        }
    }
}
