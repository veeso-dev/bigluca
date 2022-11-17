#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

use log::LevelFilter;

mod args;
mod utils;

use args::Args;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() -> anyhow::Result<()> {
    // parse arguments
    let args: Args = argh::from_env();
    // print version
    if args.version {
        anyhow::bail!("bigluca {} - developed by {}", APP_VERSION, APP_AUTHORS)
    }
    let config_dir =
        utils::dirs::init_config_dir()?.expect("your system doesn't support config directory");
    // setup logging
    let log_level = if args.debug {
        LevelFilter::Debug
    } else if args.verbose {
        LevelFilter::Info
    } else {
        LevelFilter::Off
    };
    if log_level != LevelFilter::Off {
        utils::setup_logger(log_level, &utils::dirs::get_log_path(&config_dir))?;
    }
    info!("starting bigluca {}", APP_VERSION);
    Ok(())
}
