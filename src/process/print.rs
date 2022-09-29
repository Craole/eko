#![allow(dead_code, unused_imports)]
use structopt::StructOpt;

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

pub fn echo(input: &String, debug: bool, args: &PrintOptions) {
    println!("Print: \"{}\"", input);
    if debug {
        println!("{:#?}", args);
    }
    if args.right {
        println!("{}", input);
    }
}