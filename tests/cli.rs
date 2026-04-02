use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn no_args_prints_help() {
    command()
        .assert()
        .success()
        .stdout(contains("USAGE:"))
        .stdout(contains("jao --list"));
}

#[test]
fn version_flag_prints_version() {
    command()
        .arg("--version")
        .assert()
    .failure()
    .code(2)
    .stdout(contains("jao "))
    .stderr(contains("error: jao "));
}

#[test]
fn require_fingerprint_without_ci_is_a_parse_error() {
    command()
        .args(["--require-fingerprint", "deadbeef", "check"])
        .assert()
        .failure()
    .code(1)
    .stderr("error: invalid --require-fingerprint value (expected 64 hex chars): deadbeef\n");
}

#[test]
fn invalid_shell_is_a_parse_error() {
    command()
        .args(["--completions", "fish"])
        .assert()
        .failure()
        .code(2)
    .stderr("error: Unknown shell type passed\n");
}

fn command() -> Command {
    Command::cargo_bin("jao").unwrap()
}
