//! # Args
//!
//! Cli args

use log::LevelFilter;
use std::path::PathBuf;

use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "Please, report issues to <https://github.com/cryptopapies/bigluca>")]
pub struct Args {
    #[argh(option, short = 'c', description = "specify configuration path")]
    pub config: PathBuf,
    #[argh(
        option,
        short = 'n',
        description = "specify amount of NFTs to generate",
        default = "1"
    )]
    pub count: usize,
    #[argh(switch, short = 'D', description = "enable TRACE log level")]
    pub debug: bool,
    #[argh(
        option,
        short = 'C',
        description = "specify collection to generate NFTs for"
    )]
    pub collection: String,
    #[argh(option, short = 'd', description = "specify database path")]
    pub database_path: PathBuf,
    #[argh(option, short = 'm', description = "generate NFT from metadata file")]
    pub metadata: Option<PathBuf>,
    #[argh(option, short = 'o', description = "specify output directory")]
    pub output: PathBuf,
    #[argh(switch, description = "validate configuration and exit")]
    pub validate: bool,
    #[argh(switch, short = 'v', description = "verbose mode")]
    pub verbose: bool,
    #[argh(switch, short = 'V', description = "print version")]
    pub version: bool,
}

impl From<&Args> for LevelFilter {
    fn from(args: &Args) -> Self {
        if args.debug {
            LevelFilter::Debug
        } else if args.verbose {
            LevelFilter::Info
        } else {
            LevelFilter::Off
        }
    }
}
