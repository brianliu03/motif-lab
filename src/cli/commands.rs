use crate::algorithms::transform::{augment, diminish, invert, retrograde, transpose};
use crate::core::Pitch;
use crate::io::parse::parse_motif;
use crate::io::print::{format_analysis, format_motif};
use clap::{Args, Parser, Subcommand};
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
    Transform(TransformArgs),
}

#[derive(Debug, Args)]
struct TransformArgs {
    path: PathBuf,

    #[arg(long)]
    transpose: Option<i32>,

    #[arg(long)]
    retrograde: bool,

    #[arg(long)]
    invert: Option<Pitch>,

    #[arg(long)]
    augment: Option<f32>,

    #[arg(long)]
    diminish: Option<f32>,
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
        Command::Transform(args) => {
            let input = fs::read_to_string(args.path)?;
            let mut motif = parse_motif(&input)?;

            if let Some(semitones) = args.transpose {
                motif = transpose(&motif, semitones);
            }

            if args.retrograde {
                motif = retrograde(&motif);
            }

            if let Some(axis_pitch) = args.invert {
                motif = invert(&motif, axis_pitch);
            }

            if let Some(factor) = args.augment {
                validate_positive_factor(factor, "augment")?;
                motif = augment(&motif, factor);
            }

            if let Some(factor) = args.diminish {
                validate_positive_factor(factor, "diminish")?;
                motif = diminish(&motif, factor);
            }

            println!("{}", format_motif(&motif));
        }
    }

    Ok(())
}

fn validate_positive_factor(
    factor: f32,
    name: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !factor.is_finite() || factor <= 0.0 {
        return Err(format!("{name} factor must be a positive finite number, got {factor}").into());
    }

    Ok(())
}
