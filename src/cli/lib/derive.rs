// use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use crate::cli::commands::case::lib::CaseType;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Show detailed information
    #[arg(short = 'd', long)]
    pub verbose: bool,

    /// Accept text in the form of a STRING or FILE
    #[arg(short, long, group = "input")]
    pub text: Option<String>,

    /// Transform the case of the text
    #[arg(
        short,
        long,
        requires = "input",
        require_equals = true,
        value_name = "Case",
        num_args = 0..=1,
        default_value_t = CaseType::Flat,
        default_missing_value = "flat",
        value_enum
    )]
    pub case: CaseType,
}

// #[derive(Debug, Subcommand)]
// pub enum Commands {
//     /// Select desired case
//     // Case(Case),
// }

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Input {
    Text,
    File,
}

// /// Optional name to operate on
// #[arg(short, long, value_name = "TEXT")]
// text: Option<String>,

// /// Sets a custom config file
// #[arg(short, long, value_name = "FILE")]
// file: Option<PathBuf>,

#[allow(dead_code)]
pub fn args() {
    let args = Cli::parse();
    // println!("text: {:?}", args.input);
    // let data = args.input.unwrap().collect();

    // if let Some(matches) = matches.subcommand_matches("use") {
    //     let files: Vec<_> = matches.values_of("files").unwrap().collect();
    //     println!("{}", files[0]);
    //     println!("{}", files[1]);

    match args.case {
        CaseType::Flat => {
            println!("{}", "FLAT")
        }
        CaseType::Delimited => {
            println!("{}", "DELIM")
        }
    }

    // match args.command {
    //     Commands::Clone { remote } => {
    //         println!("Cloning {}", remote);
    //     }
    //     Commands::Case {
    //         mut base,
    //         mut head,
    //         mut path,
    //         style,
    //     } => {
    //         if path.is_none() {
    //             path = head;
    //             head = None;
    //             if path.is_none() {
    //                 path = base;
    //                 base = None;
    //             }
    //         }
    //         let base = base
    //             .as_deref()
    //             .map(|s| s.to_str().unwrap())
    //             .unwrap_or("stage");
    //         let head = head
    //             .as_deref()
    //             .map(|s| s.to_str().unwrap())
    //             .unwrap_or("worktree");
    //         let path = path.as_deref().unwrap_or_else(|| OsStr::new(""));
    //         println!(
    //             "Caseing {}..{} {} (style={})",
    //             base,
    //             head,
    //             path.to_string_lossy(),
    //             style
    //         );
    //     }
    //     Commands::Push { remote } => {
    //         println!("Pushing to {}", remote);
    //     }
    //     Commands::Add { path } => {
    //         println!("Adding {:?}", path);
    //     }
    //     Commands::Stash(stash) => {
    //         let stash_cmd = stash.command.unwrap_or(StashCommands::Push(stash.push));
    //         match stash_cmd {
    //             StashCommands::Push(push) => {
    //                 println!("Pushing {:?}", push);
    //             }
    //             StashCommands::Pop { stash } => {
    //                 println!("Popping {:?}", stash);
    //             }
    //             StashCommands::Apply { stash } => {
    //                 println!("Applying {:?}", stash);
    //             }
    //         }
    //     }
    //     Commands::External(args) => {
    //         println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);
    //     }
    // }

    // Continued program logic goes here...
}
