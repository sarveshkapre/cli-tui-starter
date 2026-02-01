# PLAN

## Product pitch
Apple-level minimal, keyboard-first Rust TUI starter with themes and accessibility toggles.

## Features
- CLI commands: `demo`, `themes`, `keys`
- Themes: aurora / mono / solar
- Accessibility: high-contrast, no-color, reduced motion
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
- 2026-02-01: v0.1.1 responsive layout + adaptive help (see `CHANGELOG.md`)
- 2026-02-01: v0.1.0 initial scaffold

## Next
- Address `cargo audit` warnings by bumping `ratatui` (and dependencies) to a warning-free set
- Add config file support for defaults (theme/toggles)
- Expand smoke tests (CLI help/version, invalid args)
*** Add File: UPDATE.md
# UPDATE

## Summary
- Shipped responsive demo layout for narrow terminals and made the help popup size itself safely.
- Improved header status labels and reduced animation noise in no-color/reduced-motion modes.
- Added root-level `CHANGELOG.md` and root-level `PLAN.md` for standard repo hygiene.

## Commands run
```bash
make check
```

## Notes
- `cargo audit` currently reports allowed warnings inherited via `ratatui` dependencies (see `make check` output).

## PR instructions
If `gh` is available and authenticated:
```bash
git push -u origin HEAD
gh pr create --fill
```

If not authenticated:
1. Create a branch: `git checkout -b feat/responsive-layout`
2. Push it: `git push -u origin feat/responsive-layout`
3. Open a PR on GitHub from that branch into `main`.
