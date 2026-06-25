use crate::workspace::repo::{Remote, Rev};
use anyhow::Result;
use std::path::Path;

pub mod libgit2;

/// These are the operations any VCS needs to support to be used with this tool
pub trait Backend {
    fn is_repo(&self, repo_dir: impl AsRef<Path>) -> bool;
    fn clone_repo(&self, remote: &Remote, repo_dir: impl AsRef<Path>) -> Result<()>;
    fn fetch(&self, repo_dir: impl AsRef<Path>) -> Result<()>;
    fn checkout(&self, rev: &Rev, repo_dir: impl AsRef<Path>) -> Result<()>;
}
