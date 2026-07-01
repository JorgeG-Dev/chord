//! Contains the logic for performing the Status command
use crate::workspace::{Lockfile, Manifest, Workspace};

use anyhow::Result;
use comfy_table::{Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

const STATUS_TABLE_INDEX: usize = 1;
const DIRTY_TABLE_INDEX: usize = 2;

/// Goes through all the repos in the manifest and determines if they
/// are currently checked out to the revision specified in the lockfile
/// and if they are dirty or not.
pub fn run(workspace: Workspace) -> Result<()> {
    // 1. Open and parse the manifest file
    let mut manifest = Manifest::read(workspace.top_dir())?;

    // 2. Try to open the lockfile and get its contents
    let lockfile = match Lockfile::read(workspace.top_dir()) {
        Ok(lockfile) => lockfile,
        Err(_) => {
            println!(
                "no lockfile exists, run `chord sync` or `chord update` to generate a new one"
            );
            Lockfile::new()
        }
    };

    // 3. Create table object
    let mut table = Table::new();
    table
        .apply_modifier(UTF8_ROUND_CORNERS)
        .load_preset(UTF8_FULL)
        .set_header(vec!["NAME", "STATUS", "DIRTY"]);

    // 4. Iterate through the manifest, creating the table
    for repo in manifest.repos.drain(..) {
        let mut table_entry = vec![&repo.name, "unavailable", "unknown"];
        let current_rev = match lockfile.get(repo.name.as_str()) {
            Some(value) => value,
            None => {
                table.add_row(table_entry);
                continue;
            }
        };
        let (is_locked, is_dirty) = workspace.repo_status(&repo, current_rev.as_str())?;
        table_entry[STATUS_TABLE_INDEX] = match is_locked {
            true => "locked",
            false => "mismatch",
        };
        table_entry[DIRTY_TABLE_INDEX] = match is_dirty {
            true => "yes",
            false => "no",
        };
        table.add_row(table_entry);
    }

    println!("{table}");

    Ok(())
}
