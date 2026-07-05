//! Contains the logic for performing the Manifest command
use crate::workspace::{Manifest, Workspace};
use std::path::PathBuf;

use anyhow::Result;

pub fn add_run(
    name: String,
    remote: String,
    revision: String,
    location: PathBuf,
    workspace: Workspace,
) -> Result<()> {
    // 1. Open and parse the manifest file
    let mut manifest = Manifest::read(workspace.top_dir())?;

    // 2. Add the new repo
    manifest.add_repo(name, remote, revision, Some(location));

    // 3. Write the new manifest to disk
    manifest.write(workspace.top_dir())?;
    Ok(())
}

pub fn remove_run(name: String, workspace: Workspace) -> Result<()> {
    // 1. Open and parse the manifest file
    let mut manifest = Manifest::read(workspace.top_dir())?;

    // 2. Add the new repo
    manifest.remove_repo(name)?;

    // 3. Write the new manifest to disk
    manifest.write(workspace.top_dir())?;
    Ok(())
}
