use crate::error::UncommentError;
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Dart,
    JavaScript,
    TypeScript,
    Python,
}

impl Language {
    pub fn from_path(path: &PathBuf) -> Result<Self> {
        let extension = path
            .extension()
            .ok_or(UncommentError::MissingExtension)?
            .to_string_lossy()
            .to_lowercase();

        match extension.as_str() {
            "dart" => Ok(Self::Dart),
            "js" | "jsx" => Ok(Self::JavaScript),
            "ts" | "tsx" => Ok(Self::TypeScript),
            "py" => Ok(Self::Python),
            _ => Err(UncommentError::UnsupportedExtension(extension).into()),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Dart => "Dart",
            Self::JavaScript => "JavaScript",
            Self::TypeScript => "TypeScript",
            Self::Python => "Python",
        }
    }
}
