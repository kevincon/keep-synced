use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;
use textwrap::dedent;

#[test]
fn help_usage() {
    let temp = assert_fs::TempDir::new().unwrap();
    let input_file = temp.child("foo.txt");
    input_file.touch().unwrap();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg("--help").assert();
    assert
        .success()
        .stdout(predicate::str::starts_with("Usage: keep-synced [PATHS]..."));

    input_file.assert("");
    // or
    input_file.assert(predicate::str::is_empty());

    temp.child("bar.txt").assert(predicate::path::missing());

    temp.close().unwrap();
}

#[test]
fn already_synced() {
    let temp = assert_fs::TempDir::new().unwrap();
    let source_of_truth_file = temp.child("pyproject.toml");
    source_of_truth_file
        .write_str(&dedent(
            r#"
            [project]
            # keep-synced /"(?<PROJECT_VERSION>.+)"/
            version = "2020.0.0"
            "#,
        ))
        .unwrap();

    let reference_file = temp.child("__init__.py");
    reference_file
        .write_str(&dedent(
            r#"
            # keep-synced /"${PROJECT_VERSION}"/
            __version__ = "2020.0.0"
            "#,
        ))
        .unwrap();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(temp.path()).assert();
    assert.success();

    temp.close().unwrap();
}

#[test]
fn sync_reference() {
    let temp = assert_fs::TempDir::new().unwrap();
    let source_of_truth_file = temp.child("pyproject.toml");
    source_of_truth_file
        .write_str(&dedent(
            r#"
            [project]
            # keep-synced /"(?<PROJECT_VERSION>.+)"/
            version = "2025.0.0"
            "#,
        ))
        .unwrap();

    let reference_file = temp.child("__init__.py");
    reference_file
        .write_str(&dedent(
            r#"
            # keep-synced /"${PROJECT_VERSION}"/
            __version__ = "2020.0.0"
            "#,
        ))
        .unwrap();

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let assert = cmd.arg(temp.path()).assert();
    assert.success();

    // input_file.assert("");
    // // or
    reference_file.assert(&dedent(
        r#"
        # keep-synced /"${PROJECT_VERSION}"/
        __version__ = "2025.0.0"
        "#,
    ));

    temp.close().unwrap();
}
