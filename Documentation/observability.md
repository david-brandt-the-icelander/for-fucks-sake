# Observability

**Source of truth:** here. Indie scale: logs and a health check; add more only when something hurts.

## Health

- `ffs-server`: `/healthz` returns OK when the process is up and Postgres is reachable. Poll with a free uptime checker (TBD).
- Single-player: nothing to monitor; a static site. The host's status page is enough.

## Logs

- Server: structured (JSON) logs to stdout; the host or Docker keeps them. Keep ~14 days (TBD).
- Never log: tokens, raw player free text beyond what moderation needs, or LLM API keys. GM payloads must not contain other players' private info.
- Single-player sends no telemetry. If we ever want crash reports or analytics, it's opt-in and a decision in `docs/decisions.md`.

## Metrics

Nothing yet. If MP launches, the few numbers worth watching: connected players, active rooms, command latency, GM call rate and fallback rate, error rate.

## Traces

None.

## Alerts

| Alert | Condition | Pages | Runbook |
| --- | --- | --- | --- |
| MP down | health check fails for 5 min | David (email/push) | `operations.md`: Multiplayer is down |
| GM cost spike | provider bill/usage over the budget (TBD) | David (email) | `operations.md`: GM provider |

No other alerts. An alert needs a runbook or it doesn't exist.

## Balance & quality "observability" (offline)

The closest thing the game has to metrics is `ffs bot`: median days survived, ending mix, option pick rates. That's the balance dashboard (M5 targets), run nightly in CI.

## What is not observed

- Whether players actually have fun (ask playtesters).
- Silent GM quality regressions (spot-check with the real-model nightly from M3).
- Client-side bugs in the field, since single-player phones nothing home (players report them via GitHub issues).
