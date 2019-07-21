mod args;
mod error;
mod fs;
mod version;

use crate::args::{Args, Command};
use crate::error::Result;
use log::*;
use pretty_env_logger;

use std::env;
use std::process::exit;
use structopt::StructOpt;

enum ExitStatus {
    Success,
    Failure,
}

fn run(args: Vec<String>) -> Result<String> {
    let clap = Args::clap().get_matches_from_safe(args.into_iter())?;
    let args = Args::from_clap(&clap);

    let version = match &args.cmd {
        Command::Patch { ver } => ver.bump_patch(),
        Command::Minor { ver } => ver.bump_minor(),
        Command::Major { ver } => ver.bump_major(),
        Command::Pre { pre, ver } => ver.update_pre_release(pre),
        Command::Build { build, ver } => ver.update_build(build),
    };

    debug!("{:?}", &version);
    Ok(version.to_string())
}

fn main() {
    pretty_env_logger::init();
    let args = env::args().collect::<Vec<String>>();
    let code = match run(args) {
        Ok(view) => {
            println!("{}", view);
            ExitStatus::Success
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            ExitStatus::Failure
        }
    };
    exit(code as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inc_patch() {
        let version = "0.1.0";
        let expect = "0.1.1";
        let args = vec!["bump", "patch", version]
            .into_iter()
            .map(String::from)
            .collect();

        let actual = run(args);
        assert_eq!(actual.unwrap(), String::from(expect));
    }

    #[test]
    fn inc_minor() {
        let version = "0.1.1";
        let expect = "0.2.0";
        let args = vec!["bump", "minor", version]
            .into_iter()
            .map(String::from)
            .collect();

        let actual = run(args);
        assert_eq!(actual.unwrap(), String::from(expect));
    }

    #[test]
    fn inc_major() {
        let version = "0.1.1";
        let expect = "1.0.0";
        let args = vec!["bump", "major", version]
            .into_iter()
            .map(String::from)
            .collect();

        let actual = run(args);
        assert_eq!(actual.unwrap(), String::from(expect));
    }

    #[test]
    fn add_pre() {
        let version = "1.0.0";
        let pre = "alpha.0";
        let expect = "1.0.0-alpha.0";
        let args = vec!["bump", "pre", pre, version]
            .into_iter()
            .map(String::from)
            .collect();

        let actual = run(args);
        assert_eq!(actual.unwrap(), String::from(expect));
    }

    #[test]
    fn replace_pre() {
        let version = "1.0.0-alpha.0";
        let pre = "beta.0";
        let expect = "1.0.0-beta.0";
        let args = vec!["bump", "pre", pre, version]
            .into_iter()
            .map(String::from)
            .collect();

        let actual = run(args);
        assert_eq!(actual.unwrap(), String::from(expect));
    }

    #[test]
    fn ignore_ver_prefix() {
        let version = "v1.0.0";
        let expect = "v2.0.0";
        let args = vec!["bump", "major", version]
            .into_iter()
            .map(String::from)
            .collect();

        let actual = run(args);
        assert_eq!(actual.unwrap(), String::from(expect));
    }

}
