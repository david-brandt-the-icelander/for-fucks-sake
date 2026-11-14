# Business plan

**Source of truth:** here. (Indie: this is a one-pager, not a pitch deck.)

## Purpose

FOR FUCK'S SAKE! (FFS!) is a ten-minute, keyboard-only, green-terminal dark comedy about
surviving as a sysadmin. It exists because David wants to make it and because the target
audience (sysadmins, devs, people who've survived IT) will find it funny. It is a passion
project first and a product second.

## Who pays, and what they get

- **Budget owner:** David, out of pocket and spare time.
- **Costs today:** ~nothing. Repo hosting, a laptop, AI tooling subscriptions (TBD: monthly).
- **Costs later:** content generation API spend (TBD per milestone M5), static hosting for
  single-player (≈ free tier), and a small VPS + Postgres for multiplayer (TBD, M6).
- **Return:** fun, portfolio, community goodwill. Revenue is a bonus, not the plan.
  Possible later: pay-what-you-want / tip jar, Steam/itch listing. Undecided; see open
  questions in `docs/decisions.md` (branding, store/age rating for the profane name).

## Who uses it

Intended: sysadmins, developers, and IT-adjacent people who like dark workplace satire and
terminals. Expected numbers: unknown, probably small. Nobody uses it yet (pre-code).

## Alternatives considered

- Do nothing / just keep it as a spec: cheaper, but less fun.
- Make it a conventional GUI game: rejected, the terminal look is the whole identity.
- Roguelike with meta-progression: rejected, runs are independent by design (`docs/decisions.md` #6).

## Shutdown criteria

Stop (or freeze at a playable state) if any of these happen:

- Content generation or hosting costs exceed what David is willing to pay with no sign of
  players (set a number once M5/M6 budgets are known: TBD).
- A rights/legal problem we can't cheaply fix (e.g. a content similarity complaint).
- Multiplayer moderation becomes a chore nobody wants: switch multiplayer off, leave
  single-player as a static site, which costs nothing to keep alive.

Single-player is built as a static site specifically so the game can outlive everything else.

## History

- 2026-11: Spec v0.5 written and merged (PR #1). Design decisions 1-17 settled.
- 2026-11: Project documentation drafted (this folder).
