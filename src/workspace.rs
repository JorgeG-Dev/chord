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

    pub fn get_top_dir(&self) -> &Path {
        &self.top_dir
    }

    pub fn sync(&self, mut repo: ManifestRepo) -> Result<ManifestRepo> {
        let location = repo
            .location
            .as_ref()
            .map(|l| self.top_dir.join(l))
            .unwrap_or_else(|| self.top_dir.to_path_buf());
        let repo_dir = PathBuf::from(&self.top_dir)
            .join(location)
            .join(repo.name.as_str());

        if !self.backend.is_repo(&repo_dir) {
            self.backend.clone_repo(&repo.remote, &repo_dir)?;
        }
        self.backend.fetch(&repo_dir)?;
        repo.revision = self.backend.rev_as_hash(&repo_dir, &repo.revision)?;
        self.backend.checkout(&repo.revision, &repo_dir)?;
        Ok(repo)
    }
}
