mod cli;
mod commands;
mod git;
mod manifest;
mod utils;
mod workspace;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use commands::{init, sync, topdir};
use git::libgit2;

fn main() -> Result<()> {
    let args = Cli::parse();
    let backend = libgit2::Git2Backend;
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
            sync::run(&backend)?;
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
