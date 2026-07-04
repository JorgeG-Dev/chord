use super::*;
use anyhow::Result;
use git2::{
    Cred, CredentialType, Error, FetchOptions, RemoteCallbacks, Repository, build::RepoBuilder,
};
use std::path::Path;

/// Don't need any fields, just a target for implementing the backend
pub struct Git2Backend;

impl Operations for Git2Backend {
    fn is_repo(&self, repo_dir: impl AsRef<Path>) -> bool {
        Repository::open(repo_dir).is_ok()
    }

    fn is_dirty(&self, repo_dir: impl AsRef<Path>) -> Result<bool> {
        let repo = Repository::open(repo_dir)?;
        let statuses = repo.statuses(None)?;
        Ok(!statuses.is_empty())
    }

    fn get_current_hash(&self, repo_dir: impl AsRef<Path>) -> Result<String> {
        let repo = Repository::open(repo_dir)?;
        let head = repo.head()?;
        let obj = head.peel_to_commit().map(|c| c.into_object())?;
        Ok(obj
            .id()
            .as_bytes()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>())
    }

    fn rev_as_hash(&self, repo_dir: impl AsRef<Path>, rev: &str) -> Result<String> {
        let repo = Repository::open(repo_dir)?;
        let obj = repo.revparse_single(rev).or_else(|_| {
            repo.find_reference(&format!("refs/remotes/origin/{}", rev))
                .and_then(|r| r.peel_to_commit())
                .map(|c| c.into_object())
        })?;
        Ok(obj
            .id()
            .as_bytes()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>())
    }

    fn clone_repo(&self, remote: &str, repo_dir: impl AsRef<Path>) -> Result<()> {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(credentials_callback);

        // Keeping the fetch options simple
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fetch_options);

        builder.clone(remote, repo_dir.as_ref())?;
        Ok(())
    }

    fn checkout(&self, hash: &str, repo_dir: impl AsRef<Path>) -> Result<()> {
        let repo = Repository::open(repo_dir)?;

        // 1. Try to parse a hash
        let obj = repo.revparse_single(hash)?;

        // 2. Checkout to commit
        repo.checkout_tree(&obj, None)?;

        // 3. Set the deatched head at the commit
        repo.set_head_detached(obj.id())?;

        Ok(())
    }

    fn fetch(&self, repo_dir: impl AsRef<Path>) -> Result<()> {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(credentials_callback);

        // Keeping the fetch options simple
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        let repo = Repository::open(repo_dir)?;
        let mut remote = repo.find_remote("origin")?;
        remote.fetch(&[] as &[&str], Some(&mut fetch_options), None)?;
        Ok(())
    }
}

fn credentials_callback(
    _: &str,
    username: Option<&str>,
    allowed_types: CredentialType,
) -> Result<Cred, Error> {
    if allowed_types.is_ssh_key() {
        match username {
            Some(username) => Cred::ssh_key_from_agent(username),
            None => Err(Error::from_str("no username provided in SSH URL")),
        }
    } else {
        Err(Error::from_str("unsupported authentication type requested"))
    }
}
