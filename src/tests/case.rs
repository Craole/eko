use clap::Parser;

#[derive(Parser)] // requires `derive` feature
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum Cargo {
    Case(Case),
}

#[derive(clap::Args)]
struct Case {
    #[arg(long)]
    lower: Option<String>,
}

pub fn main() {
    let Cargo::Case(args) = Cargo::parse();
    println!("{:?}", args.lower);
}