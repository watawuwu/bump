use assert_cmd::prelude::*;
use exitcode;
use predicates::prelude::*;
use std::process::Command;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

#[test]
fn help_err() -> Result<()> {
    let mut cmd = Command::cargo_bin("bump")?;
    cmd.arg("-h");
    cmd.assert()
        .failure()
        .code(exitcode::USAGE)
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn none_args_err() -> Result<()> {
    let mut cmd = Command::cargo_bin("bump")?;
    cmd.arg("");
    cmd.assert().failure().code(exitcode::USAGE);
    Ok(())
}

#[test]
fn ok() -> Result<()> {
    let mut cmd = Command::cargo_bin("bump")?;
    cmd.args(&["patch", "0.1.0"]);
    cmd.assert().success().code(exitcode::OK).stdout("0.1.1\n");

    let mut cmd = Command::cargo_bin("bump")?;
    cmd.args(&["minor", "0.1.0"]);
    cmd.assert().success().code(exitcode::OK).stdout("0.2.0\n");

    let mut cmd = Command::cargo_bin("bump")?;
    cmd.args(&["major", "0.1.0"]);
    cmd.assert().success().code(exitcode::OK).stdout("1.0.0\n");

    Ok(())
}

#[test]
fn ng() -> Result<()> {
    let mut cmd = Command::cargo_bin("bump")?;
    cmd.args(&["patch", "x.x.x"]);
    cmd.assert()
        .failure()
        .code(exitcode::USAGE)
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn stdin_input_ok() -> Result<()> {
    let mut cmd = Command::cargo_bin("bump")?;
    cmd.args(&["patch"]);
    cmd.with_stdin()
        .buffer("0.1.0")
        .assert()
        .success()
        .code(exitcode::OK)
        .stdout("0.1.1\n");

    let mut cmd = Command::cargo_bin("bump")?;
    cmd.args(&["minor"]);
    cmd.with_stdin()
        .buffer("0.1.0")
        .assert()
        .success()
        .code(exitcode::OK)
        .stdout("0.2.0\n");

    let mut cmd = Command::cargo_bin("bump")?;
    cmd.args(&["major"]);
    cmd.with_stdin()
        .buffer("0.1.0")
        .assert()
        .success()
        .code(exitcode::OK)
        .stdout("1.0.0\n");

    Ok(())
}

#[test]
fn hyphen_ok() -> Result<()> {
    let mut cmd = Command::cargo_bin("bump")?;
    cmd.args(&["patch", "-"]);
    cmd.with_stdin()
        .buffer("0.1.0")
        .assert()
        .success()
        .code(exitcode::OK)
        .stdout("0.1.1\n");

    let mut cmd = Command::cargo_bin("bump")?;
    cmd.args(&["minor", "-"]);
    cmd.with_stdin()
        .buffer("0.1.0")
        .assert()
        .success()
        .code(exitcode::OK)
        .stdout("0.2.0\n");

    let mut cmd = Command::cargo_bin("bump")?;
    cmd.args(&["major", "-"]);
    cmd.with_stdin()
        .buffer("0.1.0")
        .assert()
        .success()
        .code(exitcode::OK)
        .stdout("1.0.0\n");

    Ok(())
}
