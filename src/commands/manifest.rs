//! Contains the logic for performing the Manifest command
use crate::workspace::{Manifest, Workspace};
use std::path::PathBuf;

use anyhow::{Result, bail};

pub fn add_run(
    name: String,
    remote: String,
    revision: String,
    location: Option<PathBuf>,
    workspace: Workspace,
) -> Result<()> {
    // 1. Open and parse the manifest file
    let mut manifest = Manifest::read(workspace.top_dir())?;

    // 2. Add the new repo
    manifest.add_repo(name, remote, revision, location)?;

    // 3. Write the new manifest to disk
    manifest.write(workspace.top_dir())?;
    Ok(())
}

pub fn remove_run(name: String, workspace: Workspace) -> Result<()> {
    // 1. Open and parse the manifest file
    let mut manifest = Manifest::read(workspace.top_dir())?;

    // 2. Remove the specified repo
    manifest.remove_repo(name)?;

    // 3. Write the new manifest to disk
    manifest.write(workspace.top_dir())?;
    Ok(())
}

pub fn modify_run(
    name: String,
    new_name: Option<String>,
    new_remote: Option<String>,
    new_revision: Option<String>,
    new_location: Option<PathBuf>,
    workspace: Workspace,
) -> Result<()> {
    // 1. Check to see if at least one field was provided
    if new_name.is_none()
        && new_remote.is_none()
        && new_revision.is_none()
        && new_location.is_none()
    {
        bail!("no new values were provided")
    }
    // 2. Open and parse the manifest file
    let mut manifest = Manifest::read(workspace.top_dir())?;

    // 3. Modify the specified repo
    manifest.modify_repo(name, new_name, new_remote, new_revision, new_location)?;

    // 4. Write the new manifest to disk
    manifest.write(workspace.top_dir())?;
    Ok(())
}
