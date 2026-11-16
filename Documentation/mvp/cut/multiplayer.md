# Multiplayer (cut from MVP)

Authoritative spec: `docs/03-architecture.md` §9 and `docs/05-milestones.md` M6.

**What it was:** rooms of up to 12 players, each with their own corporation and server-side engine; shared war-room chat,
boasts, challenges, cross-player kompromat and attacks; Postgres for players, runs, chat and leaderboards; server GM;
moderation (blocklist, mute/report, rate limits); fairness rule (incoming attacks resolve only after the target's next turn).

**Why cut:** needs a server, database, moderation, abuse handling, hosting spend and an on-call. None of it makes the core
loop more fun until the core loop is fun.

**What the MVP keeps so it's easy to add later:** the engine already has a "rival agent" interface; scripted bots implement it.
Later, a networked player is just another implementation. Commands/events/view are JSON and the engine is deterministic and
replayable, which is the same foundation the server needs.

**Revive checklist:** single-player is shipped and liked; decide accounts (`docs/decisions.md` open question); add
`SECURITY.md`; do `operations.md` and `observability.md` for real; set a monthly cost ceiling.
