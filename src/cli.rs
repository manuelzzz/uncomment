use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "uncomment",
    version,
    about = "Remove comments from source code files"
)]
pub struct Cli {
    #[arg(help = "Path to the source code file")]
    pub file: PathBuf,

    #[arg(help = "Dry run: do not modify the file")]
    #[arg(short, long)]
    pub dry_run: bool,

    #[arg(help = "Backup: create a backup of the original file")]
    #[arg(short, long)]
    pub backup: bool,
}
