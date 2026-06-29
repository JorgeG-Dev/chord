use crate::workspace::{GitOperations, LockedRepo, Lockfile, Manifest, Operations};

use anyhow::{Result, bail};
use serde_saphyr;
use std::{fs::File, path::PathBuf};

/// Performs the same process, as sync, key difference being that it does not
/// use the lockfile. A new one will be generated each time this command is
/// run. This is mainly used for updating repos that are pinned to a branch
/// as opposed to a tag or commit hash.
pub fn run(workspace: &impl Operations) -> Result<()> {
    let top_dir = workspace.top_dir();
    let operations = workspace.git();

    // 1. Open and parse the manifest file
    let manifest_file = match File::open(top_dir.join("chord.yaml")) {
        Ok(file) => file,
        Err(_) => {
            bail!("Failed to open Chord manifest")
        }
    };
    let mut manifest: Manifest = serde_saphyr::from_reader(manifest_file)?;

    // 2. Drain the manifest repos, perform sync operations, and create lockfile
    // struct
    let mut lockfile_repos = vec![];
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
        lockfile_repos.push(LockedRepo {
            name: repo.name,
            revision,
        });
    }

    // 3. Create a lockfile out of the contents in the current manifest struct
    let mut new_lockfile = File::create(top_dir.join("chord.lock.yaml"))?;
    let new_lockfile_contents = Lockfile {
        repos: lockfile_repos,
    };
    serde_saphyr::to_io_writer(&mut new_lockfile, &new_lockfile_contents)?;
    Ok(())
}

