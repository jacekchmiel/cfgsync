#[macro_use]
extern crate log;

#[macro_use]
extern crate anyhow;

mod cli;
mod config;
mod error;
mod git;
mod model;

use clap::Parser;
use error::*;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    let config = config::Config::load()?;
    pretty_env_logger::init();

    debug!("Cli: {:?}", cli);
    debug!("Config: {:?}", config);

    cli.command.run(&config)
}
