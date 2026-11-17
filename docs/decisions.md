# Decisions log

Append-only. Newest at the bottom. Each entry: date, decision, why, and what it
replaces. Implementation agents add entries whenever the spec is silent or
proves wrong.

## Settled by the designer (David), Nov 2026

1. **Premise:** sysadmin survival comedy in the spirit of classic sysadmin satire,
   with entirely original names, characters and stories.
2. **Time model:** turn-based.
3. **Stress model:** patience visible graphically (Remaining Fucks meter).
4. **Economy:** simple: cash + investments + liabilities, schemes.
5. **Content:** all incidents, characters and art generated in advance by AI and
   curated; the runtime LLM only interprets, ranks, judges and narrates.
6. **Run length:** ~10 minutes. Runs are completely independent.
7. **Meltdown is (nearly) inevitable;** score is how long you last. Retirement
   is possible but legendary, and ends in a grand scene (e.g. manure delivered to
   the ex-boss's lawn).
8. **Multiplayer:** players run different corporations and interact through the
   war-room (boasts, one-ups, challenges, legal warfare).
9. **Nerd Power:** leaderboard + chat room; war stories; one-upping; beating the
   NP high score is a victory condition with titles (Lord of Nerds, God of IT…).
10. **Legal warfare:** rivals can be jailed using their own war stories, and they
    try the same; difficulty sets how well that works.
11. **Kompromat:** inventory about employees and rivals; can be **shared
    (copied), never transferred**.
12. **Tech:** shared engine in Rust compiled to WASM (browser) and native
    (server); thin TypeScript + HTML UI; headless playable for testing; browser
    LLM in single-player, server LLM in multiplayer.
13. **Look:** green-on-black terminal; ASCII art only; mixed text resolutions on
    screen allowed; no 3D; no images.
14. **Input:** keyboard only; F-keys and shortcuts; F1 shows shortcuts and costs
    a nerd point; F2 shows the nerd-point log for free; keymaps are configurable.
15. **Chat flair:** rivals send ASCII art and text emoticons; emoji-picker emoji
    are translated to text; occasional emoticons/1337 earn a nerd point, overuse
    costs one.
16. **Screen:** must fit a 15" laptop; multiple desktops.
17. **Desktop switching on F-keys** (F3–F6) rather than arrow combos; Ctrl+←/→
    kept only as a secondary binding (macOS reserves it for Spaces).
    *Interpretation of "the FN keys instead of arrows is actually better" — confirm.*

## Decided in the spec (v0.5) — change freely if playtesting disagrees

- Grid 160×45 at 9×18 px (1440×810); four text layers (0.5×, 1×, 2×, 3×).
- Four desktops: Helpdesk, War-room, Kompromat, Corp & Ledger.
- F-key map: F1 help, F2 NP log, F3–F6 desktops, F7 intern, F8 coffee,
  F9 config, F10 take the money and run. F11/F12 left to the browser.
- Retirement goal $10M; titles and NP thresholds per 01-game-design §8.6.
- Flair rule: window of previous 4 messages; ≤ 1 flair → +1, ≥ 2 → −1; +3/day cap.
- Content format TOML + plain-text art.

## Open questions (ask David before resolving if possible)

- Domain and branding (forfucksake.* ?), store/age rating approach for the
  profane name.
- Accounts for multiplayer beyond handle + device token (passkeys? GitHub?).
- Whether single-player runs may be submitted to a separate "unverified" global
  board.
- Language(s) beyond English (an Icelandic pack would be on-brand).

## MVP adopted (2026-11-15)

Build the cut-down game described in `Documentation/mvp/` before M3+ of the full plan.
The full spec in `docs/` stays the long-term vision; `Documentation/mvp/cut/README.md` lists what was
deferred and when to bring it back.

MVP implementation decisions (all easy to change):

- **Crates:** `ffs-content`, `ffs-core`, `ffs-wasm`, `ffs-cli`. No server, no GM. Content is TOML embedded in the
  binary (`content/packs/mvp/`, format in its README).
- **Clock:** 480-minute day (09:00-17:00). Incident 120 min, rant/coffee/boast/dirt 60, schemes 60-120. Tuned by `ffs bot`.
- **Pressure:** per-day erosion is `3 + max(day-2,0) + day^2/4` fucks; suspicion decays 8/day; unresolved tickets cost
  2 fucks each overnight (cap 8). Meltdown at fucks <= 0, fired at suspicion 100.
- **Retire goal:** $25,000 clean cash with suspicion < 60 (the spec's $10M does not fit MVP scale).
- **Dirt, stories, boasts:** options may carry a `story` (boastable); schemes always create one. Boast value vs rival
  `ego` decides reaction and NP (v > ego: +2v, v == ego: +v, v < ego: +0 and the rival gains 1). Risky stories cost +6
  suspicion when bragged. The 3rd boast of a day is worth 1 less. Dirt on a requester is a free one-shot win (+10 fucks, +2 NP).
- **Rivals** are scripted: NP grows by a per-day range and by their posted boasts; they never interact beyond chat.
- **NP floors at 0** but the NP log still records a -1 for each F1 open (so the log proves it).
- **Rant text** is never part of the command log or sent anywhere; the engine only sees `send: true|false`.
- **Save format:** `{seed, log, hash}` in `localStorage`; a hash mismatch (new game version) discards the save.
- **WASM seed** is a `u32` (JS-friendly); the CLI takes `u64`.
- **UI:** one 160x45 character grid rendered into a `<pre>`; 2 desktops (F3 Helpdesk, F4 War-room); one binding per action
  (the spec's second slot is cut); the game renders at 9x18 px cells and scales to fit; below 0.75 scale it shows a size message.
- **TS types** are generated from Rust by `ts-rs` into `web/src/gen/` (`cargo test` regenerates; CI checks they are committed).
- **E2E tests** live in `web/e2e/` (not top-level `e2e/`) so they resolve the web package's Playwright.
- **Balance targets (MVP, asserted by `ffs bot --assert`):** lazy bot median <= 6 days, clever bot median 8-16 days and
  >= 70% meltdowns (replaced 2026-11-17 by "retires 25-65%", see below), random bot essentially never retires. Humans land between the lazy and clever bots.

## Bigger schemes, cooling off, chattier rivals (2026-11-17)

David: retirement was out of reach, Suspicion only ever went up in practice, and the rivals repeated
themselves. Before: the clever bot never retired (max $20.7k against the $25k goal) and died around day 14.

- **Jackpots:** each scheme has a small `jackpot_chance` (6-10%) that multiplies its cash (x2 to x8 depending
  on the scheme) and adds a little extra Suspicion. Shown as a chip ("8% x3-5") so the hints stay honest.
- **Decommission sale:** a fourth, big scheme from day 4: $2,500-6,000, +18 Suspicion, 35% audit, 180 minutes.
- **Cover your tracks** (`Command::CoverTracks`, key T, CLI `t`): 120 minutes and -4 fucks for -6 Suspicion,
  then -3, then -1 the same day (resets daily). Toast openers live in `quips.toml` (`cover`). The overnight
  cool-down (-8/day, unchanged) is now shown in the day-end toast.
- **Rivals:** each has 25 idle lines, 20 boasts, 10 of each reaction and 8 help-mocks; the validator enforces
  20/16/8/6 minimums.
- **Balance targets changed:** clever bot retires 25-65% of runs (measured: about 45%, median 9 days); the
  ">= 70% meltdowns" target is replaced. Lazy <= 6 days and random never retiring are unchanged. Without the
  cover action the clever bot retires 0.2% of the time, so cooling off is what makes retirement reachable.

## Schemes are a one-use ladder (2026-11-17)

David: more schemes; each available once, replaced by a new one when taken; escalating gain and risk.

- 21 schemes in 7 tiers (`tier` in `schemes.toml`), from petty (tier 1: $200-700, +2-3 susp, 5% audit, 60 min)
  to legendary (tier 7: $6,000-11,000, +25-27 susp, 35% audit, 240 min). Tier 5 and up summon the external
  auditor (`audit = "sys_auditor_visit"`) instead of the CFO's audit scare.
- Each run shuffles the schemes within each tier (engine RNG, so replays hold) and stacks the tiers in order.
  Three are on offer (`SCHEME_SLOTS`); the taken one is replaced in its slot by the next on the ladder.
  Each scheme runs once per run. This replaces "each scheme once per day", `min_day` and the 8%/day cash
  growth: the ladder is now what makes schemes grow.
- Errors: `SchemeUsedToday` and `SchemeLocked` are replaced by `SchemeNotOffered`.
- Balance after retuning susp per tier: clever bot retires about 42% (median 8-9 days); lazy and random
  unchanged (random never retires).

## Web host: GitHub Pages (2026-11-17)

David asked for a publishing step and a host. GitHub Pages: the repository is public, it costs nothing, and it needs
no account, API token or secret (the workflow deploys with its own short-lived token). A `v*` tag push runs
`.github/workflows/release.yml`, which re-runs `cargo test` and the web unit tests before publishing, and stamps the
tag into the title bar (`FFS_VERSION`, "dev" for local builds). Manual runs can republish or roll back to any tag.
A custom domain can be added later without changing the workflow.
