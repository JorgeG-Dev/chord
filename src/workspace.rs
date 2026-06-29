use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

mod git;
mod lockfile;
mod manifest;
pub mod utils;

pub use git::GitBackend;
pub use git::Operations as GitOperations;
pub use lockfile::Lockfile;
pub use lockfile::Repo as LockedRepo;
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

/// The operations that can be performed by a Workspace struct
pub trait Operations {
    /// Gets the workspace's top directory where the manifest is located
    fn top_dir(&self) -> &Path;
    /// Gets the backend used for all git related operations
    fn git(&self) -> &impl GitOperations;
}

impl Operations for Workspace {
    fn top_dir(&self) -> &Path {
        &self.top_dir
    }

    fn git(&self) -> &impl GitOperations {
        &self.backend
    }
}

impl Workspace {
    pub fn new(backend: GitBackend) -> Result<Self> {
        let top_dir = match utils::get_top_dir() {
            Some(dir) => dir,
            None => bail!("Not within a Chord workspace"),
        };
        Ok(Self { top_dir, backend })
    }
}
