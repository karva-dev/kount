use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use kount_count::{CountConfig, SortOrder};

#[derive(Debug, Clone, ValueEnum)]
pub enum SortBy {
    Lines,
    Name,
    None,
}

/// Count lines in files and directories
#[derive(Debug, Parser)]
#[command(name = "kount", version, about)]
pub struct Args {
    /// Files or directories to count (default: current directory)
    #[arg()]
    pub paths: Vec<PathBuf>,

    /// Filter by extension (comma-separated: rs,py,js)
    #[arg(short = 'e', long = "ext", value_delimiter = ',')]
    pub ext: Vec<String>,

    /// Filter by glob pattern (repeatable)
    #[arg(short = 'g', long = "glob")]
    pub glob: Vec<String>,

    /// Sort order [default: lines]
    #[arg(short = 's', long = "sort", default_value = "lines")]
    pub sort: SortBy,

    /// Include hidden files, ignore .gitignore
    #[arg(long = "no-ignore")]
    pub no_ignore: bool,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,

    /// Show only totals and per-extension breakdown
    #[arg(long)]
    pub summary: bool,

    /// Show only the top N files by line count
    #[arg(long)]
    pub top: Option<usize>,
}

impl Args {
    pub fn to_config(&self) -> CountConfig {
        CountConfig {
            paths: self.paths.clone(),
            extensions: self.ext.clone(),
            globs: self.glob.clone(),
            use_ignore: !self.no_ignore,
            sort: match self.sort {
                SortBy::Lines => SortOrder::Lines,
                SortBy::Name => SortOrder::Name,
                SortBy::None => SortOrder::None,
            },
        }
    }
}
