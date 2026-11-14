# Operations

> **Cut from the MVP** (no server, nothing to run). Kept as the plan for when multiplayer returns. See README.md in this folder.

**Source of truth:** here. Mostly future tense: single-player is a static site (nothing to operate); multiplayer is where operations begin.

## Routine

| Task | Cadence | How | Last done |
| --- | --- | --- | --- |
| Check multiplayer is up and the DB is backing up | weekly once MP is live | health endpoint + backup list | n/a |
| Restore a DB backup to prove it works | before MP launch, then every few months | restore to a scratch DB | never |
| Review chat reports / mutes | when notified, or weekly during playtests | `reports` table | n/a |
| Dependency updates (Rust, npm) | monthly-ish, or when something is vulnerable | `cargo update`, `npm update`, run `make test` | n/a |
| Cost glance (hosting, LLM API) | monthly | provider dashboards | n/a |
| Rotate API keys | when leaked, or yearly | host env | n/a |

## Runbooks (stubs; fill in when they happen)

### Multiplayer is down
Check the container is running, check Postgres, check logs. Restart. If it's the DB: restore the last backup. Single-player is unaffected, so there's no panic.

### GM provider is failing or too expensive
Unset the GM key or lower the per-player GM cap; the server falls back to the deterministic interpreter. Games continue.

### Abuse in chat
Mute/remove the player, add to the blocklist, note it here. If it keeps happening: turn off public rooms.

### Bad release
Roll back per `deployment.md`.

## Incident handling

It's a game. Severity is "annoying" or "multiplayer is down". David is the on-call and nobody is paged at 3am. Tell players via the site banner / repo README if something is broken for more than a day. Write a few lines in `risk-assessment.md` ("Incidents that changed this document") if something teaches us a lesson. No formal post-incident reviews.

## Access

Production access: David only. Agents never get prod credentials; they work in repo and CI.

## Capacity and cost

Unknown until M6 load test (target: 50 rooms x 12 players on one modest instance). Set a monthly ceiling before launch (TBD).

## Known issues

None yet.
