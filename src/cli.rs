use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initializes the chord manifest directory and file
    Init {
        /// Path where the chord manifest directory should be initialized,
        /// defaults to current working directory
        #[arg(long, default_value = ".")]
        path: PathBuf,
    },
    /// Checks the status of the chord workspace against the manifest
    Status,
    /// Prints the chord workspace root
    Topdir {
        /// Path from where the top directory search should start,
        /// defaults to current working directory
        #[arg(long, default_value = ".")]
        path: PathBuf,
    },
    /// Clones missing repos, fetches, and checks out to whatever is in the
    /// lockfile, defaults to chord manifest if there is no lockfile provided
    Sync,
    /// Performs same operations as sync, key difference being that it uses
    /// the manifest, regardless of whether there's a lockfile or not
    Update,
    /// Runs a user provided command in each repo in the chord workspace
    Forall {
        /// Command to run in each repo
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        command: Vec<String>,
    },
}
