//! Contains the logic for performing the Sync command
use crate::workspace::{Lockfile, Manifest, Workspace};

use anyhow::Result;

/// Attempts to parse the manifest file and sync it to the revisions outlined
/// in the lockfile, if it exists. If not, a new lockfile is created and each
/// successfully synced repo is pinned to a revision there.
pub fn run(workspace: Workspace) -> Result<()> {
    // 1. Open and parse the manifest file
    let mut manifest = Manifest::read(workspace.top_dir())?;

    // 2. Try to open the lockfile and get its contents
    let lockfile = match Lockfile::read(workspace.top_dir()) {
        Ok(lockfile) => Some(lockfile),
        Err(_) => None,
    };

    // 3. If a lockfile was actually parsed, go through the manifest, updating
    // updating the revisions and location
    if let Some(mut lockfile) = lockfile {
        manifest.apply_lock(&mut lockfile);
    }

    // 4. Drain the manifest repos, perform sync operations, and create lockfile
    // struct
    let mut new_lockfile = Lockfile::new();
    for repo in manifest.repos.drain(..) {
        let repo = workspace.resolve_repo(repo)?;
        new_lockfile.insert(repo.name, repo.revision);
    }

    // 5. Write the new lockfile to disk
    new_lockfile.write(workspace.top_dir())?;

    Ok(())
}
