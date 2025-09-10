use crate::data::cargo::Cargo;
use crate::data::toml::ManifestToml;
use crate::shell::Shell;
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
            "magick {} -resize 128x128 icons/{}-128.png",
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
            "magick {}.png -resize 128x128 icons/{}-128.png",
            file.bin, file.bin,
        ))
        .spawn();

        Ok(new_file)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Module {
    name: String,
    buildsystem: String,
    #[serde(rename = "build-commands")]
    build_commands: Option<Vec<String>>,
    builddir: Option<bool>,
    #[serde(rename = "config-opts")]
    config_opts: Option<Vec<String>>,
    sources: Option<Vec<Source>>,
    cleanup: Option<Vec<String>>,
    #[serde(rename = "no-autogen")]
    no_autogen: Option<bool>,
    #[serde(rename = "make-install-args")]
    make_install_args: Option<Vec<String>>,
}
impl Module {
    fn from_fields(name: impl Into<String>, command: impl Into<String>, path: PathBuf) -> Self {
        Self {
            name: name.into(),
            buildsystem: "simple".to_string(),
            build_commands: Some(vec![command.into()]),
            sources: Some(vec![Source {
                r#type: "file".to_string(),
                path: Some(path.to_str().unwrap().to_string()),
                url: None,
                tag: None,
                paths: None,
                commands: None,
                commit: None,
                dest: None,
                dest_filename: None,
                filename: None,
                mirror_urls: None,
                only_arches: None,
                options: None,
                sha256: None,
                size: None,
                strip_components: None,
                //  derive,
            }]),
            config_opts: None,
            builddir: None,
            cleanup: None,
            make_install_args: None,
            no_autogen: None,
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
struct Source {
    #[serde(rename = "type")]
    r#type: String,
    path: Option<String>,
    #[serde(rename = "only-arches")]
    only_arches: Option<Vec<String>>,
    // Patch
    paths: Option<Vec<String>>,
    options: Option<Vec<String>>,
    // Download/Archive
    url: Option<String>,
    filename: Option<String>,
    sha256: Option<String>,
    #[serde(rename = "dest-filename")]
    dest_filename: Option<String>,
    #[serde(rename = "mirror-urls")]
    mirror_urls: Option<Vec<String>>,
    #[serde(rename = "strip-components")]
    strip_components: Option<i32>,
    size: Option<i32>,
    // Git
    commit: Option<String>,
    tag: Option<String>,
    dest: Option<String>,
    // Shell
    commands: Option<Vec<String>>,
}

impl From<ManifestToml> for ManifestYaml {
    fn from(value: ManifestToml) -> Self {
        let bin = value.bin;
        let profile = value.profile;
        let mut modules: Vec<Module> = vec![];

        for module in value.modules {
            modules.push(module);
        };

        modules.push(Module::from_fields(
            "app",
            format!("install -D {bin} /app/bin/{bin}"),
            format!("./target/{profile}/{bin}").into(),
        ));
        modules.push(Module::from_fields(
            "icon",
            format!(
                "install -D {bin}-128.png /app/share/icons/hicolor/128x128/apps/{}.png",
                value.app_id.clone()
            ),
            format!("./icons/{bin}-128.png").into(),
        ));
        modules.push(Module::from_fields(
            "desktop",
            format!(
                "install -D {}.desktop /app/share/applications/{}.desktop",
                value.app_id.clone(),
                value.app_id.clone()
            ),
            format!("{}.desktop", value.app_id).into(),
        ));

        Self {
            id: value.app_id.clone(),
            runtime: format!("org.{}.Platform", &value.runtime.clone()),
            runtime_version: value.runtime_version.unwrap(),
            sdk: format!("org.{}.Sdk", &value.runtime.clone()),
            command: bin.clone(),
            finish_args: value.permissions,
            modules,
        }
    }
}
