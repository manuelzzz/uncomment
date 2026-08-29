mod cli;
mod error;
mod file;
mod language;
mod parser;
mod transformer;

use anyhow::Result;
use clap::Parser;

use cli::Cli;
use language::Language;

fn main() -> Result<()> {
    let cli = Cli::parse();

    if !cli.file.exists() {
        return Err(error::UncommentError::FileNotFound(cli.file).into());
    }

    let language = Language::from_path(&cli.file)?;
    let source = file::read(&cli.file)?;

    println!("File: {}", cli.file.display());
    println!("Language: {}", language.name());
    println!("Dry run: {}", cli.dry_run);
    println!("Backup: {}", cli.backup);

    Ok(())
}
