//! Contains the logic for performing the Topdir command
use crate::workspace::utils;
use anyhow::Result;
use std::path::Path;

/// Walks from the given directory (or the current directory if none is
/// provided) upwards until either the root is reached or until a
/// chord.yaml is found.
pub fn run(path: impl AsRef<Path>) -> Result<()> {
    match utils::get_top_dir(&path) {
        Some(dir) => println!("{}", dir.display()),
        None => println!("not within a workspace"),
    }

    Ok(())
}
