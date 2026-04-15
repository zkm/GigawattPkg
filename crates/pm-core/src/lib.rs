mod backend;
mod model;

pub use backend::PackageManagerBackend;
pub use model::{BackendKind, Distro, PackageSummary, RunSummary};
