mod data;
mod dep_check;
mod flatpak;
mod shell;
use crate::data::toml::ManifestToml;
use crate::data::yaml::ManifestYaml;
use crate::dep_check::check;
use crate::flatpak::Flatpak;
use crate::shell::Shell;
use std::env::args;

pub type Result<T> = core::result::Result<T, anyhow::Error>;

fn main() {
    check();
    let mut args: Vec<String> = args().collect();
    args.remove(0);

    println!("args: {:?}", args);

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
    Flatpak(file.app_id).uninstall();

    Ok(())
}

fn build() -> Result<()> {
    let file = ManifestToml::read_file()?;
    Shell::cmd(format!("mold --run cargo b --bin {} -r", file.bin)).spawn();
    Flatpak(file.app_id).build();

    Ok(())
}

fn install() -> Result<()> {
    let file = ManifestToml::read_file()?;
    println!("install");

    Flatpak::install_freedesktop();
    Flatpak(file.app_id).install();
    Ok(())
}
