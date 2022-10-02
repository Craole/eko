use clap::{Parser, Subcommand};

use crate::cli::commands::case::lib::CaseCommand;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Show detailed information
    #[arg(short = 'd', long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Transforms text per a given case convention.
    Case(CaseCommand),
}

#[allow(dead_code)]
pub fn args() {
    let args = Cli::parse();

    match args.command {
        _ => todo!(),
    };
}
