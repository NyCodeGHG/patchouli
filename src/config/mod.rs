use std::fs;

use color_eyre::Result;
use serde::Deserialize;

mod git_repo;

pub use git_repo::GitRepo;

#[derive(Debug, Deserialize)]
#[non_exhaustive]
#[serde(deny_unknown_fields)]
pub struct PatchouliConfig {
    pub name: String,
    pub upstream: GitRepo,
    pub r#ref: String,
    #[serde(default)]
    pub unneeded_files: Vec<String>,
}

pub fn read() -> Result<PatchouliConfig> {
    let config = fs::read_to_string("patchouli.toml")?;
    Ok(toml::from_str(&config)?)
}
