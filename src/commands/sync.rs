use crate::workspace::{GitOperations, Lockfile, Manifest, utils};
use serde_saphyr;
use std::collections::HashMap;
use std::{fs::File, path::PathBuf};

use anyhow::{Result, bail};

pub fn run(operations: &impl GitOperations) -> Result<()> {
    // 1. Find the top directory where the manifest is located
    let top_dir = match utils::get_top_dir() {
        Some(dir) => dir,
        None => {
            bail!("Not within a Chord workspace")
        }
    };

    // 2. Open and parse the manifest file
    let manifest_file = match File::open(top_dir.join("chord.yaml")) {
        Ok(file) => file,
        Err(_) => {
            bail!("Failed to open Chord manifest")
        }
    };
    let mut manifest: Manifest = serde_saphyr::from_reader(manifest_file)?;

    // 3. Try to open the lockfile and get its contents
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

    // 4. If a lockfile was actually parsed, go through the manifest, updating
    //  updating the revisions and location
    match locked_repos {
        Some(repos) => {
            for manifest_repo in &mut manifest.repos {
                match repos.get(&manifest_repo.name) {
                    Some(revision) => {
                        manifest_repo.revision = String::from(revision);
                    }
                    None => {
                        continue;
                    }
                };
            }
        }
        None => {}
    };

    // 5. Go through each manifest repo, cloning, fetching and checking out
    //  the correct revision
    for repo in &mut manifest.repos {
        let remote = repo.remote.clone();
        let mut revision = repo.revision.clone();
        let name = repo.name.clone();
        let location = match repo.location.clone() {
            Some(value) => value,
            None => top_dir.clone(),
        };

        let repo_dir = PathBuf::from(&top_dir).join(location).join(name.as_str());
        if !operations.is_repo(&repo_dir) {
            operations.clone_repo(&remote, &repo_dir)?;
        }
        revision = operations.rev_as_hash(&repo_dir, revision.as_str())?;
        operations.fetch(&repo_dir)?;
        operations.checkout(&revision, &repo_dir)?;
        repo.revision = revision;
    }

    // 6. Create a lockfile out of the contents in the current manifest struct
    let mut new_lockfile = File::create(top_dir.join("chord.lock.yaml"))?;
    let new_lockfile_contents = Lockfile::from(manifest);
    serde_saphyr::to_io_writer(&mut new_lockfile, &new_lockfile_contents)?;
    Ok(())
}
