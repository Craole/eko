use clap::{Args};

#[derive(Debug, Args)]
pub enum CaseCommand {

    Lower {
        ///     #[structopt(short = "l", long, help = "Transforms a string to lowercase")]
        pub lower: bool,
    }

    // ///
    // /// "Transforms a string to uppercase")]
    // pub upper: bool,

    // // "First letter of each compound word is capitalized, delimited by space"
    // pub title: bool,
    // //     #[structopt(
    // //         short = "p",
    // //         long,
    // //         help = "First letter of each compound word is capitalized"
    // //     )]
    // pub pascal: bool,
    // //     #[structopt(
    // //         short = "c",
    // //         long,
    // //         help = "After the first word, the first letter of each compound word is capitalized."
    // //     )]
    // pub camel: bool,
    // //     #[structopt(short = "k", long, help = "Lowercase and delimited by hyphen (-).")]
    // //     pub kebab: bool,
    // //     #[structopt(short = "K", long, help = "Uppercase and delimited by hyphen (-).")]
    // pub cobol: bool,
    // //     #[structopt(
    // //         short = "s",
    // //         long,
    // //         help = "Lowercase and delimited by underscores (_)."
    // //     )]
    // pub snake: bool,
    // //     #[structopt(
    // //         short = "S",
    // //         long,
    // //         help = "Uppercase and delimited by underscores (_)."
    // //     )]
    // pub screamingsnake: bool,
    //     // #[structopt(short, long, help = "Reverses a string")]
    //     // pub reverse: bool,
    //     // #[structopt(
    //     //     // short = "pre",
    //     //     long,
    //     //     help = "Adds a prefix to the string",
    //     //     env = "STRINGS__PREFIX"
    //     // )]
    //     // pub prefix: Option<String>,
    //     // #[structopt(
    //     //     // short = "suf",
    //     //     long,
    //     //     help = "Adds a suffix to the string",
    //     //     env = "STRINGS__SUFFIX"
    //     // )]
    //     // pub suffix: Option<String>,
}
