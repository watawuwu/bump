mod args;
use crate::args::{Args, SubCommand};
use bump_bin::version::Version;
use clap::Parser;
use log::*;

use anyhow::{bail, Result};
use std::process::exit;

const EXIT_CODE_OK: i32 = 0;
const EXIT_CODE_USAGE: i32 = 2;

fn run(args: Args) -> Result<String> {
    let subcommand = move || -> Result<Version> {
        let version = match args.sub {
            SubCommand::Patch { file, ver } => {
                let ver = Version::try_from((file, ver))?;
                ver.bump_patch()
            }
            SubCommand::Minor { file, ver } => {
                let ver = Version::try_from((file, ver))?;
                ver.bump_minor()
            }
            SubCommand::Major { file, ver } => {
                let ver = Version::try_from((file, ver))?;
                ver.bump_major()
            }
            SubCommand::Pre { file, pre, ver } => {
                let ver = Version::try_from((file, ver))?;
                ver.update_pre_release(pre)?
            }
            SubCommand::Build { file, build, ver } => {
                let ver = Version::try_from((file, ver))?;
                ver.update_build(build)?
            }
        };

        Ok(version)
    };

    let version = match subcommand() {
        Ok(v) => v,
        Err(err) => bail!("{err}"),
    };
    debug!("version: {:?}", &version);
    Ok(version.to_string())
}

fn main() {
    pretty_env_logger::init();

    let args = Args::parse();

    let code = match run(args) {
        Ok(view) => {
            println!("{view}");
            EXIT_CODE_OK
        }
        Err(err) => {
            eprintln!("{err}");
            EXIT_CODE_USAGE
        }
    };
    exit(code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bump_bin::fs::*;
    use tempfile::tempdir;

    fn test_ok(row_args: Vec<&str>, expect: &str) -> Result<()> {
        let args = Args::try_parse_from(row_args)?;

        let actual = run(args);
        assert_eq!(actual.unwrap(), String::from(expect));
        Ok(())
    }

    fn test_err(row_args: Vec<&str>) -> Result<()> {
        let args = Args::try_parse_from(row_args)?;

        let actual = run(args);
        assert!(actual.is_err());
        Ok(())
    }

    #[test]
    fn inc_patch_ok() -> Result<()> {
        let version = "0.1.0";
        let expect = "0.1.1";
        let args = vec!["bump", "patch", version];
        test_ok(args, expect)?;

        let version = "0.10.0";
        let expect = "0.10.1";
        let args = vec!["bump", "patch", version];
        test_ok(args, expect)?;

        let version = "10000.10000.10000";
        let expect = "10000.10000.10001";
        let args = vec!["bump", "patch", version];
        test_ok(args, expect)?;
        Ok(())
    }

    #[test]
    fn inc_minor_ok() -> Result<()> {
        let version = "0.1.1";
        let expect = "0.2.0";
        let args = vec!["bump", "minor", version];
        test_ok(args, expect)?;

        let version = "0.10.0";
        let expect = "0.11.0";
        let args = vec!["bump", "minor", version];
        test_ok(args, expect)?;

        let version = "10000.10000.10000";
        let expect = "10000.10001.0";
        let args = vec!["bump", "minor", version];
        test_ok(args, expect)?;

        Ok(())
    }

    #[test]
    fn inc_major_ok() -> Result<()> {
        let version = "0.1.1";
        let expect = "1.0.0";
        let args = vec!["bump", "major", version];
        test_ok(args, expect)?;

        let version = "0.10.0";
        let expect = "1.0.0";
        let args = vec!["bump", "major", version];
        test_ok(args, expect)?;

        let version = "10000.10000.10000";
        let expect = "10001.0.0";
        let args = vec!["bump", "major", version];
        test_ok(args, expect)?;

        Ok(())
    }

    #[test]
    fn remove_pre_ok() -> Result<()> {
        let version = "1.0.0-alpha.0";
        let expect = "1.0.1";
        let args = vec!["bump", "patch", version];
        test_ok(args, expect)?;

        Ok(())
    }

    #[test]
    fn add_pre_ok() -> Result<()> {
        let version = "1.0.0";
        let pre = "alpha.0";
        let expect = "1.0.0-alpha.0";
        let args = vec!["bump", "pre", pre, version];
        test_ok(args, expect)?;

        Ok(())
    }

    #[test]
    fn replace_pre_ok() -> Result<()> {
        let version = "1.0.0-alpha.0";
        let pre = "beta.0";
        let expect = "1.0.0-beta.0";
        let args = vec!["bump", "pre", pre, version];
        test_ok(args, expect)?;

        Ok(())
    }

    #[test]
    fn remove_build_ok() -> Result<()> {
        let version = "1.0.0+20190722";
        let expect = "1.0.1";
        let args = vec!["bump", "patch", version];
        test_ok(args, expect)?;

        Ok(())
    }

    #[test]
    fn add_build() -> Result<()> {
        let version = "1.0.0";
        let build = "20190722";
        let expect = "1.0.0+20190722";
        let args = vec!["bump", "build", build, version];
        test_ok(args, expect)?;

        Ok(())
    }

    #[test]
    fn replace_build_ok() -> Result<()> {
        let version = "1.0.0+20190722";
        let build = "20190723";
        let expect = "1.0.0+20190723";
        let args = vec!["bump", "build", build, version];
        test_ok(args, expect)?;

        Ok(())
    }

    #[test]
    fn ignore_default_prefix_ok() -> Result<()> {
        let version = "v1.0.0";
        let expect = "v2.0.0";
        let args = vec!["bump", "major", version];
        test_ok(args, expect)?;

        Ok(())
    }

    #[test]
    fn prefix_ok() -> Result<()> {
        let version = "release-1.0.0";
        let expect = "release-2.0.0";
        let args = vec!["bump", "major", version];
        test_ok(args, expect)?;

        let version = "release-1.0.0";
        let expect = "release-1.1.0";
        let args = vec!["bump", "minor", version];
        test_ok(args, expect)?;

        let version = "release-1.0.0";
        let expect = "release-1.0.1";
        let args = vec!["bump", "patch", version];
        test_ok(args, expect)?;

        let version = "release-1.0.0-alpha.0";
        let expect = "release-1.0.0-beta.0";
        let args = vec!["bump", "pre", "beta.0", version];
        test_ok(args, expect)?;

        let version = "release-1.0.0+20190722";
        let expect = "release-1.0.0+20190723";
        let args = vec!["bump", "build", "20190723", version];
        test_ok(args, expect)?;

        Ok(())
    }

    #[test]
    fn version_invalid() -> Result<()> {
        let version = "0.0";
        let args = vec!["bump", "patch", version];
        test_err(args)?;

        let version = "release";
        let args = vec!["bump", "patch", version];
        test_err(args)?;

        let version = "0-0-0";
        let args = vec!["bump", "patch", version];
        test_err(args)?;

        let version = "0.0.0@20190722";
        let args = vec!["bump", "patch", version];
        test_err(args)?;

        Ok(())
    }

    #[test]
    fn file_ok() -> Result<()> {
        let version = "0.0.0";

        let tmp_dir = tempdir()?;
        let version_file = tmp_dir.path().join("version.txt");
        write_file(&version_file, version.as_bytes())?;

        let expect = "0.0.1";
        let args = vec!["bump", "patch", "-f", version_file.to_str().unwrap()];
        test_ok(args, expect)?;

        let expect = "0.1.0";
        let args = vec!["bump", "minor", "-f", version_file.to_str().unwrap()];
        test_ok(args, expect)?;

        let expect = "1.0.0";
        let args = vec!["bump", "major", "-f", version_file.to_str().unwrap()];
        test_ok(args, expect)?;

        Ok(())
    }
}
