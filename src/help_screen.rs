use std::io::IsTerminal;

const BOLD: &str = "\x1b[1m";
const UNDERLINE: &str = "\x1b[4m";
const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

pub fn print_help() {
    if should_style() {
        println!("{BOLD}{CYAN}jao - discover, inspect, and run workspace scripts{RESET}");
        line("  Finds platform scripts recursively and executes them from their own directory.");
        println!();
        section("USAGE");
        line("  jao --list");
        line("  jao --fingerprint <SCRIPT_COMMAND_PART>...");
        line("  jao <SCRIPT_COMMAND_PART>...");
        println!();

        section("OPTIONS");
        option("  -h, --help", "Show this help screen");
        option(
            "      --list",
            "List runnable scripts discovered from the current directory downward",
        );
        option(
            "      --fingerprint <SCRIPT_COMMAND_PART>...",
            "Resolve a script command, then print SHA-256 of canonical path + file contents",
        );
        option("  -V, --version", "Print version");
        println!();

        section("SCRIPT COMMAND INPUT");
        line("  Positional parts are joined with '.' to form the script base name.");
        line("  Example: jao deploy api prod  -> base name deploy.api.prod");
        line("  Matching extension is chosen by OS: .sh on Unix-like systems, .bat on Windows.");
        line("  The script runs with working directory set to the script's folder.");
        println!();

        section("EXAMPLES");
        example("  jao --list");
        line("    Show all discovered runnable scripts.");
        example("  jao --fingerprint deploy api prod");
        line("    Resolve deploy.api.prod.sh/.bat, then fingerprint that script file.");
        example("  jao test");
        line("    Run test.sh / test.bat if found.");
        example("  jao deploy api prod");
        line("    Run deploy.api.prod.sh / .bat if found.");
    } else {
        println!("jao - discover, inspect, and run workspace scripts");
        println!(
            "  Finds platform scripts recursively and executes them from their own directory."
        );
        println!();
        println!("USAGE:");
        println!("  jao --list");
        println!("  jao --fingerprint <SCRIPT_COMMAND_PART>...");
        println!("  jao <SCRIPT_COMMAND_PART>...");
        println!();
        println!("OPTIONS:");
        println!("  -h, --help                Show this help screen");
        println!(
            "      --list                List runnable scripts discovered from the current directory downward"
        );
        println!(
            "      --fingerprint <SCRIPT_COMMAND_PART>...  Resolve a script command, then print SHA-256 of canonical path + file contents"
        );
        println!("  -V, --version             Print version");
        println!();
        println!("SCRIPT COMMAND INPUT:");
        println!("  Positional parts are joined with '.' to form the script base name.");
        println!("  Example: jao deploy api prod  -> base name deploy.api.prod");
        println!(
            "  Matching extension is chosen by OS: .sh on Unix-like systems, .bat on Windows."
        );
        println!("  The script runs with working directory set to the script's folder.");
        println!();
        println!("EXAMPLES:");
        println!("  jao --list");
        println!("    Show all discovered runnable scripts.");
        println!("  jao --fingerprint deploy api prod");
        println!("    Resolve deploy.api.prod.sh/.bat, then fingerprint that script file.");
        println!("  jao test");
        println!("    Run test.sh / test.bat if found.");
        println!("  jao deploy api prod");
        println!("    Run deploy.api.prod.sh / .bat if found.");
    }
}

fn should_style() -> bool {
    let no_color = std::env::var_os("NO_COLOR").is_some();
    let force_color = std::env::var("CLICOLOR_FORCE").ok().as_deref() == Some("1");
    (std::io::stdout().is_terminal() || force_color) && !no_color
}

fn section(name: &str) {
    println!("{BOLD}{UNDERLINE}{name}:{RESET}");
}

fn option(flag: &str, desc: &str) {
    println!("{BOLD}{flag:<28}{RESET}{desc}");
}

fn line(text: &str) {
    println!("{text}");
}

fn example(cmd: &str) {
    println!("{GREEN}{cmd}{RESET}");
}
