use std::io::Read;
use std::process::Command;

pub struct Shell(String);

impl Shell {
    pub fn cmd(i: impl Into<String>) -> Self {
        Self(i.into())
    }
    pub fn spawn(self) {
        let input = self.0;
        let mut string: Vec<&str> = input.split_ascii_whitespace().collect();

        if let Ok(mut child) = Command::new(string.remove(0)).args(string).spawn() {
            let _ = child.wait();
        }
    }
    pub fn exec(self) -> Option<String> {
        let mut string: Vec<&str> = self.0.split_ascii_whitespace().collect();
        let cmd = string.remove(0);

        String::from_utf8(
            Command::new(cmd)
                .args(string)
                .output()
                .ok()?
                .stdout
                .bytes()
                .filter_map(|a| a.ok())
                .collect(),
        )
        .ok()
    }
}
