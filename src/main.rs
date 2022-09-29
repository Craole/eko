#![allow(dead_code)]

use eko::{inspect, modify, SubCommand};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "eko",
    author = "Craole <iamcraole@gmail.com>",
    about = "Manipulate and/or inspect strings"
)]
struct Opt {
    #[structopt(long, short, global = true, help = "Prints debug information")]
    debug: bool,
    input: String,

    #[structopt(subcommand)]
    cmd: SubCommand,
}

fn main() {
    let args = Opt::from_args();
    match args.cmd {
        SubCommand::Inspect(opt) => {
            inspect(&args.input, args.debug, &opt);
        }
        SubCommand::Modify(opt) => {
            modify(&args.input, args.debug, &opt);
        }
    }
}
