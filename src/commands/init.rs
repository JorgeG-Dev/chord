use crate::manifest;
use anyhow::{Context, Result, anyhow, bail};
use serde_saphyr;
use std::fs::File;
use std::io::ErrorKind;
use std::path;

/// Initializes a Chord workspace by creating a chord.yaml file at the
/// specified directory
pub fn run(path: impl AsRef<path::Path>) -> Result<()> {
    let manifest_dir = path.as_ref();

    // 1. Check if the provided manifest directory actually exists
    if !manifest_dir.exists() {
        return Err(anyhow!(
            "Can't create workspace because {} does not exist",
            manifest_dir.display()
        ));
    }

    // 2. Create the default configuration
    let default_repo = manifest::Repo {
        url: String::from("https://github.com/JorgeG-Dev/chord"),
        rev: Some(String::from("main")),
        name: Some(String::from("chord")),
        path: Some(String::from(".")),
    };
    let default_manifest = manifest::File {
        repos: vec![default_repo],
    };

    // 3. Attempt to create the new file and write the default manifest to it
    let mut manifest_file = match File::create_new(manifest_dir.join("chord.yaml")) {
        Ok(file) => file,
        Err(e) if e.kind() == ErrorKind::AlreadyExists => bail!(
            "There is already a chord.yaml at {}",
            manifest_dir.display()
        ),
        Err(e) => bail!(
            "Could not create {}/chord.yaml due to following error {}",
            manifest_dir.display(),
            e
        ),
    };
    serde_saphyr::to_io_writer(&mut manifest_file, &default_manifest).with_context(|| {
        format!(
            "Failed to write default configuration to {} due to following error",
            manifest_dir.display()
        )
    })?;

    Ok(())
}
