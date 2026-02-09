# PLAN

## Product pitch
Apple-level minimal, keyboard-first Rust TUI starter with themes, accessibility toggles, and config-driven defaults.

## Features
- CLI commands: `demo`, `themes`, `keys`
- Themes: aurora / mono / solar
- Accessibility: high-contrast, no-color, reduced motion
- Config file defaults for demo behavior
- Panic-safe terminal restore via guard
- Smoke tests for non-interactive commands

## Risks / unknowns
- Tiny terminals: layout readability and help panel fit
- Terminal quirks across platforms (ANSI, color depth, key events)
- Accessibility expectations vary (screen readers, reduced motion defaults)

## Commands
See `docs/PROJECT.md` for the full list. Quickstart:
```bash
make setup
make dev
make check
```

## Shipped (most recent)
- 2026-02-09: Config init/validate + JSON outputs + Windows CI test coverage (see `CHANGELOG.md`)
- 2026-02-09: CI gitleaks reliability hardening + config file defaults + expanded smoke coverage (see `CHANGELOG.md`)
- 2026-02-01: v0.1.3 demo TTY guard + smoke test (see `CHANGELOG.md`)
- 2026-02-01: v0.1.2 dependency refresh + env-based no-color (see `CHANGELOG.md`)
- 2026-02-01: v0.1.1 responsive layout + adaptive help (see `CHANGELOG.md`)
- 2026-02-01: v0.1.0 initial scaffold

## Next
- Add `demo --no-tty` static preview output for CI/docs.
- Add mouse support behind an explicit toggle.
- Add automated snapshot tests for narrow and wide terminal layouts.
