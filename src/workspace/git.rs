use anyhow::Result;
use std::path::Path;

mod libgit2;

pub use libgit2::Git2Backend as GitBackend;

/// These are the git operations a git backend has to support for proper functionality
pub trait Operations {
    fn is_repo(&self, repo_dir: impl AsRef<Path>) -> bool;
    fn clone_repo(&self, remote: &str, repo_dir: impl AsRef<Path>) -> Result<()>;
    fn fetch(&self, repo_dir: impl AsRef<Path>) -> Result<()>;
    fn checkout(&self, rev: &str, repo_dir: impl AsRef<Path>) -> Result<()>;
}
