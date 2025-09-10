use crate::{data::{desktop::DesktopFile, yaml::Module}};
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
    #[serde(default = "runtime")]
    pub runtime: String,
    pub runtime_version: Option<String>,
    #[serde(rename = "desktopfile")]
    pub desktop_file: DesktopFile,
    pub modules: Vec<Module>,
}

impl ManifestToml {
    pub fn read_file() -> crate::Result<Self> {
        let mut file: ManifestToml = toml::from_str::<ManifestToml>(&String::from_utf8(
            std::fs::read(PathBuf::from_str("pak.toml")?)?,
        )?)?;
   
        // might add support later?
        // if file.runtime.is_none() { Some("freedesktop".to_string());  };
        // if file.runtime_version.is_none() { Some(crate::flatpak::Flatpak::runtime_version(&file.runtime.clone().unwrap())); };


        if file.runtime_version.is_none() {
            file.runtime_version = Some(crate::flatpak::Flatpak::runtime_version(&file.runtime.clone()));
        };
        Ok(file)
    }
}

fn profile() -> String {
    String::from("release")
}

fn runtime() -> String {
    String::from("freedesktop")
}