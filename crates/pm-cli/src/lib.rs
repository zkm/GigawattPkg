use anyhow::Context;
use clap::{Parser, Subcommand, ValueEnum};
use owo_colors::OwoColorize;
use pm_common::AppConfig;
use pm_core::{BackendKind, PackageSummary, RunSummary};
use pm_integrations::{detect_distro, resolve_backend, BackendOptions};

#[derive(Debug, Parser)]
#[command(name = "gigawattpkg", version, about = "Fast colorful package manager wrapper for Arch and Fedora")]
struct Cli {
    #[arg(long, value_enum)]
    backend: Option<BackendArg>,

    #[arg(long)]
    use_paru: bool,

    #[arg(long)]
    no_color: bool,

    #[arg(long)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum BackendArg {
    Pacman,
    Paru,
    Dnf,
}

impl From<BackendArg> for BackendKind {
    fn from(value: BackendArg) -> Self {
        match value {
            BackendArg::Pacman => BackendKind::Pacman,
            BackendArg::Paru => BackendKind::Paru,
            BackendArg::Dnf => BackendKind::Dnf,
        }
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    Search { query: String },
    List,
    Install { packages: Vec<String> },
    Remove { packages: Vec<String> },
    Update,
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = AppConfig::load().context("failed to load config")?;
    let distro = detect_distro().context("failed to detect distro")?;

    let backend = resolve_backend(
        distro,
        BackendOptions {
            prefer_paru: cli.use_paru || config.prefer_paru,
            explicit_backend: cli.backend.map(Into::into),
        },
    )
    .context("failed to resolve package backend")?;

    match cli.command {
        Commands::Search { query } => {
            let results = backend.search(&query).context("search failed")?;
            render_packages(
                &results,
                cli.json,
                config.color && !cli.no_color,
                &config.theme.icon_search,
            )?;
        }
        Commands::List => {
            let results = backend.list_installed().context("list failed")?;
            render_packages(&results, cli.json, config.color && !cli.no_color, "")?;
        }
        Commands::Install { packages } => {
            if packages.is_empty() {
                anyhow::bail!("install requires at least one package");
            }
            let result = backend.install(&packages, true).context("install failed")?;
            render_run_summary(
                &result,
                cli.json,
                config.color && !cli.no_color,
                &config.theme.icon_install,
            )?;
        }
        Commands::Remove { packages } => {
            if packages.is_empty() {
                anyhow::bail!("remove requires at least one package");
            }
            let result = backend.remove(&packages, true).context("remove failed")?;
            render_run_summary(
                &result,
                cli.json,
                config.color && !cli.no_color,
                &config.theme.icon_remove,
            )?;
        }
        Commands::Update => {
            let result = backend.update(true).context("update failed")?;
            render_run_summary(
                &result,
                cli.json,
                config.color && !cli.no_color,
                &config.theme.icon_update,
            )?;
        }
    }

    Ok(())
}

fn render_packages(
    packages: &[PackageSummary],
    as_json: bool,
    color: bool,
    icon: &str,
) -> anyhow::Result<()> {
    if as_json {
        println!("{}", serde_json::to_string_pretty(packages)?);
        return Ok(());
    }

    for pkg in packages {
        let name = pkg.name.as_str();
        let version = pkg.version.as_deref().unwrap_or("unknown");
        let desc = pkg.description.as_deref().unwrap_or("");

        if color {
            println!(
                "{} {} {} {}",
                icon.cyan(),
                name.bold().magenta(),
                version.yellow(),
                desc.dimmed()
            );
        } else {
            println!("{} {} {} {}", icon, name, version, desc);
        }
    }

    Ok(())
}

fn render_run_summary(
    summary: &RunSummary,
    as_json: bool,
    color: bool,
    icon: &str,
) -> anyhow::Result<()> {
    if as_json {
        println!("{}", serde_json::to_string_pretty(summary)?);
        return Ok(());
    }

    if color {
        println!(
            "{} backend={} command={}",
            icon.green(),
            summary.backend.bold().cyan(),
            summary.command.bright_blue()
        );
        println!("{}", summary.stdout);
    } else {
        println!("{} backend={} command={}", icon, summary.backend, summary.command);
        println!("{}", summary.stdout);
    }

    Ok(())
}
