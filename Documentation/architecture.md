# Architecture

**Source of truth:** `docs/03-architecture.md` (full spec). This file is the short version plus the decisions people will ask "why?" about.

## In one paragraph

A Rust engine (`ffs-core`) holds *all* game rules and compiles to WASM for the browser and native for the server and CLI. A thin TypeScript UI renders a text-grid view model and sends commands; it contains no rules or randomness. An optional LLM "game master" can interpret free text and narrate, but only by picking from an engine-provided catalog. State is a deterministic function of seed + command log, so runs can be replayed and verified.

## Context

```
 browser: web/ (TS) -- ffs-wasm -- ffs-core        server: ffs-server (axum) -- ffs-core, Postgres
 headless: ffs-cli -- ffs-core                      content/ packs (AI-generated offline, curated)
```

| External thing | Direction | Breaks if it's down |
| --- | --- | --- |
| Local LLM (WebLLM/WebGPU) | browser -> model | GM falls back to deterministic interpreter |
| Server LLM (HTTP provider, MP only) | server -> provider | same fallback |
| Postgres (MP only) | server -> db | MP down; single-player unaffected |
| Content-generation LLM API | offline tooling only | can't make new content; shipped game fine |

## Components

`ffs-core` (rules) · `ffs-content` (schemas, load, validate) · `ffs-wasm` (browser facade) · `ffs-cli` (play, JSONL, bot, replay) · `ffs-server` (rooms, websockets) · `web/` (renderer, keymap, overlays, GM adapters) · `tools/contentgen/` (offline generation + curation). Layout in `docs/03-architecture.md` §2.

## Data

- Run state lives in the engine; saved as seed + command log (autosave in browser storage).
- Single-player: browser storage only.
- Multiplayer: Postgres (`players`, `runs`, `leaderboard`, `chat`, `reports`). Personal data: handle only.
- Content packs: TOML + plain-text art, versioned in `content/packs/`.

## Key decisions

### 2026-11: Rust core, shared by every host
**Why.** One rules implementation = no drift between browser, server, CLI. Enables replay verification and a headless bot for balance.
**Alternatives.** TS everywhere (simpler, but no deterministic native server/bot story).
**Consequence.** WASM build step in the loop; types generated from Rust.

### 2026-11: The LLM is never the referee
**Why.** Safety, fairness and testability. LLM scores style/plausibility, ranks, narrates; engine validates and caps everything.
**Consequence.** Always need a deterministic fallback, and no-LLM must be fully playable.

### 2026-11: Content generated offline, curated, then shipped
**Why.** Cost, quality control, and no runtime surprises.
**Consequence.** Needs the `curated` flag workflow and a validator.

### 2026-11: Keyboard-only, text-only
**Why.** Identity of the game; cheap to render; accessible by construction.

More: `docs/decisions.md`.

## Known debt

None yet; nothing is built. Debt discovered during implementation goes in `docs/decisions.md` with a date.
