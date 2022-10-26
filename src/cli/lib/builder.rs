use clap::{arg, Arg, ArgAction, Command};

use crate::functions::case::lib::{transform::Modifiable, case::Case};

fn cli() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .args_conflicts_with_subcommands(true)
        .arg(
            Arg::new("verbose")
                .short('d')
                .long("verbose")
                .action(ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("case")
                .about("Change the case of a string")
                .args_conflicts_with_subcommands(true)
                .long_about("Transforms the input text to the desired case convention.")
                .subcommand(
                    Command::new("lower")
                        .about("Lowercase")
                        .short_flag('l')
                        .long_flag_alias("lowercase")
                        .long_about("Transforms all the letters in a string to uppercase.")
                        .arg(arg!([STRING])),
                )
                .subcommand(
                    Command::new("flat")
                        .about("Lowercase and without whitespace")
                        .short_flag('f')
                        .long_flag_alias("lower-flat")
                        .long_about("Transform all characters to lowercase and remove whitespace.")
                        .arg(arg!([STRING])),
                )
                .subcommand(
                    Command::new("upper")
                        .about("Uppercase")
                        .short_flag('u')
                        .long_flag_alias("upper")
                        .long_about("Transforms all the letters in a string to uppercase.")
                        .arg(arg!([STRING])),
                )
                .subcommand(
                    Command::new("flatupper")
                        .about("Lowercase and without whitespace")
                        .short_flag('U')
                        .long_flag_alias("upper-flat")
                        .long_about("Transform all characters to uppercase and remove whitespace.")
                        .arg(arg!([STRING])),
                ),
        )
}

#[allow(dead_code)]
pub fn args() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("case", sub_matches)) => {
            let case_command = sub_matches.subcommand().unwrap_or(("case", sub_matches));
            match case_command {
                ("lower", sub_matches) => {
                    let input = sub_matches.get_one::<String>("STRING").expect("REASON");
                    let output = input.to_case(Case::Lower);
                    println!("Input   |>  {}", input);
                    println!("Output  |>  {}", output);
                }
                ("flat", sub_matches) => {
                    let input = sub_matches.get_one::<String>("STRING").expect("REASON");
                    let output = input.to_case(Case::Flat);
                    println!("Input   |>  {}", input);
                    println!("Output  |>  {}", output);
                }
                ("upper", sub_matches) => {
                    let input = sub_matches.get_one::<String>("STRING").expect("REASON");
                    let output = input.to_case(Case::Upper);
                    println!("Input   |>  {}", input);
                    println!("Output  |>  {}", output);
                }
                ("flatupper ", sub_matches) => {
                    let input = sub_matches.get_one::<String>("STRING").expect("REASON");
                    let output = input.to_case(Case::UpperFlat);
                    println!("Input   |>  {}", input);
                    println!("Output  |>  {}", output);
                }
                (name, _) => {
                    unreachable!("Unsupported subcommand `{}`", name)
                }
            }
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}