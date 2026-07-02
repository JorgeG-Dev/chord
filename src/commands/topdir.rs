//! Contains the logic for performing the Topdir command
use crate::workspace::utils;
use anyhow::{Result, bail};
use std::path::Path;

/// Walks from the given directory (or the current directory if none is
/// provided) upwards until either the root is reached or until a
/// chord.yaml is found. If not found, the command errors out, makes it
/// easier for use in shell scripts.
pub fn run(path: impl AsRef<Path>) -> Result<()> {
    match utils::get_top_dir(&path) {
        Some(dir) => println!("{}", dir.display()),
        None => bail!("not within a workspace"),
    }

    Ok(())
}
