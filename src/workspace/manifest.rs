use super::lockfile::Lockfile;
use anyhow::{Context, Result, bail};
use std::collections::HashSet;
use std::fs::File;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

const DEFAULT_LOCATION: &str = ".";
fn default_location() -> PathBuf {
    PathBuf::from(".")
}

/// Represents the complete Chord manifest file.
#[derive(Serialize, Deserialize)]
pub struct Manifest {
    /// The list of repos to be included in the Chord workspace.
    pub repos: Vec<Repo>,
}

/// Represents a repo in a Chord workspace
#[derive(Serialize, Deserialize)]
pub struct Repo {
    /// The URL to the repo, can be either SSH or HTTPS.
    pub remote: String,

    /// Name of folder where to clone the repo, also used as a way of referring
    /// to the repo from within the manifest for additional functionality.
    pub name: String,

    /// The revision to checkout, can be a hash or branch.
    pub revision: String,

    /// The directory to clone the repo to, defaults to the directory where
    /// the Chord manifest is located.
    #[serde(default = "default_location")]
    pub location: PathBuf,
}

impl Manifest {
    /// Initializes a manifest struct with an empty repos vector.
    pub fn new() -> Self {
        Manifest { repos: vec![] }
    }

    /// Opens and deserializes the manifest file into a Manifest struct.
    pub fn read(top_dir: impl AsRef<Path>) -> Result<Self> {
        let manifest_file =
            File::open(top_dir.as_ref().join("chord.yaml")).context("failed to open manifest")?;
        let manifest: Manifest =
            serde_saphyr::from_reader(manifest_file).context("failed to parse manifest")?;
        let mut unique = HashSet::new();
        manifest.repos.iter().all(|repo| unique.insert(&repo.name));
        if unique.len() == manifest.repos.len() {
            Ok(manifest)
        } else {
            bail!("invalid manifest, not all repo names are unique");
        }
    }

    /// Creates a manifest file of the current configuration, overwriting
    /// any existing one in the provided directory.
    pub fn write(&mut self, top_dir: impl AsRef<Path>) -> Result<()> {
        let mut manifest_file = File::create(top_dir.as_ref().join("chord.yaml"))?;
        serde_saphyr::to_io_writer(&mut manifest_file, &self)?;
        Ok(())
    }

    /// Updates the manifest with the revisions in the lockfile. This is a
    /// destructive action, meaning the lockfile gets emptied out into the
    /// manifest.
    pub fn apply_lock(&mut self, lockfile: &mut Lockfile) {
        for repo in &mut self.repos {
            if let Some(revision) = lockfile.remove(&repo.name) {
                repo.revision = revision;
            }
        }
    }

    /// Creates a new repo struct and inserts it into the manifest
    /// file. Repo name must not already exist in manifest.
    pub fn add_repo(
        &mut self,
        name: String,
        remote: String,
        revision: String,
        location: Option<PathBuf>,
    ) -> Result<()> {
        for i in 0..self.repos.len() {
            if self.repos[i].name == name {
                bail!("'{}' repo already exists in manifest", name);
            }
        }
        self.repos.push(Repo {
            name,
            remote,
            revision,
            location: match location {
                Some(location) => location,
                None => PathBuf::from(DEFAULT_LOCATION),
            },
        });
        Ok(())
    }

    /// Removes a repo with the specified name from the manifest,
    /// returns error if repo was not found.
    pub fn remove_repo(&mut self, name: String) -> Result<()> {
        for i in 0..self.repos.len() {
            if self.repos[i].name == name {
                self.repos.remove(i);
                return Ok(());
            }
        }
        bail!("'{}' repo does not exist in the manifest", name)
    }

    /// Modifies the specified repo with the provided values. Returns
    /// an error if was not found.
    pub fn modify_repo(
        &mut self,
        name: String,
        new_name: Option<String>,
        new_remote: Option<String>,
        new_revision: Option<String>,
        new_location: Option<PathBuf>,
    ) -> Result<()> {
        for i in 0..self.repos.len() {
            if self.repos[i].name == name {
                if let Some(new_name) = new_name {
                    self.repos[i].name = new_name
                }

                if let Some(new_remote) = new_remote {
                    self.repos[i].remote = new_remote
                }

                if let Some(new_revision) = new_revision {
                    self.repos[i].revision = new_revision
                }

                if let Some(new_location) = new_location {
                    self.repos[i].location = new_location
                }
                return Ok(());
            }
        }
        bail!("'{}' repo does not exist in the manifest", name)
    }
}

/// Uses new to define default instance of struct.
impl Default for Manifest {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use tempfile;

    fn test_repo(name: &str, revision: &str) -> Repo {
        Repo {
            remote: String::from("https://example.com/repo"),
            revision: String::from(revision),
            name: String::from(name),
            location: default_location(),
        }
    }

    #[test]
    fn test_apply_lock_overwrites_matching_repo_revision() {
        let mut manifest = Manifest {
            repos: vec![test_repo("repo-a", "main")],
        };
        let mut lockfile = Lockfile::new();
        lockfile.insert(String::from("repo-a"), String::from("abc123"));

        manifest.apply_lock(&mut lockfile);

        assert_eq!(manifest.repos[0].revision, "abc123");
    }

    #[test]
    fn test_apply_lock_leaves_unmatched_repo_untouched() {
        let mut manifest = Manifest {
            repos: vec![test_repo("repo-a", "main")],
        };
        let mut lockfile = Lockfile::new();
        lockfile.insert(String::from("repo-b"), String::from("abc123"));

        manifest.apply_lock(&mut lockfile);

        assert_eq!(manifest.repos[0].revision, "main");
    }

    #[test]
    fn test_apply_lock_with_empty_lockfile_changes_nothing() {
        let mut manifest = Manifest {
            repos: vec![test_repo("repo-a", "main")],
        };
        let mut lockfile = Lockfile::new();

        manifest.apply_lock(&mut lockfile);

        assert_eq!(manifest.repos[0].revision, "main");
    }

    #[test]
    fn test_read_with_duplicate_repos() {
        let dir = tempfile::tempdir().unwrap();
        let test_manifest = Manifest {
            repos: vec![test_repo("repo-a", "main"), test_repo("repo-a", "main")],
        };
        let mut writer = File::create(dir.path().join("chord.yaml")).unwrap();
        serde_saphyr::to_io_writer(&mut writer, &test_manifest).unwrap();

        assert!(Manifest::read(dir.path()).is_err())
    }

    #[test]
    fn test_remove_repo_inexistent_repo() {
        let mut test_manifest = Manifest {
            repos: vec![test_repo("repo-a", "main"), test_repo("repo-b", "main")],
        };
        assert!(test_manifest.remove_repo(String::from("repo-c")).is_err());
    }

    #[test]
    fn test_add_repo_already_exists() {
        let mut test_manifest = Manifest {
            repos: vec![test_repo("repo-a", "main"), test_repo("repo-b", "main")],
        };
        assert!(
            test_manifest
                .add_repo(
                    String::from("repo-a"),
                    String::from("url"),
                    String::from("rev"),
                    None
                )
                .is_err()
        );
    }

    #[test]
    fn test_modify_repo_inexistent_repo() {
        let mut test_manifest = Manifest {
            repos: vec![test_repo("repo-a", "main"), test_repo("repo-b", "main")],
        };
        // Arguments don't matter, repo won't be found
        assert!(
            test_manifest
                .modify_repo(String::from("repo-c"), None, None, None, None)
                .is_err()
        );
    }
}
