use pm_common::{AppError, Result};
use pm_core::Distro;

pub fn detect_distro() -> Result<Distro> {
    let os_release = std::fs::read_to_string("/etc/os-release")?;

    for line in os_release.lines() {
        if let Some(value) = line.strip_prefix("ID=") {
            let id = value.trim_matches('"').to_lowercase();
            return match id.as_str() {
                "arch" => Ok(Distro::Arch),
                "fedora" => Ok(Distro::Fedora),
                other => Err(AppError::UnsupportedDistro(other.to_string())),
            };
        }
    }

    Err(AppError::Parse(
        "unable to read distro identifier from /etc/os-release".to_string(),
    ))
}

pub fn executable_exists(name: &str) -> bool {
    which::which(name).is_ok()
}
