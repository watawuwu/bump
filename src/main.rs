mod args;
mod fs;
mod version;

use crate::args::{Args, SubCommand};
use log::*;

use anyhow::Result;
use std::env;
use std::process::exit;

fn run(row_args: Vec<String>) -> Result<String> {
    let args = Args::new(&row_args)?;

    let version = match args.sub {
        SubCommand::Patch { ver } => ver.bump_patch(),
        SubCommand::Minor { ver } => ver.bump_minor(),
        SubCommand::Major { ver } => ver.bump_major(),
        SubCommand::Pre { pre, ver } => ver.update_pre_release(pre)?,
        SubCommand::Build { build, ver } => ver.update_build(build)?,
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
            eprintln!("{}", err);
            exitcode::USAGE
        }
    };
    exit(code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::*;
    use tempfile::tempdir;

    fn test_ok(row_args: Vec<&str>, expect: &str) {
        let args = row_args.into_iter().map(String::from).collect();

        let actual = run(args);
        assert_eq!(actual.unwrap(), String::from(expect));
    }

    fn test_err(row_args: Vec<&str>) {
        let args = row_args.into_iter().map(String::from).collect();

        let actual = run(args);
        assert!(actual.is_err());
    }

    #[test]
    fn inc_patch_ok() {
        let version = "0.1.0";
        let expect = "0.1.1";
        let args = vec!["bump", "patch", version];
        test_ok(args, expect);

        let version = "0.10.0";
        let expect = "0.10.1";
        let args = vec!["bump", "patch", version];
        test_ok(args, expect);

        let version = "10000.10000.10000";
        let expect = "10000.10000.10001";
        let args = vec!["bump", "patch", version];
        test_ok(args, expect);
    }

    #[test]
    fn inc_minor_ok() {
        let version = "0.1.1";
        let expect = "0.2.0";
        let args = vec!["bump", "minor", version];
        test_ok(args, expect);

        let version = "0.10.0";
        let expect = "0.11.0";
        let args = vec!["bump", "minor", version];
        test_ok(args, expect);

        let version = "10000.10000.10000";
        let expect = "10000.10001.0";
        let args = vec!["bump", "minor", version];
        test_ok(args, expect);
    }

    #[test]
    fn inc_major_ok() {
        let version = "0.1.1";
        let expect = "1.0.0";
        let args = vec!["bump", "major", version];
        test_ok(args, expect);

        let version = "0.10.0";
        let expect = "1.0.0";
        let args = vec!["bump", "major", version];
        test_ok(args, expect);

        let version = "10000.10000.10000";
        let expect = "10001.0.0";
        let args = vec!["bump", "major", version];
        test_ok(args, expect);
    }

    #[test]
    fn remove_pre_ok() {
        let version = "1.0.0-alpha.0";
        let expect = "1.0.1";
        let args = vec!["bump", "patch", version];
        test_ok(args, expect);
    }

    #[test]
    fn add_pre_ok() {
        let version = "1.0.0";
        let pre = "alpha.0";
        let expect = "1.0.0-alpha.0";
        let args = vec!["bump", "pre", pre, version];
        test_ok(args, expect);
    }

    #[test]
    fn replace_pre_ok() {
        let version = "1.0.0-alpha.0";
        let pre = "beta.0";
        let expect = "1.0.0-beta.0";
        let args = vec!["bump", "pre", pre, version];
        test_ok(args, expect);
    }

    #[test]
    fn remove_build_ok() {
        let version = "1.0.0+20190722";
        let expect = "1.0.1";
        let args = vec!["bump", "patch", version];
        test_ok(args, expect);
    }

    #[test]
    fn add_build() {
        let version = "1.0.0";
        let build = "20190722";
        let expect = "1.0.0+20190722";
        let args = vec!["bump", "build", build, version];
        test_ok(args, expect);
    }

    #[test]
    fn replace_build_ok() {
        let version = "1.0.0+20190722";
        let build = "20190723";
        let expect = "1.0.0+20190723";
        let args = vec!["bump", "build", build, version];
        test_ok(args, expect);
    }

    #[test]
    fn ignore_default_prefix_ok() {
        let version = "v1.0.0";
        let expect = "v2.0.0";
        let args = vec!["bump", "major", version];
        test_ok(args, expect);
    }

    #[test]
    fn prefix_ok() {
        let version = "release-1.0.0";
        let expect = "release-2.0.0";
        let args = vec!["bump", "major", version];
        test_ok(args, expect);

        let version = "release-1.0.0";
        let expect = "release-1.1.0";
        let args = vec!["bump", "minor", version];
        test_ok(args, expect);

        let version = "release-1.0.0";
        let expect = "release-1.0.1";
        let args = vec!["bump", "patch", version];
        test_ok(args, expect);

        let version = "release-1.0.0-alpha.0";
        let expect = "release-1.0.0-beta.0";
        let args = vec!["bump", "pre", "beta.0", version];
        test_ok(args, expect);

        let version = "release-1.0.0+20190722";
        let expect = "release-1.0.0+20190723";
        let args = vec!["bump", "build", "20190723", version];
        test_ok(args, expect);
    }

    #[test]
    fn version_invalid() {
        let version = "0.0";
        let args = vec!["bump", "patch", version];
        test_err(args);

        let version = "release";
        let args = vec!["bump", "patch", version];
        test_err(args);

        let version = "0-0-0";
        let args = vec!["bump", "patch", version];
        test_err(args);

        let version = "0.0.0@20190722";
        let args = vec!["bump", "patch", version];
        test_err(args);
    }

    #[test]
    fn file_ok() -> Result<()> {
        let version = "0.0.0";

        let tmp_dir = tempdir()?;
        let version_file = tmp_dir.path().join("version.txt");
        write_file(&version_file, version.as_bytes()).unwrap();

        let expect = "0.0.1";
        let args = vec!["bump", "patch", "-f", version_file.to_str().unwrap()];
        test_ok(args, expect);

        let expect = "0.1.0";
        let args = vec!["bump", "minor", "-f", version_file.to_str().unwrap()];
        test_ok(args, expect);

        let expect = "1.0.0";
        let args = vec!["bump", "major", "-f", version_file.to_str().unwrap()];
        test_ok(args, expect);

        Ok(())
    }
}
