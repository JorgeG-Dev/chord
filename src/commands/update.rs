//! Contains the logic for performing the Update command
use crate::workspace::{GitOperations, LockedRepo, Lockfile, Manifest, Operations};

use anyhow::Result;
use serde_saphyr;
use std::{fs::File, path::PathBuf};

/// Syncs the repos in the manifest to the revisions specified in
/// the manifest, ignoring the lockfile. Generates a new file after
/// completion.
pub fn run(workspace: &impl Operations) -> Result<()> {
    let top_dir = workspace.top_dir();
    let operations = workspace.git();

    // 1. Open and parse the manifest file
    let mut manifest = Manifest::read(&top_dir)?;

    // 2. Drain the manifest repos, perform update operations, and create lockfile
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
            revision,
        });
    }

    // 3. Create a lockfile out of the contents in the current manifest struct
    let mut new_lockfile = File::create(top_dir.join("chord.lock.yaml"))?;
    let new_lockfile_contents = Lockfile {
        repos: lockfile_repos,
    };
    serde_saphyr::to_io_writer(&mut new_lockfile, &new_lockfile_contents)?;
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
    fn test_update_clones_if_repo_missing() {
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
    fn test_update_creates_lockfile_if_missing() {
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
    fn test_update_does_not_clone_if_exists() {
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
    fn test_multi_update_with_clone() {
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
    fn test_update_does_not_use_lockfile_revision_at_all() {
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
        assert_eq!("main", workspace.git.rev_as_hash_rev.take());
    }

    #[test]
    fn test_update_with_empty_manifest() {
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
    fn test_update_overwrites_lockfile_with_new_sha() {
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
