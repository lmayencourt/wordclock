/* SPDX-License-Identifier: MIT
 * Copyright (c) 2023 Louis Mayencourt
 */

use std::fmt;

use anyhow::{anyhow, Result};
use regex::Regex;

/// Regex for semantic version, as provided by https://semver.org
const SEMANTIC_REGEX: &str = r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$";

/// Semantic version representation
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
    identifiers: Option<String>,
}

impl Version {
    /// Return a Version structure
    pub fn new(major: u8, minor: u8, patch: u8, identifiers: Option<&str>) -> Self {
        if let Some(x) = identifiers {
            return Version {
                major,
                minor,
                patch,
                identifiers: Some(String::from(x)),
            };
        }

        Version {
            major,
            minor,
            patch,
            identifiers: None,
        }
    }

    /// Return a version represented by the given string
    ///
    /// # Error
    /// Return an error if provided string doesn't represent a semantic version.
    pub fn from_string(version: &str) -> Result<Self> {
        let re = Regex::new(SEMANTIC_REGEX)?;

        if let Some(cap) = re.captures(version) {
            let major: u8 = cap[1].parse()?;
            let minor: u8 = cap[2].parse()?;
            let patch: u8 = cap[3].parse()?;
            if let Some(meta) = cap.name("prerelease") {
                return Ok(Version {
                    major,
                    minor,
                    patch,
                    identifiers: Some(String::from(meta.as_str())),
                });
            } else {
                return Ok(Version {
                    major,
                    minor,
                    patch,
                    identifiers: None,
                });
            }
        } else {
            return Err(anyhow!("Provided input is not a valid version {}", version));
        }
    }

    /// Compare against the provided version
    ///
    /// Return `true` if provided version is older.
    /// Ignore the 'identifiers' part.
    pub fn is_greater_than(&self, other: &Version) -> bool {
        if self.major > other.major {
            return true;
        } else if self.minor > other.minor {
            return true;
        } else if self.patch > other.patch {
            return true;
        } else {
            return false;
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(id) = &self.identifiers {
            write!(f, "{}:{}:{}-{}", self.major, self.minor, self.patch, id)
        } else {
            write!(f, "{}:{}:{}", self.major, self.minor, self.patch)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_version() {
        let version = Version::new(1, 2, 3, None);
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
        assert_eq!(version.identifiers, None);
    }

    #[test]
    fn from_string() {
        let version = Version::from_string("1.2.3").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
        assert_eq!(version.identifiers, None);

        let version = Version::from_string("1.2.3-rc1").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
        assert_eq!(version.identifiers, Some(String::from("rc1")));
    }

    #[test]
    fn release_candidate() {
        let version = Version::new(1, 2, 3, Some("rc1"));
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
        assert_eq!(version.identifiers, Some(String::from("rc1")));
    }

    #[test]
    fn compare_two_version() {
        let v1 = Version::new(1, 0, 0, None);
        let v2 = Version::new(2, 0, 0, None);
        assert!(v2.is_greater_than(&v1));

        let v2 = Version::new(2, 1, 0, None);
        assert!(v2.is_greater_than(&v1));

        let v2 = Version::new(2, 0, 1, None);
        assert!(v2.is_greater_than(&v1));

        let v2 = Version::new(1, 0, 0, None);
        assert_eq!(v2.is_greater_than(&v1), false);
    }
}
