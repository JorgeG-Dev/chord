//! Contains the logic for performing the Sync command
use crate::workspace::{GitOperations, Lockfile, Manifest, Operations};

use anyhow::Result;
use std::path::PathBuf;

/// Attempts to parse the manifest file and sync it to the revisions outlined
/// in the lockfile, if it exists. If not, a new lockfile is created and each
/// successfully synced repo is pinned to a revision there.
pub fn run(workspace: &impl Operations) -> Result<()> {
    let top_dir = workspace.top_dir();
    let operations = workspace.git();

    // 1. Open and parse the manifest file
    let mut manifest = Manifest::read(&top_dir)?;

    // 2. Try to open the lockfile and get its contents
    let lockfile = match Lockfile::read("chord.lock.yaml") {
        Ok(lockfile) => Some(lockfile),
        Err(_) => None,
    };

    // 3. If a lockfile was actually parsed, go through the manifest, updating
    // updating the revisions and location
    if let Some(mut lockfile) = lockfile {
        manifest.sync(&mut lockfile);
    }

    // 4. Drain the manifest repos, perform sync operations, and create lockfile
    // struct
    let mut new_lockfile = Lockfile::new();
    for repo in manifest.repos.drain(..) {
        let location = repo
            .location
            .as_ref()
            .map(|l| top_dir.join(l))
            .unwrap_or_else(|| top_dir.to_path_buf());
        let repo_dir = PathBuf::from(&top_dir)
            .join(location)
            .join(repo.name.as_str());

        if !operations.is_repo(&repo_dir) {
            operations.clone_repo(&repo.remote, &repo_dir)?;
        }
        operations.fetch(&repo_dir)?;
        let revision = operations.rev_as_hash(&repo_dir, &repo.revision)?;
        operations.checkout(&revision, &repo_dir)?;
        new_lockfile.insert(repo.name, revision);
    }

    // 5. Write the new lockfile to disk
    new_lockfile.write(top_dir)?;

    Ok(())
}
