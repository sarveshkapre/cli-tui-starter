# PROJECT_MEMORY

## 2026-02-10 - Tabbed showcase panels + scrolling list demo + configurable navigation keys
- Decision: Expand the demo showcase into tabbed panels (Overview + List) and add a minimal scrolling list demo with configurable panel/list navigation key bindings.
- Why:
  - Tabs + lists are common building blocks; having them in the starter demo improves product usefulness and gives contributors a concrete reference pattern.
  - Keeping navigation keys inside the existing `[keys]` config prevents help/`keys` drift as the demo grows.
- Evidence:
  - Files: `src/app.rs`, `src/keys.rs`, `src/config.rs`, `src/main.rs`, `src/ui.rs`, `tests/snapshots/demo_*.txt`, `tests/snapshots/demo_*_ascii.txt`, `README.md`, `docs/PROJECT.md`, `docs/ROADMAP.md`, `CHANGELOG.md`, `CLONE_FEATURES.md`
  - Commands (pass):
    - `make check`
    - `cargo run -- demo --no-tty --width 80 --height 24 --theme aurora --color --normal-contrast --motion`
    - `cargo run -- keys --format json`
    - `gh run watch 21855370758 --exit-status`
- Commit: `e0981e4bf67f5a56cc242814523b04a9884c2f15`
- Confidence: High
- Trust label: Trusted (local tests + CI green)

## 2026-02-09 - ASCII-only `demo --no-tty` output + newline-stable snapshot fixtures
- Decision:
  - Add `cli-tui-starter demo --no-tty --ascii` and config support (`[demo] ascii = true`) to render ASCII-only static previews.
  - Add `.gitattributes` rule to keep `tests/snapshots/*.txt` checked out with LF to avoid Windows CRLF diffs.
- Why:
  - ASCII previews are more reliable in logs and terminals that don't render box-drawing glyphs well.
  - Line-ending stability prevents cross-platform snapshot churn and CI failures.
- Evidence:
  - Files: `.gitattributes`, `src/cli.rs`, `src/config.rs`, `src/main.rs`, `src/ui.rs`, `tests/demo_no_tty_ascii_snapshots.rs`, `tests/snapshots/demo_*_ascii.txt`, `README.md`, `docs/PROJECT.md`, `CHANGELOG.md`, `CLONE_FEATURES.md`
  - Commands (pass):
    - `make check`
    - `cargo run -- demo --no-tty --ascii --width 80 --height 24 --theme aurora --color --normal-contrast --motion`
    - `git check-attr eol -- tests/snapshots/demo_80x24.txt tests/snapshots/demo_80x24_ascii.txt`
    - `gh run watch 21845631145 --exit-status`
    - `gh run watch 21845690264 --exit-status`
- Commit: `0851f5d341a3c6e60b239b2953d777f85e257927` (feature) + `fbdf55e14ef333227ddb2120bc53e2ede257de4b` (gitattributes)
- Confidence: High
- Trust label: Trusted (local code/tests + CI green)

## 2026-02-09 - Demo "Showcase" panel + clearer status labels
- Decision: Expand the demo body with a compact "Showcase" panel that demonstrates common widget patterns (gauge + table), and clarify header status labels to match config semantics (`No color`, `Reduced motion`).
- Why: A starter's demo is the product; showing real widget primitives plus clear state labels improves onboarding, expectations, and regressions visibility.
- Evidence:
  - Files: `src/ui.rs`, `README.md`, `docs/ROADMAP.md`
  - Commands: `cargo test`, `cargo run -- demo --no-tty --width 80 --height 24 --theme aurora --color --normal-contrast --motion`
- Commit: `9f2a09a19f65cbedd9c316bc4b089d382c5bacd1`
- Confidence: High
- Trust label: Trusted (local code/tests)
- Follow-ups:
  - If the demo grows further, consider a dedicated "widgets gallery" view with tabs and scrolling.

## 2026-02-09 - Golden snapshots for `demo --no-tty`
- Decision: Add golden snapshot tests for `demo --no-tty` at fixed sizes (60x18, 80x24, 120x24).
- Why: Catch UI regressions without requiring an interactive terminal; verify both narrow and wide layouts cross-platform.
- Evidence:
  - Files: `tests/demo_no_tty_snapshots.rs`, `tests/snapshots/demo_*.txt`
  - Commands: `cargo test`
- Commit: `5a184f5169dfbeacd1b95303e98ba76f30fe9e18`
- Confidence: High
- Trust label: Trusted (local code/tests)

## 2026-02-09 - Release hygiene: bump to v0.1.6 + align changelog
- Decision: Bump crate version to `0.1.6` and align `CHANGELOG.md` so shipped features are listed under the correct release section (avoid "Unreleased" drift on main).
- Why: Keep `cli-tui-starter --version` and the changelog consistent, and make it obvious which user-visible changes shipped together.
- Evidence:
  - Files: `Cargo.toml`, `Cargo.lock`, `CHANGELOG.md`
  - Commands: `make check`, `cargo run -- --version`
- Commit: `3888074184ceae67ec4b3da553b904392d0a1d17`
- Confidence: High
- Trust label: Trusted (local code/tests)

## 2026-02-09 - Non-interactive `demo --no-tty` static preview rendering
- Decision: Add `cli-tui-starter demo --no-tty` to render a one-frame static preview to stdout (default 80x24) with `--width/--height` overrides.
- Why: Enables CI/docs-friendly verification and makes it possible to demonstrate the UI without an interactive TTY (avoids hangs and improves onboarding).
- Evidence:
  - Files: `src/cli.rs`, `src/main.rs`, `src/ui.rs`, `tests/smoke.rs`, `README.md`, `docs/PROJECT.md`, `docs/ROADMAP.md`, `CHANGELOG.md`
  - Commands: `cargo test`, `make check`, `cargo run -- demo --no-tty --width 60 --height 18`
- Commit: `7aa5460f4049b4132a6ef86a0ff3420d5bf0a2bf`
- Confidence: High
- Trust label: Trusted (local code/tests)
- Follow-ups:
  - Consider adding an ASCII-only render mode for terminals/logs that dislike box-drawing glyphs.

## 2026-02-09 - Machine-readable `config validate` output + Windows notes
- Decision:
  - Add `--format json` to `cli-tui-starter config validate` (success output).
  - Document minimal Windows terminal compatibility notes in `README.md` and `docs/PROJECT.md`.
- Why: Improve scripting ergonomics and reduce common cross-platform setup friction.
- Evidence:
  - Files: `src/cli.rs`, `src/main.rs`, `tests/smoke.rs`, `README.md`, `docs/PROJECT.md`, `CHANGELOG.md`
  - Commands: `make check`, `tmp=$(mktemp -d) && XDG_CONFIG_HOME=$tmp cargo run -- config init && XDG_CONFIG_HOME=$tmp cargo run -- config validate --format json`
- Commit: `0c6722596f2853fed6c930f1b39ff49d49f86e12`
- Confidence: High
- Trust label: Trusted (local code/tests)
- Follow-ups:
  - If needed, extend JSON output to represent failures without double-printing (requires centralizing error handling).

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

### 2026-02-09 - Prioritization snapshot (scores 1-5)
- Selected:
  - `config init` (impact 5, effort 2, strategic fit 5, differentiation 3, risk 1, confidence 5)
  - `config validate` (impact 4, effort 2, strategic fit 5, differentiation 2, risk 1, confidence 5)
  - `themes/keys --format json` (impact 4, effort 3, strategic fit 4, differentiation 2, risk 2, confidence 4)
  - Always advertise `ctrl+c` quit (impact 3, effort 1, strategic fit 4, differentiation 1, risk 1, confidence 5)
  - Windows CI `cargo test` (impact 4, effort 1, strategic fit 4, differentiation 1, risk 2, confidence 4)
- Deferred:
  - `demo --no-tty` static preview (impact 4, effort 3, strategic fit 4, differentiation 2, risk 2, confidence 3)
  - Windows compatibility notes (impact 3, effort 2, strategic fit 3, differentiation 1, risk 1, confidence 3)

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
- Commit: `fa22bff07f5c0e0880db615397105c8a73bbbb37`
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

## 2026-02-10 - Verification evidence
- `make check` (pass)
- `cargo run -- demo --no-tty --width 80 --height 24 --theme aurora --color --normal-contrast --motion` (pass)
- `cargo run -- keys --format json` (pass)
- `gh run watch 21855370758 --exit-status` (pass)

## 2026-02-09 - Verification evidence
- `gh auth status` (pass)
- `gh run list -L 10 --branch main --workflow ci` (pass; latest runs on 2026-02-09 were `success`)
- `make check` (pass)
- `cargo test` (pass)
- `cargo run -- demo --no-tty --width 60 --height 18` (pass)
- `cargo run -- demo --no-tty --ascii --width 80 --height 24 --theme aurora --color --normal-contrast --motion` (pass)
- `git check-attr eol -- tests/snapshots/demo_80x24.txt tests/snapshots/demo_80x24_ascii.txt` (pass; `eol: lf`)
- `gh run watch 21845631145 --exit-status` (pass)
- `gh run watch 21845690264 --exit-status` (pass)
- `tmp=$(mktemp -d) && XDG_CONFIG_HOME=$tmp cargo run -- config init && XDG_CONFIG_HOME=$tmp cargo run -- config validate --format json` (pass)
- `gh issue list --limit 20 --json number,title,author,state --jq ...` (pass; no open issues)
- `gh run watch 21829728935 --exit-status` (pass)
- `gh run watch 21829808952 --exit-status` (pass)
- `gh run watch 21829857898 --exit-status` (pass)
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
- `cargo run -- --version` (pass; prints `0.1.6`)
- `cargo run -- demo --no-tty --width 80 --height 24 --theme aurora --color --normal-contrast --motion` (pass)
- `make check` (pass)
- `gh run list --limit 10 --branch main` (fail; HTTP 429 throttled)
- `gh run watch 21838748447 --exit-status` (pass)
- `gh run watch 21838814243 --exit-status` (pass)

## 2026-02-09 - Mistakes and fixes
- Mistake: Key spec parsing for modified keys (e.g. `ctrl+c`) initially validated the full input string instead of the key segment, rejecting valid specs.
- Fix: Parse modifiers and key segment separately in `src/keys.rs` and add unit tests + integration smoke coverage.
- Mistake: Golden snapshot tests compared normalized stdout (`\n`) against expected files that can be checked out as CRLF (`\r\n`) on Windows, causing CI failures.
- Fix: Normalize newlines for both actual stdout and expected snapshot text in `tests/demo_no_tty_snapshots.rs`.
