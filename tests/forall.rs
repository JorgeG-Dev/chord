mod common;

use chord_ws::commands::{forall, update};
use chord_ws::workspace::{GitBackend, Workspace};
use std::fs;

#[test]
fn test_forall_runs_in_each_repo_dir() {
    let workspace_dir = common::setup_workspace(common::default_multi_repo_manifest().as_str());
    update(Workspace::new(
        workspace_dir.path().to_path_buf(),
        GitBackend,
    ))
    .unwrap();

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = forall(vec!["touch".into(), "marker.txt".into()], workspace);

    assert!(result.is_ok());
    assert!(
        workspace_dir
            .path()
            .join(format!("{}1", common::VALID_REPO_NAME))
            .join("marker.txt")
            .exists()
    );
    assert!(
        workspace_dir
            .path()
            .join(format!("{}2", common::VALID_REPO_NAME))
            .join("marker.txt")
            .exists()
    );
}

#[test]
fn test_forall_command_fails_in_one_repo() {
    let workspace_dir = common::setup_workspace(common::default_multi_repo_manifest().as_str());
    update(Workspace::new(
        workspace_dir.path().to_path_buf(),
        GitBackend,
    ))
    .unwrap();

    // remove one repo's directory entirely so repo_run's is_repo check fails for it
    fs::remove_dir_all(
        workspace_dir
            .path()
            .join(format!("{}1", common::VALID_REPO_NAME)),
    )
    .unwrap();

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = forall(vec!["touch".into(), "marker.txt".into()], workspace);

    assert!(result.is_err());
    // the other repo should still have gotten the command run against it
    assert!(
        workspace_dir
            .path()
            .join(format!("{}2", common::VALID_REPO_NAME))
            .join("marker.txt")
            .exists()
    );
}

#[test]
fn test_forall_command_returns_nonzero_exit() {
    let workspace_dir = common::setup_workspace(common::default_manifest().as_str());
    update(Workspace::new(
        workspace_dir.path().to_path_buf(),
        GitBackend,
    ))
    .unwrap();

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = forall(vec!["exit".into(), "1".into()], workspace);

    assert!(result.is_err());
}

#[test]
fn test_forall_missing_manifest() {
    let workspace_dir = common::setup_workspace(common::default_manifest().as_str());
    fs::remove_file(workspace_dir.path().join("chord.yaml")).unwrap();

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    let result = forall(vec!["echo".into(), "hi".into()], workspace);

    assert!(result.is_err());
}

#[test]
fn test_forall_command_with_spaces_and_args() {
    let workspace_dir = common::setup_workspace(common::default_manifest().as_str());
    update(Workspace::new(
        workspace_dir.path().to_path_buf(),
        GitBackend,
    ))
    .unwrap();

    let workspace = Workspace::new(workspace_dir.path().to_path_buf(), GitBackend);
    // command.join(" ") should reconstruct this properly for sh -c
    let result = forall(
        vec![
            "echo".into(),
            "hello".into(),
            "world".into(),
            ">".into(),
            "out.txt".into(),
        ],
        workspace,
    );

    assert!(result.is_ok());
    let contents = fs::read_to_string(
        workspace_dir
            .path()
            .join(common::VALID_REPO_NAME)
            .join("out.txt"),
    )
    .unwrap();
    assert_eq!(contents.trim(), "hello world");
}
