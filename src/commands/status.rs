//! Contains the logic for performing the Status command
use crate::workspace::{GitOperations, Lockfile, Manifest, Operations};

use anyhow::{Result, bail};
use comfy_table::{Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};
use serde_saphyr;
use std::collections::HashMap;
use std::{fs::File, path::PathBuf};

const STATUS_TABLE_INDEX: usize = 1;
const DIRTY_TABLE_INDEX: usize = 2;

/// Runs the Chord workspace status process
///
/// Goes through all the repos in the manifest and checks if they exist
/// on disk and if are checked out to the correct revision. The revision
/// specified in the lockfile is the source of truth. If the lockfile does
/// not exist, just the disk status will be relevant.
///
/// # Arguments
/// `workspace` - An object that implements the workspace operations trait
///
/// # Returns
///
/// Always returns Ok, the output printed to stdout varies based on the state
/// of the workspace, but the general structure involves the repo name, disk
/// status, and revision status in a tabulated form.
///
/// # Errors
///
/// No errors are returned from this function
///
/// # Panics
///
/// This function does not panic
///
pub fn run(workspace: &impl Operations) -> Result<()> {
    let top_dir = workspace.top_dir();
    let operations = workspace.git();

    // 1. Open and parse the manifest file
    let mut manifest = Manifest::read(&top_dir);

    // 2. Try to open the lockfile and get its contents
    let mut locked_repos = HashMap::new();
    if let Ok(file) = File::open(top_dir.join("chord.lock.yaml")) {
        let lockfile: Lockfile = serde_saphyr::from_reader(file)?;
        for repo in lockfile.repos {
            locked_repos.insert(repo.name, repo.revision);
        }
    } else {
        println!("No lockfile exists, run `chord sync` or `chord update` to generate a new one");
    }

    // 3. Create table object
    let mut table = Table::new();
    table
        .apply_modifier(UTF8_ROUND_CORNERS)
        .load_preset(UTF8_FULL)
        .set_header(vec!["NAME", "STATUS", "DIRTY"]);

    // 4. Iterate through the manifest, creating the table
    for repo in manifest.repos.drain(..) {
        let mut table_entry = vec![repo.name.as_str(), "unavailable", "unknown"];
        let location = repo
            .location
            .as_ref()
            .map(|l| top_dir.join(l))
            .unwrap_or_else(|| top_dir.to_path_buf());
        let repo_dir = PathBuf::from(&top_dir)
            .join(location)
            .join(repo.name.as_str());

        if operations.is_repo(&repo_dir) {
            table_entry[STATUS_TABLE_INDEX] = "mismatch";
            let current_head = operations.get_current_hash(&repo_dir)?;
            if let Some(locked_rev) = locked_repos.get(&repo.name) {
                if locked_rev.as_str() == current_head.as_str() {
                    table_entry[STATUS_TABLE_INDEX] = "locked";
                }
            }

            if let Ok(dirty) = operations.is_dirty(&repo_dir) {
                if !dirty {
                    table_entry[DIRTY_TABLE_INDEX] = "no"
                } else {
                    table_entry[DIRTY_TABLE_INDEX] = "yes"
                }
            }
        }
        table.add_row(table_entry);
    }

    println!("{table}");

    Ok(())
}
