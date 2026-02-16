use std::io::{self, Write};

use kount_count::CountResult;

pub fn print_table(result: &CountResult, top: Option<usize>) -> io::Result<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();

    let max_lines = result
        .files
        .iter()
        .map(|f| f.lines)
        .max()
        .unwrap_or(0)
        .max(result.total_lines);
    let width = max_lines.to_string().len().max(5);

    writeln!(out, "{:>width$}  File", "Lines", width = width)?;
    writeln!(
        out,
        "{:\u{2500}<width$}  {:\u{2500}<30}",
        "",
        "",
        width = width
    )?;

    let files = match top {
        Some(n) => &result.files[..n.min(result.files.len())],
        None => &result.files,
    };

    for file in files {
        writeln!(
            out,
            "{:>width$}  {}",
            file.lines,
            file.path.display(),
            width = width
        )?;
    }

    writeln!(
        out,
        "{:\u{2500}<width$}  {:\u{2500}<30}",
        "",
        "",
        width = width
    )?;
    writeln!(
        out,
        "{:>width$}  total ({} files)",
        result.total_lines,
        result.total_files,
        width = width
    )?;

    Ok(())
}

pub fn print_json(result: &CountResult, top: Option<usize>) -> io::Result<()> {
    match top {
        Some(n) => {
            let truncated = CountResult {
                files: result.files[..n.min(result.files.len())].to_vec(),
                ..result.clone()
            };
            serde_json::to_writer_pretty(io::stdout().lock(), &truncated)?;
        }
        None => {
            serde_json::to_writer_pretty(io::stdout().lock(), result)?;
        }
    }
    writeln!(io::stdout().lock())?;
    Ok(())
}

pub fn print_summary(result: &CountResult, _top: Option<usize>) -> io::Result<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();

    writeln!(
        out,
        "Total: {} lines in {} files",
        result.total_lines, result.total_files
    )?;

    if !result.by_extension.is_empty() {
        writeln!(out)?;
        writeln!(out, "By extension:")?;
        for ext in &result.by_extension {
            let label = if ext.extension.is_empty() {
                "(no ext)".to_string()
            } else {
                format!(".{}", ext.extension)
            };
            writeln!(
                out,
                "  {:<8} {:>6} lines  {:>3} files",
                label, ext.total_lines, ext.file_count
            )?;
        }
    }

    Ok(())
}
