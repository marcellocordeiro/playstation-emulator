use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional BIOS path
    #[arg(short, long)]
    pub bios: Option<String>,

    #[arg(short, long, default_value_t)]
    pub run_amidogs: bool,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
