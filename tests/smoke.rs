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
