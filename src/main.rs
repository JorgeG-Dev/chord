mod cli;
mod commands;
mod workspace;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let args = Cli::parse();
    let backend = workspace::GitBackend;
    match args.command {
        Commands::Init { path } => {
            commands::init(path)?;
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
            commands::topdir()?;
        }
        Commands::Sync => {
            commands::sync(&backend)?;
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
