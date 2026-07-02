mod common;

use chord_ws::commands::topdir;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_topdir_at_workspace_root() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("chord.yaml"), "repos: []").unwrap();

    let result = topdir(dir.path());

    assert!(result.is_ok());
}

#[test]
fn test_topdir_from_nested_subdirectory() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("chord.yaml"), "repos: []").unwrap();
    let nested = dir.path().join("a/b/c");
    fs::create_dir_all(&nested).unwrap();

    let result = topdir(&nested);

    assert!(result.is_ok());
}

#[test]
fn test_topdir_not_in_workspace() {
    let dir = tempdir().unwrap();
    // no chord.yaml anywhere under dir

    let result = topdir(dir.path());

    assert!(result.is_ok()); // still Ok — just prints "not within a workspace"
}
