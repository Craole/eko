#![allow(dead_code,unused_imports)]
mod options;
use convert_case::{
    Case::{
        Camel, Cobol, Flat, Kebab, Lower, Pascal, ScreamingSnake, Snake, Title, Toggle, Train,
        Upper, UpperCamel, UpperFlat, UpperSnake,
    },
    Casing,
};
use options::{opts, CaseOptions};

pub fn change_case(input: &String, debug: bool, args: &CaseOptions) {
// pub fn change_case() {
    if debug {
        println!("Case called for {}", input);
        println!("{:#?}", args);
    };
    if args.lower {
        println!("{}", input.to_case(Lower));
        println!("{}", input.to_lowercase())
    }
    if args.upper {
        println!("{}", input.to_case(Upper));
        println!("{}", input.to_uppercase())
    }
}