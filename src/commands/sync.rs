//! Contains the logic for performing the Sync command
use crate::workspace::{GitOperations, Lockfile, Manifest, Operations};

use anyhow::Result;
use std::path::PathBuf;

/// Attempts to parse the manifest file and sync it to the revisions outlined
/// in the lockfile, if it exists. If not, a new lockfile is created and each
/// successfully synced repo is pinned to a revision there.
pub fn run(workspace: &impl Operations) -> Result<()> {
    let top_dir = workspace.top_dir();
    let operations = workspace.git();

    // 1. Open and parse the manifest file
    let mut manifest = Manifest::read(&top_dir)?;

    // 2. Try to open the lockfile and get its contents
    let lockfile = match Lockfile::read("chord.lock.yaml") {
        Ok(lockfile) => Some(lockfile),
        Err(_) => None,
    };

    // 3. If a lockfile was actually parsed, go through the manifest, updating
    // updating the revisions and location
    if let Some(mut lockfile) = lockfile {
        manifest.sync(&mut lockfile);
    }

    // 4. Drain the manifest repos, perform sync operations, and create lockfile
    // struct
    let mut new_lockfile = Lockfile::new();
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
        new_lockfile.insert(repo.name, repo.revision);
    }

    // 5. Write the new lockfile to disk
    new_lockfile.write(top_dir)?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::workspace::mock::*;
    use pretty_assertions::assert_eq;
    use std::fs;
    use tempfile::tempdir;

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

    #[test]
    fn test_sync_uses_manifest_revision_if_repo_not_in_lockfile() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("chord.yaml"), default_manifest()).unwrap();
        fs::write(
            dir.path().join("chord.lock.yaml"),
            r#"
repos:
  - name: dummy 
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

        assert_eq!("main", workspace.git.rev_as_hash_rev.take());
    }

    #[test]
    fn test_sync_overwrites_lockfile_with_new_sha() {
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
        workspace
            .git
            .rev_as_hash_return
            .set(String::from("11223344"));

        run(&workspace).unwrap();

        let lockfile_contents = fs::read_to_string(dir.path().join("chord.lock.yaml")).unwrap();
        println!("{:?}", lockfile_contents);
        assert!(lockfile_contents.contains("11223344"));
    }
}
