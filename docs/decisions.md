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
