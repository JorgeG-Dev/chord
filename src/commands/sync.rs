use crate::workspace::{GitOperations, LockedRepo, Lockfile, Manifest, Operations};

use anyhow::{Result, bail};
use serde_saphyr;
use std::collections::HashMap;
use std::{fs::File, path::PathBuf};

/// Attempts to sync the workspace to the lockfile. If no lockfile exists,
/// falls back to the manifest file and creates a new lockfile. Clones (if
/// necessary), fetches, and checks out the specified revision
pub fn run(workspace: &impl Operations) -> Result<()> {
    let top_dir = workspace.top_dir();
    let operations = workspace.git();

    // 1. Open and parse the manifest file
    let manifest_file = match File::open(top_dir.join("chord.yaml")) {
        Ok(file) => file,
        Err(_) => {
            bail!("Failed to open Chord manifest")
        }
    };
    let mut manifest: Manifest = serde_saphyr::from_reader(manifest_file)?;

    // 2. Try to open the lockfile and get its contents
    let locked_repos = match File::open(top_dir.join("chord.lock.yaml")) {
        Ok(file) => {
            let lockfile: Lockfile = serde_saphyr::from_reader(file)?;
            let mut parsed_repos = HashMap::new();
            for repo in lockfile.repos {
                parsed_repos.insert(repo.name, repo.revision);
            }
            Some(parsed_repos)
        }
        Err(_) => None,
    };

    // 3. If a lockfile was actually parsed, go through the manifest, updating
    // updating the revisions and location
    if let Some(repos) = locked_repos {
        for manifest_repo in &mut manifest.repos {
            if let Some(revision) = repos.get(&manifest_repo.name) {
                manifest_repo.revision = revision.clone();
            }
        }
    }

    // 4. Drain the manifest repos, perform sync operations, and create lockfile
    // struct
    let mut lockfile_repos = vec![];
    for repo in manifest.repos.drain(..) {
        let location = repo
            .location
            .as_ref()
            .map(|l| top_dir.join(l))
            .unwrap_or_else(|| top_dir.to_path_buf());
        let repo_dir = PathBuf::from(&top_dir)
            .join(location)
            .join(repo.name.as_str());

        if !operations.is_repo(&repo_dir) {
            operations.clone_repo(&repo.remote, &repo_dir)?;
        }
        operations.fetch(&repo_dir)?;
        let revision = operations.rev_as_hash(&repo_dir, &repo.revision)?;
        operations.checkout(&revision, &repo_dir)?;
        lockfile_repos.push(LockedRepo {
            name: repo.name,
            revision: repo.revision,
        });
    }

    // 5. Create a lockfile out of the contents in the current manifest struct
    let mut new_lockfile = File::create(top_dir.join("chord.lock.yaml"))?;
    let new_lockfile_contents = Lockfile {
        repos: lockfile_repos,
    };
    serde_saphyr::to_io_writer(&mut new_lockfile, &new_lockfile_contents)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::workspace;

    use super::*;
    use pretty_assertions::assert_eq;
    use std::cell::Cell;
    use std::fs;
    use tempfile::tempdir;

    // Used for stubbing out operations requiring the git backend and topdir
    struct MockWorkspace {
        top_dir: PathBuf,
        git: MockGitBackend,
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
    struct MockGitBackend {
        is_repo_count: Cell<u32>,
        is_repo_return: Cell<bool>,
        clone_count: Cell<u32>,
        fetch_count: Cell<u32>,
        rev_as_hash_count: Cell<u32>,
        rev_as_hash_rev: Cell<String>,
        checkout_count: Cell<u32>,
    }

    impl GitOperations for MockGitBackend {
        fn is_repo(&self, repo_dir: impl AsRef<std::path::Path>) -> bool {
            repo_dir;
            self.is_repo_count.set(self.is_repo_count.get() + 1);
            self.is_repo_return.get()
        }
        fn rev_as_hash(&self, repo_dir: impl AsRef<std::path::Path>, rev: &str) -> Result<String> {
            repo_dir;
            self.rev_as_hash_rev.set(String::from(rev));
            self.rev_as_hash_count.set(self.rev_as_hash_count.get() + 1);
            Ok(String::from("0123456789012345678901234567890123456789"))
        }

        fn clone_repo(&self, remote: &str, repo_dir: impl AsRef<std::path::Path>) -> Result<()> {
            remote;
            repo_dir;
            self.clone_count.set(self.clone_count.get() + 1);
            Ok(())
        }

        fn fetch(&self, repo_dir: impl AsRef<std::path::Path>) -> Result<()> {
            repo_dir;
            self.fetch_count.set(self.fetch_count.get() + 1);
            Ok(())
        }

        fn checkout(&self, hash: &str, repo_dir: impl AsRef<std::path::Path>) -> Result<()> {
            hash;
            repo_dir;
            self.checkout_count.set(self.checkout_count.get() + 1);
            Ok(())
        }
    }

    impl MockGitBackend {
        fn new() -> Self {
            Self {
                is_repo_count: Cell::new(0),
                is_repo_return: Cell::new(false),
                clone_count: Cell::new(0),
                fetch_count: Cell::new(0),
                rev_as_hash_count: Cell::new(0),
                rev_as_hash_rev: Cell::new(String::new()),
                checkout_count: Cell::new(0),
            }
        }
    }

    fn default_manifest() -> &'static str {
        r#"
    repos:
      - name: chord
        remote: https://github.com/JorgeG-Dev/chord
        revision: main
    "#
    }

    // This will fail in practice
    fn multi_repo_manifest() -> &'static str {
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

    #[test]
    fn test_sync_clones_if_repo_missing() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("chord.yaml"), default_manifest()).unwrap();

        let workspace = MockWorkspace {
            top_dir: dir.path().to_path_buf(),
            git: MockGitBackend::new(),
        };
        workspace.git.is_repo_return.set(false);

        run(&workspace).unwrap();
        assert_eq!(1, workspace.git.clone_count.get());
        assert_eq!(1, workspace.git.fetch_count.get());
        assert_eq!(1, workspace.git.is_repo_count.get());
        assert_eq!(1, workspace.git.rev_as_hash_count.get());
        assert_eq!(1, workspace.git.checkout_count.get());
    }

    #[test]
    fn test_sync_creates_lockfile_if_missing() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("chord.yaml"), default_manifest()).unwrap();

        let workspace = MockWorkspace {
            top_dir: dir.path().to_path_buf(),
            git: MockGitBackend::new(),
        };
        workspace.git.is_repo_return.set(false);

        // Just ensuring the lockfile doesn't exist
        assert_eq!(false, dir.path().join("chord.lock.yaml").exists());
        run(&workspace).unwrap();
        assert_eq!(true, dir.path().join("chord.lock.yaml").exists());
    }

    #[test]
    fn test_sync_does_not_clone_if_exists() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("chord.yaml"), default_manifest()).unwrap();

        let workspace = MockWorkspace {
            top_dir: dir.path().to_path_buf(),
            git: MockGitBackend::new(),
        };
        workspace.git.is_repo_return.set(true);

        run(&workspace).unwrap();
        assert_eq!(0, workspace.git.clone_count.get());
        assert_eq!(1, workspace.git.fetch_count.get());
        assert_eq!(1, workspace.git.is_repo_count.get());
        assert_eq!(1, workspace.git.rev_as_hash_count.get());
        assert_eq!(1, workspace.git.checkout_count.get());
    }

    #[test]
    fn test_missing_manifest() {
        let dir = tempdir().unwrap();

        let workspace = MockWorkspace {
            top_dir: dir.path().to_path_buf(),
            git: MockGitBackend::new(),
        };

        assert_eq!(true, run(&workspace).is_err());
    }

    #[test]
    fn test_multi_sync_with_clone() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("chord.yaml"), multi_repo_manifest()).unwrap();

        let workspace = MockWorkspace {
            top_dir: dir.path().to_path_buf(),
            git: MockGitBackend::new(),
        };
        workspace.git.is_repo_return.set(false);

        run(&workspace).unwrap();
        assert_eq!(2, workspace.git.clone_count.get());
        assert_eq!(2, workspace.git.fetch_count.get());
        assert_eq!(2, workspace.git.is_repo_count.get());
        assert_eq!(2, workspace.git.rev_as_hash_count.get());
        assert_eq!(2, workspace.git.checkout_count.get());
    }

    #[test]
    fn test_sync_uses_lockfile_revision_if_exists() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("chord.yaml"), default_manifest()).unwrap();
        fs::write(
            dir.path().join("chord.lock.yaml"),
            r#"
repos:
  - name: chord
    revision: 0123456789012345678901234567890123456789
"#,
        )
        .unwrap();
        let workspace = MockWorkspace {
            top_dir: dir.path().to_path_buf(),
            git: MockGitBackend::new(),
        };
        workspace.git.is_repo_return.set(false);

        run(&workspace).unwrap();
        assert_eq!(
            "0123456789012345678901234567890123456789",
            workspace.git.rev_as_hash_rev.take()
        );
    }

    #[test]
    fn test_sync_with_empty_manifest() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("chord.yaml"), "repos: []").unwrap();

        let workspace = MockWorkspace {
            top_dir: dir.path().to_path_buf(),
            git: MockGitBackend::new(),
        };

        run(&workspace).unwrap();
        assert_eq!(0, workspace.git.clone_count.get());
        assert_eq!(0, workspace.git.fetch_count.get());
        assert_eq!(0, workspace.git.is_repo_count.get());
        assert_eq!(0, workspace.git.rev_as_hash_count.get());
        assert_eq!(0, workspace.git.checkout_count.get());
    }
}
