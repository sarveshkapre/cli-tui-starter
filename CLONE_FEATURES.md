# Clone Feature Tracker

## Context Sources
- README and docs
- TODO/FIXME markers in code
- Test and build failures
- Gaps found during codebase exploration

## Candidate Features To Do
### Backlog
- [ ] P2: Add optional mouse input support behind explicit config/CLI opt-in (`[demo] mouse = true`) and document accessibility tradeoffs.
- [ ] P2: Add a `--theme random` option and persist last-used theme to config on exit (opt-in).
- [ ] P3: Add a minimal plugin hook for additional panels (compile-time feature flag, no runtime loading).
- [ ] P3: Add a `demo --record` mode that writes key events + terminal size to a file for reproducible UI debugging.
- [ ] P3: Add release automation helper (`cargo xtask release-check`) to run `make check`, ensure changelog bumped, and print next release steps.

## Implemented
- [x] 2026-02-10: Expanded the demo showcase into tabbed panels (Overview + Scrolling List) with configurable key bindings for panel switching and list navigation; updated in-app help + `keys` output; added a UI regression test for the list panel.
  Evidence: `src/app.rs`, `src/keys.rs`, `src/config.rs`, `src/main.rs`, `src/ui.rs`, `tests/snapshots/demo_*.txt`, `tests/snapshots/demo_*_ascii.txt`; command `make check`.
- [x] 2026-02-09: Added `cli-tui-starter demo --no-tty --ascii` for an ASCII-only static preview (better for logs/limited terminals) and snapshot coverage.
  Evidence: `src/cli.rs`, `src/config.rs`, `src/main.rs`, `src/ui.rs`, `tests/demo_no_tty_ascii_snapshots.rs`, `tests/snapshots/demo_*_ascii.txt`, `README.md`, `docs/PROJECT.md`; command `make check`.
- [x] 2026-02-09: Stabilized snapshot fixtures on Windows by enforcing LF checkout via `.gitattributes`.
  Evidence: `.gitattributes`; command `git check-attr eol -- tests/snapshots/demo_80x24.txt` (and Windows CI).
- [x] 2026-02-09: Expanded `demo` with a compact "Showcase" panel (gauge + table) and clarified header status labels to match config semantics.
  Evidence: `src/ui.rs`, `README.md`, `docs/ROADMAP.md`; commands `cargo test`, `cargo run -- demo --no-tty --width 80 --height 24 --theme aurora --color --normal-contrast --motion`.
- [x] 2026-02-09: Added golden snapshot tests for `demo --no-tty` output at fixed sizes (60x18, 80x24, 120x24).
  Evidence: `tests/demo_no_tty_snapshots.rs`, `tests/snapshots/demo_*.txt`; command `cargo test`.
- [x] 2026-02-09: Release hygiene: fixed changelog drift and bumped crate version to `0.1.6`.
  Evidence: `CHANGELOG.md`, `Cargo.toml`, `Cargo.lock`; commands `cargo test`, `cargo run -- --version`.
- [x] 2026-02-09: Added `--format json` to `cli-tui-starter config validate` for machine-readable validation output.
  Evidence: `src/cli.rs`, `src/main.rs`, `tests/smoke.rs`, `README.md`, `docs/PROJECT.md`; command `tmp=$(mktemp -d) && XDG_CONFIG_HOME=$tmp cargo run -- config init && XDG_CONFIG_HOME=$tmp cargo run -- config validate --format json`.
- [x] 2026-02-09: Added minimal Windows terminal compatibility notes to docs.
  Evidence: `README.md`, `docs/PROJECT.md`.
- [x] 2026-02-09: Added `cli-tui-starter demo --no-tty` to render a one-frame static preview to stdout (default 80x24) for CI/docs, with `--width/--height` overrides.
  Evidence: `src/cli.rs`, `src/main.rs`, `src/ui.rs`, `tests/smoke.rs`, `README.md`, `docs/PROJECT.md`, `docs/ROADMAP.md`, `CHANGELOG.md`; command `cargo run -- demo --no-tty --width 80 --height 24`.
- [x] 2026-02-09: Added `cli-tui-starter config init` and `cli-tui-starter config validate` for starter config generation and validation.
  Evidence: `src/cli.rs`, `src/main.rs`, `src/config.rs`, `tests/smoke.rs`, `README.md`, `docs/PROJECT.md`; command `make check`.
- [x] 2026-02-09: Added `--format json` output to `themes` and `keys` for scripting use.
  Evidence: `src/cli.rs`, `src/main.rs`, `tests/smoke.rs`; command `cargo run -- themes --format json`.
- [x] 2026-02-09: Always advertise `ctrl+c` as an emergency quit key in `keys` output and the in-app help.
  Evidence: `src/keys.rs`, `src/ui.rs`, `tests/smoke.rs`.
- [x] 2026-02-09: Added a Windows CI job that runs `cargo test` on `windows-latest`.
  Evidence: `.github/workflows/ci.yml`, run `21821839896` (`windows-test` job passed).
- [x] 2026-02-09: Stabilized GitHub Actions `gitleaks` job by using full-history checkout and removing unsupported inputs.
  Evidence: `.github/workflows/ci.yml`, run `21808825344` (`gitleaks` step passed).
- [x] 2026-02-09: Added `config.toml` support for demo defaults (`theme`, `no_color`, `high_contrast`, `reduced_motion`) with CLI/config/env precedence.
  Evidence: `src/config.rs`, `src/cli.rs`, `src/main.rs`, `src/app.rs`, `README.md`.
- [x] 2026-02-09: Added config-driven key binding overrides via `[keys]` with validation, and ensured the in-app help/hints and `cli-tui-starter keys` output always reflect the active keymap.
  Evidence: `src/keys.rs`, `src/config.rs`, `src/app.rs`, `src/ui.rs`, `src/cli.rs`, `src/main.rs`, `tests/smoke.rs`, command `make check`.
- [x] 2026-02-09: Added non-interactive UI regression tests using the `ratatui` test backend (narrow + wide layout + keymap label propagation).
  Evidence: `src/ui.rs`, command `cargo test`.
- [x] 2026-02-09: Expanded CLI smoke coverage (`--help`, `--version`, invalid subcommand, demo help flags) and validated config error paths.
  Evidence: `tests/smoke.rs`; commands `cargo test`, `cargo run -- demo --help`, invalid theme smoke with `XDG_CONFIG_HOME`.
- [x] 2026-02-09: Added persistent automation records for decisions and incidents.
  Evidence: `PROJECT_MEMORY.md`, `INCIDENTS.md`.
- [x] 2026-02-09: Repaired docs drift and release hygiene.
  Evidence: `PLAN.md` cleanup, `docs/PROJECT.md`, `docs/ROADMAP.md`, `docs/CONTRIBUTING.md`, `CHANGELOG.md`.
- [x] 2026-02-09: Resolved new RustSec advisory in quality gate by updating transitive `time` to `0.3.47`.
  Evidence: `Cargo.lock`; command `cargo update -p time --precise 0.3.47`; `make check` passed.

## Insights
- Gap map (trusted, local):
  - Missing: opt-in mouse support.
  - Weak: demo breadth (needs a minimal form/text-input example).
  - Parity: help overlay + configurable key bindings (now present).
  - Differentiator: CI-friendly `demo --no-tty` plus golden UI snapshots.
- CI failure pattern from runs `21557276579`, `21557279815`, and `21557375125` is a flaky gitleaks push-range scan on shallow checkout (`fetch-depth: 1`).
- No open GitHub issues from `sarveshkapre` or trusted bot accounts as of 2026-02-09.
- A new advisory (`RUSTSEC-2026-0009`) affected transitive `time`; lockfile updates should be part of routine maintenance before release tagging.
- Market expectations (untrusted, external):
  - Most mature TUIs provide a discoverable in-app help overlay and user-customizable keybindings (often via config).
  - Tabs and scrolling list widgets are common in TUI demo galleries/templates (useful as a starter's "widget cookbook").
  - The Ratatui ecosystem explicitly encourages snapshot-style testing using `TestBackend` to catch UI regressions.
  - Several popular TUI stacks (e.g. Charm's Bubble Tea) include a compact, auto-generated help view derived from key bindings.
    Sources:
    - https://ratatui.rs
    - https://github.com/ratatui/templates
    - https://github.com/ohmyroot/ratatui-template
    - https://ratatui.rs/recipes/testing/snapshots/
    - https://ratatui.rs/widgets/tabs/
    - https://ratatui.rs/widgets/list/
    - https://github.com/charmbracelet/bubbles
    - https://github.com/charmbracelet/bubbles/tree/master/help
    - https://textual.textualize.io/guide/input/#bindings
    - https://github.com/vadimdemedes/ink
- Product insight:
  - Centralizing the keymap removes hard-coded drift across `keys` output and the TUI help overlay and makes future features (mouse toggle, command palette, custom actions) safer to ship.

## Notes
- This file is maintained by the autonomous clone loop.

### Auto-discovered Open Checklist Items (2026-02-09)
- /Users/sarvesh/code/cli-tui-starter/docs/RELEASE.md:- [x] `make check` passes locally (as of 2026-02-09)
- /Users/sarvesh/code/cli-tui-starter/docs/RELEASE.md:- [x] CI is green on main (run `21821839896`)
- /Users/sarvesh/code/cli-tui-starter/docs/RELEASE.md:- [x] `CHANGELOG.md` updated
- /Users/sarvesh/code/cli-tui-starter/docs/RELEASE.md:- [ ] Tag version `vX.Y.Z`
- /Users/sarvesh/code/cli-tui-starter/docs/RELEASE.md:- [ ] Publish GitHub release
