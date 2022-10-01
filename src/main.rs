// use convert_case::{Casing, Case};


use utilities::info::type_of;

use crate::commands::case::{transform::Modifiable, case::Case};

mod cli;
mod commands;
mod functions;
mod utilities;

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn main() {
    // crate::cli::clap::git_example::opts();
    // crate::cli::clap::opts_main::opts();
    // cli::structopt::opts_main::opts();
    // let input = case_lower(String::from("Hello World"));
    // let input = "Hello World";
    // let input = case_lower(format!("Hello World"));
    let input = "Hello using case";
    type_of(&input);
    let input = "Hello-using case_+=-*&^%$#@!".to_case(Case::ScreamingSnake);
    type_of(&input);
    // println!("Lower: {}", input.case_lower);
    // println!("LowerFlat: {}", input.case_lower_flat);
    // println!("Upper: {}", input.case_upper);
    // println!("UpperFlat: {}", input.case_upper_flat);
    // println!("Pascal: {}", input.case_pascal);
    // println!("Camel: {}", input.case_camel);
    println!("{}", (input));
    // println!("{}", (input));
    // println!("{:?}", (case_lower("Hello World")));
}
