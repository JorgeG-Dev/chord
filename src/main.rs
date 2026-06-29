mod cli;
mod commands;
mod workspace;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Init { path } => {
            commands::init(path)?;
        }
        Commands::Topdir => {
            commands::topdir()?;
        }
        _ => {
            let backend = workspace::GitBackend;
            let workspace = workspace::Workspace::new(backend)?;
            match args.command {
                Commands::Status => {
                    println!("Getting status of workspace")
                }
                Commands::List => {
                    println!("Printing out info about each repo in the workspace")
                }
                Commands::Diff => {
                    println!("Printing out the diff for each repo in the workspace")
                }
                Commands::Sync => {
                    commands::sync(&workspace)?;
                }
                Commands::Update => {
                    commands::update(&workspace)?;
                }
                Commands::Forall { command } => {
                    println!("Running command {:} across all repos", command);
                }
                _ => unreachable!(),
            }
        }
    }

    Ok(())
}
