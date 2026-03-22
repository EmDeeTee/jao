use clap::{Parser, error::ErrorKind};
use std::path::PathBuf;

mod actions;
mod errors;
mod help_screen;
mod script_discovery;

#[derive(Debug, Parser)]
#[command(name = "jao")]
#[command(version)]
#[command(about = "Discover and run workspace scripts")]
#[command(arg_required_else_help = true)]
struct Cli {
    /// Print SHA-256 fingerprint of canonical path + file contents.
    #[arg(long, value_name = "FILE")]
    #[arg(conflicts_with_all = ["list", "script_command"])]
    fingerprint: Option<PathBuf>,

    /// List discovered scripts for this OS
    #[arg(long, conflicts_with = "script_command")]
    list: bool,

    /// Script command, e.g. 'deploy api prod'
    #[arg(value_name = "SCRIPT_COMMAND", num_args = 1..)]
    script_command: Vec<String>,
}

fn main() {
    if let Err(err) = run_cli() {
        match err {
            errors::ActionError::Cli(clap_err) => {
                let _ = clap_err.print();
                std::process::exit(clap_err.exit_code());
            }
            other => {
                eprintln!("error: {other}");
                std::process::exit(1);
            }
        }
    }
}

fn run_cli() -> errors::ActionResult<()> {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => match err.kind() {
            ErrorKind::DisplayHelp | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => {
                help_screen::print_help();
                return Ok(());
            }
            ErrorKind::DisplayVersion => {
                let _ = err.print();
                return Ok(());
            }
            _ => return Err(err.into()),
        },
    };

    if let Some(file_path) = cli.fingerprint {
        let hash = actions::fingerprint(file_path)?;
        println!("{hash}");
    }

    if cli.list {
        let cwd = std::env::current_dir()?;
        for script_path in actions::list_scripts(cwd) {
            println!("{}", script_path.display());
        }
    }

    if !cli.script_command.is_empty() {
        let cwd = std::env::current_dir()?;
        actions::run_script(&cli.script_command, cwd)?;
    }

    Ok(())
}
