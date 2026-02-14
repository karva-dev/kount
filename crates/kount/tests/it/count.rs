use assert_fs::prelude::*;

use crate::common::TestContext;
use crate::kount_snapshot;

#[test]
fn count_single_file() {
    let ctx = TestContext::new();
    ctx.root
        .child("hello.txt")
        .write_str("line 1\nline 2\nline 3\n")
        .unwrap();

    kount_snapshot!(ctx.filters(), ctx.command().arg("hello.txt"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Lines  File
    ─────  ──────────────────────────────
        3  hello.txt
    ─────  ──────────────────────────────
        3  total (1 files)

    ----- stderr -----
    ");
}

#[test]
fn count_directory() {
    let ctx = TestContext::new();
    ctx.root.child("a.rs").write_str("fn main() {}\n").unwrap();
    ctx.root
        .child("b.rs")
        .write_str("fn foo() {}\nfn bar() {}\n")
        .unwrap();

    kount_snapshot!(ctx.filters(), ctx.command().arg("--sort").arg("name").arg("."), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Lines  File
    ─────  ──────────────────────────────
        1  ./a.rs
        2  ./b.rs
    ─────  ──────────────────────────────
        3  total (2 files)

    ----- stderr -----
    ");
}

#[test]
fn count_with_extension_filter() {
    let ctx = TestContext::new();
    ctx.root
        .child("code.rs")
        .write_str("fn main() {}\n")
        .unwrap();
    ctx.root.child("readme.md").write_str("# Hello\n").unwrap();
    ctx.root.child("data.json").write_str("{}\n").unwrap();

    kount_snapshot!(ctx.filters(), ctx.command().args(["--ext", "rs", "--sort", "name", "."]), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Lines  File
    ─────  ──────────────────────────────
        1  ./code.rs
    ─────  ──────────────────────────────
        1  total (1 files)

    ----- stderr -----
    ");
}

#[test]
fn count_json_output() {
    let ctx = TestContext::new();
    ctx.root
        .child("test.txt")
        .write_str("hello\nworld\n")
        .unwrap();

    let output = ctx
        .command()
        .args(["--json", "test.txt"])
        .output()
        .expect("Failed to run kount");

    assert!(output.status.success());
    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("Invalid JSON output");
    assert_eq!(json["total_lines"], 2);
    assert_eq!(json["total_files"], 1);
}

#[test]
fn count_summary_output() {
    let ctx = TestContext::new();
    ctx.root.child("a.rs").write_str("line1\nline2\n").unwrap();
    ctx.root.child("b.rs").write_str("line1\n").unwrap();

    kount_snapshot!(ctx.filters(), ctx.command().args(["--summary", "."]), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Total: 3 lines in 2 files

    By extension:
      .rs           3 lines    2 files

    ----- stderr -----
    ");
}

#[test]
fn count_empty_directory() {
    let ctx = TestContext::new();

    kount_snapshot!(ctx.filters(), ctx.command().arg("."), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Lines  File
    ─────  ──────────────────────────────
    ─────  ──────────────────────────────
        0  total (0 files)

    ----- stderr -----
    ");
}

#[test]
fn count_nonexistent_path() {
    let ctx = TestContext::new();

    kount_snapshot!(ctx.filters(), ctx.command().arg("nonexistent"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Lines  File
    ─────  ──────────────────────────────
    ─────  ──────────────────────────────
        0  total (0 files)

    ----- stderr -----
    ");
}

#[test]
fn help_output() {
    let ctx = TestContext::new();

    let output = ctx
        .command()
        .arg("--help")
        .output()
        .expect("Failed to run kount");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Count lines in files and directories"));
    assert!(stdout.contains("--ext"));
    assert!(stdout.contains("--json"));
    assert!(stdout.contains("--summary"));
}

#[test]
fn version_output() {
    let ctx = TestContext::new();

    let output = ctx
        .command()
        .arg("--version")
        .output()
        .expect("Failed to run kount");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("kount"));
}
