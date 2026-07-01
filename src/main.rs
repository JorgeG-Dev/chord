mod cli;
mod commands;
mod workspace;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let args = Cli::parse();

    let result = match args.command {
        Commands::Init { path } => commands::init(path),
        Commands::Topdir => commands::topdir(),
        _ => {
            let backend = workspace::GitBackend;
            match workspace::Workspace::new(backend) {
                Ok(workspace) => match args.command {
                    Commands::Status => commands::status(&workspace),
                    Commands::Sync => commands::sync(&workspace),
                    Commands::Update => commands::update(&workspace),
                    Commands::Forall { command } => commands::forall(command, &workspace),
                    _ => unreachable!(),
                },
                Err(e) => Err(e),
            }
        }
    };

    if let Err(e) = &result {
        println!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
