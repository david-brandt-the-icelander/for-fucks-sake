# Development

**Source of truth:** here, plus `AGENTS.md` for agent rules. The `Makefile` is the executable truth once it exists.

## Prerequisites

- Rust stable + `wasm-pack`
- Node (LTS) + npm
- Playwright browsers (Chromium comes pre-installed in cloud sessions)
- Optional: a WebGPU-capable browser to try the local GM; an LLM API key for content generation

## Getting it running

Pre-code today: only `docs/` exists. Once scaffolded (M1+):

```bash
make dev        # wasm-pack watch + Vite dev server
make test       # cargo test + vitest + Playwright
cargo run -p ffs-cli -- play --seed 42
```

Full command list is in `AGENTS.md`; keep it accurate as the Makefile evolves.

## Project layout

See `docs/03-architecture.md` §2. Short version: `crates/` (Rust), `web/` (TS UI), `content/` (packs, prompts, schemas), `tools/contentgen/`, `e2e/`, `docs/` (game spec), `Documentation/` (this).

## How a change gets in

- Branch, small commits, imperative subject, mention the milestone: `M2: keymap persistence`.
- PR as draft first; mark ready when CI is green.
- If you touched rules or content on purpose, regenerate the replay corpus in the same commit.
- If the spec was silent or wrong, append to `docs/decisions.md` (date, decision, reason).
- If you add a dependency, port, secret, alert or external service, update the relevant file here in the same PR.
- Work milestone by milestone: don't start M(n+1) until M(n)'s acceptance list passes.

## Conventions

The non-negotiables are in `AGENTS.md`: rules only in `ffs-core`, determinism, LLM is never the referee, keyboard only, original content only, text only. Tests accompany every rule and every UI behaviour.

## Common problems

None yet. Add them here when you hit one, delete when fixed.
