# Requirements

**Source of truth:** `docs/` for what the game does. This file is an index plus the out-of-scope list and open questions.

## Capabilities (where each is specified)

| Capability | In one line | Spec |
| --- | --- | --- |
| Run loop | ~10-minute turn-based run; days, incidents, meters, meltdown/retire endings | `docs/01-game-design.md` §4-7, §9 |
| Helpdesk | Respond to incidents with 1-4 options or free text; the rant | §6, `docs/02-ui-and-input.md` §5.1 |
| Money & schemes | Cash, investments, liabilities, schemes, suspicion | §6.5, §7.4 |
| Kompromat | Collect, share (copy, never transfer), use as leverage | §8.7 |
| War-room | Chat, boasts, challenges, Nerd Power, titles, rivals, legal warfare | §8 |
| Keyboard-only UI | Terminal grid, 4 desktops, F-keys, configurable keymap | `docs/02-ui-and-input.md` |
| Game master (LLM) | Interpret free text, rank, judge style, narrate; never the referee | §10, `docs/03-architecture.md` §6 |
| Headless play | CLI play, JSONL for agents/bots, balance bot, replay verify | `docs/03-architecture.md` §8 |
| Multiplayer | Rooms of up to 12, server-authoritative, leaderboards | `docs/03-architecture.md` §9 |

Build order and acceptance lists per milestone: `docs/05-milestones.md`. "Done" means that list passes.

## Minimum shippable game ("v0.1")

M1 + M2: core engine, starter content pack, web UI, single-player, no LLM. Everything after is a bonus.

## Out of scope (so nobody builds it twice)

- Images, 3D, emoji glyphs, anything not text/ASCII.
- Mouse-driven UI.
- Meta-progression between runs.
- Physical violence, real-world actionable crime/hacking, real people/companies, anything from existing franchises.
- Accounts with email in v1 (handle + device token only).
- Client-side prediction in multiplayer.

## Open questions

Tracked in `docs/decisions.md` ("Open questions"). Currently: branding/domain, store/age rating,
multiplayer account method, unverified global board, extra languages (an Icelandic pack would be on-brand).
