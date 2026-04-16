use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Distro {
    Arch,
    Fedora,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendKind {
    Pacman,
    Dnf,
}

#[derive(Debug, Clone, Serialize)]
pub struct PackageSummary {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub installed: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunSummary {
    pub backend: String,
    pub command: String,
    pub stdout: String,
}
