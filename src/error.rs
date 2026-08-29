use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UncommentError {
    #[error("file not found: {0}")]
    FileNotFound(PathBuf),

    #[error("unsupported file extension: {0}")]
    UnsupportedExtension(String),

    #[error("Could not determine file extension")]
    MissingExtension,

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
