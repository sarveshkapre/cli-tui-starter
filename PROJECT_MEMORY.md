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

## 2026-02-09 - Mistakes and fixes
- Mistake: Key spec parsing for modified keys (e.g. `ctrl+c`) initially validated the full input string instead of the key segment, rejecting valid specs.
- Fix: Parse modifiers and key segment separately in `src/keys.rs` and add unit tests + integration smoke coverage.
