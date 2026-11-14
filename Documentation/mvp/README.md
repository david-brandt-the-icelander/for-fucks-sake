# MVP

**Status:** proposal. The full spec in `../../docs/` is unchanged and still the long-term vision.
This folder is the cut-down game to build *first*. If we adopt it, add one line to
`docs/decisions.md` ("build the MVP in `Documentation/mvp/` before M3+") so the spec and plan agree.

**Rule of thumb:** if a cut makes the game unplayable or unfunny, un-cut it. Otherwise it stays cut.

## The MVP in one paragraph

A single-player, ten-minute, keyboard-only, green-terminal game in the browser. You are a sysadmin at
one dysfunctional company. Each turn an idiot sends you an incident with 3-4 options. Bad options
are tempting; venting empties your **Remaining Fucks**; at zero you melt down in a comic scene. On
the side you skim money with a few schemes, hoard a little dirt on coworkers, and brag in a chat
room of four **scripted rival sysadmins** to earn **Nerd Power**. Score = days survived, with
Nerd Power and stash as flavour. Hit F10 with enough stash and you retire legendary.

## The loop (this is the whole game)

1. **Day starts.** Fucks erode a little. 3-4 incidents queue up for the day.
2. **Resolve incidents** (1-4 to pick, Enter to confirm). Each option has visible hints and real effects on
   Fucks, Suspicion, Stash, and sometimes Dirt on someone.
3. **Optional side actions** (each costs a turn): rant (unsent email, restores Fucks), coffee (restores
   Fucks, costs time), a scheme (Stash up, Suspicion up), brag in the war-room.
4. **War-room.** Rivals post scripted boasts. You can one-up with a story from something you really did
   today (Nerd Power up). Overuse of the same story, or boasting about a scheme, bumps Suspicion.
5. **Day ends.** Summary quip, scores, next day.
6. **Run ends:** meltdown (normal), fired (Suspicion maxed), bankrupt-less retire (F10 with $ stash).

## What's in

| Area | MVP version |
| --- | --- |
| Engine | Rust core (`ffs-core`) + WASM, deterministic, seeded. Same shape as the full spec, much smaller. |
| UI | Thin TS, text grid, **2 desktops**: Helpdesk (F3) and War-room (F4). F1 help, F9 keymap, F10 retire. Keyboard only. |
| Meters | Remaining Fucks (visible), Suspicion, Stash ($). Hidden stress/meltdown rules simplified to: Fucks <= 0 triggers meltdown. |
| Incidents | 1 corp, ~40 incidents in 3 tiers, ~8 repeat chains, 3-4 options each, hints honest. |
| Schemes | 3 (simple: pay cost in time, gain Stash, gain Suspicion, some risk of audit incident). |
| Dirt | A tiny version of kompromat: some options give you "dirt" on a named coworker; spend it once to auto-win a later incident involving them. No sharing, no rival interaction. |
| Rivals | 4 scripted bots with personalities. They post boasts on a schedule from a content table; Nerd Power rank vs them is the leaderboard. |
| Nerd Power | Earned by clever options, bragging, F2 log. F1 still costs 1 NP (keep the joke). |
| Endings | 3 meltdown scenes, 1 fired, 1 retire scene, GAME OVER card with share text. |
| Persistence | Autosave + local best scores in browser storage. |
| Ship | Static site. No server. No accounts. Nothing phones home. |

## What's out (details in `cut/README.md`)

Multiplayer and the server; LLM game master and free-text input; jail and legal warfare; kompromat
sharing and rival investigations; 4 desktops (two of them); difficulty levels beyond one tuned Normal
(maybe a "Hard" toggle that's just numbers); achievements; sound; multiple corps; Pip the intern
and the auditor as systems; emoji/flair/leet rules; content generation pipeline beyond a script and
a hand-curation pass; full balance harness.

## Milestones

| # | Goal | Done when |
| --- | --- | --- |
| MV0 | Spike: text-grid renderer + key capture on Chrome/Safari/Firefox | Helpdesk mockup renders; F-keys, 1-4, Enter reliably captured; fallback bindings noted |
| MV1 | Core + tiny headless CLI with starter content (1 corp, ~15 incidents, 2 rivals) | A full run plays in a terminal; meltdown, fired and retire all reachable; replay hash verifies |
| MV2 | Web UI, 2 desktops, autosave | Playable start to finish by keyboard only; Playwright covers the happy path |
| MV3 | Full MVP content (~40 incidents, 4 rivals, 3 schemes, scenes) + small bot for balance | Playtest checklist below passes; static deploy |

Estimates deliberately not given; each is "as long as it takes to be fun". Don't start MV(n+1) until MV(n) is done.

## Fun test (the actual acceptance criteria)

Hand it to 3-5 people, no explanation beyond "F1 shows keys":

- [ ] They finish a run in 8-15 minutes.
- [ ] They laugh at least once at an incident or a meltdown scene.
- [ ] At least one person says "one more run" and the second run is different (different incidents/chains).
- [ ] Someone tries a scheme and gets caught or gets away with it, and cares.
- [ ] Nobody gets stuck not knowing what to press.
- [ ] A lazy bot (always option 1) and a clever bot (best-hint) land in clearly different day counts.

## Ground rules that do NOT get cut

From `AGENTS.md`: rules only in the Rust core, determinism, keyboard-only, original content only
(no existing franchises' names, characters or stories), no violence, no actionable real-world crime,
text-only. These are cheap to keep now and expensive to retrofit.

## Open questions for David

1. Keep Rust+WASM for the MVP, or go **TypeScript-only** to cut the toolchain? Recommendation: keep Rust. It's
   the settled decision, the core is small, and determinism/replays are what let a bot balance the game.
2. Is 2 desktops right, or a single split screen (incidents left, chat right)? Single screen is simpler to render and
   to learn; two desktops keeps the F-key identity.
3. Is the "dirt" mini-system worth it, or cut entirely and add in v2? It's the main thing that keeps the game from being
   "just pick the least bad option".
4. Branding/domain is still open (`docs/decisions.md`).
