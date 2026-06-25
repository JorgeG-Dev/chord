use crate::manifest::Manifest;
use anyhow::Result;

pub mod repo;

/// Struct representing the Chord workspace
pub struct Workspace {
    pub repos: Vec<repo::Repo>,
}

/// We want a way of converting from a raw string to a valid repo remote
impl TryFrom<Manifest> for Workspace {
    type Error = anyhow::Error;
    fn try_from(manifest: Manifest) -> Result<Self> {
        let mut workspace_repos = vec![];
        for repo in manifest.repos {
            let parsed_repo = repo::Repo::try_from(repo)?;
            workspace_repos.push(parsed_repo);
        }
        Ok(Self {
            repos: workspace_repos,
        })
    }
}
