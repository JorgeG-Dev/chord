use crate::workspace::utils;
use anyhow::Result;

/// Prints out the topdir of the current workspace, assuming the command is
/// invoked somewhere within a Chord workspace
pub fn run() -> Result<()> {
    if let Some(dir) = utils::get_top_dir() {
        println!("{}", dir.display());
    } else {
        println!("Not within a Chord workspace");
    }
    Ok(())
}
