#[macro_use]
extern crate log;

mod config;
mod error;

use config::Config;
use error::*;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let config = Config::load()?;
    info!("{:?}", config);
    Ok(())
}
