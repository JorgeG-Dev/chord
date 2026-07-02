//! Contains the logic for performing the Update command
use crate::workspace::{Lockfile, Manifest, Workspace};

use anyhow::{Result, bail};

/// Syncs the repos in the manifest to the revisions specified in
/// the manifest, ignoring the lockfile. Generates a new file after
/// completion.
pub fn run(workspace: Workspace) -> Result<()> {
    // 1. Open and parse the manifest file
    let mut manifest = Manifest::read(workspace.top_dir())?;

    // 2. Drain the manifest repos, perform update operations, and create lockfile
    // struct
    let mut failed_repos = 0;
    let total_repos = manifest.repos.len();
    let mut new_lockfile = Lockfile::new();
    for mut repo in manifest.repos.drain(..) {
        match workspace.resolve_repo(&mut repo) {
            Ok(_) => new_lockfile.insert(repo.name, repo.revision),
            Err(_) => {
                failed_repos += 1;
                continue;
            }
        }
    }

    // 3. Write the new lockfile to disk
    new_lockfile.write(workspace.top_dir())?;

    // 4. Bail with a message if a repo failed to sync
    if failed_repos > 0 {
        bail!(
            "{} out of {} repos failed to sync",
            failed_repos,
            total_repos
        )
    }
    Ok(())
}
