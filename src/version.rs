use crate::fs::{read_file, read_from_stdin};
use anyhow::Result;
use anyhow::{anyhow, bail, Error};

use log::*;
use once_cell::sync::Lazy;
use regex::Regex;
use semver::{BuildMetadata, Prerelease, Version as SemVer};
use std::fmt;
use std::io::IsTerminal;
use std::str::FromStr;

static PREFIX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?P<prefix>.*?)(?P<version>[0-9]+?.[0-9]+?.[0-9]+?(?:.*)$)").unwrap()
});

#[derive(PartialEq, Debug, Clone)]
pub struct Version {
    prefix: String,
    ver: SemVer,
}

impl FromStr for Version {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = PREFIX
            .captures(s)
            .ok_or_else(|| anyhow!("Can't find semver format. value: {}", s))?;

        let cap_pre = caps.name("prefix");
        let cap_ver = caps.name("version");

        let (prefix, version) = match (cap_pre, cap_ver) {
            (Some(p), Some(v)) => (p.as_str(), v.as_str()),
            (None, Some(v)) => ("", v.as_str()),
            _ => bail!("Can't find semver format. value: {}", s),
        };

        debug!("prefix: {}", prefix);
        debug!("version: {}", version);

        Ok(Version {
            prefix: prefix.to_string(),
            ver: SemVer::parse(version)?,
        })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.prefix, self.ver)
    }
}

impl TryFrom<(Option<String>, Option<String>)> for Version {
    type Error = Error;

    fn try_from(value: (Option<String>, Option<String>)) -> std::result::Result<Self, Self::Error> {
        let v = match value {
            (Some(ref ver), None) if ver == "-" => {
                let buf = read_from_stdin()?;
                Version::from_str(buf.trim_end())?
            }
            (None, None) if !std::io::stdin().is_terminal() => {
                let buf = read_from_stdin()?;
                Version::from_str(buf.trim_end())?
            }
            (None, Some(ref ver)) => Version::from_str(ver)?,
            (Some(_), Some(_)) => {
                bail!("Invalid argument, specify either ver args or file option")
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

impl Version {
    pub fn bump_patch(&self) -> Version {
        let mut v = self.clone();
        v.ver.patch += 1;
        v.ver.pre = Prerelease::EMPTY;
        v.ver.build = BuildMetadata::EMPTY;
        v
    }

    pub fn bump_minor(&self) -> Version {
        let mut v = self.clone();
        v.ver.minor += 1;
        v.ver.patch = 0;
        v.ver.pre = Prerelease::EMPTY;
        v.ver.build = BuildMetadata::EMPTY;
        v
    }

    pub fn bump_major(&self) -> Version {
        let mut v = self.clone();
        v.ver.major += 1;
        v.ver.minor = 0;
        v.ver.patch = 0;
        v.ver.pre = Prerelease::EMPTY;
        v.ver.build = BuildMetadata::EMPTY;
        v
    }

    pub fn update_pre_release(&self, pre: impl Into<String>) -> Result<Version> {
        let mut v = self.clone();
        v.ver.pre = Prerelease::new(pre.into().as_str())?;
        Ok(v)
    }

    pub fn update_build(&self, build: impl Into<String>) -> Result<Version> {
        let mut v = self.clone();
        v.ver.build = BuildMetadata::new(build.into().as_str())?;
        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_ok() {
        let result = Version::from_str("0.0.0");
        assert!(result.is_ok());
    }

    #[test]
    fn from_str_ng() {
        let inputs = vec!["x.x.x", "vx.x.x", "x", "x.x"];
        for input in inputs {
            let result = Version::from_str(input);
            assert!(result.is_err());
            let msg = result.expect_err("").to_string();
            assert!(msg.contains("Can't find semver format"));
        }
    }

    #[test]
    fn try_from_ok() {
        let result = Version::try_from((None, Some(String::from("0.0.0"))));

        let prefix = String::from("");
        let ver = SemVer::new(0, 0, 0);
        let _version = Version { prefix, ver };
        assert!(matches!(result, Ok(_version)));
    }

    #[test]
    fn try_from_ng() {
        let inputs = vec![
            (Some(String::from("test.md")), Some(String::from("x.x.x"))),
            (None, None),
        ];
        for (file, ver) in inputs {
            let result = Version::try_from((file, ver));
            assert!(result.is_err());
        }
    }
}
