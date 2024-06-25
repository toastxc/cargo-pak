use serde::{Deserialize, Serialize};
use std::fs;

pub struct Cargo();

impl Cargo {
    pub fn values() -> crate::Result<CargoToml> {
        let mut cargo = toml::from_str::<CargoToml>(&String::from_utf8(fs::read("Cargo.toml")?)?)?;

        cargo.package.version.remove(0);
        cargo.package.version.remove(0);
        Ok(cargo)
    }

    pub fn name() -> String {
        Self::values().unwrap().package.name
    }
    pub fn version() -> String {
        Self::values().unwrap().package.version
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CargoToml {
    package: Package,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Package {
    version: String,
    name: String,
}
