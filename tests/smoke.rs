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
fn prints_themes_json() {
    let mut cmd = cargo_bin_cmd!("cli-tui-starter");
    cmd.args(["themes", "--format", "json"])
        .assert()
        .success()
        .stdout(contains("\"themes\""))
        .stdout(contains("\"aurora\""));
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
fn prints_keys_json() {
    let mut cmd = cargo_bin_cmd!("cli-tui-starter");
    cmd.args(["keys", "--format", "json"])
        .assert()
        .success()
        .stdout(contains("\"cycle_theme\""))
        .stdout(contains("\"quit\""));
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
fn demo_no_tty_renders_static_preview() {
    let mut cmd = cargo_bin_cmd!("cli-tui-starter");
    cmd.args(["demo", "--no-tty", "--width", "80", "--height", "24"])
        .assert()
        .success()
        .stdout(contains("CLI TUI Starter"))
        .stdout(contains("Commands"))
        .stdout(contains("Accessibility"));
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
        .stdout(contains("--no-tty"))
        .stdout(contains("--width"))
        .stdout(contains("--height"))
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

#[test]
fn config_init_writes_starter_config_to_default_xdg_path() {
    let root = unique_temp_dir();

    let mut cmd = cargo_bin_cmd!("cli-tui-starter");
    cmd.args(["config", "init"])
        .env("XDG_CONFIG_HOME", &root)
        .assert()
        .success()
        .stdout(contains("Wrote config:"));

    let path = root.join("cli-tui-starter").join("config.toml");
    let contents = fs::read_to_string(&path).expect("read config");
    assert!(contents.contains("[demo]"));
    assert!(contents.contains("[keys]"));
}

#[test]
fn config_init_refuses_to_overwrite_without_force() {
    let root = unique_temp_dir();
    let dir = root.join("cli-tui-starter");
    fs::create_dir_all(&dir).expect("create dir");
    fs::write(dir.join("config.toml"), "sentinel").expect("write sentinel");

    let mut cmd = cargo_bin_cmd!("cli-tui-starter");
    cmd.args(["config", "init"])
        .env("XDG_CONFIG_HOME", &root)
        .assert()
        .failure()
        .stderr(contains("already exists"))
        .stderr(contains("--force"));
}

#[test]
fn config_init_force_overwrites_existing_file() {
    let root = unique_temp_dir();
    let dir = root.join("cli-tui-starter");
    fs::create_dir_all(&dir).expect("create dir");
    fs::write(dir.join("config.toml"), "sentinel").expect("write sentinel");

    let mut cmd = cargo_bin_cmd!("cli-tui-starter");
    cmd.args(["config", "init", "--force"])
        .env("XDG_CONFIG_HOME", &root)
        .assert()
        .success();

    let contents = fs::read_to_string(dir.join("config.toml")).expect("read config");
    assert!(contents.contains("# cli-tui-starter config"));
    assert!(!contents.contains("sentinel"));
}

#[test]
fn config_validate_succeeds_for_default_xdg_path() {
    let root = unique_temp_dir();
    let config_dir = root.join("cli-tui-starter");
    fs::create_dir_all(&config_dir).expect("create config dir");
    fs::write(config_dir.join("config.toml"), dummy_config_for_validate()).expect("write config");

    let mut cmd = cargo_bin_cmd!("cli-tui-starter");
    cmd.args(["config", "validate"])
        .env("XDG_CONFIG_HOME", &root)
        .assert()
        .success()
        .stdout(contains("Config OK:"));
}

#[test]
fn config_validate_json_succeeds_for_default_xdg_path() {
    let root = unique_temp_dir();
    let config_dir = root.join("cli-tui-starter");
    fs::create_dir_all(&config_dir).expect("create config dir");
    fs::write(config_dir.join("config.toml"), dummy_config_for_validate()).expect("write config");

    let mut cmd = cargo_bin_cmd!("cli-tui-starter");
    cmd.args(["config", "validate", "--format", "json"])
        .env("XDG_CONFIG_HOME", &root)
        .assert()
        .success()
        .stdout(contains("\"ok\": true"))
        .stdout(contains("\"path\""));
}

#[test]
fn config_validate_fails_when_default_missing() {
    let root = unique_temp_dir();

    let mut cmd = cargo_bin_cmd!("cli-tui-starter");
    cmd.args(["config", "validate"])
        .env("XDG_CONFIG_HOME", &root)
        .assert()
        .failure()
        .stderr(contains("config file not found"))
        .stderr(contains("config init"));
}

fn dummy_config_for_validate() -> &'static str {
    r#"
    [demo]
    theme = "aurora"

    [keys]
    cycle_theme = "t"
    quit = "q"
    "#
}
