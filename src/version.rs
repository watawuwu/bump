use crate::error::AppError;
use failure::*;
use lazy_static::lazy_static;
use log::*;
use regex::Regex;
use semver::{Identifier, Version as SemVer};
use std::fmt;
use std::str::FromStr;

lazy_static! {
    static ref PREFIX: Regex =
        Regex::new(r"^(?P<prefix>.*?)(?P<version>[0-9].[0-9].[0-9](?:.*)$)").unwrap();
}

#[derive(PartialEq, Debug, Clone)]
pub struct Version {
    prefix: String,
    ver: SemVer,
}

impl FromStr for Version {
    type Err = AppError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = PREFIX
            .captures(s)
            .ok_or_else(|| format_err!("Can't find semver format. value: {}", s))?;

        let cap_pre = caps.name("prefix");
        let cap_ver = caps.name("version");

        let (prefix, version) = match (cap_pre, cap_ver) {
            (Some(p), Some(v)) => (p.as_str(), v.as_str()),
            (None, Some(v)) => ("", v.as_str()),
            _ => bail!("Can't find semver format. value: {}", s),
        };

        debug!("prefix: {}", prefix);
        debug!("version: {}", version);

        let ver = SemVer::parse(version)?;
        Ok(Version {
            prefix: prefix.to_string(),
            ver,
        })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.prefix, self.ver)
    }
}

impl Version {
    pub fn bump_patch(&self) -> Version {
        let mut v = self.clone();
        v.ver.increment_patch();
        v
    }

    pub fn bump_minor(&self) -> Version {
        let mut v = self.clone();
        v.ver.increment_minor();
        v
    }

    pub fn bump_major(&self) -> Version {
        let mut v = self.clone();
        v.ver.increment_major();
        v
    }

    pub fn update_pre_release(&self, pre: impl Into<String>) -> Version {
        let mut v = self.clone();
        v.ver.pre = vec![Identifier::AlphaNumeric(pre.into())];
        v
    }

    pub fn update_build(&self, build: impl Into<String>) -> Version {
        let mut v = self.clone();
        v.ver.build = vec![Identifier::AlphaNumeric(build.into())];
        v
    }
}
