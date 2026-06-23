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
    /// Prints info about each repo in the chord workspace
    List,
    /// Runs git diff across all repos in the chord workspace
    Diff,
    /// Prints the chord workspace root
    Topdir,
    /// Clones missing repos, fetches, and checks out to whatever is in the
    /// lockfile, defaults to chord manifest if there is no lockfile provided
    Sync,
    /// Updates any repos pointing to a branch to the latest commit and
    /// updates the lockfile
    Update,
    /// Resolves current repo HEADs to SHAs and writes a lockfile without
    /// changing anything
    Lock,
    /// Runs a user provided command in each repo in the chord workspace
    Forall {
        /// Command to run in each repo
        #[arg(long)]
        command: String,
    },
}
