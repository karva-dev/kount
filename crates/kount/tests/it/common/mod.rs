// The `unreachable_pub` is to silence false positives in RustRover.
#![allow(dead_code, unreachable_pub, unused_imports)]

use assert_cmd::Command;
use assert_fs::fixture::ChildPath;
use assert_fs::prelude::*;
use regex::Regex;
use std::path::{Path, PathBuf};

/// Test context for running kount commands.
pub struct TestContext {
    pub root: ChildPath,
    /// Standard filters for this test context.
    filters: Vec<(String, String)>,
    /// The temporary directory for this test.
    pub _root: tempfile::TempDir,
}

impl TestContext {
    /// Create a new test context with a temporary directory.
    pub fn new() -> Self {
        let root = tempfile::TempDir::with_prefix("kount-test")
            .expect("Failed to create test root directory");

        let mut filters = Vec::new();

        filters.extend(
            Self::path_patterns(root.path())
                .into_iter()
                .map(|pattern| (pattern, "[TEMP]/".to_string())),
        );

        if cfg!(windows) {
            let pattern = regex::escape(
                &dunce::simplified(root.path())
                    .display()
                    .to_string()
                    .replace('/', "\\"),
            );
            filters.push((pattern, "[TEMP]".to_string()));
        }

        Self {
            root: ChildPath::new(root.path()),
            _root: root,
            filters,
        }
    }

    /// Generate various escaped regex patterns for the given path.
    pub fn path_patterns(path: impl AsRef<Path>) -> Vec<String> {
        let mut patterns = Vec::new();

        if path.as_ref().exists() {
            patterns.push(Self::path_pattern(
                path.as_ref()
                    .canonicalize()
                    .expect("Failed to create canonical path"),
            ));
        }

        patterns.push(Self::path_pattern(path));

        patterns
    }

    /// Generate an escaped regex pattern for the given path.
    fn path_pattern(path: impl AsRef<Path>) -> String {
        format!(
            r"{}(\\|\/)*",
            regex::escape(&dunce::simplified(path.as_ref()).display().to_string())
                .replace(r"\\", r"(\\|\/)+")
        )
    }

    /// Standard snapshot filters _plus_ those for this test context.
    pub fn filters(&self) -> Vec<(&str, &str)> {
        self.filters
            .iter()
            .map(|(p, r)| (p.as_str(), r.as_str()))
            .chain(INSTA_FILTERS.iter().copied())
            .collect()
    }

    /// Create a kount command for testing.
    #[allow(clippy::unused_self)]
    pub fn command(&self) -> Command {
        let mut command = Self::new_command();
        command.current_dir(self.root.path());
        command
    }

    /// Read a file and return its contents as a string.
    pub fn read_file(&self, path: &str) -> String {
        std::fs::read_to_string(self.root.join(path))
            .unwrap_or_else(|_| panic!("Failed to read file: {path}"))
    }

    /// Creates a new `Command` suitable for use in all tests.
    fn new_command() -> Command {
        Command::new(get_bin())
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns the kount binary that cargo built before launching the tests.
pub fn get_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_kount"))
}

/// Common filters for snapshot testing.
pub static INSTA_FILTERS: &[(&str, &str)] = &[
    // Normalize Windows line endings
    (r"\r\n", "\n"),
    // Normalize Windows paths
    (r"\\", "/"),
    // Rewrite Windows output to Unix output
    (r"\\([\w\d]|\.)", "/$1"),
    (r"kount\.exe", "kount"),
    // kount version display
    (
        r"kount(-.*)? \d+\.\d+\.\d+(-(alpha|beta|rc)\.\d+)?",
        r"kount [VERSION]",
    ),
    // Strip ANSI color codes
    (r"[\x1b]\[[0-9;]*m", ""),
];

/// Helper method to apply filters to a string.
pub fn apply_filters<T: AsRef<str>>(mut snapshot: String, filters: impl AsRef<[(T, T)]>) -> String {
    for (matcher, replacement) in filters.as_ref() {
        let re = Regex::new(matcher.as_ref()).expect("Do you need to regex::escape your filter?");
        if re.is_match(&snapshot) {
            snapshot = re.replace_all(&snapshot, replacement.as_ref()).to_string();
        }
    }
    snapshot
}

/// Execute the command and format its output status, stdout and stderr into a snapshot string.
#[allow(clippy::print_stderr)]
pub fn run_and_format(
    cmd: &mut Command,
    filters: &[(&str, &str)],
    _test_name: &str,
) -> (String, std::process::Output) {
    let program = cmd.get_program().to_string_lossy().to_string();

    let output = cmd
        .output()
        .unwrap_or_else(|err| panic!("Failed to spawn {program}: {err}"));

    eprintln!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Unfiltered output ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!(
        "----- stdout -----\n{}\n----- stderr -----\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    eprintln!("────────────────────────────────────────────────────────────────────────────────\n");

    let snapshot = apply_filters(
        format!(
            "success: {:?}\nexit_code: {}\n----- stdout -----\n{}\n----- stderr -----\n{}",
            output.status.success(),
            output.status.code().unwrap_or(!0),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        ),
        filters,
    );

    (snapshot, output)
}

/// Run snapshot testing with the kount command.
#[allow(unused_macros)]
#[macro_export]
macro_rules! kount_snapshot {
    ($cmd:expr, @$snapshot:literal) => {{
        kount_snapshot!($crate::common::INSTA_FILTERS.to_vec(), $cmd, @$snapshot)
    }};
    ($filters:expr, $cmd:expr, @$snapshot:literal) => {{
        let (snapshot, output) = $crate::common::run_and_format(
            $cmd,
            &$filters,
            $crate::function_name!(),
        );
        ::insta::assert_snapshot!(snapshot, @$snapshot);
        output
    }};
}

/// Get the function name for snapshot naming.
#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

#[allow(unused_imports)]
pub(crate) use kount_snapshot;
