use crate::common::TestContext;
use crate::kount_snapshot;

#[test]
fn help_output() {
    let ctx = TestContext::new();

    kount_snapshot!(ctx.filters(), ctx.command().arg("--help"), @"
    success: true
    exit_code: 0
    ----- stdout -----
    Count lines in files and directories

    Usage: kount [OPTIONS] [PATHS]...

    Arguments:
      [PATHS]...  Files or directories to count (default: current directory)

    Options:
      -e, --ext <EXT>    Filter by extension (comma-separated: rs,py,js)
      -g, --glob <GLOB>  Filter by glob pattern (repeatable)
      -s, --sort <SORT>  Sort order [default: lines] [default: lines] [possible values: lines, name, none]
          --no-ignore    Include hidden files, ignore .gitignore
          --json         Output as JSON
          --summary      Show only totals and per-extension breakdown
          --top <TOP>    Show only the top N files by line count
      -h, --help         Print help
      -V, --version      Print version

    ----- stderr -----
    ");
}
