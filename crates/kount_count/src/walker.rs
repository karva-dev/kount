use std::collections::HashMap;

use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use rayon::prelude::*;

use crate::counter::count_lines;
use crate::types::{CountConfig, CountResult, ExtensionSummary, FileCount, SortOrder};

pub fn count(config: &CountConfig) -> CountResult {
    let paths = if config.paths.is_empty() {
        vec![std::env::current_dir().unwrap_or_else(|_| ".".into())]
    } else {
        config.paths.clone()
    };

    // Build walker from first path, add the rest
    let first = &paths[0];
    let mut builder = WalkBuilder::new(first);
    for path in &paths[1..] {
        builder.add(path);
    }

    if !config.use_ignore {
        builder
            .hidden(false)
            .ignore(false)
            .git_ignore(false)
            .git_global(false)
            .git_exclude(false);
    }

    // Apply extension and glob filters
    let has_filters = !config.extensions.is_empty() || !config.globs.is_empty();
    if has_filters {
        let mut overrides = OverrideBuilder::new(first);
        for ext in &config.extensions {
            overrides.add(&format!("*.{ext}")).ok();
        }
        for glob in &config.globs {
            overrides.add(glob).ok();
        }
        if let Ok(built) = overrides.build() {
            builder.overrides(built);
        }
    }

    // Phase 1: Collect file paths
    let entries: Vec<_> = builder
        .build()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type()?.is_file() {
                Some(entry.into_path())
            } else {
                None
            }
        })
        .collect();

    // Phase 2: Count lines in parallel
    let files: Vec<FileCount> = entries
        .par_iter()
        .filter_map(|path| match count_lines(path) {
            Ok(lines) => Some(FileCount {
                path: path.clone(),
                lines,
            }),
            Err(e) => {
                eprintln!("warning: {}: {e}", path.display());
                None
            }
        })
        .collect();

    // Phase 3: Sort results
    let mut files = files;
    match config.sort {
        SortOrder::Lines => files.sort_by(|a, b| b.lines.cmp(&a.lines)),
        SortOrder::Name => files.sort_by(|a, b| a.path.cmp(&b.path)),
        SortOrder::None => {}
    }

    // Compute totals
    let total_lines = files.iter().map(|f| f.lines).sum();
    let total_files = files.len() as u64;

    // Per-extension breakdown
    let mut ext_map: HashMap<String, (u64, u64)> = HashMap::new();
    for file in &files {
        let ext = file
            .path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string();
        let entry = ext_map.entry(ext).or_default();
        entry.0 += 1;
        entry.1 += file.lines;
    }
    let mut by_extension: Vec<ExtensionSummary> = ext_map
        .into_iter()
        .map(|(extension, (file_count, total_lines))| ExtensionSummary {
            extension,
            file_count,
            total_lines,
        })
        .collect();
    by_extension.sort_by(|a, b| b.total_lines.cmp(&a.total_lines));

    CountResult {
        files,
        total_lines,
        total_files,
        by_extension,
    }
}
