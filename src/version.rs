use crate::error::AppError;
use semver::{Identifier, Version as SemVer};
use std::fmt;
use std::str::FromStr;

const SHORTHAND_VERSION_PREFIX: &str = "v";

#[derive(PartialEq, Debug)]
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
        Ok(Version {
            has_prefix: prefix,
            ver,
        })
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
        let mut ver = self.ver.clone();
        ver.increment_patch();
        Version {
            has_prefix: self.has_prefix,
            ver,
        }
    }

    pub fn bump_minor(&self) -> Version {
        let mut ver = self.ver.clone();
        ver.increment_minor();
        Version {
            has_prefix: self.has_prefix,
            ver,
        }
    }

    pub fn bump_major(&self) -> Version {
        let mut ver = self.ver.clone();
        ver.increment_major();
        Version {
            has_prefix: self.has_prefix,
            ver,
        }
    }

    pub fn update_pre_release(&self, pre: impl Into<String>) -> Version {
        let mut ver = self.ver.clone();
        ver.pre = vec![Identifier::AlphaNumeric(pre.into())];
        Version {
            has_prefix: self.has_prefix,
            ver,
        }
    }

    pub fn update_build(&self, build: impl Into<String>) -> Version {
        let mut ver = self.ver.clone();
        ver.build = vec![Identifier::AlphaNumeric(build.into())];
        Version {
            has_prefix: self.has_prefix,
            ver,
        }
    }
}
