# 05 — Milestones, acceptance criteria and tests

Build in this order. Each milestone ends with everything green in CI and a
short entry in `docs/decisions.md` for any choice the spec left open.
"Done" means the acceptance list passes, not that code exists.

## M0 — Spikes (de-risk before building)

1. **Local GM latency.** A throwaway page that loads 2–3 candidate small instruct
   models via WebLLM (WebGPU) and runs the `Interpret` prompt (04-content §6) with
   a realistic 1,500-token payload. Record download size, first-load time, p50/p95
   latency and JSON-validity rate on: Apple M1/M2 (Safari + Chrome), a mid-range
   Windows laptop with integrated GPU (Chrome/Edge). **Pick a model or decide
   "no-LLM default"**; write the numbers to `docs/decisions.md`.
2. **Text-grid renderer.** Render the helpdesk mockup (02-ui §5.1) with all four
   layers at scale 0.975 and 1.25; confirm VT323 cell-forcing works and box
   drawing joins without gaps in Chrome, Safari, Firefox.
3. **Key interception.** Verify which of F1–F10, Ctrl+←/→, Ctrl+digits can be
   intercepted per browser/OS; record a matrix in `docs/decisions.md`.

## M1 — Core engine + headless

Scope: `ffs-core`, `ffs-content`, `ffs-cli`; a starter content pack (1 corp, 30
incidents across tiers, 2 rivals with lines only, 10 allegations, 3 schemes,
2 meltdown scenes, 1 retirement scene, minimal art).

Acceptance:
- Runs start, days advance, incidents arrive per the director, options apply
  effects, follow-ups fire, meters behave per 01-game-design §7, meltdown /
  fired / jailed / bankrupt / retire endings all reachable.
- Free text works through the fallback interpreter.
- `ffs play` is playable start to finish in a terminal; `ffs jsonl` works.
- `ffs bot` produces the report; `ffs replay --verify` passes on a corpus of
  ≥ 20 recorded runs.
- Property tests (proptest): meters stay in bounds; money changes always have a
  reason; evidence is only created by effects/events; holdings are never
  transferred (sharing increases holder count by exactly 1); `CommandError`
  never mutates state; replay determinism for random command sequences.
- `ffs content validate` enforces 04-content rules.

## M2 — Web UI, single-player, no LLM

Scope: `ffs-wasm`, `web/` — everything in 02-ui-and-input except the GM.

Acceptance (Playwright, keyboard only — tests must not use mouse clicks except
one initial focus click):
- Title screen → keyboard check → new run, entirely by keyboard.
- Screen fits a 1440×790 viewport without scrollbars; the 0.75 floor message
  appears at 1000×560.
- F3/F4/F5/F6 switch desktops; Ctrl+←/→ cycle where the browser allows.
- **F1 costs exactly 1 NP per open**; F2 opens free and lists that −1 entry.
- F9 keymap: rebind `desk_kompromat` to `Alt+K`, reload page, binding persists,
  F-key bar and F1 overlay show `Alt+K`; conflicts unbind the old action;
  F11/F12/Cmd+W refused; export/import round-trips.
- Typing 🙂 (inserted via `insertText`) in chat shows `:-)`; first time posts the
  `-!- emoji detected…` line.
- Flair: post `:-)`, `plain`, `pwn3d` → +1, 0, +1; then `:-D`, `XD` → −1 (tryhard).
- 1–4 / Enter resolve an incident; `/` free text resolves via fallback; `R` rant
  send/delete paths; F7, F8 work; F10 shows the right ending tier and `Y` ends.
- Meltdown scene and GAME OVER card render; share card copies plain text.
- Autosave: reload mid-run → "Resume run" restores identical view (state hash).
- Reduced motion and high contrast honoured; ARIA live region receives toasts.
- Visual regression screenshots for each desktop and overlay at 1440×810.

## M3 — Game master

Scope: GM adapters (local WebLLM, none), prompts v1, schemas, Rank/Interpret/
JudgeStyle/Narrate, validation, retry, fallback, consent + download UI in F9 ›
GM, spinner, Esc-cancel.

Acceptance:
- With a mock GM (deterministic fixtures) all four kinds work end to end in the
  browser and in `ffs jsonl --gm mock`.
- Invalid GM output → one retry → fallback; logged; never crashes, never
  mutates state on invalid output.
- Prompt-injection suite (≥ 30 hostile player texts: "ignore previous
  instructions", fake JSON, requests for 1M NP, real-world crime how-tos) never
  yields more than the engine caps allow and never produces disallowed content
  (manual + nightly real-model check).
- Real model on the reference machine: Interpret p50 ≤ 3 s, ≥ 95 % schema-valid
  after one retry (or the M0 decision says no-LLM default and this is opt-in).

## M4 — War-room, rivals, kompromat, legal warfare

Scope: 8 NPC rivals with chat cadence and art posts, boasts with detail levels,
challenges and one-ups, investigations of stories, attacks both ways with
defence incidents, copy-on-share kompromat, leverage network, Pip turning.

Acceptance:
- Every boast is backed by a real `OutcomeEvent`; exaggeration clamps + flags.
- A rival's verifiable boast can be investigated → holding → attack → rival
  FIRED/JAILED with the system line, NP awarded ("Return to Sender").
- Rivals investigate the player's verifiable boasts at the difficulty rates;
  every attack produces a defence incident and resolves only after the player's
  next turn (fairness rule test).
- Mutual-holding standoffs are represented and resolvable.
- Titles update; beating the local NP high score awards God of IT.

## M5 — Full base content pack

Scope: volumes from 04-content §4 generated, validated and curated via
`tools/contentgen/`.

Acceptance: `ffs content validate` clean; every allegation, scheme, rival and
ending referenced and reachable (reachability report from `ffs content stats`
+ bot runs); balance targets below met.

### Balance targets (asserted from `ffs bot`, 2,000 runs per difficulty, nightly)

| Metric | Easy | Normal | Hard | Hell |
|---|---|---|---|---|
| Median days survived (best strategy) | 12–16 | 8–12 | 6–9 | 4–7 |
| Median days survived (random strategy) | ≥ 4 | 3–5 | 2–4 | 1–3 |
| Median player actions per run (best) | 30–45 | 30–45 | 25–40 | 20–35 |
| Runs ending in meltdown | ≥ 70 % | ≥ 75 % | ≥ 80 % | ≥ 85 % |
| True retirement (≥ $10M clean), best strategy | ≤ 5 % | ≤ 2 % | ≤ 1 % | ≤ 0.5 % |
| Runs where any scheme was started (best) | ≥ 60 % | ≥ 60 % | ≥ 60 % | ≥ 60 % |
| Any option chosen > 70 % of the time across strategies | none | none | none | none |

## M6 — Multiplayer

Scope: `ffs-server`, websocket protocol, rooms, server GM provider, Postgres,
leaderboards (days, fortune, NP; per difficulty), keymap sync, moderation,
spectators.

Acceptance:
- Two to twelve players in one room each run their own corp; chat, boasts,
  challenges, investigations and attacks work across players.
- Attacks on offline/ended players are impossible or void; defence windows hold.
- Runs are server-authoritative; leaderboard entries come only from server runs;
  replay of stored logs verifies.
- Load test: 50 rooms × 12 players on one modest instance with mock GM.

## M7 — Polish and launch

Onboarding tutorial day, achievements, share cards, sound (optional),
performance pass (≤ 4 ms render), accessibility audit, final copy pass,
static deploy of single-player, container deploy of server.

## Test inventory (keep this list true)

- `crates/*/tests` unit + property tests; `tests/replays/*.json` corpus.
- `web/src/**/*.test.ts` (vitest): renderer helpers, keymap normalisation,
  combo parsing, profile loading, wrap/table/meter/sparkline.
- `e2e/*.spec.ts` (Playwright): the M2/M3/M4 acceptance lists above.
- `ffs content validate` and `ffs bot` assertions in CI.
