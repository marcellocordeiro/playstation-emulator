use std::path::PathBuf;

use clap::{Parser, ValueHint};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional BIOS path
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    pub bios: Option<PathBuf>,

    /// Optional ROM path (exe only)
    #[arg(value_hint = ValueHint::FilePath)]
    pub rom: Option<PathBuf>,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
