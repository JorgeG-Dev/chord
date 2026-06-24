use serde::{Deserialize, Serialize};

/// Represents the complete Chord manifest file
#[derive(Serialize, Deserialize)]
pub struct File {
    /// The list of repos to be included in the Chord workspace
    pub repos: Vec<Repo>,
}

/// Represents a repo in a Chord workspace
#[derive(Serialize, Deserialize)]
pub struct Repo {
    /// The URL to the repo, can be either SSH or HTTPS
    pub remote: String,

    /// The revision to checkout, can be a hash or branch. Defaults to main
    pub rev: Option<String>,

    /// The name of the directory to clone the repo under, defaults to the
    /// name of the repository if not provided
    pub name: Option<String>,

    /// The directory to clone the repo to, defaults to the directory where
    /// the Chord manifest is located
    pub location: Option<String>,
}
