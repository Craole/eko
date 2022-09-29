#![allow(dead_code, unused_imports)]
use structopt::StructOpt;

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



pub fn check(input: &String, debug: bool, args: &InspectOptions) {
    println!("Inspect: \"{}\"", input);
    if debug {
        println!("{:#?}", args);
    }
    if args.characters {
        println!("{}", input.chars().count());
    }
    if args.numbers {
        println!("{:#?}", input.matches(char::is_numeric).count());
    }
    if args.spaces {
        println!("{}", input.matches(char::is_whitespace).count());
    }
    if args.lowercase {
        println!("{}", input.matches(char::is_lowercase).count());
    }
    if args.uppercase {
        println!("{}", input.matches(char::is_uppercase).count());
    }
}