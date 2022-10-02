use crate::commands::structopt::{case::case, inspect::inspect, modify::modify, print::print};

use super::opts_sub::{PrintOptions, ModifyOptions, CaseOptions, InspectOptions};
use structopt::StructOpt;

// Main Options
#[derive(Debug, StructOpt)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
)]

pub struct Cli {
    #[structopt(long, short, global = true, help = "Prints debug information")]
    pub debug: bool,
    pub input: String,

    #[structopt(subcommand)]
    pub cmd: CommandType,
}

#[derive(Debug, StructOpt)]
pub enum CommandType {
    #[structopt(name = "print", about = "Use `echo` to print to STDOUT")]
    Print(PrintOptions),
    #[structopt(name = "modify", about = "Use `mod` to transform strings")]
    Modify(ModifyOptions),
    #[structopt(name = "case", about = "Transform string to a different case")]
    Case(CaseOptions),
    #[structopt(name = "inspect", about = "Use `insp` to inspect strings")]
    Inspect(InspectOptions),
}

#[allow(dead_code)]
pub fn opts() {
    let args = Cli::from_args();
    match args.cmd {
        CommandType::Inspect(opt) => {
            inspect(&args.input, args.debug, &opt);
        }
        CommandType::Print(opt) => {
            print(&args.input, args.debug, &opt);
        }
        CommandType::Modify(opt) => {
            modify(&args.input, args.debug, &opt);
        }
        CommandType::Case(opt) => {
            case(&args.input, args.debug, &opt);
        }
    }
}
