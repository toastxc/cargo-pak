use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DesktopFile {
    pub generic_name: Option<String>,
    pub terminal: bool,
}
