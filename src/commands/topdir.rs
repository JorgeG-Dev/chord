//! Contains the logic for performing the Topdir command
use crate::workspace::utils;
use anyhow::Result;

/// Walks from the directory where the command was invoked
/// upwards until either the root is reached or until a
/// chord.yaml is found.
pub fn run() -> Result<()> {
    if let Some(dir) = utils::get_top_dir() {
        println!("{}", dir.display());
    } else {
        println!("Not within a Chord workspace");
    }
    Ok(())
}
