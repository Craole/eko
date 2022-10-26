// use clap::{arg, Subcommand};

// #[derive(Debug, Subcommand)]
// pub enum CaseCommands {
//     /// Lowercase
//     Lower {
//         #[arg(short, long)]
//         input: Option<String>,
//     },
//     //     // Upper { input: Option<String> },
//     //     // Flat { input: Option<String> },
//     //     // UpperFlat { input: Option<String> },
//     //     // Pascal { input: Option<String> },
//     //     // Camel { input: Option<String> },
//     //     // Snake { input: Option<String> },
//     //     // ScreamingSnake { input: Option<String> },
// }

// use clap::Args;

// #[derive(Debug, Args)]
// pub struct Case {
//     /// Lowercase
//     #[arg(short, long)]
//     lower:bool,

//     /// Uppercase
//     #[arg(short, long)]
//     upper:bool,
// }

// pub enum Input {

// }

use clap::ValueEnum;

// #[derive(Debug, Args)]
// #[command(args_conflicts_with_subcommands = true)]
// pub struct Case {
//     // #[command(subcommand)]
//     // command: Option<CaseCommands>,
//     #[arg(
//       short,
//       long,
//       require_equals = true,
//       value_name = "Lower",
//       num_args = 0..=1,
//       default_value_t = Style::Flat,
//       default_missing_value = "flat",
//       value_enum
//   )]
//     lower: Style,
//     // #[command(flatten)]
//     // push: CasePush,
// }

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum CaseType {
    Flat,
    Delimited,
}

impl std::fmt::Display for CaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}
