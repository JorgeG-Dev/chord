use crate::manifest;
use gix_url;
use std::path::{Path, PathBuf};

use anyhow::{Result, bail};

const HASH_LENGTH: usize = 40;

/// Represents a complete repo
#[derive(Clone, Debug)]
pub struct Repo {
    pub remote: Remote,
    pub rev: Rev,
    pub name: Name,
    pub location: Location,
}

/// Conversion from manifest to actual repo consists of converting each
/// individual field and setting them to a sane default if not provided
impl TryFrom<manifest::Repo> for Repo {
    type Error = anyhow::Error;
    fn try_from(raw_repo: manifest::Repo) -> Result<Self> {
        let remote = Remote::try_from(raw_repo.remote)?;
        let rev = match raw_repo.rev {
            Some(value) => Rev::try_from(value)?,
            None => Rev::try_from(manifest::Rev::Branch(String::from("main")))?,
        };
        let location = match raw_repo.location {
            Some(value) => Location::try_from(value)?,
            None => Location::try_from(String::from("."))?,
        };
        let name = match raw_repo.name {
            Some(value) => Name::try_from(value)?,
            None => Name(remote.repo_name()?),
        };

        Ok(Self {
            remote,
            rev,
            location,
            name,
        })
    }
}

/// Newtype wrapper around a string that represents a valid git repo url
#[derive(Clone, Debug)]
pub struct Remote(String);

/// We want a way of converting from a raw string to a valid repo remote
impl TryFrom<String> for Remote {
    type Error = anyhow::Error;
    fn try_from(raw: String) -> Result<Self> {
        // Don't really need the parsed object, just make sure the
        // raw string can be parsed
        let parsed = gix_url::parse(raw.as_bytes().into())?;
        match parsed.scheme {
            gix_url::Scheme::Http | gix_url::Scheme::Https => Ok(Self(raw)),
            _ => bail!("Unsupported git scheme: {}", parsed.scheme.to_string()),
        }
    }
}

impl Remote {
    pub fn repo_name(&self) -> Result<String> {
        let parsed = gix_url::parse(self.0.as_bytes().into())?;
        let repo_path = parsed.path.to_string();
        let name = repo_path
            .trim_end_matches('/')
            .trim_end_matches(".git")
            .rsplit('/')
            .next()
            .filter(|s| !s.is_empty())
            .ok_or_else(|| anyhow::anyhow!("Could not derive repo name from remote: {}", self.0))?;
        Ok(name.to_string())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Newtype wrapper around a string that represents a valid git revision.
/// Validation is loose since it can either be a hash or a branch name
#[derive(Clone, Debug)]
pub enum Rev {
    Hash(String),
    Branch(String),
    Tag(String),
}

/// Just checking the length
impl TryFrom<manifest::Rev> for Rev {
    type Error = anyhow::Error;
    fn try_from(raw: manifest::Rev) -> Result<Self> {
        let rev = match raw {
            manifest::Rev::Tag(tag) => {
                if tag.len() > 0 && !tag.contains(' ') {
                    Rev::Tag(tag)
                } else {
                    bail!("Empty tag specified")
                }
            }
            manifest::Rev::Branch(branch) => {
                if branch.len() > 0 && !branch.contains(' ') {
                    Rev::Branch(branch)
                } else {
                    bail!("Empty branch specified")
                }
            }
            manifest::Rev::Hash(hash) => {
                if hash.len() == HASH_LENGTH && hash.chars().all(|c| c.is_ascii_hexdigit()) {
                    Rev::Hash(hash)
                } else {
                    bail!("Invalid hash specified: {}", hash)
                }
            }
        };
        Ok(rev)
    }
}

impl Rev {
    pub fn as_str(&self) -> &str {
        match self {
            Rev::Hash(hash) => hash.as_str(),
            Rev::Branch(branch) => branch.as_str(),
            Rev::Tag(tag) => tag.as_str(),
        }
    }
}

/// Newtype wrapper around a string that represents a valid repo name,
/// basically just ensuring the name is not empty
#[derive(Clone, Debug)]
pub struct Name(String);

/// Just checking the length of the name isn't 0
impl TryFrom<String> for Name {
    type Error = anyhow::Error;
    fn try_from(raw: String) -> Result<Self> {
        if raw.len() > 0 {
            Ok(Self(raw))
        } else {
            bail!("Name length is 0")
        }
    }
}

impl Name {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Newtype wrapper around a string that represents a valid repo location,
/// meaning the path has to be relative. Absolute paths are not allowed
#[derive(Clone, Debug)]
pub struct Location(PathBuf);

/// Making sure the location provided is relative so it can be evaluated
/// correctly with respect to the topdir
impl TryFrom<String> for Location {
    type Error = anyhow::Error;
    fn try_from(raw: String) -> Result<Self> {
        if raw.len() == 0 {
            bail!("Location length is 0")
        }
        let repo_path = PathBuf::from(raw);
        if repo_path.is_absolute() {
            bail!("Location must be relative to the Chord workspace topdir")
        }
        Ok(Self(repo_path))
    }
}

impl Location {
    pub fn as_path(&self) -> &Path {
        self.0.as_path()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    mod remote {
        use super::*;
        use pretty_assertions::assert_eq;

        #[rstest]
        #[case("https://github.com/org/repo.git")]
        #[case("http://github.com/org/repo.git")]
        fn valid_remote(#[case] remote: String) {
            assert_eq!(true, Remote::try_from(remote).is_ok());
        }

        #[rstest]
        #[case("git@github.com:org/repo.git")]
        #[case("ssh://git@github.com/org/repo.git")]
        #[case("not-a-url")]
        #[case("")]
        #[case("scp://git@github.com/org/repo.git")]
        fn invalid_remote(#[case] remote: String) {
            assert_eq!(true, Remote::try_from(remote).is_err())
        }
    }

    mod location {
        use super::*;
        use pretty_assertions::assert_eq;

        #[rstest]
        #[case(".")]
        #[case("test")]
        fn valid_location(#[case] location: String) {
            assert_eq!(true, Location::try_from(location).is_ok())
        }

        #[rstest]
        #[case("/")]
        #[case("/etc")]
        #[case("")]
        fn invalid_location(#[case] location: String) {
            assert_eq!(true, Location::try_from(location).is_err())
        }
    }

    mod rev {
        use super::*;
        use pretty_assertions::assert_eq;

        #[rstest]
        #[case("main")]
        fn valid_branch(#[case] branch: String) {
            assert_eq!(
                true,
                Rev::try_from(manifest::Rev::Branch(String::from(branch))).is_ok()
            )
        }

        #[rstest]
        #[case("test branch")]
        #[case("")]
        #[case(" ")]
        fn invalid_branch(#[case] branch: String) {
            assert_eq!(
                true,
                Rev::try_from(manifest::Rev::Branch(String::from(branch))).is_err()
            )
        }

        #[rstest]
        #[case("v0.1.0")]
        fn valid_tag(#[case] tag: String) {
            assert_eq!(
                true,
                Rev::try_from(manifest::Rev::Tag(String::from(tag))).is_ok()
            )
        }

        #[rstest]
        #[case("test tag")]
        #[case("")]
        #[case(" ")]
        fn invalid_tag(#[case] tag: String) {
            assert_eq!(
                true,
                Rev::try_from(manifest::Rev::Tag(String::from(tag))).is_err()
            )
        }

        #[rstest]
        #[case("0123456789012345678901234567890123456789")]
        fn valid_hash(#[case] hash: String) {
            assert_eq!(
                true,
                Rev::try_from(manifest::Rev::Hash(String::from(hash))).is_ok()
            )
        }

        #[rstest]
        #[case("beef")]
        #[case("temp")]
        fn invalid_hash(#[case] hash: String) {
            assert_eq!(
                true,
                Rev::try_from(manifest::Rev::Hash(String::from(hash))).is_err()
            )
        }
    }

    mod name {
        use super::*;
        use pretty_assertions::assert_eq;

        #[rstest]
        #[case("repo_name")]
        fn valid_name(#[case] name: String) {
            assert_eq!(true, Name::try_from(name).is_ok())
        }

        #[rstest]
        #[case("")]
        fn invalid_name(#[case] name: String) {
            assert_eq!(true, Name::try_from(name).is_err())
        }
    }

    mod repo {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn missing_name() {
            let manifest_repo = manifest::Repo {
                remote: String::from("https://github.com/JorgeG-Dev/chord.git"),
                rev: Some(manifest::Rev::Branch(String::from("branch"))),
                location: Some(String::from("deps")),
                name: None,
            };

            let parsed_repo = Repo::try_from(manifest_repo).unwrap();
            assert_eq!(String::from("chord"), parsed_repo.name.0);
        }

        #[test]
        fn missing_location() {
            let manifest_repo = manifest::Repo {
                remote: String::from("https://github.com/JorgeG-Dev/chord.git"),
                rev: Some(manifest::Rev::Branch(String::from("branch"))),
                location: None,
                name: Some(String::from("name")),
            };

            let parsed_repo = Repo::try_from(manifest_repo).unwrap();
            assert_eq!(String::from("."), parsed_repo.location.0);
        }

        #[test]
        fn missing_rev() {
            let manifest_repo = manifest::Repo {
                remote: String::from("https://github.com/JorgeG-Dev/chord.git"),
                rev: None,
                location: Some(String::from("deps")),
                name: Some(String::from("name")),
            };

            let parsed_repo = Repo::try_from(manifest_repo).unwrap();
            assert_eq!(String::from("main"), parsed_repo.rev.as_str());
        }

        #[rstest]
        #[case("https://github.com/org/chord.git")]
        #[case("http://github.com/org/chord.git")]
        fn missing_all_except_remote(#[case] remote: String) {
            let manifest_repo = manifest::Repo {
                remote,
                rev: None,
                location: None,
                name: None,
            };
            let parsed_repo = Repo::try_from(manifest_repo).unwrap();
            assert_eq!(String::from("main"), parsed_repo.rev.as_str());
            assert_eq!(PathBuf::from("."), parsed_repo.location.as_path());
            assert_eq!(String::from("chord"), parsed_repo.name.as_str());
        }
    }
}
