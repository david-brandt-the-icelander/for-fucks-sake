# MVP delivery: dev, CI, deploy in one page

Replaces `../development.md`, `../cicd.md` and `../deployment.md` for the MVP. Those stay as the full-game plan.

## Dev

```bash
make dev        # build wasm, then the Vite dev server
make test       # fmt, clippy, cargo test, web typecheck + vitest
make e2e        # Playwright, keyboard only
make balance    # bot report + assertions
cargo run -p ffs-cli -- play --seed 42
```

Needs Rust stable with the `wasm32-unknown-unknown` target, `wasm-bindgen-cli` 0.2.130 (pinned to match the crate),
and Node 22. CI: `.github/workflows/ci.yml`.

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

Hosted on **icelander.net** at <https://icelander.net/games/ffs/>, for signed-in players. icelander.net (its own
repository, `david-brandt-the-icelander/icelander.net`) serves the built files from its `site/ffs-game/`, and its
`docs/ffs.md` is the runbook.

- **Release:** tag `main` here (`v0.x.y`) and push the tag. CI's `web` job builds the `ffs-web` artifact with the
  version (`git describe --tags`) in the title bar; local builds say `dev`.
- **Publish:** copy that build into icelander.net's `site/ffs-game/` in a pull request there. Merging it deploys.
- **Roll back:** the same, with the previous tag's build.
Saves are keyed by content hash so a content change starts a fresh save rather than corrupting an old one.

No secrets, no server, no database, no on-call. If content generation needs an API key, it lives only on the developer's machine.
