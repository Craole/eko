pub fn check(input: &String, debug: bool, args: &crate::cli::structopt::opts_sub::InspectOptions) {
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
