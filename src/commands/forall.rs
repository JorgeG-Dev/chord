//! Contains the logic for performing the Forall command
use anyhow::{Result, bail};
use colored::Colorize;

use crate::workspace::{Manifest, Workspace};

/// Parses the manifest, ensures each repo is valid, goes into each
/// repo and executes the specified command. Output of each command
/// execution is displayed to stdout.
pub fn run(command: Vec<String>, workspace: Workspace) -> Result<()> {
    // 1. Open and parse the manifest file
    let mut manifest = Manifest::read(workspace.top_dir())?;

    // 2. Create the command string
    let command_str = command.join(" ");

    // 3. Go through each repo, checking if it's a valid repo and running
    // the specified command
    let total_repos = manifest.repos.len();
    let mut failed_repos = 0;
    for repo in manifest.repos.drain(..) {
        println!("{}", format!("========== {} ==========", repo.name).blue());
        match workspace.repo_run(&repo, &command_str) {
            Ok(_) => continue,
            Err(e) => {
                println!("command failed in {}: {}", repo.name, e);
                failed_repos += 1;
                continue;
            }
        }
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
