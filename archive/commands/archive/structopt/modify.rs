pub fn modify(input: &String, debug: bool, args: &crate::cli::structopt::opts_sub::ModifyOptions) {
    if debug {
        println!("Modify: \"{}\"", input);
        println!("{:#?}", args);
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
