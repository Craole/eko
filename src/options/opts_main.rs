use structopt::StructOpt;

use crate::process::{inspect::check, print::echo, transform::modify};

use super::opts_sub::SubCommand;

// use super::{inspect::check, opts_sub::SubCommand, print::echo, transform::modify};

// Main Options
#[derive(Debug, StructOpt)]
#[structopt(
    name = "eko",
    author = "Craole <iamcraole@gmail.com>",
    about = "Transform or inspect strings"
)]

pub struct Opt {
    #[structopt(long, short, global = true, help = "Prints debug information")]
    debug: bool,
    input: String,

    #[structopt(subcommand)]
    cmd: SubCommand,
}

pub fn opts() {
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
