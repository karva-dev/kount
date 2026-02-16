use crate::common::TestContext;
use crate::kount_snapshot;

#[test]
fn version_output() {
    let ctx = TestContext::new();

    kount_snapshot!(ctx.filters(), ctx.command().arg("--version"), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    kount [VERSION]

    ----- stderr -----
    ");
}
