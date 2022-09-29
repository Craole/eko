use structopt::StructOpt;

use crate::via_structopt::process::{
    inspect::InspectOptions, print::PrintOptions, transform::TransformOptions,
};

// Subcommands
#[derive(Debug, StructOpt)]
pub enum SubCommand {
    #[structopt(name = "echo", about = "Use `echo` to print to STDOUT")]
    Print(PrintOptions),
    #[structopt(name = "mod", about = "Use `mod` to transform strings")]
    Transform(TransformOptions),
    #[structopt(name = "insp", about = "Use `insp` to inspect strings")]
    Inspect(InspectOptions),
}
