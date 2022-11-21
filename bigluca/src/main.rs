#[macro_use]
extern crate bigluca_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

use log::LevelFilter;

mod args;
mod config;
mod generator;
mod nft;
mod utils;

use args::Args;

use crate::config::{Configuration, Validate};

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() -> anyhow::Result<()> {
    // parse arguments
    let args: Args = argh::from_env();
    // print version
    if args.version {
        anyhow::bail!("bigluca {} - developed by {}", APP_VERSION, APP_AUTHORS)
    }
    // setup logging
    let log_level = if args.debug {
        LevelFilter::Debug
    } else if args.verbose {
        LevelFilter::Info
    } else {
        LevelFilter::Off
    };
    if log_level != LevelFilter::Off {
        env_logger::builder().filter_level(log_level).init();
    }
    info!("starting bigluca {}", APP_VERSION);
    info!("parsing configuration at {}", args.config.display());
    let configuration = Configuration::parse(&args.config)?;
    debug!("validating configuration");
    configuration.validate()?;
    info!("configuration is valid");

    Ok(())
}
