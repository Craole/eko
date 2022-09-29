// #![allow(dead_code, unused_imports)]

mod options;
// use regex::Regex;
use convert_case::{
    Case::{Camel, Cobol, Flat, Kebab, Pascal, ScreamingSnake, Snake, Title, UpperFlat},
    Casing,
};
use options::{opts, InspectOptions, PrintOptions, TransformOptions};

pub fn transform(input: &String, debug: bool, args: &TransformOptions) {
    if debug {
        println!("CTransform: \"{}\"", input);
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

pub fn inspect(input: &String, debug: bool, args: &InspectOptions) {
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

pub fn print(input: &String, debug: bool, args: &PrintOptions) {
    println!("Print: \"{}\"", input);
    if debug {
        println!("{:#?}", args);
    }
    if args.right {
        println!("{}", input);
    }
}

pub fn run() {
    opts();
}
