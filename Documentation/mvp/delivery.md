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

Live at **<https://forfuckssake.today>**, **sign-in required** (icelander.net's login service; no rank is asked, the game is unranked). A Cloudflare Worker with static assets
(`wrangler.jsonc`, worker `for-fucks-sake`) serves `web/dist/`; `worker/index.js` sends `www.` to the bare
domain, answers `/auth/*` (the login service's sign-in, through the vendored `worker/vendor/icelander-auth/`, never
edited here: change it in icelander.net's `packages/icelander-auth` and copy it with `sync.sh`) and asks a visitor
who is not signed in to sign in before the page. Scripts, styles and WASM stay public. Security and cache headers are in `web/public/_headers`, the 404 page in `web/public/404.html`.

- **Release:** tag `main` (`v0.x.y`) and push the tag. `.github/workflows/release.yml` runs `cargo test` (replays
  included) and the web tests, builds with the tag in the title bar, and deploys. Local builds say `dev`.
- **Redeploy / roll back:** Actions > release > Run workflow. Blank deploys the newest `v*` tag; an older tag
  rolls back to it (that tag's game, today's Worker config). Minutes.
- **By hand:** `make deploy` with `CLOUDFLARE_API_TOKEN` set (builds the current checkout, `git describe` version).
- **Secret:** the repository secret `CLOUDFLARE_API_TOKEN` (Workers Scripts: Edit on the account; Workers Routes
  and DNS: Edit on the `forfuckssake.today` zone). Without it the release workflow fails before deploying.
- icelander.net also carries a copy at `/games/ffs/` for its signed-in visitors (its `docs/ffs.md`), updated by
  hand from CI's `ffs-web` artifact.
Saves are keyed by content hash so a content change starts a fresh save rather than corrupting an old one.

One secret (`CLOUDFLARE_API_TOKEN`, above), no server of our own, no database, no on-call. **One external service: icelander.net's login service** (a service binding `ACCOUNTS` to the Worker `accounts` in the same Cloudflare account, its public keys in `ACCOUNTS_JWKS` in `wrangler.jsonc`, and the app `ffs` with the callback `https://forfuckssake.today/auth/callback` registered in its `APPS`); no new secret. If that service is down, nobody can sign in, so nobody can start the game (it fails closed). If content generation needs an API key, it lives only on the developer's machine.
