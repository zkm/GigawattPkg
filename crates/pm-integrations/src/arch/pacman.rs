use pm_common::Result;
use pm_core::{PackageManagerBackend, PackageSummary, RunSummary};

use crate::process::run_command;

pub struct PacmanBackend;

impl PackageManagerBackend for PacmanBackend {
    fn backend_name(&self) -> &'static str {
        "pacman"
    }

    fn search(&self, query: &str) -> Result<Vec<PackageSummary>> {
        let out = run_command("pacman", &["-Ss", query], false)?;
        Ok(parse_pacman_search(&out.stdout))
    }

    fn list_installed(&self) -> Result<Vec<PackageSummary>> {
        let out = run_command("pacman", &["-Q"], false)?;
        Ok(parse_pacman_list(&out.stdout))
    }

    fn install(&self, packages: &[String], elevate: bool) -> Result<RunSummary> {
        let mut args = vec!["-S", "--needed"];
        let dynamic: Vec<&str> = packages.iter().map(String::as_str).collect();
        args.extend(dynamic);

        let out = run_command("pacman", &args, elevate)?;
        Ok(RunSummary {
            backend: self.backend_name().to_string(),
            command: out.command_display,
            stdout: out.stdout,
        })
    }

    fn remove(&self, packages: &[String], elevate: bool) -> Result<RunSummary> {
        let mut args = vec!["-R"];
        let dynamic: Vec<&str> = packages.iter().map(String::as_str).collect();
        args.extend(dynamic);

        let out = run_command("pacman", &args, elevate)?;
        Ok(RunSummary {
            backend: self.backend_name().to_string(),
            command: out.command_display,
            stdout: out.stdout,
        })
    }

    fn update(&self, elevate: bool) -> Result<RunSummary> {
        let out = run_command("pacman", &["-Syu"], elevate)?;
        Ok(RunSummary {
            backend: self.backend_name().to_string(),
            command: out.command_display,
            stdout: out.stdout,
        })
    }
}

pub(crate) fn parse_pacman_search(stdout: &str) -> Vec<PackageSummary> {
    let mut results = Vec::new();
    let mut pending: Option<(String, Option<String>)> = None;

    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let is_package_header = trimmed.contains('/')
            && !line.starts_with(' ')
            && !line.starts_with('\t')
            && trimmed.split_whitespace().count() >= 2;

        if is_package_header {
            if let Some((name, version)) = pending.take() {
                results.push(PackageSummary {
                    name,
                    version,
                    description: None,
                    installed: false,
                });
            }

            let mut parts = trimmed.split_whitespace();
            let repo_and_name = parts.next().unwrap_or_default();
            let version = parts.next().map(ToString::to_string);
            let name = repo_and_name
                .split_once('/')
                .map(|(_, pkg)| pkg.to_string())
                .unwrap_or_else(|| repo_and_name.to_string());
            pending = Some((name, version));
            continue;
        }

        if let Some((name, version)) = pending.take() {
            results.push(PackageSummary {
                name,
                version,
                description: Some(trimmed.to_string()),
                installed: false,
            });
        }
    }

    if let Some((name, version)) = pending.take() {
        results.push(PackageSummary {
            name,
            version,
            description: None,
            installed: false,
        });
    }

    results
}

pub(crate) fn parse_pacman_list(stdout: &str) -> Vec<PackageSummary> {
    stdout
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let name = parts.next()?;
            let version = parts.next().map(ToString::to_string);

            Some(PackageSummary {
                name: name.to_string(),
                version,
                description: None,
                installed: true,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{parse_pacman_list, parse_pacman_search};

    #[test]
    fn parses_search_output() {
        let output = "extra/ripgrep 14.1.1-1\n    Search tool\ncore/bash 5.2-3\n    GNU shell\n";
        let parsed = parse_pacman_search(output);

        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].name, "ripgrep");
        assert_eq!(parsed[0].version.as_deref(), Some("14.1.1-1"));
        assert_eq!(parsed[0].description.as_deref(), Some("Search tool"));
    }

    #[test]
    fn parses_installed_output() {
        let output = "ripgrep 14.1.1-1\nbash 5.2-3\n";
        let parsed = parse_pacman_list(output);

        assert_eq!(parsed.len(), 2);
        assert!(parsed[1].installed);
        assert_eq!(parsed[1].name, "bash");
    }
}
