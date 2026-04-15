use pm_common::Result;

use crate::{PackageSummary, RunSummary};

pub trait PackageManagerBackend {
    fn backend_name(&self) -> &'static str;

    fn search(&self, query: &str) -> Result<Vec<PackageSummary>>;

    fn list_installed(&self) -> Result<Vec<PackageSummary>>;

    fn install(&self, packages: &[String], elevate: bool) -> Result<RunSummary>;

    fn remove(&self, packages: &[String], elevate: bool) -> Result<RunSummary>;

    fn update(&self, elevate: bool) -> Result<RunSummary>;
}
