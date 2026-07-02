use super::lockfile::Lockfile;
use anyhow::{Context, Result};
use std::fs::File;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

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
    pub location: Option<PathBuf>,
}

impl Manifest {
    /// Opens and deserializes the manifest file into a Manifest struct.
    pub fn read(top_dir: impl AsRef<Path>) -> Result<Self> {
        let manifest_file =
            File::open(top_dir.as_ref().join("chord.yaml")).context("failed to open manifest")?;
        let manifest =
            serde_saphyr::from_reader(manifest_file).context("failed to parse manifest")?;
        Ok(manifest)
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn test_repo(name: &str, revision: &str) -> Repo {
        Repo {
            remote: String::from("https://example.com/repo"),
            revision: String::from(revision),
            name: String::from(name),
            location: None,
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
}
