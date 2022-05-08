#[macro_use]
extern crate log;

use cfgsync;
use clap::Parser;

fn main() -> cfgsync::Result<()> {
    let cli = cfgsync::Cli::parse();
    let config = cfgsync::Config::load()?;
    pretty_env_logger::init();

    debug!("Cli: {:?}", cli);
    debug!("Config: {:?}", config);

    cli.command.run(&config)
}
