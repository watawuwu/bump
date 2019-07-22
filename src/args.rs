use crate::error::Result;
use crate::fs::read_from_stdin;
use crate::version::Version;
use atty::Stream;
use structopt::StructOpt;

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(name = "bump")]
pub struct Args {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, PartialEq, Debug)]
pub enum Command {
    /// Increment patch version
    #[structopt(name = "patch")]
    Patch {
        #[structopt(name = "SEMANTIC_VER")]
        ver: Version,
    },
    /// Increment minor version
    #[structopt(name = "minor")]
    Minor {
        #[structopt(name = "SEMANTIC_VER")]
        ver: Version,
    },
    /// Increment major version
    #[structopt(name = "major")]
    Major {
        #[structopt(name = "SEMANTIC_VER")]
        ver: Version,
    },
    /// Replace pre-release version
    #[structopt(name = "pre")]
    Pre {
        #[structopt(name = "PRERELEASE")]
        pre: String,
        #[structopt(name = "SEMANTIC_VER")]
        ver: Version,
    },
    /// Replace build metadata
    #[structopt(name = "build")]
    Build {
        #[structopt(name = "BUILD")]
        build: String,
        #[structopt(name = "SEMANTIC_VER")]
        ver: Version,
    },
}

impl Args {
    pub fn new(row_args: &[String]) -> Result<Args> {
        let replaced = Self::replace_stdin(row_args)?;
        let extended = Self::read_pipe(replaced.as_ref())?;
        let clap = Args::clap().get_matches_from_safe(extended)?;
        let args = Args::from_clap(&clap);

        Ok(args)
    }

    // todo type
    fn replace_stdin(input: &[String]) -> Result<Vec<String>> {
        let vec = if input.contains(&String::from("-")) {
            let buf = read_from_stdin()?;
            input
                .to_vec()
                .iter()
                .map(String::from)
                .map(|s| if s == "-" { buf.clone() } else { s })
                .collect::<Vec<_>>()
        } else {
            input.to_vec()
        };

        Ok(vec)
    }

    fn read_pipe(input: &[String]) -> Result<Vec<String>> {
        let vec = if !atty::is(Stream::Stdin) {
            let mut ex: Vec<String> = input.to_vec();
            let buf = read_from_stdin()?;
            ex.push(String::from(buf.trim_end()));
            ex
        } else {
            input.into()
        };
        Ok(vec)
    }
}
