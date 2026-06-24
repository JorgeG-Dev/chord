mod cli;
mod commands;
mod manifest;
mod workspace;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use commands::{init, topdir};

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Init { path } => {
            init::run(path)?;
        }
        Commands::Status => {
            println!("Getting status of workspace")
        }
        Commands::List => {
            println!("Printing out info about each repo in the workspace")
        }
        Commands::Diff => {
            println!("Printing out the diff for each repo in the workspace")
        }
        Commands::Topdir => {
            topdir::run()?;
        }
        Commands::Sync => {
            println!("Syncing current directory with manifest lock file, uses manifest as fallback")
        }
        Commands::Update => {
            println!("Updating the hashes for all repos pinned to branches")
        }
        Commands::Lock => {
            println!("Locking in the current workspace revisions")
        }
        Commands::Forall { command } => {
            println!("Running command {:} across all repos", command);
        }
    }

    Ok(())
}
