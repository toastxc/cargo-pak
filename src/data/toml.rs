use crate::data::desktop::DesktopFile;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr;

use crate::data::cargo::Cargo;

fn name() -> String {
    Cargo::name()
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ManifestToml {
    pub app_id: String,
    #[serde(default = "name")]
    pub app_name: String,
    #[serde(default = "name")]
    pub bin: String,
    #[serde(default = "profile")]
    pub profile: String,
    pub permissions: Option<HashSet<String>>,
    #[serde(rename = "desktopfile")]
    pub desktop_file: DesktopFile,
    pub runtime: Option<String>,
    pub runtime_version: Option<String>,
}

impl ManifestToml {
    pub fn read_file() -> crate::Result<Self> {
        let mut file: ManifestToml = toml::from_str::<ManifestToml>(&String::from_utf8(
            std::fs::read(PathBuf::from_str("pak.toml")?)?,
        )?)?;
        // set defaults
        if file.runtime.is_none() { file.runtime = Some("freedesktop".to_string()) };
        if file.runtime_version.is_none() { file.runtime_version = Some(crate::flatpak::Flatpak::runtime_version(&file.runtime.clone().unwrap())) };
        Ok(file)
    }
}

fn profile() -> String {
    String::from("release")
}
