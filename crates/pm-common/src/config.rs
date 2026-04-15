use serde::{Deserialize, Serialize};

use crate::{Result, Theme};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub prefer_paru: bool,
    pub color: bool,
    pub theme: Theme,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            prefer_paru: false,
            color: true,
            theme: Theme::default(),
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let path = config_path();
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(path)?;
        let parsed: Self = toml::from_str(&content)?;
        Ok(parsed)
    }
}

fn config_path() -> std::path::PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        return std::path::PathBuf::from(home)
            .join(".config")
            .join("gigawattpkg")
            .join("config.toml");
    }

    std::path::PathBuf::from(".gigawattpkg.toml")
}
