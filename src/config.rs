use serde::{Deserialize, Serialize};

pub const NAME: &str = "gdbr";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct AppConfig {
    pub zoom: f32,
    #[serde(skip)]
    pub zoom_temp: f32,
    #[serde(skip)]
    pub logs: String,
    #[serde(skip)]
    pub console_input: String,
    #[serde(skip)]
    pub layout_initialized: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            zoom_temp: 1.0,
            logs: "Logs...\nMore logs...\nSome other log...\nNew log...\nAnother log".into(),
            console_input: String::new(),
            layout_initialized: false,
        }
    }
}
