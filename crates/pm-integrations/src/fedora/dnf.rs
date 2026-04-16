use pm_common::Result;
use pm_core::{PackageManagerBackend, PackageSummary, RunSummary};

use crate::process::{run_command, run_interactive};

pub struct DnfBackend;

impl PackageManagerBackend for DnfBackend {
    fn backend_name(&self) -> &'static str {
        "dnf"
    }

    fn search(&self, query: &str) -> Result<Vec<PackageSummary>> {
        let out = run_command("dnf", &["search", query], false)?;
        Ok(parse_dnf_search(&out.stdout))
    }

    fn list_installed(&self) -> Result<Vec<PackageSummary>> {
        let out = run_command("dnf", &["list", "installed"], false)?;
        Ok(parse_dnf_installed(&out.stdout))
    }

    fn install(&self, packages: &[String], elevate: bool) -> Result<RunSummary> {
        let mut args = vec!["install", "-y"];
        let dynamic: Vec<&str> = packages.iter().map(String::as_str).collect();
        args.extend(dynamic);

        let cmd = run_interactive("dnf", &args, elevate)?;
        Ok(RunSummary {
            backend: self.backend_name().to_string(),
            command: cmd,
            stdout: String::new(),
        })
    }

    fn remove(&self, packages: &[String], elevate: bool) -> Result<RunSummary> {
        let mut args = vec!["remove", "-y"];
        let dynamic: Vec<&str> = packages.iter().map(String::as_str).collect();
        args.extend(dynamic);

        let cmd = run_interactive("dnf", &args, elevate)?;
        Ok(RunSummary {
            backend: self.backend_name().to_string(),
            command: cmd,
            stdout: String::new(),
        })
    }

    fn update(&self, elevate: bool) -> Result<RunSummary> {
        let cmd = run_interactive("dnf", &["upgrade", "-y"], elevate)?;
        Ok(RunSummary {
            backend: self.backend_name().to_string(),
            command: cmd,
            stdout: String::new(),
        })
    }
}

fn parse_dnf_search(stdout: &str) -> Vec<PackageSummary> {
    stdout
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('=') {
                return None;
            }

            let (left, desc) = trimmed.split_once(" : ")?;
            let pkg = left.split_whitespace().next()?;
            let name = pkg
                .split_once('.')
                .map(|(n, _)| n.to_string())
                .unwrap_or_else(|| pkg.to_string());

            Some(PackageSummary {
                name,
                version: None,
                description: Some(desc.to_string()),
                installed: false,
            })
        })
        .collect()
}

fn parse_dnf_installed(stdout: &str) -> Vec<PackageSummary> {
    stdout
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("Installed") {
                return None;
            }

            let mut parts = trimmed.split_whitespace();
            let pkg = parts.next()?;
            let version = parts.next().map(ToString::to_string);
            let name = pkg
                .split_once('.')
                .map(|(n, _)| n.to_string())
                .unwrap_or_else(|| pkg.to_string());

            Some(PackageSummary {
                name,
                version,
                description: None,
                installed: true,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{parse_dnf_installed, parse_dnf_search};

    #[test]
    fn parses_dnf_search_lines() {
        let output = "=============== Name Matched: rg ===============\nripgrep.x86_64 : Fast line-oriented search tool\n";
        let parsed = parse_dnf_search(output);

        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].name, "ripgrep");
        assert_eq!(
            parsed[0].description.as_deref(),
            Some("Fast line-oriented search tool")
        );
    }

    #[test]
    fn parses_dnf_installed_lines() {
        let output = "Installed Packages\nripgrep.x86_64 14.1.1-1.fc40 @updates\n";
        let parsed = parse_dnf_installed(output);

        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].name, "ripgrep");
        assert_eq!(parsed[0].version.as_deref(), Some("14.1.1-1.fc40"));
        assert!(parsed[0].installed);
    }
}
