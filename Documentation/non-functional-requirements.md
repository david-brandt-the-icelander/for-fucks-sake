# Non-functional requirements

**Source of truth:** here, where a number exists; otherwise the linked spec section.

Status: `met`, `not met`, `not checked`. Pre-code, so almost everything is `not checked`. Targets are starting points; change them when playtesting says so.

| Area | Requirement | Measured by | Status |
| --- | --- | --- | --- |
| Run length | ~10 min / 30-40 actions | `ffs bot` report (M5 balance table) | not checked |
| Determinism | Same seed + command log = same state hash on every host | `ffs replay --verify` on corpus of >= 20 runs | not checked |
| Render speed | <= 4 ms per frame render | perf pass (M7) | not checked |
| Screen fit | No scrollbars at 1440x790; graceful message at 1000x560 | Playwright (M2) | not checked |
| GM latency | `Interpret` p50 <= 3 s, >= 95% schema-valid (or "no-LLM default") | M0 spike, M3 | not checked |
| Download size | TBD after M0 (WASM + font + optional model weights, model only after consent) | build output | not checked |
| Privacy | Single-player sends nothing anywhere; MP stores handle, runs, chat (30 days); no email | code review + CSP | not checked |
| Security | Strict CSP; no third-party scripts/fonts; player text is untrusted; server-authoritative MP | e2e + injection suite | not checked |
| Accessibility | Keyboard-only; ARIA live region; reduced motion; high contrast; 4.5:1 text contrast | Playwright + manual audit (M7) | not checked |
| Browsers | Latest Chromium, WebKit, Firefox | Playwright matrix | not checked |
| Multiplayer capacity | 50 rooms x 12 players on one modest instance (mock GM) | load test (M6) | not checked |
| Cost | Single-player: ~$0 to host. MP and GM spend ceiling: TBD (set before M6) | invoices | not checked |
| Language | English first; Icelandic pack is a nice-to-have | n/a | n/a |
| Maintainability | `clippy -D warnings`, fmt, no `unwrap()` in core; tests accompany rules | CI | not checked |

## Trade-offs

- Determinism over convenience: integer maths and ordered maps everywhere in `ffs-core`.
- Single-player privacy over leaderboard coverage: single-player runs are unverified and stay local.
- GM quality traded for laptop-friendliness: small local model, engine caps what it can do.
