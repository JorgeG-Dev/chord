use anyhow::{Result, bail};
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;

use crate::workspace::{GitOperations, Manifest, Operations};

pub fn run(command: Vec<String>, workspace: &impl Operations) -> Result<()> {
    let top_dir = workspace.top_dir();
    let operations = workspace.git();

    // 1. Open and parse the manifest file
    let manifest_file = match File::open(top_dir.join("chord.yaml")) {
        Ok(file) => file,
        Err(_) => {
            bail!("Failed to open Chord manifest")
        }
    };
    let mut manifest: Manifest = serde_saphyr::from_reader(manifest_file)?;

    // 2. Create the command string
    let command_str = command.join(" ");

    // 3. Go through each repo, checking if it's a valid repo and running
    // the specified command
    for repo in manifest.repos.drain(..) {
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
            println!(
                "Directory {} is not a valid repo, skipping",
                repo_dir.display()
            );
            continue;
        }

        // Run the command in each repo directory
        Command::new("sh")
            .arg("-c")
            .arg(&command_str)
            .current_dir(&repo_dir)
            .status()?;
    }
    Ok(())
}
