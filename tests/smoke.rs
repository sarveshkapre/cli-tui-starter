use assert_cmd::cargo::cargo_bin_cmd;
use predicates::str::contains;

#[test]
fn prints_themes() {
    let mut cmd = cargo_bin_cmd!("cli-tui-starter");
    cmd.arg("themes")
        .assert()
        .success()
        .stdout(contains("Available themes"));
}

#[test]
fn prints_keys() {
    let mut cmd = cargo_bin_cmd!("cli-tui-starter");
    cmd.arg("keys")
        .assert()
        .success()
        .stdout(contains("Key bindings"));
}

#[test]
fn demo_requires_tty() {
    let mut cmd = cargo_bin_cmd!("cli-tui-starter");
    cmd.arg("demo")
        .assert()
        .failure()
        .stderr(contains("requires a real terminal (TTY)"));
}

#[test]
fn prints_help() {
    let mut cmd = cargo_bin_cmd!("cli-tui-starter");
    cmd.arg("--help").assert().success().stdout(contains(
        "Minimal TUI starter with themes and accessibility",
    ));
}

#[test]
fn prints_version() {
    let mut cmd = cargo_bin_cmd!("cli-tui-starter");
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn invalid_subcommand_exits_with_usage_error() {
    let mut cmd = cargo_bin_cmd!("cli-tui-starter");
    cmd.arg("bogus")
        .assert()
        .failure()
        .stderr(contains("unrecognized subcommand"));
}

#[test]
fn demo_help_includes_config_and_override_flags() {
    let mut cmd = cargo_bin_cmd!("cli-tui-starter");
    cmd.args(["demo", "--help"])
        .assert()
        .success()
        .stdout(contains("--config"))
        .stdout(contains("--color"))
        .stdout(contains("--motion"))
        .stdout(contains("--normal-contrast"));
}
