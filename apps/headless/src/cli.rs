use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional BIOS path
    #[arg(short, long)]
    pub bios: Option<String>,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
