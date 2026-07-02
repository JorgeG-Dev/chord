use anyhow::Result;
use anyhow::bail;
use std::path::{Path, PathBuf};
use std::process::Command;

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

    fn repo_dir(&self, repo: &ManifestRepo) -> Result<PathBuf> {
        match &repo.location {
            Some(location) => {
                if location.is_absolute() {
                    bail!("repo has an absolute location, must be relative")
                }
                Ok(self.top_dir.join(location).join(&repo.name))
            }
            None => Ok(self.top_dir.join(&repo.name)),
        }
    }

    /// Resolves the specified repo to the revision specified in its manifest
    /// entry
    pub fn resolve_repo(&self, repo: &mut ManifestRepo) -> Result<()> {
        let repo_dir = self.repo_dir(&repo)?;

        if !self.backend.is_repo(&repo_dir) {
            self.backend.clone_repo(&repo.remote, &repo_dir)?;
        }
        self.backend.fetch(&repo_dir)?;
        repo.revision = self.backend.rev_as_hash(&repo_dir, &repo.revision)?;
        self.backend.checkout(&repo.revision, &repo_dir)?;
        Ok(())
    }

    pub fn repo_status(
        &self,
        repo: &ManifestRepo,
        locked_rev: impl AsRef<str>,
    ) -> Result<(bool, bool)> {
        let repo_dir = self.repo_dir(&repo)?;

        let (revision, is_dirty) = match self.backend.is_repo(&repo_dir) {
            true => (
                self.backend.get_current_hash(&repo_dir)?.as_str() == locked_rev.as_ref(),
                self.backend.is_dirty(&repo_dir)?,
            ),
            false => bail!("{} is not a valid repo", repo.name),
        };

        Ok((revision, is_dirty))
    }

    pub fn repo_run(&self, repo: &ManifestRepo, command: impl AsRef<str>) -> Result<()> {
        let repo_dir = self.repo_dir(&repo)?;
        if !self.backend.is_repo(&repo_dir) {
            bail!("{} is not a valid repo", repo.name);
        }
        match Command::new("sh")
            .arg("-c")
            .arg(command.as_ref())
            .current_dir(&repo_dir)
            .status()
        {
            Ok(status) => {
                if !status.success() {
                    bail!("error occurred during command execution");
                }
            }
            Err(_) => {
                bail!("failed to run command");
            }
        };

        Ok(())
    }
}
