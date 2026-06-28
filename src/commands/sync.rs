use crate::workspace::{GitOperations, LockedRepo, Lockfile, Manifest, Operations};
use serde_saphyr;
use std::collections::HashMap;
use std::{fs::File, path::PathBuf};

use anyhow::{Result, bail};

pub fn run(workspace: impl Operations) -> Result<()> {
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

    // 2. Try to open the lockfile and get its contents
    let locked_repos = match File::open(top_dir.join("chord.lock.yaml")) {
        Ok(file) => {
            let lockfile: Lockfile = serde_saphyr::from_reader(file)?;
            let mut parsed_repos = HashMap::new();
            for repo in lockfile.repos {
                parsed_repos.insert(repo.name, repo.revision);
            }
            Some(parsed_repos)
        }
        Err(_) => None,
    };

    // 3. If a lockfile was actually parsed, go through the manifest, updating
    // updating the revisions and location
    if let Some(repos) = locked_repos {
        for manifest_repo in &mut manifest.repos {
            if let Some(revision) = repos.get(&manifest_repo.name) {
                manifest_repo.revision = revision.clone();
            }
        }
    }

    // 4. Drain the manifest repos, perform sync operations, and create lockfile
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
            revision: repo.revision,
        });
    }

    // 5. Create a lockfile out of the contents in the current manifest struct
    let mut new_lockfile = File::create(top_dir.join("chord.lock.yaml"))?;
    let new_lockfile_contents = Lockfile {
        repos: lockfile_repos,
    };
    serde_saphyr::to_io_writer(&mut new_lockfile, &new_lockfile_contents)?;
    Ok(())
}
