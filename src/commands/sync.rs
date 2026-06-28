use crate::workspace::GitOperations;
use crate::workspace::Manifest;
use crate::workspace::utils;
use serde_saphyr;
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
    let manifest: Manifest = serde_saphyr::from_reader(manifest_file)?;

    // 3. Make sure the fields in the repo entries have some sort of valid value
    for repo in manifest.repos {
        let remote = repo.remote;
        let revision = repo.revision;
        let name = repo.name;
        let location = match repo.location {
            Some(value) => value,
            None => top_dir.clone(),
        };

        let repo_dir = PathBuf::from(&top_dir).join(location).join(name.as_str());
        if !operations.is_repo(&repo_dir) {
            operations.clone_repo(&remote, &repo_dir)?;
        }
        operations.fetch(&repo_dir)?;
        operations.checkout(&revision, &repo_dir)?;
    }
    /*
    for repo in workspace.repos {
        let repo_dir = PathBuf::new()
            .join(&top_dir)
            .join(repo.location.as_path())
            .join(repo.name.as_str());

        // Only try to clone if the repo doesn't exist yet
        if !operations.is_repo(&repo_dir) {
            operations.clone_repo(&repo.remote, &repo_dir)?;
        }
        operations.fetch(&repo_dir)?;
        operations.checkout(&repo.rev, &repo_dir)?;
    }
    */

    Ok(())
}
