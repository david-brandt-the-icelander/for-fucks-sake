# `.zazzle/zizd/`: for-fucks-sake's zizd pipeline

The steps zazzle's builders and deployers run for this repository, written against zazzle's step-pipeline spec
(`src/macos/zazzle-pipeline/pipeline-spec.md`, sections 1–4). They take over from `.github/workflows/ci.yml` and
`.github/workflows/release.yml`, which stay until the cutover.

There is one pipeline, `default`, so each step is a folder `.zazzle/zizd/STEP/` with a `Dockerfile` and a `run.sh`.
The pipeline file lives in the orchestrator (`.zazzle/zizd/pipelines/david-brandt-the-icelander/for-fucks-sake/default.json`),
and the copy below is what goes there.

**Releases come from a branch, not a tag.** The pipeline builds `main`, and the branch `release` as the environment
`release`. Only the `release` environment deploys. To release, bring `release` up to the commit to ship, which
replaces "tag `main` `v0.x.y` and push the tag".

## The steps

| Step | Phase | Image | What it does | Leaves in `/zazzle/output` |
|---|---|---|---|---|
| `check` | build | `rust:1.100.0-slim-bookworm`, clippy, rustfmt | the `rust` job of `ci.yml`: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (the replays included), `ffs content validate`, `ffs bot --runs 100 --assert` (release build), and the generated TypeScript types in `web/src/gen` being up to date | nothing |
| `build` | build | `rust:1.100.0-slim-bookworm` with `wasm32-unknown-unknown`, wasm-bindgen-cli 0.2.130, Node 22.21.1, wrangler 4.156.0 | `make wasm`; in `web/`, `npm ci`, `npm run typecheck`, `npm test` and `npm run build` with `FFS_VERSION`; then `wrangler deploy --dry-run --outdir` bundles the Worker | `web/dist/` (the game, the Worker's assets), `bundle/` (the Worker), `wrangler.jsonc`, `wasm/` (the engine, for `e2e`), `VERSION`, `BUILD` |
| `e2e` | build | `mcr.microsoft.com/playwright:v1.65.1-noble` (the Playwright of `web/package-lock.json`) | the keyboard-only Playwright tests in Chromium, with the build's `wasm/` in `web/src/wasm`, so there is no Rust here. The image's own Node runs it (26, where `ci.yml` used 22). | nothing |
| `deploy` | deploy (`deploys: true`) | `node:22.21.1-bookworm-slim`, wrangler 4.156.0 | in `release` only, from the build's artifacts only: `wrangler deploy bundle/index.js --no-bundle`, with `web/dist` as the assets, to forfuckssake.today. It never rebuilds. | nothing |

The Rust steps keep cargo's target directory inside `/tmp`. `check` uses `/tmp/target`, and `build` uses
`/tmp/repo/target`, the Makefile's `target/` in the copy of the repository. Every step works on a copy in `/tmp`,
because the input is read-only.

The deploy step:

- **Refusals.** It refuses outside the `release` environment, without `CLOUDFLARE_API_TOKEN`, without the build's
  artifacts, and with a custom `build` command in `wrangler.jsonc`, which wrangler would run with the token.
- **Halting.** It checks `ZAZZLE_HALT_FILE` before the deploy and exits **75** when asked to halt.
- **No migrations.** It applies none, because the game has no database.
- **Dry runs.** With `DEPLOY_DRY_RUN=1` (by hand only: no pipeline names it), it needs no credentials and runs
  `wrangler deploy --dry-run` on the artifacts.

`FFS_VERSION`, which the title bar shows, is `git describe --tags --always`, as `make deploy` has it. With no new tags
it reads as the last `v*` tag, the number of commits since it, and the commit, such as `v0.3.0-12-gabc1234`. If
releases should carry a version of their own again, the version has to be written in the repository (for example
`web/package.json`), and the build reads it from there. That is the owner's choice and is not made here.

## The pipeline file

```json
{
  "id": "david-brandt-the-icelander/for-fucks-sake:default",
  "repos": ["https://github.com/david-brandt-the-icelander/for-fucks-sake.git"],
  "paths": ["**", "!docs/**", "!Documentation/**", "!*.md"],
  "steps": [
    { "name": "check", "path": ".zazzle/zizd/check" },
    { "name": "build", "path": ".zazzle/zizd/build" },
    { "name": "e2e", "path": ".zazzle/zizd/e2e" },
    { "name": "deploy", "path": ".zazzle/zizd/deploy", "deploys": true }
  ],
  "environments": {
    "release": { "branch": "release", "config": { "deploy": ["CLOUDFLARE_API_TOKEN"] } }
  },
  "supersede": true,
  "halt": { "deployment": "should" }
}
```

`paths` leaves out the game spec (`docs/`), the project paperwork (`Documentation/`) and the Markdown at the root.
Every other change builds.

A deployer for this repository serves `david-brandt-the-icelander/for-fucks-sake:default@release` only. It does not
serve the base `default`, built from `main`, whose deploy step would only refuse.

## Config keys

These are names only. The values are in the orchestrator's config files (later zeecrets), never here.

| Key | Step | File | Where it goes |
|---|---|---|---|
| `CLOUDFLARE_API_TOKEN` | `deploy`, in `release` | `config/david-brandt-the-icelander/for-fucks-sake/default@release.json` | `deploy_keys`. It needs Workers Scripts: Edit on the account, and Workers Routes: Edit and DNS: Edit on the forfuckssake.today zone, the token that is today the repository secret of the same name. |

`CLOUDFLARE_ACCOUNT_ID` is not needed, because `wrangler.jsonc` names `account_id`. The build-phase steps need no key:
the repository is public, `worker/vendor/icelander-auth` is committed, and every dependency comes from public
registries. The builder's own `GITHUB_TOKEN` and z3 keys (`shared.json`) are zazzle's and reach no step.

## Rolling back

`release.yml` rolled back by redeploying an older `v*` tag, which rebuilt it. Now every step's output is kept in z3 at
`zizd/build-artifacts/david-brandt-the-icelander/for-fucks-sake/default@release/COMMIT/STEP/`. To roll back, rerun
the deploy step for an older commit of `release`, on a machine that is also a deployer serving
`david-brandt-the-icelander/for-fucks-sake:default@release`:

```bash
zazzle ci builder run david-brandt-the-icelander/for-fucks-sake:default --env release --from-step deploy --ref SHA --deploy
```

What this does:

- **Nothing is rebuilt or retested.** The steps before `deploy` are not run. Their artifacts for `SHA` are downloaded
  and checked against their manifests, and the deploy step deploys them as they are. `SHA` must be a commit that was
  built in the `release` environment, so that its artifacts are there.
- **The commit's own Worker.** It deploys that commit's Worker and `wrangler.jsonc` as well as its game. The old
  rollback deployed the tag's game with the current Worker configuration.
- **Later releases.** Deployers deploy only the newest finished build, so a rollback stays until the next commit on
  `release` finishes building and is deployed. To roll back for good, move `release` back with a revert commit.

## Trying it by hand

Each build-phase step was run with the spec's recipe ("Trying a step by hand"), each step's output copied into the
next one's `/zazzle/input/artifacts/STEP/`. The deploy step was run only outside `release`, without credentials, with
a halt file and with `DEPLOY_DRY_RUN=1`.

## What goes at cutover

Not removed yet. These go once the pipeline is registered and has deployed:

- `.github/workflows/ci.yml` (the `check`, `build` and `e2e` steps), and `.github/workflows/release.yml` (the `deploy`
  step and the rollback above) with its repository secret `CLOUDFLARE_API_TOKEN`.
- The tag-based release in `Documentation/mvp/delivery.md` ("Deploy") and in `AGENTS.md` (`make deploy`'s note), which
  change with the cutover.
- The `ffs-web` artifact that icelander.net's copy at `/games/ffs/` is updated from, by hand. It becomes the build
  step's `web/dist/` in z3.
