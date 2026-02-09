# AGENTS

## Working agreements
- Keep the TUI minimal, fast, and keyboard-first.
- Prefer small, readable functions and clear names.
- Avoid adding heavy dependencies without discussion.
- Keep accessibility toggles and no-color mode working.

## Commands
- `make setup` - install Rust components and audit tooling
- `make dev` - run the demo locally
- `make test` - run tests
- `make lint` - run clippy
- `make typecheck` - run cargo check
- `make build` - release build
- `make check` - full quality gate

## Architecture map
- `src/main.rs` - CLI entry point and command routing
- `src/app.rs` - app state and key handling
- `src/config.rs` - config file loading and CLI/config/env precedence resolution
- `src/ui.rs` - layout and rendering
- `src/theme.rs` - themes and accessibility palettes
- `src/terminal.rs` - terminal lifecycle guard
