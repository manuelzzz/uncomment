mod cli;
mod error;
mod file;
mod language;
mod parser;
mod transformer;

use anyhow::Result;
use clap::Parser;

use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("File: {}", cli.file.display());
    println!("Dry run: {}", cli.dry_run);
    println!("Backup: {}", cli.backup);

    Ok(())
}
