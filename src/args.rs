use crate::version::Version;
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
