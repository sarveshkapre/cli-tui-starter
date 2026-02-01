# PLAN

## Goal
Ship a beautiful minimal TUI template that demonstrates commands, themes, and accessibility toggles.

## Stack
- Rust 2021 + Cargo
- `ratatui` for layout/rendering
- `crossterm` for terminal events
- `clap` for CLI commands

Rationale: Rust + ratatui is stable, fast, and keeps dependencies small while providing a great TUI developer experience.

## Architecture
- `main.rs` wires CLI commands to the demo or text output.
- `app.rs` owns state, key handling, and accessibility toggles.
- `ui.rs` renders the layout and help overlay.
- `theme.rs` defines palettes and provides accessibility overrides.
- `terminal.rs` ensures the terminal is always restored.

## Milestones
1. Scaffold repo with required docs, CI, and Makefile.
2. Implement CLI commands and TUI demo.
3. Add tests and polish accessibility details.
4. Prepare v0.1.0 release notes.

## MVP checklist
- [x] `demo` command launches a working TUI
- [x] `themes` command lists available themes
- [x] `keys` command lists key bindings
- [x] No-color + high-contrast + reduced-motion toggles
- [x] One integration smoke test
- [x] CI runs `make check`

## Risks
- Terminal mode not restored on panic (mitigated with guard in `terminal.rs`).
- Theme colors not visible on some terminals (mitigated with no-color and high-contrast toggles).
- Users on older terminals lacking ANSI support (document no-color usage).
