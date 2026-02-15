use std::path::PathBuf;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct FileCount {
    pub path: PathBuf,
    pub lines: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExtensionSummary {
    pub extension: String,
    pub file_count: u64,
    pub total_lines: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct CountResult {
    pub files: Vec<FileCount>,
    pub total_lines: u64,
    pub total_files: u64,
    pub by_extension: Vec<ExtensionSummary>,
}

#[derive(Debug, Clone, Default)]
pub enum SortOrder {
    #[default]
    Lines,
    Name,
    None,
}

#[derive(Debug, Clone)]
pub struct CountConfig {
    pub paths: Vec<PathBuf>,
    pub extensions: Vec<String>,
    pub globs: Vec<String>,
    pub use_ignore: bool,
    pub sort: SortOrder,
}
