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
    pub url: String,
    /// The revision to checkout, can be a hash or branch. Defaults to main
    pub rev: Option<String>,
    /// The location in the Chord workspace to clone the repo to. Defaults to
    /// the repo name as the location
    pub name: Option<String>,
}
