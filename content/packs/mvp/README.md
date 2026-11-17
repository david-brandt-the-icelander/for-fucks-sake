# MVP content pack — format

All files are TOML, embedded into the binary by `crates/ffs-content`. Everything is
original: no names, characters, catchphrases or stories from existing works (nothing from
"Bastard Operator From Hell": not the name, not its stories). No real people or companies.
No violence. No actionable real-world crime or hacking instructions. Profanity is fine and
encouraged where it's funny. Tone: dark, cynical, workplace satire; idiocy is behavioural.

## Conventions

- Sign: a **positive** `fucks` means patience restored (good), negative means lost.
- Money is whole dollars (`cash = 1500`). `susp` is suspicion (0-100 scale, positive = worse).
- UI width: option `text` <= 58 chars; `quote` <= 220 chars; `result` <= 110 chars.
- Hints shown to the player are generated from `effects`, so they are always honest.
  Hidden consequences go through `spawn` (a follow-up incident) and `chance` effects.

## `corp.toml`
Employees (`id`, `name`, `role`, `rank` 1-6, `voice`). Use these ids in incidents: arthur,
margaret, richard, dana, agnes, kevin, brynja, tomas, pip (and `ingrid` only in `system` incidents).

## `incidents/*.toml`

```toml
[[incident]]
id = "monitor_off"          # unique, snake_case
title = "MONITOR BROKEN"    # short SHOUTY ticket title, <= 30 chars
tier = 1                    # 1 trivial users, 2 department nonsense, 3 management/corporate absurdity
who = "richard"             # employee id
quote = "..."               # what they say, first person
details = "..."             # optional one-line ticket metadata (attachments, history...)
cause = "Richard ..."       # full sentence, used for "Cause of breakdown: <cause>" if this breaks you
chain = "richard_email"     # optional repeat chain key
step = 0                    # optional 0-based step in the chain; chain steps must be 0,1,2,... contiguous
system = false              # optional; true = never drawn randomly, only spawned (see below)

  [[incident.option]]
  id = "reboot"             # unique within the incident
  text = "Switch it off and on again"
  kind = "professional"     # professional | malicious | profitable | vent | dodge | leverage
  temptation = false        # optional: the satisfying-but-bad option (usually a vent/malicious one)
  minutes = 90              # optional, default is 90
  result = "One line shown after choosing; funny, <= 110 chars."
  story = "..."             # optional: makes this boastable in the war-room, past tense, <= 90 chars,
                            #   written to follow "I " e.g. "locked the CEO out of his own calendar"
  story_value = 2           # 1-3, how impressive the boast is (required with story)
  effects = [
    { fucks = 4 },
    { susp = 3 },
    { cash = 800 },
    { np = 2 },             # nerd power (clever / technically defensible malice)
    { dirt = true },        # you obtain dirt on the incident's `who` (once per available dirt item)
    { spawn = "other_id" }, # a follow-up incident arrives in the queue now (by incident id)
    { chance = 30, then = [ { susp = 20 }, { spawn = "sys_audit_scare" } ] },   # risky part
  ]
```

Each effect table sets ONE thing (or is a `chance`/`then` pair). Option rules:

- 3 or 4 options per incident. At least one `professional` and at least one `malicious` or `vent`.
  From tier 2 up, include a `profitable` or `leverage` option where it makes sense.
- Magnitudes: tier 1 fucks deltas about -4..+5, tier 2 -6..+8, tier 3 -8..+10.
  `professional`: fucks -2..-8, susp 0, sometimes np 0-1.
  `malicious` (technically defensible spite, malicious compliance): fucks +3..+8, susp +2..+8, np 1-3.
  `vent` (just telling them): fucks +6..+12, susp +5..+10, or a `chance` of a big susp/spawn. Mark `temptation = true`.
  `profitable`: cash +300..+4000, susp +3..+12, fucks -1..-4, usually a `chance` of audit spawn.
  `leverage`: `dirt = true` plus fucks +1..+4.
  `dodge`: fucks -1..-3, susp 0, often `spawn` a worse follow-up.
- Chains: 3-5 incidents sharing `chain`, steps 0..n, same `who`, each escalating in absurdity,
  quoted as the same idiot coming back. The game shows "(2nd time)" etc. automatically.
- `system = true` incidents are only reachable via `spawn`. Required ids: `sys_audit_scare` (who margaret),
  `sys_hr_meeting` (who dana), `sys_auditor_visit` (who ingrid). Optional extras are fine.
  Spawn targets must exist. A system incident's options must not `spawn` themselves.

## `schemes.toml` (written by the engine author)
Fields per scheme: `id`, `name`, `desc`, `minutes`, `cash = [lo, hi]` (grows 8%/day), `susp`,
`audit_chance`, `story`, `story_value`, and optionally `min_day` (first day it can run),
`jackpot_chance` (percent), `jackpot_mult = [lo, hi]` (cash multiplier, >= 2), `jackpot_susp`
(extra suspicion on a jackpot) and `jackpot_line` (the toast). A jackpot's story is always worth 3.

## `quips.toml`
`cover` holds the openers for "cover your tracks" toasts (at least 6); the engine appends the numbers.

## `rivals.toml`
Each rival needs at least 20 `idle`, 16 `boast`, 8 of each `react_*` and 6 `mock_help` lines, so a
ten-minute run rarely repeats one. Keep each rival's voice.

## `dirt.toml`, `scenes.toml`, `quips.toml`, `art.toml`
See the headers of each file.
