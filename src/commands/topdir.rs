//! Contains the logic for performing the Topdir command
use crate::workspace::utils;
use anyhow::Result;

/// Runs the Chord workspace topdir process
///
/// Starting from where the command is invoked, the app will walk up the
/// directories until one of the following occurs:
///
/// 1. A `chord.yaml` is found
/// 2. No more directories to walk (e.g. root reached)
///
/// If a `chord.yaml` is found, that directory is printed, otherwise an
/// error message printed out.
///
/// ***If you have nested Chord workspaces, the innermost one will be
/// returned. Meaning if you run this command a subdirectory that has a `chord.yaml`
/// that's within an upper directory with a `chord.yaml`, the subdirectory path
/// will be printed. The same logic applies to all other commands, they all
/// run with respect to the innermost Chord workspace***
///
/// # Arguments
///
/// # Returns
///
/// The function always returns Ok. The output printed to stdout does change
/// based on whether a `chord.yaml` was found or not.
///
/// # Errors
///
/// No errors are returned from this function
///
/// # Panics
///
/// This function does not panic
///
pub fn run() -> Result<()> {
    if let Some(dir) = utils::get_top_dir() {
        println!("{}", dir.display());
    } else {
        println!("Not within a Chord workspace");
    }
    Ok(())
}
