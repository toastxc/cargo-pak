use crate::data::cargo::Cargo;
use crate::data::toml::ManifestToml;
use crate::shell::Shell;
use crate::version;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ManifestYaml {
    pub id: String,
    pub runtime: String,
    #[serde(rename = "runtime-version")]
    pub runtime_version: String,
    pub sdk: String,
    pub command: String,
    #[serde(rename = "finish-args")]
    pub finish_args: Option<HashSet<String>>,
    pub modules: Vec<Module>,
}

impl ManifestYaml {
    pub fn generate() -> crate::Result<Self> {
        let file = ManifestToml::read_file()?;
        Shell::cmd("mkdir icons").exec();
        Shell::cmd(format!(
            "convert {} -resize 128x128 icons/{}-128.png",
            file.bin, file.bin,
        ))
        .exec();

        let desktop_file = format!(
            "\
[Desktop Entry]
Type=Application
Version={}
Name={}
Terminal={}
Icon={}
Exec={}
",
            Cargo::version(),
            file.app_name,
            file.desktop_file.terminal,
            file.app_id,
            file.bin,
        );

        fs::write(format!("{}.desktop", file.app_id.clone()), desktop_file)?;

        let new_file: ManifestYaml = file.clone().into();

        fs::write(
            format!("{}.yaml", file.app_id),
            serde_yaml::to_string(&new_file)?,
        )?;

        let icon_path = format!("{}.png", file.bin);
        if fs::read(&icon_path).is_err() {
            println!("warning! icon not found at path {}", icon_path)
        };

        Shell::cmd(format!(
            "convert {}.png -resize 128x128 icons/{}-128.png",
            file.bin, file.bin,
        ))
        .spawn();

        Ok(new_file)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Module {
    name: String,
    buildsystem: String,
    #[serde(rename = "build-commands")]
    build_commands: Vec<String>,
    sources: Vec<Source>,
}
impl Module {
    fn from_fields(name: impl Into<String>, command: impl Into<String>, path: PathBuf) -> Self {
        Self {
            name: name.into(),
            buildsystem: "simple".to_string(),
            build_commands: vec![command.into()],
            sources: vec![Source {
                r#type: "file".to_string(),
                path: path.to_str().unwrap().to_string(),
            }],
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Source {
    #[serde(rename = "type")]
    r#type: String,
    path: String,
}

impl From<ManifestToml> for ManifestYaml {
    fn from(value: ManifestToml) -> Self {
        let bin = value.bin;
        let profile = value.profile;

        Self {
            id: value.app_id.clone(),
            runtime: "org.freedesktop.Platform".to_string(),
            runtime_version: version(),
            sdk: "org.freedesktop.Sdk".to_string(),
            command: bin.clone(),
            finish_args: value.permissions,
            modules: [
                Module::from_fields(
                    "app",
                    format!("install -D {bin} /app/bin/{bin}"),
                    format!("./target/{profile}/{bin}").into(),
                ),
                Module::from_fields(
                    "icon",
                    format!(
                        "install -D {bin}-128.png /app/share/icons/hicolor/128x128/apps/{}.png",
                        value.app_id.clone()
                    ),
                    format!("./icons/{bin}-128.png").into(),
                ),
                Module::from_fields(
                    "desktop",
                    format!(
                        "install -D {}.desktop /app/share/applications/{}.desktop",
                        value.app_id.clone(),
                        value.app_id.clone()
                    ),
                    format!("{}.desktop", value.app_id).into(),
                ),
            ]
            .into(),
        }
    }
}
