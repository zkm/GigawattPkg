use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("config parse error: {0}")]
    ConfigParse(#[from] toml::de::Error),

    #[error("unsupported distro: {0}")]
    UnsupportedDistro(String),

    #[error("missing system tool: {0}")]
    MissingTool(String),

    #[error("command failed: {command} ({code:?})\n{stderr}")]
    CommandFailed {
        command: String,
        code: Option<i32>,
        stderr: String,
    },

    #[error("parse error: {0}")]
    Parse(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
