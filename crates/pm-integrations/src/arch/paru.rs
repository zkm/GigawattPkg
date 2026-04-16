use pm_common::Result;
use pm_core::{PackageManagerBackend, PackageSummary, RunSummary};

use crate::process::{run_command, run_interactive};

pub struct ParuBackend;

impl PackageManagerBackend for ParuBackend {
    fn backend_name(&self) -> &'static str {
        "paru"
    }

    fn search(&self, query: &str) -> Result<Vec<PackageSummary>> {
        let out = run_command("paru", &["-Ss", query], false)?;
        Ok(parse_paru_search(&out.stdout))
    }

    fn list_installed(&self) -> Result<Vec<PackageSummary>> {
        let out = run_command("paru", &["-Q"], false)?;
        Ok(parse_paru_list(&out.stdout))
    }

    fn install(&self, packages: &[String], elevate: bool) -> Result<RunSummary> {
        let mut args = vec!["-S", "--needed"];
        let dynamic: Vec<&str> = packages.iter().map(String::as_str).collect();
        args.extend(dynamic);

        let cmd = run_interactive("paru", &args, elevate)?;
        Ok(RunSummary {
            backend: self.backend_name().to_string(),
            command: cmd,
            stdout: String::new(),
        })
    }

    fn remove(&self, packages: &[String], elevate: bool) -> Result<RunSummary> {
        let mut args = vec!["-R"];
        let dynamic: Vec<&str> = packages.iter().map(String::as_str).collect();
        args.extend(dynamic);

        let cmd = run_interactive("paru", &args, elevate)?;
        Ok(RunSummary {
            backend: self.backend_name().to_string(),
            command: cmd,
            stdout: String::new(),
        })
    }

    fn update(&self, elevate: bool) -> Result<RunSummary> {
        let cmd = run_interactive("paru", &["-Syu"], elevate)?;
        Ok(RunSummary {
            backend: self.backend_name().to_string(),
            command: cmd,
            stdout: String::new(),
        })
    }
}

fn parse_paru_search(stdout: &str) -> Vec<PackageSummary> {
    super::pacman::parse_pacman_search(stdout)
}

fn parse_paru_list(stdout: &str) -> Vec<PackageSummary> {
    super::pacman::parse_pacman_list(stdout)
}
