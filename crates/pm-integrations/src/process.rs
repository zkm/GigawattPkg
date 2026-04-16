use std::process::{Command, Stdio};

use pm_common::{AppError, Result};

pub struct CmdOutput {
    pub stdout: String,
}

pub fn run_command(bin: &str, args: &[&str], elevate: bool) -> Result<CmdOutput> {
    let command_display = if elevate {
        format!("sudo {} {}", bin, args.join(" "))
    } else {
        format!("{} {}", bin, args.join(" "))
    };

    let output = if elevate {
        let mut sudo_args = vec![bin];
        sudo_args.extend(args);
        Command::new("sudo").args(&sudo_args).output()?
    } else {
        Command::new(bin).args(args).output()?
    };

    if !output.status.success() {
        return Err(AppError::CommandFailed {
            command: command_display,
            code: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    Ok(CmdOutput {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
    })
}

/// Run an interactive command that inherits the terminal's stdin/stdout/stderr.
/// Use this for mutating operations (install, remove, update) where the user
/// may need to respond to prompts and see live output.
pub fn run_interactive(bin: &str, args: &[&str], elevate: bool) -> Result<String> {
    let command_display = if elevate {
        format!("sudo {} {}", bin, args.join(" "))
    } else {
        format!("{} {}", bin, args.join(" "))
    };

    let status = if elevate {
        let mut sudo_args = vec![bin];
        sudo_args.extend(args);
        Command::new("sudo")
            .args(&sudo_args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?
    } else {
        Command::new(bin)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?
    };

    if !status.success() {
        return Err(AppError::CommandFailed {
            command: command_display,
            code: status.code(),
            stderr: String::new(),
        });
    }

    Ok(command_display)
}
