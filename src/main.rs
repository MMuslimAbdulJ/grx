use anyhow::{Context, Result, Ok};
use clap::Parser;

// Search for a pattern
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf
}

fn main() -> Result<()> {
    // Input gate
    let args = Cli::parse();

    // Read the file
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file {}", args.path.display()))?;

    // Iterate the file, line by line
    for line in content.lines() {
        if line.contains(&args.pattern) {
            print!("{}\n", line);
        }
    }

    Ok(())
}