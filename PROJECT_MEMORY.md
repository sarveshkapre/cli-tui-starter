# PROJECT_MEMORY

## 2026-02-09 - Deterministic gitleaks CI scan on push events
- Decision: Configure the `gitleaks` CI job to checkout full history (`fetch-depth: 0`) and rely on the action default invocation.
- Why: The previous shallow checkout intermittently broke push-range scanning (`fatal: ambiguous argument <base>^..<head>`), creating flaky CI failures.
- Evidence:
  - Failed runs: `21557276579`, `21557279815`, `21557375125`
  - Fixed run after change: `21808825344` (`gitleaks` job passed)
  - Files: `.github/workflows/ci.yml`
- Commit: `01340f4d1bb6e6cea0e44eb1ed3689bf3e376509`
- Confidence: High
- Trust label: Verified from GitHub Actions logs and run outcomes
- Follow-ups:
  - Monitor next 5 push runs for recurrence of git-range scan errors.

## 2026-02-09 - Config file defaults for demo runtime behavior
- Decision: Add `config.toml` support for demo defaults (`theme`, `no_color`, `high_contrast`, `reduced_motion`) with explicit precedence rules.
- Why: This was the highest-impact roadmap item for product usability, allowing persistent defaults without requiring long CLI invocations.
- Evidence:
  - Files: `src/config.rs`, `src/cli.rs`, `src/main.rs`, `src/app.rs`, `tests/smoke.rs`, `README.md`
  - Commands: `cargo test`, `make check`
- Commit: `11513356a9215150c372a851014eb84d847090f1`
- Confidence: High
- Trust label: Verified by automated tests and local smoke commands
- Follow-ups:
  - Add keybinding customization to the same config file format.

## 2026-02-09 - Security maintenance response for RustSec advisory
- Decision: Update transitive `time` dependency to `0.3.47` in lockfile to clear `RUSTSEC-2026-0009`.
- Why: `cargo audit` became a hard quality gate failure in local/CI, blocking production readiness.
- Evidence:
  - Local failure and fix commands: `make check`, `cargo update -p time --precise 0.3.47`, `make check`
  - CI failure reference: run `21808825344` (`check` job failed in `Quality gate`)
  - Files: `Cargo.lock`
- Commit: `11513356a9215150c372a851014eb84d847090f1`
- Confidence: High
- Trust label: Verified from `cargo audit` output and successful local re-run
- Follow-ups:
  - Periodically refresh lockfile before release tagging to avoid last-minute audit breakage.

## 2026-02-09 - Configurable keymap with validated `[keys]` config
- Decision: Centralize key handling in a `KeyBindings` keymap, load `[keys]` overrides from `config.toml`, and ensure the in-app help/hints + `cli-tui-starter keys` output always reflect the active keymap.
- Why:
  - Avoid hard-coded drift across `App::handle_key`, help overlay text, and the `keys` command.
  - Improve product usability and match baseline expectations from mature TUIs (custom key bindings + discoverable help).
- Evidence:
  - Files: `src/keys.rs`, `src/config.rs`, `src/app.rs`, `src/ui.rs`, `src/cli.rs`, `src/main.rs`, `tests/smoke.rs`, `docs/PROJECT.md`, `README.md`
  - Verification: `make check`, `cargo run -- keys`, `XDG_CONFIG_HOME=... cargo run -- keys`
- Commit: `9bd32384655b3d1f97bad2138d6577b44c4fa755`
- Confidence: High
- Trust label: Verified by automated tests and local CLI smoke commands
- Follow-ups:
  - Consider adding `cli-tui-starter config init` to generate a starter config with commented `[keys]` defaults.

## 2026-02-09 - Release hygiene: align crate version with changelog
- Decision: Bump `Cargo.toml` version to `0.1.4` to match the existing `CHANGELOG.md` section for 2026-02-09.
- Why: Prevent docs/version drift (`cli-tui-starter --version` should match the shipped changelog version).
- Evidence:
  - Files: `Cargo.toml`, `Cargo.lock`, `CHANGELOG.md`
  - Verification: `make check` (includes `--version` smoke)
- Commit: `a6591ec5c5ea9d866cae8553bd99b08786c696fe`
- Confidence: High
- Trust label: Verified by automated tests

## 2026-02-09 - Cycle 2: Config scaffolding + machine-readable outputs

### 2026-02-09 - Always advertise emergency quit keys
- Decision: Always advertise `esc` and `ctrl+c` as quit keys in the help UI and `cli-tui-starter keys`.
- Why: `ctrl+c` is the expected emergency exit in terminals; showing it reduces confusion and support churn.
- Evidence:
  - Files: `src/keys.rs`, `src/ui.rs`, `tests/smoke.rs`
  - Verification: `make check`
- Commit: `500aa9cc7de68c5e3d0b9f2cf7b885c5c9d98c17`
- Confidence: High
- Trust label: Verified by local tests

### 2026-02-09 - Add `config init`
- Decision: Add `cli-tui-starter config init` (with `--force` and `--stdout`) to generate a commented starter config at the default path.
- Why: Reduces onboarding friction and makes the config format discoverable without reading docs.
- Evidence:
  - Files: `src/cli.rs`, `src/main.rs`, `src/config.rs`, `tests/smoke.rs`, `README.md`, `docs/PROJECT.md`
  - Verification: `make check`, `cargo run -- config init --stdout`
- Commit: `0c5338a613b2705b1d27be0a08df91b37ed3f3c4`
- Confidence: High
- Trust label: Verified by local tests

### 2026-02-09 - Add `config validate`
- Decision: Add `cli-tui-starter config validate` to validate config files without launching the TUI.
- Why: Enables fast CI/scripting validation of config edits and improves error feedback loops.
- Evidence:
  - Files: `src/cli.rs`, `src/main.rs`, `src/config.rs`, `tests/smoke.rs`, `README.md`, `docs/PROJECT.md`
  - Verification: `make check`, `XDG_CONFIG_HOME=$(mktemp -d) cargo run -- config init && cargo run -- config validate`
- Commit: `6179c35fb0b20d86bb1b038117293d1f0bf7b96f`
- Confidence: High
- Trust label: Verified by local tests

### 2026-02-09 - Add `--format json` for `themes` and `keys`
- Decision: Add `--format json` to `cli-tui-starter themes` and `cli-tui-starter keys`.
- Why: Provides a stable integration surface for scripts and higher-level tools built on top of the starter.
- Evidence:
  - Files: `src/cli.rs`, `src/main.rs`, `tests/smoke.rs`, `Cargo.toml`, `Cargo.lock`
  - Verification: `make check`, `cargo run -- themes --format json`, `cargo run -- keys --format json`
- Commit: `75b3133219208becd0bb7693d9d8bb0ad7656464`
- Confidence: High
- Trust label: Verified by local tests

### 2026-02-09 - Add Windows CI coverage
- Decision: Add a Windows job to run `cargo test` on `windows-latest`.
- Why: Catch cross-platform regressions early (terminal event handling and path env behavior).
- Evidence:
  - Files: `.github/workflows/ci.yml`
  - Verification: GitHub Actions run `21821839896` passed (`windows-test`).
- Commit: `374221f95be6657900c79b9f0d22201b405a0e97`
- Confidence: High
- Trust label: Verified by CI

### 2026-02-09 - Release hygiene: bump to v0.1.5
- Decision: Bump crate version to `0.1.5` and add a matching `CHANGELOG.md` entry for the new CLI features.
- Why: Keep `--version` aligned with shipped behavior and avoid “silent” feature drift on main.
- Evidence:
  - Files: `Cargo.toml`, `Cargo.lock`, `CHANGELOG.md`
  - Verification: `make check`
- Commit: (pending)
- Confidence: High
- Trust label: Verified by local tests

## 2026-02-09 - Bounded market scan notes (untrusted)
- Signals:
  - Custom key bindings are a documented feature in Textual (bindings API/config patterns).
  - `ratatui` is the core Rust TUI ecosystem target for this repo; help overlays and key hints are common expectations.
  - Ink demonstrates common CLI UI patterns (multi-step flows, key-driven interactions).
- Sources:
  - https://textual.textualize.io/guide/input/#bindings
  - https://ratatui.rs
  - https://github.com/vadimdemedes/ink
- Trust label: Untrusted (external web)

## 2026-02-09 - Verification evidence
- `gh auth status` (pass)
- `gh run list -L 10 --branch main --workflow ci` (pass; latest runs on 2026-02-09 were `success`)
- `make check` (pass)
- `cargo run -- keys` (pass)
- `cargo run -- themes` (pass)
- `XDG_CONFIG_HOME=$(mktemp -d) cargo run -- keys` (pass; reflected `[keys]` overrides)
- `cargo run -- config init --stdout` (pass)
- `XDG_CONFIG_HOME=$(mktemp -d) cargo run -- config init` (pass)
- `XDG_CONFIG_HOME=$(mktemp -d) cargo run -- config validate` (pass)
- `cargo run -- themes --format json` (pass)
- `cargo run -- keys --format json` (pass)
- `gh run watch 21821839896 --exit-status` (pass)
- `cargo run -- --version` (pass; prints `0.1.5`)

## 2026-02-09 - Mistakes and fixes
- Mistake: Key spec parsing for modified keys (e.g. `ctrl+c`) initially validated the full input string instead of the key segment, rejecting valid specs.
- Fix: Parse modifiers and key segment separately in `src/keys.rs` and add unit tests + integration smoke coverage.
