use std::process::Command;

use pm_common::{AppError, Result};

pub struct CmdOutput {
    pub command_display: String,
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
        command_display,
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
    })
}
