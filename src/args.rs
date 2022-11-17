//! # Args
//!
//! Cli args

use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "Please, report issues to <https://github.com/cryptopapies/bigluca>")]
pub struct Args {
    #[argh(switch, short = 'D', description = "enable TRACE log level")]
    pub debug: bool,
    #[argh(switch, short = 'v', description = "verbose mode")]
    pub verbose: bool,
    #[argh(switch, short = 'V', description = "print version")]
    pub version: bool,
}
