mod args;
mod error;
mod fs;
mod version;

use crate::args::{Args, Command};
use crate::error::Result;
use exitcode;
use log::*;
use pretty_env_logger;

use std::env;
use std::process::exit;
use structopt::StructOpt;

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

    debug!("version: {:?}", &version);
    Ok(version.to_string())
}

fn main() {
    pretty_env_logger::init();
    let args = env::args().collect::<Vec<String>>();
    let code = match run(args) {
        Ok(view) => {
            println!("{}", view);
            exitcode::OK
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            exitcode::USAGE
        }
    };
    exit(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_success(row_args: Vec<&str>, expect: &str) {
        let args = row_args.into_iter().map(String::from).collect();

        let actual = run(args);
        assert_eq!(actual.unwrap(), String::from(expect));
    }

    fn test_fail(row_args: Vec<&str>) {
        let args = row_args.into_iter().map(String::from).collect();

        let actual = run(args);
        assert!(actual.is_err());
    }

    #[test]
    fn inc_patch() {
        let version = "0.1.0";
        let expect = "0.1.1";
        let args = vec!["bump", "patch", version];
        test_success(args, expect);

        let version = "0.10.0";
        let expect = "0.10.1";
        let args = vec!["bump", "patch", version];
        test_success(args, expect);

        let version = "10000.10000.10000";
        let expect = "10000.10000.10001";
        let args = vec!["bump", "patch", version];
        test_success(args, expect);
    }

    #[test]
    fn inc_minor() {
        let version = "0.1.1";
        let expect = "0.2.0";
        let args = vec!["bump", "minor", version];
        test_success(args, expect);

        let version = "0.10.0";
        let expect = "0.11.0";
        let args = vec!["bump", "minor", version];
        test_success(args, expect);

        let version = "10000.10000.10000";
        let expect = "10000.10001.0";
        let args = vec!["bump", "minor", version];
        test_success(args, expect);
    }

    #[test]
    fn inc_major() {
        let version = "0.1.1";
        let expect = "1.0.0";
        let args = vec!["bump", "major", version];
        test_success(args, expect);

        let version = "0.10.0";
        let expect = "1.0.0";
        let args = vec!["bump", "major", version];
        test_success(args, expect);

        let version = "10000.10000.10000";
        let expect = "10001.0.0";
        let args = vec!["bump", "major", version];
        test_success(args, expect);
    }

    #[test]
    fn remove_pre() {
        let version = "1.0.0-alpha.0";
        let expect = "1.0.1";
        let args = vec!["bump", "patch", version];
        test_success(args, expect);
    }

    #[test]
    fn add_pre() {
        let version = "1.0.0";
        let pre = "alpha.0";
        let expect = "1.0.0-alpha.0";
        let args = vec!["bump", "pre", pre, version];
        test_success(args, expect);
    }

    #[test]
    fn replace_pre() {
        let version = "1.0.0-alpha.0";
        let pre = "beta.0";
        let expect = "1.0.0-beta.0";
        let args = vec!["bump", "pre", pre, version];
        test_success(args, expect);
    }

    #[test]
    fn remove_build() {
        let version = "1.0.0+20190722";
        let expect = "1.0.1";
        let args = vec!["bump", "patch", version];
        test_success(args, expect);
    }

    #[test]
    fn add_build() {
        let version = "1.0.0";
        let build = "20190722";
        let expect = "1.0.0+20190722";
        let args = vec!["bump", "build", build, version];
        test_success(args, expect);
    }

    #[test]
    fn replace_build() {
        let version = "1.0.0+20190722";
        let build = "20190723";
        let expect = "1.0.0+20190723";
        let args = vec!["bump", "build", build, version];
        test_success(args, expect);
    }

    #[test]
    fn ignore_default_prefix() {
        let version = "v1.0.0";
        let expect = "v2.0.0";
        let args = vec!["bump", "major", version];
        test_success(args, expect);
    }

    #[test]
    fn prefix_major() {
        let version = "release-1.0.0";
        let expect = "release-2.0.0";
        let args = vec!["bump", "major", version];
        test_success(args, expect);
    }

    #[test]
    fn prefix_minor() {
        let version = "release-1.0.0";
        let expect = "release-1.1.0";
        let args = vec!["bump", "minor", version];
        test_success(args, expect);
    }

    #[test]
    fn prefix_patch() {
        let version = "release-1.0.0";
        let expect = "release-1.0.1";
        let args = vec!["bump", "patch", version];
        test_success(args, expect);
    }

    #[test]
    fn prefix_pre() {
        let version = "release-1.0.0-alpha.0";
        let expect = "release-1.0.0-beta.0";
        let args = vec!["bump", "pre", "beta.0", version];
        test_success(args, expect);
    }

    #[test]
    fn prefix_build() {
        let version = "release-1.0.0+20190722";
        let expect = "release-1.0.0+20190723";
        let args = vec!["bump", "build", "20190723", version];
        test_success(args, expect);
    }

    #[test]
    fn invalid_version() {
        let version = "0.0";
        let args = vec!["bump", "patch", version];
        test_fail(args);

        let version = "release";
        let args = vec!["bump", "patch", version];
        test_fail(args);

        let version = "0-0-0";
        let args = vec!["bump", "patch", version];
        test_fail(args);

        let version = "0.0.0@20190722";
        let args = vec!["bump", "patch", version];
        test_fail(args);
    }

}
