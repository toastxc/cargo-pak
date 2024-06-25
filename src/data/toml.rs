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
}

impl ManifestToml {
    pub fn read_file() -> crate::Result<Self> {
        let file: ManifestToml = toml::from_str::<ManifestToml>(&String::from_utf8(
            std::fs::read(PathBuf::from_str("pak.toml")?)?,
        )?)?;
        Ok(file)
    }
}

fn profile() -> String {
    String::from("release")
}
