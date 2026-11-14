# Risk assessment

**Source of truth:** here.

**Last reviewed:** 2026-11-14 (initial draft, David + agent). Re-read at the end of each milestone.

Likelihood / impact: L / M / H. Keep this list short and real.

| # | Risk | L | I | What we do about it | Status |
| --- | --- | --- | --- | --- | --- |
| R1 | Scope creep: the spec is big (multiplayer, LLM GM, 8 rivals, 150 incidents). It never ships. | H | H | Milestones are ordered so M1+M2 (core + web, no LLM) is already a shippable game. Cut M3/M6 before cutting quality. | open |
| R2 | Solo developer / bus factor. | M | H | Everything is written down in `docs/` and `AGENTS.md`; repo is the memory. | accepted |
| R3 | Content looks derived from existing works (esp. "Bastard Operator From Hell"). | M | H | `AGENTS.md` bans names/stories; validator duplicate checks; human curation pass (`curated = true`); content bible bans it. | mitigating |
| R4 | LLM misbehaves: injection, real-world crime how-tos, off-tone text. | M | H | Engine is the referee; LLM picks from a catalog only; schema validation, caps, fallback; 30+ hostile-prompt suite (M3). | planned |
| R5 | Local LLM too slow/large for normal laptops (WebGPU gaps). | H | M | M0 spike decides; "no-LLM default" is an acceptable outcome; GM is opt-in. | open (M0) |
| R6 | Keyboard-only breaks on some browsers/OS (F-keys, Ctrl+arrows eaten by OS/browser). | H | M | M0 key-interception matrix; configurable keymaps; secondary bindings. | open (M0) |
| R7 | Determinism drift (floats, hash-map order, clocks) breaks replays and the leaderboard. | M | H | Integer maths, ordered maps, seeded ChaCha, replay corpus in CI. | planned |
| R8 | Profane title blocks store listing / age rating / domain. | M | M | Web-first on a self-hosted static site; branding is an open question. | open |
| R9 | Multiplayer harassment / abuse in chat. | M | M | Blocklist, mute/report, rate limits; kill switch: turn multiplayer off. | planned |
| R10 | AI content quality is mediocre or samey (150 incidents is a lot of jokes). | H | M | Generate, validate, then human-curate; bot report flags dominated options; ship fewer-but-better if needed. | open |
| R11 | Content/API spend runs away (generation, hosted GM calls). | M | M | Per-player GM call caps; cost ceiling in NFRs; generate in batches, not continuously. | planned |
| R12 | AI-generated content has unclear copyright status. | M | M | Treat as accepted; keep generation prompts + provenance in `tools/contentgen/`; human edits add authorship. Repo license is MPL-2.0. | accepted |

## Accepted risks

- R2 (solo): accepted; it's a hobby project.
- R12 (AI content copyright): accepted until a legal opinion says otherwise.

## Incidents that changed this document

None yet.
