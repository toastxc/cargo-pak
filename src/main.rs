mod data;
mod dep_check;
mod shell;

use crate::data::toml::ManifestToml;
use crate::data::yaml::ManifestYaml;
use crate::dep_check::check;
use crate::shell::Shell;
use std::env::args;

pub type Result<T> = core::result::Result<T, anyhow::Error>;

fn main() {
    check();
    let mut args: Vec<String> = args().collect();
    args.remove(0);

    let Some(arg) = args.first() else { return };

    println!("Cargo_pack starting...");
    if let Err(error) = {
        match arg.to_lowercase().as_str() {
            "generate" => ManifestYaml::generate().map(|_| ()),
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

fn remove() -> Result<()> {
    let file = ManifestToml::read_file()?;
    println!("remove");
    Shell::cmd(format!("sudo flatpak uninstall {} -y", file.app_id)).spawn();

    Ok(())
}

fn build() -> Result<()> {
    let file = ManifestToml::read_file()?;

    Shell::cmd("mold --run cargo b -r").spawn();
    Shell::cmd(format!(
        "sudo flatpak-builder  --user build-dir {}.yaml  --force-clean",
        file.app_id
    ))
    .spawn();
    Ok(())
}

fn install() -> Result<()> {
    let file = ManifestToml::read_file()?;

    println!("install");
    remove()?;

    Shell::cmd(format!(
        "sudo flatpak-builder --install --force-clean build-dir {}.yaml",
        file.app_id
    ))
    .spawn();
    Ok(())
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
