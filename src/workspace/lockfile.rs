use serde::{Deserialize, Serialize};

/// Represents the complete Chord lockfile.
#[derive(Serialize, Deserialize)]
pub struct Lockfile {
    /// The list of repos to be included in the Chord lockfile.
    pub repos: Vec<Repo>,
}

/// Represents a repo in a Chord workspace
#[derive(Serialize, Deserialize)]
pub struct Repo {
    /// Name of folder where to clone the repo, also used as a way of referring
    /// to the repo from within the manifest for additional functionality.
    pub name: String,

    /// The revision to checkout, can be a hash or branch.
    pub revision: String,
}
