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
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Display project information.
    About,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::About) => {
            println!("Zashikura {}", env!("CARGO_PKG_VERSION"));
            println!("A local-first tool for understanding and looking after your computer.");
        }
        None => unreachable!("Clap requires a command or displays help"),
    }
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::Parser;

    #[test]
    fn help_flag_is_accepted() {
        let result = Cli::try_parse_from(["zashikura", "--help"]);
        assert!(result.is_err());
    }

    #[test]
    fn about_command_is_accepted() {
        let result = Cli::try_parse_from(["zashikura", "about"]);
        assert!(result.is_ok());
    }
}
