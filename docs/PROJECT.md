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
- CLI override flags:
  - color: `--no-color` / `--color`
  - contrast: `--high-contrast` / `--normal-contrast`
  - motion: `--reduced-motion` / `--motion`

## Release flow
1. Update `CHANGELOG.md`.
2. Tag a version `vX.Y.Z`.
3. Publish GitHub release with notes from `docs/RELEASE.md`.

## Next 3 improvements
1. Keybinding customization via config file.
2. Wider terminal size auto-scaling and responsive layout hints.
3. Optional mouse support with toggles for accessibility.
