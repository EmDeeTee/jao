use clap::Parser;
use std::path::PathBuf;

mod actions;

#[derive(Debug, Parser)]
#[command(name = "jao")]
#[command(about = "A tiny modern CLI example", long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[arg(long, value_name = "FILE")]
    fingerprint: Option<PathBuf>,
}

fn main() {
    if let Err(err) = run_cli() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run_cli() -> std::io::Result<()> {
    let cli = Cli::parse();

    if let Some(file_path) = cli.fingerprint {
        let hash = actions::fingerprint(file_path)?;
        println!("{hash}");
    }

    Ok(())
}
