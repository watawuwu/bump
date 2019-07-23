use crate::error::{AppError, Result};
use crate::fs::*;
use crate::version::Version;
use atty::Stream;
use failure::*;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::result;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(name = "bump")]
pub struct Args {
    #[structopt(subcommand)]
    cmd: RowCommand,
}

#[derive(StructOpt, PartialEq, Debug)]
enum RowCommand {
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

pub enum Command {
    Patch { ver: Version },
    Minor { ver: Version },
    Major { ver: Version },
    Pre { pre: String, ver: Version },
    Build { build: String, ver: Version },
}

impl Command {
    pub fn new(row_args: &[String]) -> Result<Command> {
        let clap = Args::clap().get_matches_from_safe(row_args)?;
        let args = Args::from_clap(&clap);
        let cmd = args.try_into()?;
        Ok(cmd)
    }
}

impl Command {
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
            _ => {
                // todo command help message
                bail!("Invalid argument")
            }
        };
        Ok(v)
    }
}

impl TryFrom<Args> for Command {
    type Error = AppError;

    fn try_from(args: Args) -> result::Result<Self, Self::Error> {
        let cmd = match args.cmd {
            RowCommand::Patch { file, ver } => Command::Patch {
                ver: Self::version(file, ver)?,
            },
            RowCommand::Minor { file, ver } => Command::Minor {
                ver: Self::version(file, ver)?,
            },
            RowCommand::Major { file, ver } => Command::Major {
                ver: Self::version(file, ver)?,
            },
            RowCommand::Pre { file, pre, ver } => Command::Pre {
                pre,
                ver: Self::version(file, ver)?,
            },
            RowCommand::Build { file, build, ver } => Command::Build {
                build,
                ver: Self::version(file, ver)?,
            },
        };

        Ok(cmd)
    }
}
