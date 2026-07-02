use std::path::{Path, PathBuf};

/// Walks from the given directory up to the root until it either finds
/// a Chord manifest file or there are no more directories left to check
pub fn get_top_dir(start: impl AsRef<Path>) -> Option<PathBuf> {
    let mut current = start.as_ref().to_path_buf();
    loop {
        if current.join("chord.yaml").exists() {
            return Some(current);
        }

        match current.parent() {
            Some(next) => current = next.to_path_buf(),
            None => return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_get_top_dir_at_root() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("chord.yaml"), "repos: []").unwrap();

        let result = get_top_dir(dir.path());

        assert_eq!(result, Some(dir.path().to_path_buf()));
    }

    #[test]
    fn test_get_top_dir_from_nested_subdirectory() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("chord.yaml"), "repos: []").unwrap();
        let nested = dir.path().join("a/b/c");
        fs::create_dir_all(&nested).unwrap();

        let result = get_top_dir(&nested);

        assert_eq!(result, Some(dir.path().to_path_buf()));
    }

    #[test]
    fn test_get_top_dir_stops_exactly_at_first_manifest_found() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("chord.yaml"), "repos: []").unwrap();
        let nested = dir.path().join("nested");
        fs::create_dir_all(&nested).unwrap();
        fs::write(nested.join("chord.yaml"), "repos: []").unwrap();

        // starting from nested, should find nested's manifest, not walk further up to dir's
        let result = get_top_dir(&nested);

        assert_eq!(result, Some(nested));
    }
}
