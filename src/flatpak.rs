use crate::Shell;
pub struct Flatpak(pub String);
impl Flatpak {
    pub fn runtime_version(runtime: &str) -> String {
        let a = Shell::cmd(format!("flatpak install org.{runtime}.Sdk"))
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
            .expect("could not get runtime version")
            .to_string()
    }

    pub fn install_runtime(runtime: &str) {
        let version = Self::runtime_version(runtime);

        println!("VERSION: {version}");
        Shell::cmd(format!(
            "flatpak install runtime/org.{runtime}.Platform/x86_64/{version} -y"
        ))
        .spawn();
        Shell::cmd(format!(
            "flatpak install runtime/org.{runtime}.Sdk/x86_64/{version} -y"
        ))
        .spawn();
    }

    pub fn install(&self) {
        Shell::cmd(format!(
            "flatpak-builder --user --install --force-clean build-dir {}.yaml",
            self.0
        ))
        .spawn();
    }

    pub fn build(&self) {
        Shell::cmd(format!(
            "flatpak-builder  --user build-dir {}.yaml  --force-clean",
            self.0
        ))
        .spawn();
    }

    pub fn uninstall(&self) {
        Shell::cmd(format!("flatpak --user uninstall {} -y", self.0)).spawn();
    }
}
