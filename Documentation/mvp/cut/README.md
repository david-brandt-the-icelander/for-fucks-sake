# What the MVP cuts

Nothing here is rejected; it's *deferred*. To bring something back: write down why in this file, add a line to
`docs/decisions.md`, and do it after the MVP ships.

| Cut | Spec | MVP replacement | Bring back when |
| --- | --- | --- | --- |
| Multiplayer, rooms, server, Postgres, leaderboards, moderation | `docs/03` §9, M6 | 4 scripted rival bots; local best list | People ask to play together, and the single-player game is good |
| LLM game master (interpret/rank/judge/narrate), consent + model download, prompts, injection suite | `docs/01` §10, `docs/03` §6, M0.1, M3 | none; fixed options only | Free text feels like the missing feature, and the M0 latency spike is positive. See `llm-game-master.md` |
| Free-text input and fallback interpreter | `docs/03` §6.5 | 3-4 fixed options (plus the rant text box, which is never interpreted) | With the GM |
| Jail, legal warfare, rival investigations, attacks, defence incidents, standoffs | `docs/01` §8.8, M4 | Suspicion meter -> "fired" ending | v2, if dirt + schemes aren't enough depth |
| Full kompromat (inventory, sharing/copying, leverage network) | `docs/01` §8.7 | "Dirt": one-shot use against a named coworker | v2 |
| Desktops 3 and 4 (Kompromat, Corp & Ledger) | `docs/02` §5.3-5.4 | Stash/Dirt in status bar and a small overlay | When the status bar can't hold it |
| Difficulty tiers (Easy/Normal/Hard/Hell) | `docs/01` §11 | One tuned Normal; maybe a "hard" toggle that is just numbers | After balance is stable |
| 8 rivals, challenges, titles ladder, flair/emoji/leet rules | `docs/01` §8.4-8.6, `docs/04` §3.9-3.10 | 4 rivals, simple rank board | v2 |
| Pip the intern and the auditor as systems, F7 intern | `docs/01` §6.6 | A normal employee and a system incident | v2 |
| Achievements | `docs/01` §13 | none | When there's a share/bragging loop |
| Hidden stress model, last-straw rules | `docs/01` §7.2-7.3 | Fucks <= 0 -> meltdown | If runs feel too deterministic |
| Multiple corps (municipal, startup) | `docs/04` §4 | 1 corp | After content pipeline is solid |
| Volume targets (150 incidents, 25 chains, 40 allegations, 12 system) | `docs/04` §4 | ~40 incidents, ~8 chains | M5-style content push |
| Full balance harness and nightly 2,000-run assertions | `docs/05` M5 | Small bot, a few hundred runs, two-strategy comparison | Content > 100 incidents |
| Cross-browser CI matrix, perf budget, accessibility audit, sound, onboarding tutorial day | `docs/05` M7 | Chromium only; a one-screen intro | Before wide launch |
| Server GM, keymap sync, spectators, reports | `docs/03` §9 | n/a | With multiplayer |

## Documents cut or parked

| File | Why | Where |
| --- | --- | --- |
| `operations.md` | No server, nothing to operate | `operations.md` (this folder) |
| `observability.md` | No server; no telemetry by design | `observability.md` (this folder) |
| Multiplayer plan | Cut | `multiplayer.md` |
| LLM GM plan | Cut | `llm-game-master.md` |
| Risks R4, R5, R8, R9, R11 and similar | Cut with their features | `risks-parked.md` |

The other full-game docs in `Documentation/` (business plan, stakeholders, contacts, architecture, ...) still apply as-is
and are not duplicated here.
