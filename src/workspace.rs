mod git;
mod manifest;
pub mod utils;

pub use git::GitBackend;
pub use git::Operations as GitOperations;
pub use manifest::Manifest;
pub use manifest::Repo as ManifestRepo;
