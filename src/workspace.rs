use anyhow::Result;
use std::path::{Path, PathBuf};

mod git;
mod lockfile;
mod manifest;
pub mod utils;

pub use git::GitBackend;
pub use git::Operations as GitOperations;
pub use lockfile::Lockfile;
pub use manifest::Manifest;
pub use manifest::Repo as ManifestRepo;

/// Represents the complete Chord workspace, created at the invokation of
/// the `chord` command.
pub struct Workspace {
    /// The root of the Chord workspace.
    top_dir: PathBuf,
    /// The Git operation backend used for the subcommands
    backend: GitBackend,
}

impl Workspace {
    pub fn new(top_dir: PathBuf, backend: GitBackend) -> Self {
        Self { top_dir, backend }
    }

    /// Returns a reference to the workspace's top directory.
    pub fn top_dir(&self) -> &Path {
        &self.top_dir
    }

    /// Resolves the specified repo to the revision specified in its manifest
    /// entry
    pub fn resolve_repo(&self, mut repo: ManifestRepo) -> Result<ManifestRepo> {
        let repo_dir = match &repo.location {
            Some(location) => self.top_dir.join(location).join(&repo.name),
            None => self.top_dir.join(&repo.name),
        };

        if !self.backend.is_repo(&repo_dir) {
            self.backend.clone_repo(&repo.remote, &repo_dir)?;
        }
        self.backend.fetch(&repo_dir)?;
        repo.revision = self.backend.rev_as_hash(&repo_dir, &repo.revision)?;
        self.backend.checkout(&repo.revision, &repo_dir)?;
        Ok(repo)
    }
}
