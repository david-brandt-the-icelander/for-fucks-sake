# MVP risks

Full list: `../risk-assessment.md`. Most big risks (LLM, multiplayer abuse, hosting cost) disappear with the cuts.

| # | Risk | What we do |
| --- | --- | --- |
| M1 | It isn't fun with scripted rivals and no free text. | Fun test early (after MV1 in the CLI, with placeholder text). Fix the loop before art or polish. |
| M2 | Rival bots feel samey or obviously canned. | Few rivals, strong personalities, weighted random lines with cooldowns; accept some repetition in a ten-minute game. |
| M3 | 40 incidents is too few; runs feel repetitive. | Repeat chains and escalation make variety; tune from bot pick-rate data; add content before features. |
| M4 | Content accidentally resembles existing works. | Keep AGENTS.md ban; curate every line; no names from the genre. |
| M5 | Key capture fails on some browsers/OS. | MV0 spike; every binding rebindable. |
| M6 | Scope creep back toward the full spec. | `cut/README.md` is the bouncer: anything cut returns only via a note there and a decision in `docs/decisions.md`. |
| M7 | Solo dev burnout. | Short milestones, ship the static site early, playtest with friends. |

Parked risks (LLM, multiplayer, hosting spend, store rating) are in `cut/risks-parked.md`.
