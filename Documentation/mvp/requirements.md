# MVP requirements

Everything here is **must**, unless marked *nice*. Full-game requirements: `../requirements.md` and `../../docs/`.

## Capabilities and features

**Run loop.** Seeded run; days; 3-4 incidents/day; turn costs; end-of-day summary; autosave and resume.

**Helpdesk.**
- Incident card with portrait art (one per employee, calm/agitated), quote, 3-4 options with honest hints.
- Repeat chains: 3-5 escalating variants of the same idiot.
- Rant (F-key or `R`): write and send/delete an unsent email; restores Fucks (delete restores less, costs nothing later; send restores more, risks Suspicion).
- Coffee, and one comfort item *(nice)*.

**Meters.** Remaining Fucks (bar), Suspicion, Stash ($), Nerd Power. Meltdown at Fucks <= 0; fired at Suspicion max.

**Schemes (3).** Costs a turn; gives Stash; adds Suspicion; can spawn an audit incident. Hints show the odds.

**Dirt.** Specific options note "you now have dirt on X". One-time use later in an incident involving X to take a free good option.

**War-room (scripted rivals).** 4 rival bots post from tables on a schedule, react to your boasts with canned one-ups chosen by rules (never an LLM). You can boast about something you did *today*. Verified boasts (matching a real event) earn NP; boasts about schemes add Suspicion. Rank board of you vs the 4.

**Keyboard.** F1 help (costs 1 NP, F2 log is free), F3/F4 desktops, F9 keymap config (rebind + persist), F10 retire. 1-4/Enter/Esc. No mouse. Nothing needs F11/F12.

**Endings.** Meltdown scene (3), fired card, retire scene, GAME OVER card with copyable text summary.

**Score.** Days survived is primary; stash and NP are secondary. Local best list (browser storage). Unverified by nature.

## Explicitly not in the MVP

See `cut/README.md`. Notably: free-text input, LLM, multiplayer, jail, 4 desktops, difficulty tiers.

## Content volumes

| Content | MVP |
| --- | --- |
| Corp | 1 (Grubb Logistics hf.) with ~8 employees + the CEO |
| Incidents | ~40 (t1 15, t2 15, t3 10) plus 3 system incidents (audit, coffee machine, all-hands) |
| Repeat chains | ~8 |
| Rivals | 4 personas, ~12 boast lines each |
| Schemes | 3 |
| Dirt items | ~10 |
| Meltdown / retire scenes | 3 / 1 |
| Art | portraits for employees (calm/agitated), 4 rival avatars, ~5 scene art, logo |

Content is AI-drafted, validated, then curated by a human, like the full pipeline, but with a simple script.
