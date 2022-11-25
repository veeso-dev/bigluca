#[macro_use]
extern crate bigluca_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

use log::LevelFilter;
use std::str::FromStr;

mod app;
mod args;
mod config;
mod database;
mod generator;
mod nft;
mod render;
mod utils;

use args::Args;

use crate::{
    app::App,
    config::{Configuration, Validate},
    database::nft::NftDatabase,
    generator::Collection,
};

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
    let log_level = LevelFilter::from(&args);
    if log_level != LevelFilter::Off {
        env_logger::builder().filter_level(log_level).init();
    }
    info!("starting bigluca {}", APP_VERSION);

    let collection = Collection::from_str(&args.collection)?;
    info!("working on collection: {}", collection.to_string());

    // validate config
    info!("parsing configuration at {}", args.config.display());
    let configuration = Configuration::parse(&args.config)?;
    debug!("validating configuration");
    configuration.validate()?;
    info!("configuration is valid");
    if args.validate {
        return Ok(());
    }

    info!("loading database from {}", args.database_path.display());
    let database = if args.database_path.exists() {
        NftDatabase::load(&args.database_path)
    } else {
        debug!("database path doesn't exist, loading default db");
        Ok(NftDatabase::default())
    }?;
    info!("database loaded");

    App::new(
        collection,
        configuration,
        database,
        args.database_path,
        args.output,
    )
    .run(args.count)
}
