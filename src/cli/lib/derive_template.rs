use clap::{Parser, Args, Subcommand, ValueEnum};

/// Doc comment
#[derive(Parser)]
#[command(CMD ATTRIBUTE)]
#[group(GROUP ATTRIBUTE)]
struct Cli {
    /// Doc comment
    #[arg(ARG ATTRIBUTE)]
    field: UserType,

    #[arg(value_enum, ARG ATTRIBUTE...)]
    field: EnumValues,

    #[command(flatten)]
    delegate: Struct,

    #[command(subcommand)]
    command: Command,
}

/// Doc comment
#[derive(Args)]
#[command(PARENT CMD ATTRIBUTE)]
#[group(GROUP ATTRIBUTE)]
struct Struct {
    /// Doc comment
    #[command(ARG ATTRIBUTE)]
    field: UserType,
}

/// Doc comment
#[derive(Subcommand)]
#[command(PARENT CMD ATTRIBUTE)]
enum Command {
    /// Doc comment
    #[command(CMD ATTRIBUTE)]
    Variant1(Struct),

    /// Doc comment
    #[command(CMD ATTRIBUTE)]
    Variant2 {
        /// Doc comment
        #[arg(ARG ATTRIBUTE)]
        field: UserType,
    }
}

/// Doc comment
#[derive(ValueEnum)]
#[value(VALUE ENUM ATTRIBUTE)]
enum EnumValues {
    /// Doc comment
    #[value(POSSIBLE VALUE ATTRIBUTE)]
    Variant1,
}

fn main() {
    let cli = Cli::parse();
}