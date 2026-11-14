# MVP delivery: dev, CI, deploy in one page

Replaces `../development.md`, `../cicd.md` and `../deployment.md` for the MVP. Those stay as the full-game plan.

## Dev

```bash
make dev        # wasm-pack watch + Vite
make test       # cargo test + vitest + Playwright (Chromium)
cargo run -p ffs-cli -- play --seed 42
```

Crates in the MVP: `ffs-core`, `ffs-content`, `ffs-wasm`, `ffs-cli` (play + replay verify + tiny bot). No `ffs-server`.
Branch, small commits, PR (draft first). Rule or content change on purpose = regenerate replay corpus in the same commit.

## CI (add stages as needed, don't pre-build)

1. `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`
2. `ffs content validate`
3. `ffs replay --verify`
4. web typecheck + vitest
5. one Playwright happy-path spec (Chromium)
6. small bot run: lazy bot vs clever bot day counts differ and neither ends in < 2 days

Skipped: Firefox/WebKit matrix, nightly balance runs, model checks.

## Deploy

Static site (GitHub Pages or similar). Tag `v0.x` -> CI builds and publishes. Rollback = redeploy the previous tag.
Saves are keyed by content hash so a content change starts a fresh save rather than corrupting an old one.

No secrets, no server, no database, no on-call. If content generation needs an API key, it lives only on the developer's machine.
