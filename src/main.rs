use anyhow::{Result, anyhow};
use chord_ws::cli::{Cli, Commands, ManifestOps};
use chord_ws::workspace::{GitBackend, Workspace, utils};
use chord_ws::{commands, error_msg};
use clap::Parser;
use colored::Colorize;

fn main() -> Result<()> {
    let args = Cli::parse();

    let result = match args.command {
        Commands::Init { path } => commands::init(path),
        Commands::Topdir { path } => commands::topdir(path),
        _ => match utils::get_top_dir(".") {
            Some(top_dir) => {
                let backend = GitBackend;
                let workspace = Workspace::new(top_dir, backend);
                match args.command {
                    Commands::Status => commands::status(workspace),
                    Commands::Sync => commands::sync(workspace),
                    Commands::Update => commands::update(workspace),
                    Commands::Forall { command } => commands::forall(command, workspace),
                    Commands::Manifest(operation) => match operation {
                        ManifestOps::Add {
                            name,
                            remote,
                            revision,
                            location,
                        } => commands::manifest_add(name, remote, revision, location, workspace),
                        ManifestOps::Remove { name } => commands::manifest_remove(name, workspace),
                    },
                    _ => unreachable!(),
                }
            }
            None => Err(anyhow!("not within chord workspace")),
        },
    };

    if let Err(e) = &result {
        error_msg!("{}", e);
        std::process::exit(1);
    }

    Ok(())
}
