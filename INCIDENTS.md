# INCIDENTS

## 2026-02-09 - Flaky `gitleaks` job failures on `main` push CI
- Status: Resolved
- Impact:
  - CI showed false failures for healthy commits, blocking trust in release readiness.
  - Affected runs included `21557375125`, `21557279815`, and `21557276579`.
- Root cause:
  - Workflow used shallow checkout (`fetch-depth: 1`).
  - `gitleaks-action@v2` attempted scanning a commit range requiring the parent commit SHA, which was missing in shallow history for some push contexts.
- Detection evidence:
  - Error snippet from logs: `fatal: ambiguous argument '<base>^..<head>'`.
  - GitHub Actions logs showed `failed to scan Git repository` with non-zero exit.
- Fix:
  - Update `gitleaks` job checkout to `fetch-depth: 0` in `.github/workflows/ci.yml`.
  - Remove unsupported `args` input from `gitleaks-action@v2` invocation.
- Prevention rules:
  - Any workflow step that computes commit ranges must fetch enough history to resolve parent SHAs.
  - Treat CI tool warnings about unsupported action inputs as reliability risks and clean them up promptly.
  - Keep a recent successful run reference for every workflow change to validate runtime behavior.

## 2026-02-09 - Quality gate break from newly published RustSec advisory
- Status: Resolved
- Impact:
  - `make check` and CI `check` job failed on `cargo audit` despite no source-code regressions.
  - Affected run: `21808825344` (`check` job failed at Quality gate).
- Root cause:
  - New advisory `RUSTSEC-2026-0009` flagged transitive dependency `time 0.3.46`.
  - Lockfile had not yet advanced to patched `time >= 0.3.47`.
- Detection evidence:
  - `cargo audit` output reported medium severity DoS advisory with dependency tree via `ratatui`.
- Fix:
  - Run `cargo update -p time --precise 0.3.47` and commit updated `Cargo.lock`.
  - Re-run `make check` locally to verify audit, tests, lint, build all pass.
- Prevention rules:
  - Treat audit failures as release blockers and remediate in the same session.
  - Re-run full `make check` after any CI/workflow fix commit to catch unrelated ecosystem drift.
  - Keep dependency lockfile updates scoped and evidence-backed (command + resulting version diff).
