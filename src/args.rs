use clap::builder::{styling, Styles};
use clap::{Parser, Subcommand};

fn help_styles() -> Styles {
    styling::Styles::styled()
        .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Cyan.on_default())
}

#[derive(Parser)]
#[command(author, version, about, next_line_help = true, long_about = None, styles(help_styles()))]
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
