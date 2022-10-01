pub fn print(input: &String, debug: bool, args: &crate::cli::structopt::opts_sub::PrintOptions) {
    println!("Print: \"{}\"", input);
    if debug {
        println!("{:#?}", args);
    }
    if args.right {
        println!("{}", input);
    }
}
