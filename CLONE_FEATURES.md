# Clone Feature Tracker

## Context Sources
- README and docs
- TODO/FIXME markers in code
- Test and build failures
- Gaps found during codebase exploration

## Candidate Features To Do
- [ ] P1: Add config-driven keybinding customization and validation (`[keys]` section) with safe defaults.
- [ ] P1: Add optional mouse input support behind explicit config/CLI opt-in.
- [ ] P2: Add UI snapshot tests for narrow and wide terminal layouts to catch rendering regressions.

## Implemented
- [x] 2026-02-09: Stabilized GitHub Actions `gitleaks` job by using full-history checkout and removing unsupported inputs.
  Evidence: `.github/workflows/ci.yml`, run `21808825344` (`gitleaks` step passed).
- [x] 2026-02-09: Added `config.toml` support for demo defaults (`theme`, `no_color`, `high_contrast`, `reduced_motion`) with CLI/config/env precedence.
  Evidence: `src/config.rs`, `src/cli.rs`, `src/main.rs`, `src/app.rs`, `README.md`.
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

## Notes
- This file is maintained by the autonomous clone loop.

### Auto-discovered Open Checklist Items (2026-02-09)
- /Users/sarvesh/code/cli-tui-starter/docs/RELEASE.md:- [x] `make check` passes locally
- /Users/sarvesh/code/cli-tui-starter/docs/RELEASE.md:- [x] CI is green on main (run `21808947535`)
- /Users/sarvesh/code/cli-tui-starter/docs/RELEASE.md:- [x] `CHANGELOG.md` updated
- /Users/sarvesh/code/cli-tui-starter/docs/RELEASE.md:- [ ] Tag version `vX.Y.Z`
- /Users/sarvesh/code/cli-tui-starter/docs/RELEASE.md:- [ ] Publish GitHub release
