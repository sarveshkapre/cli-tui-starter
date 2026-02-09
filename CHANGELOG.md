# CHANGELOG

## Unreleased
- Add `cli-tui-starter demo --no-tty` to print a static preview without requiring a TTY (useful for CI/docs).
- Add `--format json` to `cli-tui-starter config validate` for scripting/CI use.

## 0.1.5 - 2026-02-09
- Add `cli-tui-starter config init` to generate a commented starter config at the default path.
- Add `cli-tui-starter config validate` to validate a config file without launching the TUI.
- Add `--format json` to `themes` and `keys` for scripting use.
- Always advertise `ctrl+c` as an emergency quit key in `keys` output and the in-app help.
- Add a Windows CI job that runs `cargo test` on `windows-latest`.

## 0.1.4 - 2026-02-09
- Add config file support for `demo` defaults via `$XDG_CONFIG_HOME/cli-tui-starter/config.toml` or `~/.config/cli-tui-starter/config.toml`.
- Add config-driven key binding overrides via `[keys]` and keep UI/help/`keys` output consistent with the active keymap.
- Add explicit CLI override pairs for config-driven toggles: `--no-color/--color`, `--high-contrast/--normal-contrast`, `--reduced-motion/--motion`.
- Expand smoke coverage for `--help`, `--version`, invalid subcommands, and `demo --help` flag surfaces.
- Harden CI secret scan stability by running `gitleaks` with full checkout history.

## 0.1.3 - 2026-02-01
- Make `demo` fail fast with a clear error message when not run in a TTY (prevents CI hangs).
- Add a smoke test ensuring `demo` requires a TTY.

## 0.1.2 - 2026-02-01
- Upgrade `ratatui` and `crossterm` to clear `cargo audit` warnings and modernize the stack.
- Honor `NO_COLOR`, `CLICOLOR=0`, and `TERM=dumb` to automatically disable color output.

## 0.1.1 - 2026-02-01
- Make the demo UI responsive on narrow terminals (single-column layout).
- Make the help popup size itself safely for small terminals.
- Improve header status labels and reduce motion/no-color animation noise.

## 0.1.0 - 2026-02-01
- Initial scaffold with CLI commands, themes, and accessibility toggles.
