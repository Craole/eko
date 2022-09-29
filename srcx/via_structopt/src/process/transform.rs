#![allow(dead_code, unused_imports)]
use structopt::StructOpt;
use clap::Parser;

use convert_case::{
    Case::{Camel, Cobol, Flat, Kebab, Pascal, ScreamingSnake, Snake, Title, UpperFlat},
    Casing,
};

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
    #[structopt(short = "k", long, help = "Lowercase and delimited by hyphen (-).")]
    pub kebab: bool,
    #[structopt(short = "K", long, help = "Uppercase and delimited by hyphen (-).")]
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

pub fn modify(input: &String, debug: bool, args: &TransformOptions) {
    if debug {
        println!("Transform: \"{}\"", input);
        println!("{:#?}", args);
    }
    if args.lower {
        println!("{}", input.to_case(Flat));
    }
    if args.upper {
        println!("{}", input.to_case(UpperFlat));
    }
    if args.title {
        println!("{}", input.to_case(Title));
    }
    if args.pascal {
        println!("{}", input.to_case(Pascal));
    }
    if args.camel {
        println!("{}", input.to_case(Camel));
    }
    if args.kebab {
        println!("{}", input.to_case(Kebab));
    }
    if args.cobol {
        println!("{}", input.to_case(Cobol));
    }
    if args.snake {
        println!("{}", input.to_case(Snake));
    }
    if args.screamingsnake {
        println!("{}", input.to_case(ScreamingSnake));
    }
}
