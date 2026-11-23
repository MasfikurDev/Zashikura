use clap::{Parser, Subcommand};

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
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::About => {
            println!("Zashikura {}", env!("CARGO_PKG_VERSION"));
            println!("A local-first tool for understanding and looking after your computer.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::{error::ErrorKind, Parser};

    #[test]
    fn help_flag_displays_help() {
        let error = Cli::try_parse_from(["zashikura", "--help"])
            .expect_err("the help flag should exit parsing");
        assert_eq!(error.kind(), ErrorKind::DisplayHelp);
    }

    #[test]
    fn missing_command_displays_help() {
        let error = Cli::try_parse_from(["zashikura"])
            .expect_err("a command is required");
        assert_eq!(error.kind(), ErrorKind::DisplayHelp);
    }

    #[test]
    fn about_command_is_accepted() {
        assert!(Cli::try_parse_from(["zashikura", "about"]).is_ok());
    }
}
