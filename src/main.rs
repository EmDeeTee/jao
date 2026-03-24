//! `jao` is a small CLI for discovering and running workspace scripts.
//!
//! It is meant for repositories that already have shell or batch scripts and
//! want a thin command layer on top, without adopting a bigger task runner.
//!
//! # What it does
//!
//! - Recursively discovers `.sh` scripts on Unix-like systems and `.bat`
//!   scripts on Windows
//! - Resolves a command like `jao deploy api prod` to a script base name like
//!   `deploy.api.prod`
//! - Runs the script from the script's own directory
//! - Supports SHA-256 fingerprint checks for CI-safe execution
//! - Optionally keeps a local trust manifest for interactive runs
//!
//! # Common usage
//!
//! ```text
//! jao --list
//! jao --fingerprint deploy api prod
//! jao deploy api prod
//! jao --ci --require-fingerprint <FINGERPRINT> deploy api prod
//! ```
//!
//! # Trust behavior
//!
//! In the default build, `jao` keeps a trust manifest under `~/.jao/`.
//!
//! - Unknown scripts prompt before first run
//! - Modified scripts prompt again
//! - `--ci` disables prompting
//! - CI runs require `--require-fingerprint`
//!
//! If the crate is built without the `trust-manifest` feature, interactive
//! trust is disabled and runs require an explicit fingerprint.
//!
//! # Fingerprints and Trust Manifests
//!
//! `jao` fingerprints a script by hashing two things together:
//!
//! - the script's canonical path
//! - the script file contents
//!
//! This means moving a script to a different real path changes the
//! fingerprint, even if the bytes are identical. That is intentional: trust is
//! attached to the exact file at the exact resolved location.
//!
//! When the `trust-manifest` feature is enabled, trusted scripts are stored in
//! a local trust manifest keyed by canonical path. Each entry records the last
//! trusted fingerprint for that script. If the current fingerprint differs from
//! the stored one, `jao` treats the script as modified and asks for trust again
//! in interactive mode.
//!
//! # Features
//!
//! - `trust-manifest` (default): Enables local trust tracking for interactive
//!   runs
//! - `config`: Enables config file support used by `trust-manifest`
//!
//! See the repository README for a fuller overview and examples aimed at end
//! users.

use std::io::ErrorKind as IoErrorKind;

use clap::Parser;
use clap::error::ErrorKind;

use crate::error::JaoError;

mod actions;
mod error;
mod help_screen;
mod script_discovery;
mod trust;

#[cfg(feature = "config")]
mod config;

#[derive(Debug, Parser)]
#[command(name = "jao")]
#[command(version)]
#[command(about = "Discover and run workspace scripts")]
#[command(arg_required_else_help = true)]
struct CliArgs {
    /// CI mode: non-interactive and no config/manifest initialization.
    #[arg(long)]
    ci: bool,

    /// Resolve script command and print SHA-256 of canonical path + file contents.
    #[arg(long, value_name = "SCRIPT_COMMAND", num_args = 1..)]
    #[arg(conflicts_with_all = ["list", "script_command"])]
    fingerprint: Option<Vec<String>>,

    /// Required script fingerprint for CI run mode.
    #[arg(long, value_name = "FINGERPRINT")]
    #[cfg_attr(feature = "trust-manifest", arg(requires_all = ["ci", "script_command"]))]
    #[cfg_attr(not(feature = "trust-manifest"), arg(requires = "script_command"))]
    #[arg(conflicts_with_all = ["list", "fingerprint"])]
    require_fingerprint: Option<String>,

    /// List discovered scripts for this OS
    #[arg(long, conflicts_with = "script_command")]
    list: bool,

    /// Script command, e.g. 'deploy api prod'
    #[arg(value_name = "SCRIPT_COMMAND", num_args = 1..)]
    script_command: Vec<String>,
}

fn main() {
    match CliArgs::try_parse() {
        Ok(cli_args) => match std::env::current_dir() {
            Ok(root) => match actions::run_jao_action(cli_args, root) {
                Err(JaoError::Io(io_err)) if io_err.kind() == IoErrorKind::BrokenPipe => std::process::exit(0),
                Err(run_err) => {
                    eprintln!("error: {run_err}");
                    std::process::exit(1)
                }
                Ok(()) => std::process::exit(0),
            },
            Err(io_err) => {
                eprintln!("error: {io_err}");
                std::process::exit(1)
            }
        },
        Err(clap_err) => match clap_err.kind() {
            ErrorKind::DisplayHelp | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => {
                help_screen::print_help();
                std::process::exit(0)
            }
            _ => {
                clap_err.print().expect("Error writing error");
                std::process::exit(clap_err.exit_code());
            }
        },
    }
}
