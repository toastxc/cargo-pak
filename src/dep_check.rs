use crate::shell::Shell;

const DEPS: [&str; 4] = ["magick", "flatpak-builder", "cargo", "mold"];

pub fn check() {
    for x in DEPS {
        if Shell::cmd(x).exec().is_none() {
            panic!("Unmet dependency! {}", x);
        }
    }
}
