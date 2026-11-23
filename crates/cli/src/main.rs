use std::process::ExitCode;

use clap::{Parser, Subcommand};
use serde::Serialize;
use zashikura_application::InspectSystem;
use zashikura_core::SystemSnapshot;
use zashikura_platform::SysinfoSystemInspector;

#[derive(Debug, Parser)]
#[command(
    name = "zashikura",
    version,
    about = "Understand and look after your computer.",
    long_about = "Zashikura is a local-first tool for understanding and maintaining your computer.",
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Display project information.
    About,
    /// Inspect information about this computer.
    Inspect {
        #[command(subcommand)]
        command: InspectCommand,
    },
}

#[derive(Debug, Subcommand)]
enum InspectCommand {
    /// Display operating-system, processor, and memory information.
    System {
        /// Print the result as pretty-printed JSON.
        #[arg(long)]
        json: bool,
    },
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Command::About => {
            println!("Zashikura {}", env!("CARGO_PKG_VERSION"));
            println!("A local-first tool for understanding and looking after your computer.");
        }
        Command::Inspect {
            command: InspectCommand::System { json },
        } => {
            let snapshot = InspectSystem::new(SysinfoSystemInspector).execute()?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&SystemOutput::from(snapshot))?
                );
            } else {
                print_system_snapshot(&snapshot);
            }
        }
    }
    Ok(())
}

#[derive(Serialize)]
struct SystemOutput {
    hostname: Option<String>,
    os_name: Option<String>,
    os_version: Option<String>,
    kernel_version: Option<String>,
    logical_cpu_count: usize,
    memory: MemoryOutput,
}

#[derive(Serialize)]
struct MemoryOutput {
    total_bytes: u64,
    used_bytes: u64,
}

impl From<SystemSnapshot> for SystemOutput {
    fn from(snapshot: SystemSnapshot) -> Self {
        Self {
            hostname: snapshot.hostname,
            os_name: snapshot.os_name,
            os_version: snapshot.os_version,
            kernel_version: snapshot.kernel_version,
            logical_cpu_count: snapshot.logical_cpu_count,
            memory: MemoryOutput {
                total_bytes: snapshot.memory.total_bytes,
                used_bytes: snapshot.memory.used_bytes,
            },
        }
    }
}

fn print_system_snapshot(snapshot: &SystemSnapshot) {
    println!("System");
    println!("  Hostname: {}", display_optional(&snapshot.hostname));
    println!(
        "  Operating system: {}",
        display_optional(&snapshot.os_name)
    );
    println!("  OS version: {}", display_optional(&snapshot.os_version));
    println!(
        "  Kernel version: {}",
        display_optional(&snapshot.kernel_version)
    );
    println!("  Logical processors: {}", snapshot.logical_cpu_count);
    println!(
        "  Memory: {} / {}",
        format_bytes(snapshot.memory.used_bytes),
        format_bytes(snapshot.memory.total_bytes)
    );
}

fn display_optional(value: &Option<String>) -> &str {
    value.as_deref().unwrap_or("Unavailable")
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{bytes} {}", UNITS[unit])
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

#[cfg(test)]
mod tests {
    use super::{Cli, Command, InspectCommand, format_bytes};
    use clap::{Parser, error::ErrorKind};

    #[test]
    fn help_flag_displays_help() {
        let error = Cli::try_parse_from(["zashikura", "--help"])
            .expect_err("the help flag should exit parsing");
        assert_eq!(error.kind(), ErrorKind::DisplayHelp);
    }

    #[test]
    fn missing_command_displays_help() {
        let error = Cli::try_parse_from(["zashikura"]).expect_err("a command is required");
        assert_eq!(error.kind(), ErrorKind::DisplayHelp);
    }

    #[test]
    fn about_command_is_accepted() {
        assert!(matches!(
            Cli::try_parse_from(["zashikura", "about"]).unwrap().command,
            Command::About
        ));
    }

    #[test]
    fn system_inspection_supports_json_output() {
        let cli = Cli::try_parse_from(["zashikura", "inspect", "system", "--json"]).unwrap();
        assert!(matches!(
            cli.command,
            Command::Inspect {
                command: InspectCommand::System { json: true }
            }
        ));
    }

    #[test]
    fn byte_formatting_uses_binary_units() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1_536), "1.5 KiB");
    }
}
