mod common;

use chord_ws::commands::update;
use chord_ws::workspace::{GitBackend, Lockfile, Workspace};
use std::fs;

#[test]
fn test_update_clones_repo() {
    let workspace_dir = common::setup_workspace(common::default_manifest().as_str());

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = update(workspace);

    assert!(result.is_ok());
    assert!(workspace_dir.path().join(common::VALID_REPO_NAME).exists());
    assert!(
        workspace_dir
            .path()
            .join(format!("{}/.git", common::VALID_REPO_NAME))
            .exists()
    );
    assert!(workspace_dir.path().join("chord.lock.yaml").exists());
}

#[test]
fn test_update_clones_multiple_repos() {
    let workspace_dir = common::setup_workspace(common::default_multi_repo_manifest().as_str());

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = update(workspace);

    assert!(result.is_ok());
    assert!(
        workspace_dir
            .path()
            .join(format!("{}1", common::VALID_REPO_NAME))
            .exists()
    );
    assert!(
        workspace_dir
            .path()
            .join(format!("{}2", common::VALID_REPO_NAME))
            .exists()
    );
    assert!(
        workspace_dir
            .path()
            .join(format!("{}1/.git", common::VALID_REPO_NAME))
            .exists()
    );
    assert!(
        workspace_dir
            .path()
            .join(format!("{}2/.git", common::VALID_REPO_NAME))
            .exists()
    );
    assert!(workspace_dir.path().join("chord.lock.yaml").exists());
}

#[test]
fn test_update_does_not_discard_local_changes() {
    let workspace_dir = common::setup_workspace(common::default_manifest().as_str());

    // Set up a workspace properly
    update(Workspace::new(
        workspace_dir.path().to_path_buf(),
        GitBackend,
    ))
    .unwrap();

    // "Dirtying" the repo
    let marker = workspace_dir
        .path()
        .join(common::VALID_REPO_NAME)
        .join("local-marker.txt");
    fs::write(&marker, "should survive").unwrap();

    let result = update(Workspace::new(
        workspace_dir.path().to_path_buf(),
        GitBackend,
    ));

    assert!(result.is_ok());
    assert!(marker.exists());
}

#[test]
fn test_update_repo_clone_fail() {
    let workspace_dir = common::setup_workspace(common::invalid_remote_manifest().as_str());

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = update(workspace);

    assert!(result.is_err());
    assert!(
        !workspace_dir
            .path()
            .join(common::INVALID_REPO_NAME)
            .exists()
    );
    assert!(workspace_dir.path().join("chord.lock.yaml").exists());
    let lockfile = Lockfile::read(workspace_dir.path()).unwrap();
    assert!(lockfile.get(common::INVALID_REPO_NAME).is_none());
}

#[test]
fn test_update_multi_repo_clone_fail() {
    let workspace_dir =
        common::setup_workspace(common::invalid_remote_multi_repo_manifest().as_str());

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = update(workspace);

    assert!(result.is_err());
    assert!(
        !workspace_dir
            .path()
            .join(format!("{}1", common::INVALID_REPO_NAME))
            .exists()
    );
    assert!(
        !workspace_dir
            .path()
            .join(format!("{}2", common::INVALID_REPO_NAME))
            .exists()
    );
    assert!(workspace_dir.path().join("chord.lock.yaml").exists());
    let lockfile = Lockfile::read(workspace_dir.path()).unwrap();
    assert!(
        lockfile
            .get(format!("{}1", common::INVALID_REPO_NAME).as_str())
            .is_none()
    );
    assert!(
        lockfile
            .get(format!("{}2", common::INVALID_REPO_NAME).as_str())
            .is_none()
    );
}

#[test]
fn test_update_partial_repo_clone_fail() {
    let workspace_dir =
        common::setup_workspace(common::valid_and_invalid_remote_repo_manifest().as_str());

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = update(workspace);

    assert!(result.is_err());
    assert!(
        !workspace_dir
            .path()
            .join(common::INVALID_REPO_NAME)
            .exists()
    );
    assert!(workspace_dir.path().join(common::VALID_REPO_NAME).exists());
    assert!(workspace_dir.path().join("chord.lock.yaml").exists());
    let lockfile = Lockfile::read(workspace_dir.path()).unwrap();
    assert!(lockfile.get(common::INVALID_REPO_NAME).is_none());
    assert!(lockfile.get(common::VALID_REPO_NAME).is_some());
}

#[test]
fn test_update_partial_repo_checkout_fail() {
    let workspace_dir =
        common::setup_workspace(common::valid_and_invalid_rev_repo_manifest().as_str());

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = update(workspace);

    assert!(result.is_err());
    assert!(
        workspace_dir
            .path()
            .join(common::INVALID_REPO_NAME)
            .exists()
    );
    assert!(workspace_dir.path().join(common::VALID_REPO_NAME).exists());
    assert!(workspace_dir.path().join("chord.lock.yaml").exists());
    let lockfile = Lockfile::read(workspace_dir.path()).unwrap();
    assert!(lockfile.get(common::INVALID_REPO_NAME).is_none());
    assert!(lockfile.get(common::VALID_REPO_NAME).is_some());
}

#[test]
fn test_update_multi_repo_checkout_fail() {
    let workspace_dir = common::setup_workspace(common::invalid_rev_multi_repo_manifest().as_str());

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = update(workspace);

    assert!(result.is_err());
    assert!(
        workspace_dir
            .path()
            .join(format!("{}1", common::INVALID_REPO_NAME))
            .exists()
    );
    assert!(
        workspace_dir
            .path()
            .join(format!("{}2", common::INVALID_REPO_NAME))
            .exists()
    );
    assert!(workspace_dir.path().join("chord.lock.yaml").exists());
    let lockfile = Lockfile::read(workspace_dir.path()).unwrap();
    assert!(
        lockfile
            .get(format!("{}1", common::INVALID_REPO_NAME).as_str())
            .is_none()
    );
    assert!(
        lockfile
            .get(format!("{}2", common::INVALID_REPO_NAME).as_str())
            .is_none()
    );
}

#[test]
fn test_update_repo_checkout_fail() {
    let workspace_dir = common::setup_workspace(common::invalid_rev_manifest().as_str());

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = update(workspace);

    assert!(result.is_err());
    assert!(
        workspace_dir
            .path()
            .join(common::INVALID_REPO_NAME)
            .exists()
    );
    assert!(workspace_dir.path().join("chord.lock.yaml").exists());
    let lockfile = Lockfile::read(workspace_dir.path()).unwrap();
    assert!(lockfile.get(common::INVALID_REPO_NAME).is_none());
}

#[test]
fn test_update_stale_lockfile() {
    let workspace_dir = common::setup_workspace(common::default_multi_repo_manifest().as_str());

    // Set up a workspace properly and generate a "stale" lockfile
    update(Workspace::new(
        workspace_dir.path().to_path_buf(),
        GitBackend,
    ))
    .unwrap();

    // Overwriting the manifest
    let manifest = workspace_dir.path().join("chord.yaml");
    fs::remove_file(&manifest).unwrap();
    fs::write(&manifest, common::default_manifest().as_str()).unwrap();

    let result = update(Workspace::new(
        workspace_dir.path().to_path_buf(),
        GitBackend,
    ));

    assert!(result.is_ok());
    assert!(
        workspace_dir
            .path()
            .join(format!("{}1", common::VALID_REPO_NAME))
            .exists()
    );
    assert!(
        workspace_dir
            .path()
            .join(format!("{}2", common::VALID_REPO_NAME))
            .exists()
    );
    assert!(
        workspace_dir
            .path()
            .join(format!("{}", common::VALID_REPO_NAME))
            .exists()
    );
    let lockfile = Lockfile::read(workspace_dir.path()).unwrap();
    assert!(lockfile.get(common::VALID_REPO_NAME).is_some());
    assert!(
        lockfile
            .get(format!("{}1", common::VALID_REPO_NAME).as_str())
            .is_none()
    );
    assert!(
        lockfile
            .get(format!("{}2", common::VALID_REPO_NAME).as_str())
            .is_none()
    );
}

#[test]
fn test_update_valid_location_specified() {
    let workspace_dir = common::setup_workspace(common::manifest_with_valid_location().as_str());
    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = update(workspace);

    assert!(result.is_ok());
    assert!(
        workspace_dir
            .path()
            .join(common::VALID_LOCATION)
            .join(format!("{}", common::VALID_REPO_NAME))
            .exists()
    );
}

#[test]
fn test_update_invalid_location_specified() {
    let workspace_dir = common::setup_workspace(common::manifest_with_invalid_location().as_str());
    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = update(workspace);

    assert!(result.is_err());
    assert!(
        !workspace_dir
            .path()
            .join(common::INVALID_LOCATION)
            .join(format!("{}", common::VALID_REPO_NAME))
            .exists()
    );
}

#[test]
fn test_update_missing_manifest() {
    let workspace_dir = common::setup_workspace(common::manifest_with_valid_location().as_str());
    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    fs::remove_file(workspace_dir.path().join("chord.yaml")).unwrap();
    let result = update(workspace);

    assert!(result.is_err());
}

#[test]
fn test_update_malformed_manifest() {
    let workspace_dir = common::setup_workspace(common::malformed_manifest().as_str());
    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = update(workspace);

    assert!(result.is_err());
}

#[test]
fn test_update_overwrite_pinned_lockfile() {
    let workspace_dir = common::setup_workspace(common::pinned_commit_manifest().as_str());
    fs::write(
        workspace_dir.path().join("chord.lock.yaml"),
        common::first_commit_lockfile().as_str(),
    )
    .unwrap();

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = update(workspace);

    assert!(result.is_ok());
    assert!(workspace_dir.path().join(common::VALID_REPO_NAME).exists());
    assert!(workspace_dir.path().join("chord.lock.yaml").exists());
    let lockfile = Lockfile::read(workspace_dir.path()).unwrap();
    assert!(common::PINNED_COMMIT_HASH == lockfile.get(common::VALID_REPO_NAME).unwrap());
}
