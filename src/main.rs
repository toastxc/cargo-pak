mod shell;

use crate::shell::Shell;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, env::args, path::PathBuf, str::FromStr};

pub type Result<T> = core::result::Result<T, anyhow::Error>;

fn main() {
    let mut args: Vec<String> = args().collect();
    args.remove(0);

    let Some(arg) = args.first() else { return };

    println!("Cargo_pack starting...");
    if let Err(error) = {
        match arg.to_lowercase().as_str() {
            "generate" => generate().map(|_| ()),
            "build" => build().map(|_| ()),
            "install" => install(),
            "remove" => remove(),
            _ => Ok(println!("Invalid command")),
        }
    } {
        println!("{:?}", error);
    } else {
        println!("Done!");
    }
}

fn read_file() -> Result<ManifestToml> {
    let file: ManifestToml = toml::from_str::<ManifestTomlInput>(&String::from_utf8(
        std::fs::read(PathBuf::from_str("pak.toml")?)?,
    )?)?
    .into();
    Ok(file)
}

fn remove() -> Result<()> {
    let file = read_file()?;
    println!("remove");
    // shell_attach(format!("sudo flatpak uninstall {} -y", file.app_id));
    Shell::cmd(format!("sudo flatpak uninstall {} -y", file.app_id)).spawn();

    Ok(())
}

fn generate() -> Result<ManifestYaml> {
    let file = read_file()?;

    // shell("mkdir icons");
    // shell(format!(
    //     "convert {} -resize 128x128 icons/{}-128.png",
    //     file.bin, file.bin,
    // ));

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
        cargo_version().unwrap().package.version,
        file.app_name,
        file.desktopfile.terminal,
        file.app_id,
        file.bin,
    );

    std::fs::write(format!("{}.desktop", file.app_id.clone()), desktop_file)?;

    let new_file: ManifestYaml = file.clone().into();

    std::fs::write(
        format!("{}.yaml", file.app_id),
        serde_yaml::to_string(&new_file)?,
    )?;
    // shell("mkdir icons");
    Shell::cmd(format!(
        "convert {}.png -resize 128x128 icons/{}-128.png",
        file.bin, file.bin,
    ))
    .exec();

    Ok(new_file)
}

fn build() -> Result<()> {
    let file = read_file()?;

    Shell::cmd("mold --run cargo b -r").spawn();
    Shell::cmd(format!(
        "sudo flatpak-builder  --user build-dir {}.yaml  --force-clean",
        file.app_id
    ))
    .spawn();
    Ok(())
}

fn install() -> Result<()> {
    let file = read_file()?;

    println!("install");
    remove()?;

    Shell::cmd(format!(
        "sudo flatpak-builder --install --force-clean build-dir {}.yaml",
        file.app_id
    ))
    .spawn();
    Ok(())
}

impl From<ManifestTomlInput> for ManifestToml {
    fn from(value: ManifestTomlInput) -> Self {
        Self {
            app_id: value.app_id,
            app_name: value.app_name,
            bin: value.bin.unwrap_or(cargo_version().unwrap().package.name),
            profile: value.profile,
            permissions: value.permissions,
            desktopfile: value.desktopfile,
        }
    }
}

fn cargo_version() -> Result<Toml> {
    let mut cargo = toml::from_str::<Toml>(&String::from_utf8(std::fs::read("Cargo.toml")?)?)?;

    cargo.package.version.remove(0);
    cargo.package.version.remove(0);
    Ok(cargo)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Toml {
    package: Package,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Package {
    version: String,
    name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ManifestTomlInput {
    app_id: String,
    app_name: String,
    bin: Option<String>,
    profile: String,
    permissions: Option<HashSet<String>>,
    desktopfile: DesktopFile,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ManifestToml {
    app_id: String,
    app_name: String,
    bin: String,
    profile: String,
    permissions: Option<HashSet<String>>,
    desktopfile: DesktopFile,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct DesktopFile {
    generic_name: Option<String>,
    terminal: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ManifestYaml {
    id: String,
    runtime: String,
    #[serde(rename = "runtime-version")]
    runtime_version: String,
    sdk: String,
    command: String,
    #[serde(rename = "finish-args")]
    finish_args: Option<HashSet<String>>,
    modules: Vec<Module>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Module {
    name: String,
    buildsystem: String,
    #[serde(rename = "build-commands")]
    build_commands: Vec<String>,
    sources: Vec<Source>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Source {
    #[serde(rename = "type")]
    r#type: String,
    path: String,
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

// find the open desktop's highest available version
fn version() -> String {
    let a = Shell::cmd("flatpak install org.freedesktop.Sdk")
        .exec()
        .unwrap();
    let mut a: Vec<String> = a.split('\n').map(String::from).collect();
    for _ in 0..3 {
        a.remove(0);
    }
    a.into_iter()
        .filter_map(|a| {
            let temp: Vec<&str> = a.split('/').collect();
            temp.last().unwrap().to_string().parse::<f32>().ok()
        })
        .reduce(f32::max)
        .unwrap()
        .to_string()
}
