use assert_cmd::Command;
use predicates::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[test]
fn help_ok() -> Result<()> {
    let mut cmd = Command::cargo_bin("bump")?;
    cmd.arg("--help");
    cmd.assert()
        .success()
        .code(exitcode::OK)
        .stdout(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn version_ok() -> Result<()> {
    let mut cmd = Command::cargo_bin("bump")?;
    cmd.arg("--version");
    cmd.assert()
        .success()
        .code(exitcode::OK)
        .stdout(predicate::str::contains("bump"));
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

    let mut cmd = Command::cargo_bin("bump")?;
    cmd.args(&["pre", "x.7.z.92", "0.1.0"]);
    cmd.assert()
        .success()
        .code(exitcode::OK)
        .stdout("0.1.0-x.7.z.92\n");

    let mut cmd = Command::cargo_bin("bump")?;
    cmd.args(&["build", "21AF26D3", "0.1.0"]);
    cmd.assert()
        .success()
        .code(exitcode::OK)
        .stdout("0.1.0+21AF26D3\n");

    Ok(())
}

#[test]
fn ng() -> Result<()> {
    let mut cmd = Command::cargo_bin("bump")?;
    cmd.args(&["patch", "x.x.x"]);
    cmd.assert()
        .failure()
        .code(exitcode::USAGE)
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn stdin_input_ok() -> Result<()> {
    let mut cmd = Command::cargo_bin("bump")?;
    cmd.args(&["patch"]);
    cmd.write_stdin("0.1.0")
        .assert()
        .success()
        .code(exitcode::OK)
        .stdout("0.1.1\n");

    let mut cmd = Command::cargo_bin("bump")?;
    cmd.args(&["minor"]);
    cmd.write_stdin("0.1.0")
        .assert()
        .success()
        .code(exitcode::OK)
        .stdout("0.2.0\n");

    let mut cmd = Command::cargo_bin("bump")?;
    cmd.args(&["major"]);
    cmd.write_stdin("0.1.0")
        .assert()
        .success()
        .code(exitcode::OK)
        .stdout("1.0.0\n");

    Ok(())
}

#[test]
fn hyphen_ok() -> Result<()> {
    let mut cmd = Command::cargo_bin("bump")?;
    cmd.args(&["patch", "-f", "-"]);
    cmd.write_stdin("0.1.0")
        .assert()
        .success()
        .code(exitcode::OK)
        .stdout("0.1.1\n");

    let mut cmd = Command::cargo_bin("bump")?;
    cmd.args(&["minor", "-f", "-"]);
    cmd.write_stdin("0.1.0")
        .assert()
        .success()
        .code(exitcode::OK)
        .stdout("0.2.0\n");

    let mut cmd = Command::cargo_bin("bump")?;
    cmd.args(&["major", "-f", "-"]);
    cmd.write_stdin("0.1.0")
        .assert()
        .success()
        .code(exitcode::OK)
        .stdout("1.0.0\n");

    Ok(())
}
