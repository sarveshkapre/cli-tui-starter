# Clone Feature Tracker

## Context Sources
- README and docs
- TODO/FIXME markers in code
- Test and build failures
- Gaps found during codebase exploration

## Candidate Features To Do
### Cycle 4 (selected)
- [ ] P1: Expand the `demo` screen to showcase a small set of common widgets/layout patterns (table + gauge) while keeping the code minimal.
- [ ] P1: Add snapshot tests for `demo --no-tty` output at a few fixed sizes to catch UI regressions without a real terminal.
- [ ] P1: Fix minor UX drift: make header status labels match config semantics (`No color`, `Reduced motion`) and keep README aligned.
- [ ] P1: Release hygiene: fix changelog drift and bump patch version for user-visible changes.

### Backlog
- [ ] P2: Add optional mouse input support behind explicit config/CLI opt-in (`[demo] mouse = true`) and document accessibility tradeoffs.
- [ ] P2: Add a `--theme random` option and persist last-used theme to config on exit (opt-in).
- [ ] P2: Add an ASCII-only render mode for `demo --no-tty` for terminals/logs that dislike box-drawing glyphs.
- [ ] P3: Add a minimal plugin hook for additional panels (compile-time feature flag, no runtime loading).
- [ ] P3: Add a `demo --record` mode that writes key events + terminal size to a file for reproducible UI debugging.
- [ ] P3: Add release automation helper (`cargo xtask release-check`) to run `make check`, ensure changelog bumped, and print next release steps.

## Implemented
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
- CI failure pattern from runs `21557276579`, `21557279815`, and `21557375125` is a flaky gitleaks push-range scan on shallow checkout (`fetch-depth: 1`).
- No open GitHub issues from `sarveshkapre` or trusted bot accounts as of 2026-02-09.
- A new advisory (`RUSTSEC-2026-0009`) affected transitive `time`; lockfile updates should be part of routine maintenance before release tagging.
- Market expectations (untrusted, external):
  - Most mature TUIs provide a discoverable in-app help overlay and user-customizable keybindings (often via config).
    Sources:
    - https://ratatui.rs
    - https://github.com/ratatui/templates
    - https://github.com/ohmyroot/ratatui-template
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
