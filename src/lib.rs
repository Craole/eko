#![allow(dead_code)]
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    #[structopt(name = "mod", about = "Use mod to modify strings")]
    Modify(ModifyOptions),
    #[structopt(name = "insp", about = "Use insp to inspect strings")]
    Inspect(InspectOptions),
}

#[derive(Debug, StructOpt)]
pub struct ModifyOptions {
    #[structopt(short, long, help = "Transforms a string to uppercase")]
    upper: bool,
    #[structopt(short, long, help = "Transforms a string to lowercase")]
    lower: bool,
    #[structopt(short, long, help = "Reverses a string")]
    reverse: bool,
    #[structopt(
        short = "pref",
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

// impl ModifyOptions {
//     pub fn modify(input: &String, debug: bool, args: &ModifyOptions) {
//         println!("Inspect called for {}", input);
//         if debug {
//             println!("{:#?}", args);
//         }
//     }
// }

#[derive(Debug, StructOpt)]
pub struct InspectOptions {
    #[structopt(short, long, help = "Count all characters in the string")]
    length: bool,
    #[structopt(short, long, help = "Count only numbers in the given string")]
    numbers: bool,
    #[structopt(short, long, help = "Count all spaces in the string")]
    spaces: bool,
}

// impl InspectOptions {
//     pub fn inspect(input: &String, debug: bool, args: &InspectOptions) {
//         println!("Inspect called for {}", input);
//         if debug {
//             println!("{:#?}", args);
//         }
//     }
// }

pub fn modify(input: &String, debug: bool, args: &ModifyOptions) {
  println!("Inspect called for {}", input);
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
