use clap::{arg, Arg, ArgAction, Command};

use crate::functions::case_convert::{lower_flat, upper_flat, upper};

fn cli() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("verbose")
                .short('d')
                .long("verbose")
                .action(ArgAction::SetTrue),
        )
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("case")
                .about("Change the case of a string")
                // .long_about("Transforms the input text to the desired case convention.")
                .args_conflicts_with_subcommands(true)
                .subcommand(
                    Command::new("lower")
                        .about("Lowercase")
                        .short_flag('l')
                        .long_flag_alias("lower")
                        .long_about("Transforms all the letters in a string to lowercase and removes whitespace.")
                        .arg(arg!([STRING])),
                ),
        )
}

pub fn opts() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("case", sub_matches)) => {
            let case_command = sub_matches.subcommand().unwrap_or(("case", sub_matches));
            match case_command {
                ("lower", sub_matches) => {
                    let input = sub_matches.get_one::<String>("STRING");
                    let input = input.expect("REASON");
                    // let output = convert_case::Casing::to_case(input, convert_case::Case::Lower);
                    lower_flat(input);
                    upper(input);
                    upper_flat(input);

                    let test = String::from("'Pop POLsa sdfsf928343&*)+_-~'");
                    upper_flat(&test);
                }
                (name, _) => {
                    println!("Default Lowercase {:?}", name);
                    // unreachable!("Unsupported subcommand `{}`", name)
                }
            }
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}
