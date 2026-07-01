mod cli;
mod commands;
mod workspace;

use anyhow::{Result, anyhow};
use clap::Parser;
use cli::{Cli, Commands};
use workspace::{GitBackend, Workspace, utils};

fn main() -> Result<()> {
    let args = Cli::parse();

    let result = match args.command {
        Commands::Init { path } => commands::init(path),
        Commands::Topdir => commands::topdir(),
        _ => match utils::get_top_dir() {
            Some(top_dir) => {
                let backend = GitBackend;
                let workspace = Workspace::new(top_dir, backend);
                match args.command {
                    Commands::Status => commands::status(workspace),
                    Commands::Sync => commands::sync(workspace),
                    Commands::Update => commands::update(workspace),
                    Commands::Forall { command } => commands::forall(command, workspace),
                    _ => unreachable!(),
                }
            }
            None => Err(anyhow!("not within chord workspace")),
        },
    };

    if let Err(e) = &result {
        println!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
