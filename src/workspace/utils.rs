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
