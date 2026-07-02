mod common;

use chord_ws::commands::status;
use chord_ws::workspace::{GitBackend, Workspace};
use std::fs;

#[test]
fn test_status_no_manifest() {
    let workspace_dir = common::setup_workspace(common::default_manifest().as_str());
    fs::remove_file(workspace_dir.path().join("chord.yaml")).unwrap();

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = status(workspace);

    assert!(result.is_err());
}

#[test]
fn test_status_malformed_manifest() {
    let workspace_dir = common::setup_workspace(common::malformed_manifest().as_str());

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = status(workspace);

    assert!(result.is_err());
}

#[test]
fn test_status_no_lockfile() {
    let workspace_dir = common::setup_workspace(common::default_manifest().as_str());

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = status(workspace);

    // No lockfile doesn't mean error, means lock state is unknown
    assert!(result.is_ok());
}

#[test]
fn test_status_after_sync_succeeds() {
    let workspace_dir = common::setup_workspace(common::default_manifest().as_str());

    let sync_workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    chord_ws::commands::sync(sync_workspace).unwrap();

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = status(workspace);

    assert!(result.is_ok());
}

#[test]
fn test_status_repo_not_cloned() {
    let workspace_dir = common::setup_workspace(common::default_manifest().as_str());

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = status(workspace);

    // repo not present should render as "unavailable", not error the whole command
    assert!(result.is_ok());
}
