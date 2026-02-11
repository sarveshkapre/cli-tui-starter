# PROJECT

## Quickstart
```bash
make setup
make dev
```

## Commands
- Setup: `make setup`
- Dev: `make dev`
- Test: `make test`
- Lint: `make lint`
- Typecheck: `make typecheck`
- Build: `make build`
- Quality gate: `make check`
- Release build: `make release`

## Demo config
- Default path: `$XDG_CONFIG_HOME/cli-tui-starter/config.toml` or `~/.config/cli-tui-starter/config.toml`
- Override path: `cli-tui-starter demo --config /path/to/config.toml`
- Static preview (no TTY required): `cli-tui-starter demo --no-tty` (optionally `--width N --height N`)
- ASCII-only preview (for logs/limited terminals): `cli-tui-starter demo --no-tty --ascii`
- Keys preview (same config): `cli-tui-starter keys --config /path/to/config.toml`
- Generate a starter config: `cli-tui-starter config init` (use `--stdout` to print)
- Validate config: `cli-tui-starter config validate` (use `--format json` for scripting)
- CLI override flags:
  - color: `--no-color` / `--color`
  - contrast: `--high-contrast` / `--normal-contrast`
  - motion: `--reduced-motion` / `--motion`
  - mouse: `--mouse` / `--no-mouse`

## Key bindings config
```toml
[keys]
cycle_theme = "t"
next_panel = "tab"
prev_panel = "backtab"    # shift+tab
list_up = "up"
list_down = "down"
toggle_high_contrast = "h"
toggle_color = "c"
toggle_reduced_motion = "r"
toggle_help = "?"
quit = ["q", "esc"]
```

## Release flow
1. Update `CHANGELOG.md`.
2. Tag a version `vX.Y.Z`.
3. Publish GitHub release with notes from `docs/RELEASE.md`.

## Windows notes
- Prefer Windows Terminal (or VS Code's integrated terminal). Older consoles can behave differently with alternate-screen TUIs.
- If some key combos do not arrive, check your terminal's own keyboard shortcuts first.
- Use `cli-tui-starter demo --no-tty` to sanity-check rendering without requiring an interactive TTY.

## Next 3 improvements
1. Add a minimal form/text-input demo panel to round out common starter patterns.
2. Add a `--theme random` option and persist last-used theme to config on exit (opt-in).
3. Add a minimal plugin hook for additional panels (compile-time feature flag, no runtime loading).
