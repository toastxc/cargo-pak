mod data;
mod dep_check;
mod flatpak;
mod shell;
use crate::data::toml::ManifestToml;
use crate::data::yaml::ManifestYaml;
use crate::dep_check::check;
use crate::flatpak::Flatpak;
use crate::shell::Shell;
use clap::{Arg, Command};
use std::env::args;

pub type Result<T> = core::result::Result<T, anyhow::Error>;

fn main() {
    check();
    //let mut args: Vec<String> = args().collect();
    //args.remove(0);

    let cmd = Command::new("cargo-pak")
        .version(clap::crate_version!())
        .subcommand(Command::new("generate").alias("gen"))
        .subcommand(Command::new("build"))
        .subcommand(Command::new("install"))
        .subcommand(Command::new("remove"))
        .subcommand(Command::new("go"));

    let matcher = cmd.clone().get_matches();

    let matcher = match matcher.subcommand() {
        None => {
            println!("`cargo-pak help` for help");
            return;
        }
        Some((matcher, _)) => matcher,
    };

    println!("Cargo_pack starting...");
    if let Err(error) = {
        match matcher {
            "generate" => ManifestYaml::generate().map(|_| ()),
            "build" => build().map(|_| ()),
            "install" => install(),
            "remove" => remove(),
            "go" => go(),
            _ => Ok(println!("Invalid command")),
        }
    } {
        println!("{:?}", error);
    } else {
        println!("Done!");
    }
}

pub fn go() -> Result<()> {
    remove()?;
    ManifestYaml::generate()?;
    build()?;
    install()?;
    Ok(())
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

    Flatpak::install_runtime(&file.runtime.unwrap_or("freedesktop".to_string()));
    Flatpak(file.app_id).install();
    Ok(())
}
