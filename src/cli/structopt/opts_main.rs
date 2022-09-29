use crate::core::{inspect::check, print::echo, transform::modify};

use super::opts_sub::SubCommand;
use structopt::StructOpt;

// Main Options
#[derive(Debug, StructOpt)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
)]

pub struct Opt {
    #[structopt(long, short, global = true, help = "Prints debug information")]
    debug: bool,
    input: String,

    #[structopt(subcommand)]
    cmd: SubCommand,
}

pub fn struct_opts() {
    let args = Opt::from_args();
    match args.cmd {
        SubCommand::Inspect(opt) => {
            check(&args.input, args.debug, &opt);
        }
        SubCommand::Print(opt) => {
            echo(&args.input, args.debug, &opt);
        }
        SubCommand::Transform(opt) => {
            modify(&args.input, args.debug, &opt);
        }
    }
}
