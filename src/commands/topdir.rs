use crate::manifest;
use anyhow::{Context, Result, anyhow, bail};
use serde_saphyr;
use std::fs::File;
use std::io::ErrorKind;
use std::{env, path};

/// Initializes a Chord workspace by creating a chord.yaml file at the
/// specified directory
pub fn run() -> Result<()> {
    // 1. Use the directory the program is being invoked from as a starting
    // point
    let mut found = false;
    let mut current = env::current_dir()?;
    loop {
        // 2. Check if a manifest file exists in the current directory, exit
        // the search loop if it does
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

    // 4. Print the result
    if found {
        println!("{}", current.display());
    } else {
        println!("Not in a Chord workspace");
    }
    Ok(())
}
