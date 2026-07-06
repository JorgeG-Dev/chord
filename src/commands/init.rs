//! Contains the logic for performing the Init command
use crate::workspace::Manifest;
use anyhow::{Result, bail};
use serde_saphyr;
use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;

/// Initializes a chord workspace by creating a chord.yaml
/// with an example repo pointing at this project's repo
pub fn run(path: impl AsRef<Path>) -> Result<()> {
    if !path.as_ref().exists() {
        bail!("{} does not exist", path.as_ref().display());
    }
    let manifest_dir = path.as_ref().canonicalize()?;

    // 2. Create the default configuration
    let mut default_manifest = Manifest::new();
    default_manifest.add_repo(
        "chord".to_string(),
        "https://github.com/JorgeG-Dev/chord.git".to_string(),
        "main".to_string(),
        None,
    )?;

    // 3. Attempt to create the new file and write the default manifest to it
    let mut manifest_file = match File::create_new(manifest_dir.join("chord.yaml")) {
        Ok(file) => file,
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            bail!("there is already a manifest at {}", manifest_dir.display())
        }
        Err(e) => bail!(e),
    };
    serde_saphyr::to_io_writer(&mut manifest_file, &default_manifest)?;

    Ok(())
}
