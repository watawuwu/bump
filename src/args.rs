use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, next_line_help = true, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub(crate) sub: SubCommand,
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// Increment patch version
    Patch {
        /// Semver file
        #[arg(short, long)]
        file: Option<String>,

        #[arg(name = "SEMANTIC_VER")]
        ver: Option<String>,
    },
    /// Increment minor version
    Minor {
        /// Semver file
        #[arg(short, long)]
        file: Option<String>,

        #[arg(name = "SEMANTIC_VER")]
        ver: Option<String>,
    },
    /// Increment major version
    Major {
        /// Semver file
        #[arg(short, long)]
        file: Option<String>,

        #[arg(name = "SEMANTIC_VER")]
        ver: Option<String>,
    },
    /// Replace pre-release version
    Pre {
        /// Semver file
        #[arg(short, long)]
        file: Option<String>,

        #[arg(name = "PRERELEASE")]
        pre: String,
        #[arg(name = "SEMANTIC_VER")]
        ver: Option<String>,
    },
    /// Replace build metadata
    Build {
        /// Semver file
        #[arg(short, long)]
        file: Option<String>,

        #[arg(name = "BUILD")]
        build: String,
        #[arg(name = "SEMANTIC_VER")]
        ver: Option<String>,
    },
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}
