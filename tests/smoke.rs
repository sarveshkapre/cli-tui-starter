use assert_cmd::cargo::cargo_bin_cmd;
use predicates::str::contains;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

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

fn unique_temp_dir() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "cli-tui-starter-test-{}-{}",
        std::process::id(),
        nanos
    ))
}

#[test]
fn keys_reflect_config_overrides_via_xdg_config_home() {
    let root = unique_temp_dir();
    let config_dir = root.join("cli-tui-starter");
    fs::create_dir_all(&config_dir).expect("create config dir");
    fs::write(
        config_dir.join("config.toml"),
        r#"
        [keys]
        cycle_theme = "n"
        quit = "x"
        "#,
    )
    .expect("write config");

    let mut cmd = cargo_bin_cmd!("cli-tui-starter");
    cmd.arg("keys")
        .env("XDG_CONFIG_HOME", &root)
        .assert()
        .success()
        .stdout(contains("- n: cycle theme"))
        .stdout(contains("- x/esc/ctrl+c: quit"));
}
