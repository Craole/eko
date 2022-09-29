#![allow(dead_code, unused_imports)]
use structopt::StructOpt;

use crate::{inspect, print, transform};

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

// Subcommand - Modify
#[derive(Debug, StructOpt)]
pub struct TransformOptions {
    #[structopt(short = "l", long, help = "Transforms a string to lowercase")]
    pub lower: bool,
    #[structopt(short = "u", long, help = "Transforms a string to uppercase")]
    pub upper: bool,
    #[structopt(
        short = "t",
        long,
        help = "First letter of each compound word is capitalized, delimited by space"
    )]
    pub title: bool,
    #[structopt(
        short = "p",
        long,
        help = "First letter of each compound word is capitalized"
    )]
    pub pascal: bool,
    #[structopt(
        short = "c",
        long,
        help = "After the first word, the first letter of each compound word is capitalized."
    )]
    pub camel: bool,
    #[structopt(
        short = "k",
    long,
    help = "Lowercase and delimited by hyphen (-)."
)]
    pub kebab: bool,
    #[structopt(
        short = "K",
    long,
    help = "Uppercase and delimited by hyphen (-)."
)]
    pub cobol: bool,
    #[structopt(
        short = "s",
        long,
        help = "Lowercase and delimited by underscores (_)."
    )]
    pub snake: bool,
    #[structopt(
        short = "S",
        long,
        help = "Uppercase and delimited by underscores (_)."
    )]
    pub screamingsnake: bool,
    #[structopt(short, long, help = "Reverses a string")]
    reverse: bool,
    #[structopt(
        // short = "pre",
        long,
        help = "Adds a prefix to the string",
        env = "STRINGS__PREFIX"
    )]
    prefix: Option<String>,
    #[structopt(
        // short = "suf",
        long,
        help = "Adds a suffix to the string",
        env = "STRINGS__SUFFIX"
    )]
    suffix: Option<String>,
}

// Subcommand - Inspect
#[derive(Debug, StructOpt)]
pub struct InspectOptions {
    #[structopt(short, long, help = "Count all characters in the string")]
    pub characters: bool,
    #[structopt(short, long, help = "Count numbers")]
    pub numbers: bool,
    #[structopt(short, long, help = "Count all spaces in the string")]
    pub spaces: bool,
    #[structopt(short, long, help = "Count lowercase letters")]
    pub lowercase: bool,
    #[structopt(short, long, help = "Count uppercase letters")]
    pub uppercase: bool,
}

// Subcommand - Print
#[derive(Debug, StructOpt)]
pub struct PrintOptions {
    #[structopt(short = "lf", long, help = "Print to the left")]
    pub left: bool,
    #[structopt(short = "right", long, help = "Print to the right")]
    pub right: bool,
    #[structopt(short = "cen", long, help = "Print in the center")]
    pub center: bool,
}

pub fn opts() {
    let args = Opt::from_args();
    match args.cmd {
        SubCommand::Inspect(opt) => {
            inspect(&args.input, args.debug, &opt);
        }
        SubCommand::Print(opt) => {
            print(&args.input, args.debug, &opt);
        }
        SubCommand::Transform(opt) => {
            transform(&args.input, args.debug, &opt);
        }
    }
}
