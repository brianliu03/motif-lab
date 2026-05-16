use crate::algorithms::compression::repeated_patterns;
use crate::algorithms::graph::{transition_graph, weighted_walk};
use crate::algorithms::similarity::compare_motifs;
use crate::algorithms::transform::{
    augment, diminish, invert_with_spelling, retrograde, transpose_with_spelling,
};
use crate::core::{Pitch, SpellingPolicy};
use crate::io::parse::parse_motif;
use crate::io::print::{
    format_analysis, format_compression_candidates, format_motif, format_pitch_walk,
    format_similarity, format_transition_graph,
};
use clap::{Args, Parser, Subcommand, ValueEnum};
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
    Compare {
        left_path: PathBuf,
        right_path: PathBuf,
    },
    Compress { path: PathBuf },
    Graph { path: PathBuf },
    Walk(WalkArgs),
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

    #[arg(long = "spelling-policy", value_enum, default_value = "preserve-context")]
    spelling_policy: CliSpellingPolicy,
}

#[derive(Debug, Args)]
struct WalkArgs {
    path: PathBuf,

    #[arg(long)]
    steps: usize,

    #[arg(long, default_value_t = 0)]
    seed: u64,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliSpellingPolicy {
    PreserveContext,
    Flats,
    Sharps,
}

impl CliSpellingPolicy {
    fn to_core(self) -> SpellingPolicy {
        match self {
            Self::PreserveContext => SpellingPolicy::PreserveContext,
            Self::Flats => SpellingPolicy::Flats,
            Self::Sharps => SpellingPolicy::Sharps,
        }
    }
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
            let spelling_policy = args.spelling_policy.to_core();

            if let Some(semitones) = args.transpose {
                motif = transpose_with_spelling(&motif, semitones, spelling_policy);
            }

            if args.retrograde {
                motif = retrograde(&motif);
            }

            if let Some(axis_pitch) = args.invert {
                motif = invert_with_spelling(&motif, axis_pitch, spelling_policy);
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
        Command::Compare {
            left_path,
            right_path,
        } => {
            let left_input = fs::read_to_string(left_path)?;
            let right_input = fs::read_to_string(right_path)?;
            let left = parse_motif(&left_input)?;
            let right = parse_motif(&right_input)?;
            let result = compare_motifs(&left, &right);

            println!("{}", format_similarity(&result));
        }
        Command::Compress { path } => {
            let input = fs::read_to_string(path)?;
            let motif = parse_motif(&input)?;
            let candidates = repeated_patterns(&motif);

            println!("{}", format_compression_candidates(&candidates));
        }
        Command::Graph { path } => {
            let input = fs::read_to_string(path)?;
            let motif = parse_motif(&input)?;
            let graph = transition_graph(&motif);

            println!("{}", format_transition_graph(&graph));
        }
        Command::Walk(args) => {
            let input = fs::read_to_string(args.path)?;
            let motif = parse_motif(&input)?;
            let graph = transition_graph(&motif);
            let start = motif.notes.last().map(|note| note.pitch);
            let walk = weighted_walk(&graph, start, args.steps, args.seed);

            println!("{}", format_pitch_walk(&walk));
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
