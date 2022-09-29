use convert_case::{
    Case::{Camel, Cobol, Flat, Kebab, Pascal, ScreamingSnake, Snake, Title, UpperFlat},
    Casing,
};

pub fn modify(
    input: &String,
    debug: bool,
    args: &crate::cli::structopt::opts_sub::TransformOptions,
) {
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
    // if args.reverse {
    //     println!("{}", input.to_case(ScreamingSnake));
    // }
    // if args.prefix {
    //     let result = input.to_lowercase();
    //     // println!("{}", result);
    //     result
    // }
    // if args.suffix {
    //     println!("{}", input.to_case(ScreamingSnake));
    // }
}
