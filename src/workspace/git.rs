use anyhow::Result;
use std::path::Path;

mod libgit2;

pub use libgit2::Git2Backend as GitBackend;

/// These are the git operations a git backend has to support for proper functionality
pub trait Operations {
    fn is_repo(&self, repo_dir: impl AsRef<Path>) -> bool;
    fn rev_as_hash(&self, repo_dir: impl AsRef<Path>, rev: &str) -> Result<String>;
    fn clone_repo(&self, remote: &str, repo_dir: impl AsRef<Path>) -> Result<()>;
    fn fetch(&self, repo_dir: impl AsRef<Path>) -> Result<()>;
    fn checkout(&self, hash: &str, repo_dir: impl AsRef<Path>) -> Result<()>;
    fn get_current_hash(&self, repo_dir: impl AsRef<Path>) -> Result<String>;
    fn is_dirty(&self, repo_dir: impl AsRef<Path>) -> Result<bool>;
}
