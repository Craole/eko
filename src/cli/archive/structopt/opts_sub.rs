use structopt::StructOpt;

// Subcommand - Inspect
#[derive(Debug, StructOpt)]
pub struct InspectOptions {
    #[structopt(short, long, help = "Count all characters in the string")]
    pub characters: bool,
    #[structopt(short, long, help = "Count numbers")]
    pub numbers: bool,
    #[structopt(short, long, help = "Count all spaces in the string")]
    pub spaces: bool,
    #[structopt(short, long, help = "Count lowercase letters")]
    pub lowercase: bool,
    #[structopt(short, long, help = "Count uppercase letters")]
    pub uppercase: bool,
}

// Subcommand - Print
#[derive(Debug, StructOpt)]
pub struct PrintOptions {
    #[structopt(short = "lf", long, help = "Print to the left")]
    pub left: bool,
    #[structopt(short = "right", long, help = "Print to the right")]
    pub right: bool,
    #[structopt(short = "cen", long, help = "Print in the center")]
    pub center: bool,
}

    // Subcommand - Case
#[derive(Debug, StructOpt)]
pub struct CaseOptions {
    #[structopt(short = "l", long, help = "Transforms a string to lowercase")]
    pub lower: bool,
    #[structopt(short = "u", long, help = "Transforms a string to uppercase")]
    pub upper: bool,
    #[structopt(
        short = "t",
        long,
        help = "First letter of each compound word is capitalized, delimited by space"
    )]
    pub title: bool,
    #[structopt(
        short = "p",
        long,
        help = "First letter of each compound word is capitalized"
    )]
    pub pascal: bool,
    #[structopt(
        short = "c",
        long,
        help = "After the first word, the first letter of each compound word is capitalized."
    )]
    pub camel: bool,
    #[structopt(short = "k", long, help = "Lowercase and delimited by hyphen (-).")]
    pub kebab: bool,
    #[structopt(short = "K", long, help = "Uppercase and delimited by hyphen (-).")]
    pub cobol: bool,
    #[structopt(
        short = "s",
        long,
        help = "Lowercase and delimited by underscores (_)."
    )]
    pub snake: bool,
    #[structopt(
        short = "S",
        long,
        help = "Uppercase and delimited by underscores (_)."
    )]
    pub screamingsnake: bool,
}

#[derive(Debug, StructOpt)]
pub struct ModifyOptions {
    #[structopt(short, long, help = "Reverses a string")]
    pub reverse: bool,
    #[structopt(
        // short = "pre",
        long,
        help = "Adds a prefix to the string",
        env = "STRINGS__PREFIX"
    )]
    pub prefix: Option<String>,
    #[structopt(
        // short = "suf",
        long,
        help = "Adds a suffix to the string",
        env = "STRINGS__SUFFIX"
    )]
    pub suffix: Option<String>,
}
