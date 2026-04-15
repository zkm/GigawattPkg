use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Theme {
    pub primary: String,
    pub accent: String,
    pub warning: String,
    pub success: String,
    pub error: String,
    pub icon_search: String,
    pub icon_install: String,
    pub icon_remove: String,
    pub icon_update: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: "cyan".to_string(),
            accent: "magenta".to_string(),
            warning: "yellow".to_string(),
            success: "green".to_string(),
            error: "red".to_string(),
            icon_search: "".to_string(),
            icon_install: "".to_string(),
            icon_remove: "".to_string(),
            icon_update: "".to_string(),
        }
    }
}
