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

Static site on **GitHub Pages**: <https://david-brandt-the-icelander.github.io/for-fucks-sake/>.
`.github/workflows/release.yml` runs on every `v*` tag push: `cargo test` (replays included), WASM build, web tests,
`vite build` with `FFS_VERSION` set to the tag (shown in the title bar), then publish. Local builds say `dev`.

- **Release:** tag `main` and push the tag (`git tag -a v0.3.0 -m ... && git push origin v0.3.0`).
- **Republish / roll back:** Actions > release > Run workflow. Leave the tag blank for the newest `v*` tag,
  or name an older tag to put that version back up. Minutes either way.
- **One-time setup:** repository Settings > Pages > Source: "GitHub Actions".
Saves are keyed by content hash so a content change starts a fresh save rather than corrupting an old one.

No secrets (Pages publishes with the workflow's own short-lived token), no server, no database, no on-call. If content generation needs an API key, it lives only on the developer's machine.
