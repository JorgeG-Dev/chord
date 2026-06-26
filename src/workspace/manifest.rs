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
    /// Defaults to the name of the repo extracted from the remote.
    pub name: Option<String>,

    /// The directory to clone the repo to, defaults to the directory where
    /// the Chord manifest is located.
    pub location: Option<String>,
    ///
    /// The revision to checkout, can be a hash or branch. Defaults to main.
    pub rev: Option<Rev>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Rev {
    Tag(String),
    Hash(String),
    Branch(String),
}
