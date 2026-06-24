use std::{env, path};

/// Walks from the current directory up to the root until it either finds
/// a Chord manifest file or there are no more directories left to check
pub fn get_top_dir() -> Option<path::PathBuf> {
    match env::current_dir() {
        Ok(mut current) => {
            let mut found = false;
            loop {
                if current.join("chord.yaml").exists() {
                    found = true;
                    break;
                }

                // 3. If manifest file doesn't exist, go up one directory
                if let Some(next) = current.parent() {
                    current = next.to_path_buf();
                } else {
                    break;
                }
            }

            if found { Some(current) } else { None }
        }
        Err(_) => None,
    }
}
