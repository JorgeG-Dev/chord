use super::*;
use anyhow::Result;
use git2::Repository;
use std::path::Path;

/// Don't need any fields, just a target for implementing the backend
pub struct Git2Backend;

impl Operations for Git2Backend {
    fn is_repo(&self, repo_dir: impl AsRef<Path>) -> bool {
        Repository::open(repo_dir).is_ok()
    }

    fn clone_repo(&self, remote: &str, repo_dir: impl AsRef<Path>) -> Result<()> {
        Repository::clone(remote, repo_dir)?;
        Ok(())
    }

    fn checkout(&self, rev: &str, repo_dir: impl AsRef<Path>) -> Result<()> {
        let repo = Repository::open(repo_dir)?;

        // 1. Try to parse a hash
        //      a. Success, go to 3
        //      b. Failed, go to 2
        // 2. Try to find the matching reference using the standard refs format
        // 3. Peel the object to a commit
        // 4. Map that into an object
        let obj = repo.revparse_single(rev).or_else(|_| {
            repo.find_reference(&format!("refs/remotes/origin/{}", rev))
                .and_then(|r| r.peel_to_commit())
                .map(|c| c.into_object())
        })?;

        // 5. Checkout to commit
        repo.checkout_tree(&obj, None)?;

        // 6. Set the deatched head at the commit
        repo.set_head_detached(obj.id())?;

        Ok(())
    }

    fn fetch(&self, repo_dir: impl AsRef<Path>) -> Result<()> {
        let repo = Repository::open(repo_dir)?;
        let mut remote = repo.find_remote("origin")?;
        remote.fetch(&[] as &[&str], None, None)?;
        Ok(())
    }
}
