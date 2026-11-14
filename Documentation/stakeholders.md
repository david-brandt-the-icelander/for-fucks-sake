# Stakeholders

**Source of truth:** here.

Small team, so most roles are one person wearing several hats. Names go in `contacts.md`.

| Role | Who | Wants | Decides on |
| --- | --- | --- | --- |
| Owner / designer / product | David (`david-brandt-the-icelander`) | A funny, finished game | Everything: scope, tone, shutdown |
| Implementation agents (Claude Code etc.) | AI agents following `AGENTS.md` | Unambiguous spec | Small gaps, logged in `docs/decisions.md` |
| Players (single-player) | Sysadmins, devs, terminal fans | A fun 10-minute run | Nothing directly |
| Players (multiplayer) | Same, in rooms of up to 12 | Fair play, banter, no harassment | Nothing directly |
| Playtesters | Friends / early players, TBD | To tell us what's broken | Nothing, but their feedback can change balance |
| Hosts / platforms | Static host, VPS provider, possibly itch/Steam | Terms-of-service compliance | Whether we're allowed to be listed |
| LLM / content tool providers | API and model vendors | Usage within terms | Pricing, availability |

## Who can say no

- **David**: anything.
- **Platform reviewers**: a profane title or content could block a store listing.
- **Law/rights holders**: anyone who credibly says we copied their work (see risk R3).

## Disagreements on record

None yet. When David and an agent disagree, David wins and the call goes in `docs/decisions.md`.
