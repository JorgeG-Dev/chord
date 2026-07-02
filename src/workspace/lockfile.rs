use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

/// Represents the complete Chord lockfile. Just a hashmap where the key
/// is the name of the repo, matching that in the manifest, and the value is
/// the revision to pin the repo to.
#[derive(Serialize, Deserialize)]
pub struct Lockfile(HashMap<String, String>);

impl Lockfile {
    pub fn read(path: impl AsRef<Path>) -> Result<Self> {
        let reader =
            File::open(path.as_ref().join("chord.lock.yaml")).context("failed to open lockfile")?;
        let lockfile = serde_saphyr::from_reader(reader).context("failed to parse lockfile")?;
        Ok(lockfile)
    }

    pub fn write(&self, path: impl AsRef<Path>) -> Result<()> {
        let mut writer = File::create(path.as_ref().join("chord.lock.yaml"))
            .context("failed to create lockfile")?;
        serde_saphyr::to_io_writer(&mut writer, self).context("failed to write lockfile")?;
        Ok(())
    }

    pub fn insert(&mut self, k: String, v: String) {
        self.0.insert(k, v);
    }

    pub fn get(&self, k: &str) -> Option<&String> {
        self.0.get(k)
    }

    pub fn remove(&mut self, k: &str) -> Option<String> {
        self.0.remove(k)
    }

    pub fn new() -> Self {
        Self { 0: HashMap::new() }
    }
}
