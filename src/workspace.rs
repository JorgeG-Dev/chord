use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

mod git;
mod lockfile;
mod manifest;
pub mod utils;

pub use git::GitBackend;
pub use git::Operations as GitOperations;
pub use lockfile::Lockfile;
pub use lockfile::Repo as LockedRepo;
pub use manifest::Manifest;
pub use manifest::Repo as ManifestRepo;

/// Represents the complete Chord workspace, created at the invokation of
/// the `chord` command.
pub struct Workspace {
    /// The root of the Chord workspace.
    top_dir: PathBuf,
    /// The Git operation backend used for the subcommands
    backend: GitBackend,
}

/// The operations that can be performed by a Workspace struct
pub trait Operations {
    /// Gets the workspace's top directory where the manifest is located
    fn top_dir(&self) -> &Path;
    /// Gets the backend used for all git related operations
    fn git(&self) -> &impl GitOperations;
}

impl Operations for Workspace {
    fn top_dir(&self) -> &Path {
        &self.top_dir
    }

    fn git(&self) -> &impl GitOperations {
        &self.backend
    }
}

impl Workspace {
    pub fn new(backend: GitBackend) -> Result<Self> {
        let top_dir = match utils::get_top_dir() {
            Some(dir) => dir,
            None => bail!("Not within a Chord workspace"),
        };
        Ok(Self { top_dir, backend })
    }
}

/// Primarily used for unit testing the commands supported by the application
#[cfg(test)]
pub mod mock {
    use super::*;
    use anyhow::Result;
    use std::cell::Cell;
    use std::path::PathBuf;

    // Used for stubbing out operations requiring the git backend and topdir
    pub struct MockWorkspace {
        pub top_dir: PathBuf,
        pub git: MockGitBackend,
    }

    impl Operations for MockWorkspace {
        fn top_dir(&self) -> &std::path::Path {
            &self.top_dir
        }
        fn git(&self) -> &impl GitOperations {
            &self.git
        }
    }

    // Used for keeping track of how many times each git operation was called
    pub struct MockGitBackend {
        pub is_repo_count: Cell<u32>,
        pub is_repo_return: Cell<bool>,
        pub clone_count: Cell<u32>,
        pub fetch_count: Cell<u32>,
        pub rev_as_hash_count: Cell<u32>,
        pub rev_as_hash_rev: Cell<String>,
        pub rev_as_hash_return: Cell<String>,
        pub checkout_count: Cell<u32>,
        pub get_current_hash_count: Cell<u32>,
        pub get_current_hash_return: Cell<String>,
        pub is_dirty_count: Cell<u32>,
        pub is_dirty_return: Cell<bool>,
    }

    impl GitOperations for MockGitBackend {
        #[allow(unused)]
        fn is_repo(&self, repo_dir: impl AsRef<std::path::Path>) -> bool {
            self.is_repo_count.set(self.is_repo_count.get() + 1);
            self.is_repo_return.get()
        }

        #[allow(unused)]
        fn rev_as_hash(&self, repo_dir: impl AsRef<std::path::Path>, rev: &str) -> Result<String> {
            self.rev_as_hash_rev.set(String::from(rev));
            self.rev_as_hash_count.set(self.rev_as_hash_count.get() + 1);
            let value = self.rev_as_hash_return.take();
            self.rev_as_hash_return.set(value.clone());
            Ok(value)
        }

        #[allow(unused)]
        fn clone_repo(&self, remote: &str, repo_dir: impl AsRef<std::path::Path>) -> Result<()> {
            self.clone_count.set(self.clone_count.get() + 1);
            Ok(())
        }

        #[allow(unused)]
        fn fetch(&self, repo_dir: impl AsRef<std::path::Path>) -> Result<()> {
            self.fetch_count.set(self.fetch_count.get() + 1);
            Ok(())
        }

        #[allow(unused)]
        fn checkout(&self, hash: &str, repo_dir: impl AsRef<std::path::Path>) -> Result<()> {
            self.checkout_count.set(self.checkout_count.get() + 1);
            Ok(())
        }

        #[allow(unused)]
        fn get_current_hash(&self, repo_dir: impl AsRef<std::path::Path>) -> Result<String> {
            self.get_current_hash_count
                .set(self.get_current_hash_count.get() + 1);
            let value = self.get_current_hash_return.take();
            self.get_current_hash_return.set(value.clone());
            Ok(value)
        }

        #[allow(unused)]
        fn is_dirty(&self, repo_dir: impl AsRef<std::path::Path>) -> Result<bool> {
            self.is_dirty_count.set(self.is_dirty_count.get() + 1);
            Ok(self.is_dirty_return.get())
        }
    }

    impl MockGitBackend {
        pub fn new() -> Self {
            Self {
                is_repo_count: Cell::new(0),
                is_repo_return: Cell::new(false),
                clone_count: Cell::new(0),
                fetch_count: Cell::new(0),
                rev_as_hash_count: Cell::new(0),
                rev_as_hash_rev: Cell::new(String::new()),
                rev_as_hash_return: Cell::new(String::new()),
                checkout_count: Cell::new(0),
                get_current_hash_count: Cell::new(0),
                get_current_hash_return: Cell::new(String::new()),
                is_dirty_count: Cell::new(0),
                is_dirty_return: Cell::new(false),
            }
        }
    }

    pub fn default_manifest() -> &'static str {
        r#"
    repos:
      - name: chord
        remote: https://github.com/JorgeG-Dev/chord
        revision: main
    "#
    }

    // This will fail in practice
    pub fn multi_repo_manifest() -> &'static str {
        r#"
    repos:
      - name: chord
        remote: https://github.com/JorgeG-Dev/chord
        revision: main
      - name: chord
        remote: https://github.com/JorgeG-Dev/chord
        revision: main
    "#
    }
}
