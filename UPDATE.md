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

## PR
- https://github.com/sarveshkapre/cli-tui-starter/pull/1
