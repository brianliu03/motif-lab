use motif_lab::cli::commands::{run, Cli};

fn main() {
    let cli = Cli::parse_args();

    if let Err(error) = run(cli) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

