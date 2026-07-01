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
    pub fn read(top_dir: &impl AsRef<Path>) -> Result<Self> {
        let manifest_file =
            File::open(top_dir.as_ref().join("chord.yaml")).context("failed to open manifest")?;
        let manifest =
            serde_saphyr::from_reader(manifest_file).context("failed to parse manifest")?;
        Ok(manifest)
    }
}
