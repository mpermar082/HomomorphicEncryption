// src/main.rs
/*
 * Main executable for HomomorphicEncryption
 */

use clap::Parser;
use homomorphicencryption::{Result, run};

/// CLI arguments for HomomorphicEncryption
#[derive(Parser)]
#[command(version, about = "HomomorphicEncryption - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Path to input file
    #[arg(short, long)]
    input: Option<String>,
    
    /// Path to output file
    #[arg(short, long)]
    output: Option<String>,
}

/// Runs the HomomorphicEncryption program
fn main() -> Result<()> {
    // Parse CLI arguments
    let args = Cli::parse();
    
    // Run the program with parsed arguments
    run(args.verbose, args.input, args.output)
}