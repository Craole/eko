#![allow(dead_code)]

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    #[structopt(name = "case", about = "Use trans to transform strings")]
    Case(CaseOptions),
    #[structopt(name = "trans", about = "Use trans to transform strings")]
    Transform(TransformOptions),
    #[structopt(name = "insp", about = "Use insp to inspect strings")]
    Inspect(InspectOptions),
}

#[derive(Debug, StructOpt)]
pub struct CaseOptions {
    #[structopt(short, long, help = "Transforms a string to uppercase")]
    upper: bool,
    #[structopt(short, long, help = "Transforms a string to lowercase")]
    lower: bool,
    #[structopt(short, long, help = "Transforms a string to title case")]
    title: bool,
}

#[derive(Debug, StructOpt)]
pub struct TransformOptions {
    #[structopt(short, long, help = "Reverses a string")]
    reverse: bool,
    #[structopt(
        short = "pre",
        long,
        help = "Adds a prefix to the string",
        env = "STRINGS__PREFIX"
    )]
    prefix: Option<String>,
    #[structopt(
        short = "suf",
        long,
        help = "Adds a suffix to the string",
        env = "STRINGS__SUFFIX"
    )]
    suffix: Option<String>,
}

#[derive(Debug, StructOpt)]
pub struct InspectOptions {
    #[structopt(short, long, help = "Count all characters in the string")]
    length: bool,
    #[structopt(short, long, help = "Count only numbers in the given string")]
    numbers: bool,
    #[structopt(short, long, help = "Count all spaces in the string")]
    spaces: bool,
}

pub fn transform(input: &String, debug: bool, args: &TransformOptions) {
    println!("Transform called for {}", input);
    if debug {
        println!("{:#?}", args);
    }
}

pub fn case(input: &String, debug: bool, args: &CaseOptions) {
    println!("Case called for {}", input);
    if debug {
        println!("{:#?}", args);
    }
}

pub fn inspect(input: &String, debug: bool, args: &InspectOptions) {
    println!("Inspect called for {}", input);
    if debug {
        println!("{:#?}", args);
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "eko",
    author = "Craole <iamcraole@gmail.com>",
    about = "Transform or inspect strings"
)]
struct Opt {
    #[structopt(long, short, global = true, help = "Prints debug information")]
    debug: bool,
    input: String,

    #[structopt(subcommand)]
    cmd: SubCommand,
}

pub fn run() {
    let args = Opt::from_args();
    match args.cmd {
        SubCommand::Inspect(opt) => {
            inspect(&args.input, args.debug, &opt);
        }
        SubCommand::Case(opt) => {
            case(&args.input, args.debug, &opt);
        }
        SubCommand::Transform(opt) => {
            transform(&args.input, args.debug, &opt);
        }
    }
}
