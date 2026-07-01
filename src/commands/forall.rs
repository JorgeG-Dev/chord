//! Contains the logic for performing the Forall command
use anyhow::{Result, bail};
use colored::Colorize;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;

use crate::workspace::{GitOperations, Manifest, Operations};

/// Runs the Chord workspace forall process
///
/// Goes through all the repos in the manifest and checks if they exist
/// on disk and runs the specified command in each repo
///
/// # Arguments
/// `command` - The command to run in each repo
/// `workspace` - An object that implements the workspace operations trait
///
/// # Returns
///
/// Returns Ok on successful of workspace, Err if the command being run
/// in the repo fails for any reason.
///
/// # Errors
///
/// No errors are returned from this function
///
/// # Panics
///
/// This function does not panic
///
pub fn run(command: Vec<String>, workspace: &impl Operations) -> Result<()> {
    let top_dir = workspace.top_dir();
    let operations = workspace.git();

    // 1. Open and parse the manifest file
    let mut manifest = Manifest::read(&top_dir)?;

    // 2. Create the command string
    let command_str = command.join(" ");

    // 3. Go through each repo, checking if it's a valid repo and running
    // the specified command
    let total_repos = manifest.repos.len();
    let mut failed_repos = 0;
    for repo in manifest.repos.drain(..) {
        println!("{}", format!("========== {} ==========", repo.name).blue());
        let location = repo
            .location
            .as_ref()
            .map(|l| top_dir.join(l))
            .unwrap_or_else(|| top_dir.to_path_buf());
        let repo_dir = PathBuf::from(&top_dir)
            .join(location)
            .join(repo.name.as_str());

        // Only run the command if a valid repo
        if !operations.is_repo(&repo_dir) {
            println!("{}: {} is not a valid repo", "error".red(), repo.name);
            failed_repos += 1;
            continue;
        }

        // Run the command in each repo directory
        match Command::new("sh")
            .arg("-c")
            .arg(&command_str)
            .current_dir(&repo_dir)
            .status()
        {
            Ok(status) => {
                if status.success() {
                    continue;
                } else {
                    failed_repos += 1;
                }
            }
            Err(_) => {
                println!("{}: failed to run command", "error".red());
                failed_repos += 1;
            }
        };
    }

    if failed_repos > 0 {
        bail!(
            "command failed in {} out of {} repos",
            failed_repos,
            total_repos
        );
    }
    Ok(())
}
