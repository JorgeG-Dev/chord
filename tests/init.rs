mod common;

use chord_ws::commands::init;
use chord_ws::workspace::Manifest;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_init_creates_manifest() {
    let dir = tempdir().unwrap();

    let result = init(dir.path());

    assert!(result.is_ok());
    assert!(dir.path().join("chord.yaml").exists());
}

#[test]
fn test_init_manifest_has_default_repo() {
    let dir = tempdir().unwrap();

    init(dir.path()).unwrap();

    let manifest = Manifest::read(dir.path()).unwrap();
    assert_eq!(manifest.repos.len(), 1);
    assert_eq!(manifest.repos[0].name, "chord");
    assert_eq!(
        manifest.repos[0].remote,
        "https://github.com/JorgeG-Dev/chord"
    );
    assert_eq!(manifest.repos[0].revision, "main");
}

#[test]
fn test_init_fails_if_path_does_not_exist() {
    let dir = tempdir().unwrap();
    let nonexistent = dir.path().join("does-not-exist");

    let result = init(&nonexistent);

    assert!(result.is_err());
    assert!(!nonexistent.join("chord.yaml").exists());
}

#[test]
fn test_init_fails_if_manifest_already_exists() {
    let dir = tempdir().unwrap();

    let first = init(dir.path());
    assert!(first.is_ok());

    let second = init(dir.path());
    assert!(second.is_err());
}

#[test]
fn test_init_does_not_overwrite_existing_manifest() {
    let dir = tempdir().unwrap();

    init(dir.path()).unwrap();
    let original_contents = fs::read_to_string(dir.path().join("chord.yaml")).unwrap();

    // second init call should fail and leave the original file untouched
    let _ = init(dir.path());

    let contents_after = fs::read_to_string(dir.path().join("chord.yaml")).unwrap();
    assert_eq!(original_contents, contents_after);
}
