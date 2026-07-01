//! Contains the logic for performing the Update command
use crate::workspace::{Lockfile, Manifest, Workspace};

use anyhow::Result;

/// Syncs the repos in the manifest to the revisions specified in
/// the manifest, ignoring the lockfile. Generates a new file after
/// completion.
pub fn run(workspace: Workspace) -> Result<()> {
    // 1. Open and parse the manifest file
    let mut manifest = Manifest::read(workspace.get_top_dir())?;

    // 2. Drain the manifest repos, perform update operations, and create lockfile
    // struct
    let mut new_lockfile = Lockfile::new();
    for repo in manifest.repos.drain(..) {
        let repo = workspace.sync(repo)?;
        new_lockfile.insert(repo.name, repo.revision);
    }

    // 3. Write the new lockfile to disk
    new_lockfile.write(workspace.get_top_dir())?;
    Ok(())
}
