use serde::{Deserialize, Serialize};

pub const NAME: &str = "gdbr";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    #[serde(skip)]
    pub logs: String,
    #[serde(skip)]
    pub console_input: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            logs: "Logs...\nMore logs...\nSome other log...\nNew log...\nAnother log".into(),
            console_input: String::new(),
        }
    }
}
