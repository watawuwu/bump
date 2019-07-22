use crate::error::AppError;
use semver::{Identifier, Version as SemVer};
use std::fmt;
use std::str::FromStr;

const SHORTHAND_VERSION_PREFIX: &str = "v";

#[derive(PartialEq, Debug, Clone)]
pub struct Version {
    has_prefix: bool,
    ver: SemVer,
}

impl FromStr for Version {
    type Err = AppError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (prefix, version) = if s.starts_with(SHORTHAND_VERSION_PREFIX) {
            (true, s.trim_start_matches(SHORTHAND_VERSION_PREFIX))
        } else {
            (false, s)
        };

        let ver = SemVer::parse(version)?;
        Ok(Version { prefix, ver })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = if self.has_prefix {
            SHORTHAND_VERSION_PREFIX
        } else {
            ""
        };
        write!(f, "{}{}", prefix, self.ver)
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
