use crate::git;
use crate::workspace::repo::{Remote, Rev};
use anyhow::Result;
use git2::{BranchType, Oid, Repository};
use std::path::{Path, PathBuf};

/// Don't need any fields, just a target for implementing the backend
pub struct Git2Backend;

impl git::Backend for Git2Backend {
    fn is_repo(&self, repo_dir: impl AsRef<Path>) -> bool {
        Repository::open(repo_dir).is_ok()
    }
    fn clone_repo(&self, remote: &Remote, repo_dir: impl AsRef<Path>) -> Result<()> {
        Repository::clone(remote.as_str(), repo_dir)?;
        Ok(())
    }

    fn checkout(&self, rev: &Rev, repo_dir: impl AsRef<Path>) -> Result<()> {
        let repo = Repository::open(repo_dir)?;
        match rev {
            Rev::Hash(hash) => {
                let oid = Oid::from_str(hash)?;
                let commit = repo.find_commit(oid)?;
                repo.checkout_tree(&commit.as_object(), None)?;
                repo.set_head_detached(commit.id())?;
            }
            Rev::Tag(tag) => {
                let reference = repo.find_reference(format!("refs/tags/{}", tag).as_str())?;
                let commit = reference.peel_to_commit()?;
                repo.checkout_tree(&commit.as_object(), None)?;
                repo.set_head_detached(commit.id())?;
            }
            Rev::Branch(branch) => {
                let remote_branch =
                    repo.find_branch(format!("origin/{}", branch).as_str(), BranchType::Remote)?;
                let commit = remote_branch.get().peel_to_commit()?;
                repo.checkout_tree(&commit.as_object(), None)?;
                repo.set_head_detached(commit.id())?;
            }
        }
        Ok(())
    }

    fn fetch(&self, repo_dir: impl AsRef<Path>) -> Result<()> {
        let repo = Repository::open(repo_dir)?;
        let mut remote = repo.find_remote("origin")?;
        remote.fetch(&[] as &[&str], None, None)?;
        Ok(())
    }
}
