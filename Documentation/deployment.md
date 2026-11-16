# Deployment

**Source of truth:** here, until real pipeline config exists; then the pipeline files win and this says what they do.

Nothing is deployed yet. This is the plan.

## Environments

| Environment | Where | Deployed by | Data |
| --- | --- | --- | --- |
| Local | dev machine (`make dev`) | you | fixtures / seeds |
| Single-player | static site (host TBD: GitHub Pages, Cloudflare Pages, or similar) | CI on tag | none (browser storage only) |
| Multiplayer staging | optional, a cheap VPS or just local Docker | manual | synthetic |
| Multiplayer prod | one container (`ffs-server`) + Postgres on a small VPS (TBD) | manual `docker compose pull && up -d` at first, CI later | real handles/runs |

## What gets deployed

- **Static web build**: Vite output + WASM + self-hosted fonts (+ base content pack). Versioned by git tag; the content pack hash is shown in the build.
- **Server**: one container image. Env vars, no vendor-specific code (`docs/03-architecture.md` §11).
- **LLM model weights**: not shipped by us; fetched only after player consent from a configured host.

## How a release happens (loose)

1. `make test` green locally, CI green on main.
2. Tag `vX.Y.Z`. CI builds the static site and publishes it.
3. For the server: build image, deploy, glance at the health check and the logs for a few minutes.
4. If you changed rules or content intentionally, the replay corpus was regenerated in the same commit (AGENTS.md rule).

Anyone with repo write access (just David for now) may deploy. No change windows; avoid deploying multiplayer while a big playtest is on.

## Config and secrets

| Name | Purpose | Set in |
| --- | --- | --- |
| `DATABASE_URL` | Postgres for the server | host env / compose file (never committed) |
| `GM_API_KEY`, `GM_BASE_URL` | server-side LLM provider | host env |
| `CONTENTGEN_API_KEY` | offline content generation | developer's local env only |
| `MODEL_HOST_URL` | where players download local model weights from | build-time config |

Never commit values. Missing `DATABASE_URL` = server refuses to start; missing GM key = server runs with the deterministic fallback.

## Rollback

- **Static site**: redeploy the previous tag. Minutes. Player saves are in their browser and are keyed by content-pack hash, so a rollback must not lose saves (verify before the first public release: TBD).
- **Server**: redeploy previous image. DB migrations should be additive so the old image still works; if one isn't, note it in the PR. Last rehearsed: never (do it once before M6 ships).

## Infrastructure

Domain, hosting account and DNS: TBD (branding is an open question). Record owners here once they exist.
