use crate::fs::*;
use crate::version::Version;
use anyhow::{bail, Error, Result};
use atty::Stream;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::result;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(name = "bump")]
struct RawArgs {
    #[structopt(subcommand)]
    sub: RawSubCommand,
}

#[derive(StructOpt, PartialEq, Debug)]
enum RawSubCommand {
    /// Increment patch version
    #[structopt(name = "patch")]
    Patch {
        /// Semver file
        #[structopt(short, long)]
        file: Option<String>,

        #[structopt(name = "SEMANTIC_VER")]
        ver: Option<String>,
    },
    /// Increment minor version
    #[structopt(name = "minor")]
    Minor {
        /// Semver file
        #[structopt(short, long)]
        file: Option<String>,

        #[structopt(name = "SEMANTIC_VER")]
        ver: Option<String>,
    },
    /// Increment major version
    #[structopt(name = "major")]
    Major {
        /// Semver file
        #[structopt(short, long)]
        file: Option<String>,

        #[structopt(name = "SEMANTIC_VER")]
        ver: Option<String>,
    },
    /// Replace pre-release version
    #[structopt(name = "pre")]
    Pre {
        /// Semver file
        #[structopt(short, long)]
        file: Option<String>,

        #[structopt(name = "PRERELEASE")]
        pre: String,
        #[structopt(name = "SEMANTIC_VER")]
        ver: Option<String>,
    },
    /// Replace build metadata
    #[structopt(name = "build")]
    Build {
        /// Semver file
        #[structopt(short, long)]
        file: Option<String>,

        #[structopt(name = "BUILD")]
        build: String,
        #[structopt(name = "SEMANTIC_VER")]
        ver: Option<String>,
    },
}

pub struct Args {
    pub sub: SubCommand,
}

impl Args {
    pub fn new(row_args: &[String]) -> Result<Args> {
        let mut app = RawArgs::clap();
        let mut buf: Vec<u8> = Vec::new();
        app.write_long_help(&mut buf)?;

        let clap = app.get_matches_from_safe(row_args)?;
        let args = RawArgs::from_clap(&clap);
        let sub = match args.sub.try_into() {
            Ok(s) => s,
            Err(e) => {
                bail!("{}\n{}", e, String::from_utf8(buf)?);
            }
        };
        let cmd = Args { sub };

        Ok(cmd)
    }
}

pub enum SubCommand {
    Patch { ver: Version },
    Minor { ver: Version },
    Major { ver: Version },
    Pre { pre: String, ver: Version },
    Build { build: String, ver: Version },
}

impl SubCommand {
    fn version(file: Option<String>, ver: Option<String>) -> Result<Version> {
        let v = match (file, ver) {
            (None, Some(ref ver)) if ver == "-" => {
                let buf = read_from_stdin()?;
                Version::from_str(buf.trim_end())?
            }
            (None, None) if !atty::is(Stream::Stdin) => {
                let buf = read_from_stdin()?;
                Version::from_str(buf.trim_end())?
            }
            (None, Some(ref ver)) => {
                //
                Version::from_str(ver)?
            }
            (Some(_), Some(_)) => {
                bail!("Invalid argument, specify eithher ver args or file option")
            }
            (Some(f), _) => {
                let buf = String::from_utf8(read_file(f)?)?;
                Version::from_str(buf.trim_end())?
            }
            _ => bail!("Invalid argument"),
        };
        Ok(v)
    }
}

impl TryFrom<RawSubCommand> for SubCommand {
    type Error = Error;

    fn try_from(raw_sub: RawSubCommand) -> result::Result<Self, Self::Error> {
        let sub = match raw_sub {
            RawSubCommand::Patch { file, ver } => SubCommand::Patch {
                ver: Self::version(file, ver)?,
            },
            RawSubCommand::Minor { file, ver } => SubCommand::Minor {
                ver: Self::version(file, ver)?,
            },
            RawSubCommand::Major { file, ver } => SubCommand::Major {
                ver: Self::version(file, ver)?,
            },
            RawSubCommand::Pre { file, pre, ver } => SubCommand::Pre {
                pre,
                ver: Self::version(file, ver)?,
            },
            RawSubCommand::Build { file, build, ver } => SubCommand::Build {
                build,
                ver: Self::version(file, ver)?,
            },
        };

        Ok(sub)
    }
}
