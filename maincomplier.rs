//! BlueBird Compiler Main Entry Point
//! Last updated: 2025-10-15

use clap::Parser;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "bluebirdc")]
#[command(about = "BlueBird language compiler")]
struct Cli {
    /// Source file to compile
    #[arg(value_name = "FILE")]
    input: PathBuf,

    /// Output file
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    println!("Compiling {}", cli.input.display());
    // TODO: Implement full compilation pipeline
    
    Ok(())
}