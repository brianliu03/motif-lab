use crate::io::parse::parse_motif;
use crate::io::print::format_analysis;
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "motif-lab")]
#[command(about = "A CLI workbench for algorithmic motif analysis")]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Analyze { path: PathBuf },
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

pub fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Command::Analyze { path } => {
            let input = fs::read_to_string(path)?;
            let motif = parse_motif(&input)?;
            println!("{}", format_analysis(&motif));
        }
    }

    Ok(())
}

