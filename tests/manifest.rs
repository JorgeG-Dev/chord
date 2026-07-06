mod common;

use chord_ws::commands::{manifest_add, manifest_modify, manifest_remove};
use chord_ws::workspace::{GitBackend, Manifest, Workspace};
use std::path::PathBuf;

#[test]
fn test_manifest_add_repo_already_exists() {
    let workspace_dir = common::setup_workspace(common::default_manifest().as_str());

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = manifest_add(
        String::from("chord"),
        "url".to_string(),
        "rev".to_string(),
        Some(PathBuf::from(".")),
        workspace,
    );

    assert!(result.is_err());
}

#[test]
fn test_manifest_add_repo_success() {
    let workspace_dir = common::setup_workspace(common::default_manifest().as_str());

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = manifest_add(
        common::INEXISTENT_REPO_NAME.to_string(),
        "url".to_string(),
        "rev".to_string(),
        Some(PathBuf::from(".")),
        workspace,
    );

    assert!(result.is_ok());
    let test_manifest = Manifest::read(workspace_dir.path()).unwrap();
    let mut found = false;
    for repo in test_manifest.repos {
        if repo.name == common::INEXISTENT_REPO_NAME {
            found = true;
            break;
        }
    }
    assert!(found);
}

#[test]
fn test_manifest_remove_inexistent_repo() {
    let workspace_dir = common::setup_workspace(common::default_manifest().as_str());

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = manifest_remove(common::INEXISTENT_REPO_NAME.to_string(), workspace);

    assert!(result.is_err());
}

#[test]
fn test_manifest_remove_success() {
    let workspace_dir = common::setup_workspace(common::default_manifest().as_str());

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = manifest_remove(common::VALID_REPO_NAME.to_string(), workspace);

    assert!(result.is_ok());
    let test_manifest = Manifest::read(workspace_dir.path()).unwrap();
    let mut found = false;
    for repo in test_manifest.repos {
        if repo.name == common::VALID_REPO_NAME {
            found = true;
            break;
        }
    }
    assert!(!found);
}

#[test]
fn test_manifest_modify_success() {
    let workspace_dir = common::setup_workspace(common::default_manifest().as_str());

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = manifest_modify(
        common::VALID_REPO_NAME.to_string(),
        Some(common::MODIFIED_REPO_NAME.to_string()),
        None,
        None,
        None,
        workspace,
    );

    assert!(result.is_ok());
    let test_manifest = Manifest::read(workspace_dir.path()).unwrap();
    let mut found = false;
    for repo in test_manifest.repos {
        if repo.name == common::MODIFIED_REPO_NAME {
            found = true;
            break;
        }
    }
    assert!(found);
}

#[test]
fn test_manifest_modify_repo_inexistent() {
    let workspace_dir = common::setup_workspace(common::default_manifest().as_str());

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = manifest_modify(
        common::INEXISTENT_REPO_NAME.to_string(),
        Some(common::MODIFIED_REPO_NAME.to_string()),
        None,
        None,
        None,
        workspace,
    );

    assert!(result.is_err());
    let test_manifest = Manifest::read(workspace_dir.path()).unwrap();
    let mut found = false;
    for repo in test_manifest.repos {
        if repo.name == common::MODIFIED_REPO_NAME {
            found = true;
            break;
        }
    }
    assert!(!found);
}

#[test]
fn test_manifest_modify_repo_no_values_provided() {
    let workspace_dir = common::setup_workspace(common::default_manifest().as_str());

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = manifest_modify(
        common::INEXISTENT_REPO_NAME.to_string(),
        None,
        None,
        None,
        None,
        workspace,
    );

    assert!(result.is_err());
}
