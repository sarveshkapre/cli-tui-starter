use assert_cmd::cargo::cargo_bin_cmd;

fn normalize_newlines(s: &str) -> String {
    s.replace("\r\n", "\n")
}

fn run_demo_no_tty_ascii(width: &str, height: &str) -> String {
    let mut cmd = cargo_bin_cmd!("cli-tui-starter");
    let assert = cmd
        .args([
            "demo",
            "--no-tty",
            "--ascii",
            "--width",
            width,
            "--height",
            height,
            // Stabilize the preview against CI env vars like NO_COLOR.
            "--theme",
            "aurora",
            "--color",
            "--normal-contrast",
            "--motion",
        ])
        .assert()
        .success();

    let out = assert.get_output().stdout.clone();
    normalize_newlines(&String::from_utf8(out).expect("utf-8 stdout"))
}

#[test]
fn demo_no_tty_snapshot_80x24_ascii() {
    let expected = normalize_newlines(include_str!("snapshots/demo_80x24_ascii.txt"));
    assert_eq!(run_demo_no_tty_ascii("80", "24"), expected);
}

#[test]
fn demo_no_tty_snapshot_60x18_ascii() {
    let expected = normalize_newlines(include_str!("snapshots/demo_60x18_ascii.txt"));
    assert_eq!(run_demo_no_tty_ascii("60", "18"), expected);
}

#[test]
fn demo_no_tty_snapshot_120x24_ascii() {
    let expected = normalize_newlines(include_str!("snapshots/demo_120x24_ascii.txt"));
    assert_eq!(run_demo_no_tty_ascii("120", "24"), expected);
}
