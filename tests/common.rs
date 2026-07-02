use std::fs;
use tempfile::TempDir;

pub const FIRST_COMMIT_HASH: &'static str = "58b8d932d68898d7e5ecccb951e55d956f65b60e";
pub const PINNED_COMMIT_HASH: &'static str = "c45cf5c7fa8ed0c325b00006a3c87220165adae7";
pub const VALID_REPO_REMOTE: &'static str = "https://github.com/JorgeG-Dev/chord";
pub const VALID_REPO_NAME: &'static str = "chord";
pub const INVALID_REPO_REV: &'static str = "invalid";
pub const INVALID_REPO_NAME: &'static str = "invalid";
pub const INVALID_REPO_REMOTE: &'static str = "https://github.com/JorgeG-Dev/invalid";
pub const VALID_LOCATION: &'static str = "valid";
pub const INVALID_LOCATION: &'static str = "/";

pub fn setup_workspace(manifest_content: &str) -> TempDir {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("chord.yaml"), manifest_content).unwrap();
    dir
}

pub fn default_manifest() -> String {
    format!(
        r#"
repos:
  - name: {} 
    remote: {}
    revision: main
"#,
        VALID_REPO_NAME, VALID_REPO_REMOTE
    )
}

pub fn malformed_manifest() -> String {
    format!(
        r#"
repos:
  - this: {} 
    is: {}
    wrong: main
"#,
        VALID_REPO_NAME, VALID_REPO_REMOTE
    )
}

pub fn default_multi_repo_manifest() -> String {
    format!(
        r#"
repos:
  - name: {}1
    remote: {}
    revision: main
  - name: {}2
    remote: {}
    revision: main
"#,
        VALID_REPO_NAME, VALID_REPO_REMOTE, VALID_REPO_NAME, VALID_REPO_REMOTE
    )
}

pub fn invalid_rev_manifest() -> String {
    format!(
        r#"
repos:
  - name: {}
    remote: {} 
    revision: {}
"#,
        INVALID_REPO_NAME, VALID_REPO_REMOTE, INVALID_REPO_REV
    )
}

pub fn invalid_remote_manifest() -> String {
    format!(
        r#"
repos:
  - name: {}
    remote: {} 
    revision: {}
"#,
        INVALID_REPO_NAME, INVALID_REPO_REMOTE, FIRST_COMMIT_HASH
    )
}

pub fn invalid_rev_multi_repo_manifest() -> String {
    format!(
        r#"
repos:
  - name: {}1
    remote: {}
    revision: {}
  - name: {}2
    remote: {}
    revision: {}
"#,
        INVALID_REPO_NAME,
        VALID_REPO_REMOTE,
        INVALID_REPO_REV,
        INVALID_REPO_NAME,
        VALID_REPO_REMOTE,
        INVALID_REPO_REV
    )
}

pub fn invalid_remote_multi_repo_manifest() -> String {
    format!(
        r#"
repos:
  - name: {}1
    remote: {}
    revision: {}
  - name: {}2
    remote: {}
    revision: {}
"#,
        INVALID_REPO_NAME,
        INVALID_REPO_REMOTE,
        PINNED_COMMIT_HASH,
        INVALID_REPO_NAME,
        INVALID_REPO_REMOTE,
        PINNED_COMMIT_HASH
    )
}

pub fn valid_and_invalid_remote_repo_manifest() -> String {
    format!(
        r#"
repos:
  - name: {}
    remote: {}
    revision: {}
  - name: {}
    remote: {}
    revision: {}
"#,
        INVALID_REPO_NAME,
        INVALID_REPO_REMOTE,
        PINNED_COMMIT_HASH,
        VALID_REPO_NAME,
        VALID_REPO_REMOTE,
        PINNED_COMMIT_HASH
    )
}

pub fn valid_and_invalid_rev_repo_manifest() -> String {
    format!(
        r#"
repos:
  - name: {}
    remote: {}
    revision: {}
  - name: {}
    remote: {}
    revision: {}
"#,
        INVALID_REPO_NAME,
        VALID_REPO_REMOTE,
        INVALID_REPO_REV,
        VALID_REPO_NAME,
        VALID_REPO_REMOTE,
        PINNED_COMMIT_HASH
    )
}

pub fn pinned_commit_manifest() -> String {
    format!(
        r#"
repos:
  - name: {}
    remote: {}
    revision: {} 
"#,
        VALID_REPO_NAME, VALID_REPO_REMOTE, PINNED_COMMIT_HASH
    )
}

pub fn first_commit_lockfile() -> String {
    format!(
        r#"
{}: {} 
"#,
        VALID_REPO_NAME, FIRST_COMMIT_HASH
    )
}

pub fn manifest_with_valid_location() -> String {
    format!(
        r#"
repos:
  - name: {} 
    remote: {}
    revision: main
    location: {}
"#,
        VALID_REPO_NAME, VALID_REPO_REMOTE, VALID_LOCATION
    )
}

pub fn manifest_with_invalid_location() -> String {
    format!(
        r#"
repos:
  - name: {} 
    remote: {}
    revision: main
    location: {}
"#,
        VALID_REPO_NAME, VALID_REPO_REMOTE, INVALID_LOCATION
    )
}
