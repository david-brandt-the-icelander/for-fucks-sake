# CI/CD

**Source of truth:** `.github/workflows/` once it exists (not yet). The target pipeline is in `docs/03-architecture.md` §11. This file says what it should do and what it deliberately doesn't.

## On every pull request (target)

| Stage | Command | Fails when |
| --- | --- | --- |
| Format | `cargo fmt --check` | unformatted Rust |
| Lint | `cargo clippy -D warnings` | any warning |
| Rust tests | `cargo test` | unit/property test fails, replay hash mismatch |
| WASM build | `make wasm` | build fails |
| Web | typecheck + `vitest` | type or unit failure |
| E2E | Playwright (Chromium, WebKit, Firefox), keyboard-only | acceptance flow breaks |
| Content | `make content` (`ffs content validate`) | schema/reference/art/duplicate problems |
| Replays | `ffs replay --verify` | any state hash differs |
| Balance (small) | `make balance` with a small run count | a target in `docs/05-milestones.md` is violated |

Until the code exists, CI is just a docs sanity check (links, optional). Don't build CI before M1 needs it; add stages as each milestone lands.

## On merge to main

Build the static site and deploy it to a preview URL (once a host is chosen). Nightly: large balance run (2,000 runs per difficulty) and, from M3, the real-model prompt-injection check.

## On release

Tag `vX.Y.Z` -> CI publishes the static site; server image build is manual until it's annoying.

## Branch protection

Loose: PRs from branches, CI should be green before merge, David merges. Agents never merge their own PRs. Bypassing is fine for typo fixes and docs.

## What the pipeline does not check

- Whether the jokes are funny (human curation, `curated = true`).
- Real-model GM quality on real hardware (manual, M0/M3).
- Visual polish beyond screenshot regression; accessibility beyond the automated bits.
- Platform key-interception quirks on real OSes (manual matrix from M0).

## Secrets the pipeline holds

None yet. Expected: deploy token for the static host, optional real-model API key for the nightly check. List by name here when added.
