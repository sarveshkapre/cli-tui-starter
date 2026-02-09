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
- Keys preview (same config): `cli-tui-starter keys --config /path/to/config.toml`
- Generate a starter config: `cli-tui-starter config init` (use `--stdout` to print)
- Validate config: `cli-tui-starter config validate`
- CLI override flags:
  - color: `--no-color` / `--color`
  - contrast: `--high-contrast` / `--normal-contrast`
  - motion: `--reduced-motion` / `--motion`

## Key bindings config
```toml
[keys]
cycle_theme = "t"
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

## Next 3 improvements
1. Add `demo --no-tty` static preview output for CI/docs.
2. Improve Windows terminal compatibility notes and common key-event quirks.
3. Optional mouse support with explicit opt-in.
