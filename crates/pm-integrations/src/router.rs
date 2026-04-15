use pm_common::{AppError, Result};
use pm_core::{BackendKind, Distro, PackageManagerBackend};

use crate::arch::{pacman::PacmanBackend, paru::ParuBackend};
use crate::detect::executable_exists;
use crate::fedora::dnf::DnfBackend;

#[derive(Debug, Clone, Copy)]
pub struct BackendOptions {
    pub prefer_paru: bool,
    pub explicit_backend: Option<BackendKind>,
}

pub fn resolve_backend(
    distro: Distro,
    options: BackendOptions,
) -> Result<Box<dyn PackageManagerBackend>> {
    if let Some(kind) = options.explicit_backend {
        return resolve_explicit(kind);
    }

    match distro {
        Distro::Arch => {
            if options.prefer_paru && executable_exists("paru") {
                Ok(Box::new(ParuBackend))
            } else if executable_exists("pacman") {
                Ok(Box::new(PacmanBackend))
            } else {
                Err(AppError::MissingTool("pacman".to_string()))
            }
        }
        Distro::Fedora => {
            if executable_exists("dnf") {
                Ok(Box::new(DnfBackend))
            } else {
                Err(AppError::MissingTool("dnf".to_string()))
            }
        }
    }
}

fn resolve_explicit(kind: BackendKind) -> Result<Box<dyn PackageManagerBackend>> {
    match kind {
        BackendKind::Pacman => {
            if executable_exists("pacman") {
                Ok(Box::new(PacmanBackend))
            } else {
                Err(AppError::MissingTool("pacman".to_string()))
            }
        }
        BackendKind::Paru => {
            if executable_exists("paru") {
                Ok(Box::new(ParuBackend))
            } else {
                Err(AppError::MissingTool("paru".to_string()))
            }
        }
        BackendKind::Dnf => {
            if executable_exists("dnf") {
                Ok(Box::new(DnfBackend))
            } else {
                Err(AppError::MissingTool("dnf".to_string()))
            }
        }
    }
}
