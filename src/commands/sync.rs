use crate::workspace::GitOperations;
use crate::workspace::Manifest;
use crate::workspace::Workspace;
use crate::workspace::utils;
use serde_saphyr;
use std::{fs::File, path::PathBuf};

use anyhow::{Result, bail};

pub fn run(backend: &impl GitOperations) -> Result<()> {
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
    let workspace = Workspace::try_from(manifest)?;
    for repo in workspace.repos {
        let repo_dir = PathBuf::new()
            .join(&top_dir)
            .join(repo.location.as_path())
            .join(repo.name.as_str());

        // Only try to clone if the repo doesn't exist yet
        if !backend.is_repo(&repo_dir) {
            backend.clone_repo(&repo.remote, &repo_dir)?;
        }
        backend.fetch(&repo_dir)?;
        backend.checkout(&repo.rev, &repo_dir)?;
    }

    Ok(())
}
