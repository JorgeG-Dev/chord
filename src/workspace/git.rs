pub(super) use super::repo::*;
use anyhow::Result;
use std::path::Path;

mod libgit2;

pub use libgit2::Git2Backend as GitBackend;

/// These are the operations any VCS needs to support to be used with this tool
pub trait Operations {
    fn is_repo(&self, repo_dir: impl AsRef<Path>) -> bool;
    fn clone_repo(&self, remote: &Remote, repo_dir: impl AsRef<Path>) -> Result<()>;
    fn fetch(&self, repo_dir: impl AsRef<Path>) -> Result<()>;
    fn checkout(&self, rev: &Rev, repo_dir: impl AsRef<Path>) -> Result<()>;
}
