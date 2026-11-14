# Instructions for implementation agents

You are implementing **FOR FUCK'S SAKE!** (FFS!), a keyboard-only, green-terminal,
turn-based dark comedy about surviving life as a sysadmin. The complete design is
in `docs/`. Read it in this order before writing code:

1. `docs/01-game-design.md` — rules, meters, systems, endings (what the game is)
2. `docs/02-ui-and-input.md` — screen grid, look, desktops, overlays, keyboard
3. `docs/03-architecture.md` — Rust core → WASM/native, thin TS UI, GM, server
4. `docs/04-content.md` — content pack formats, schemas, generation pipeline
5. `docs/05-milestones.md` — build order, acceptance criteria, balance targets
6. `docs/decisions.md` — settled decisions, spec decisions, open questions

## Ground rules

- **The spec is authoritative.** If it is silent or wrong, make the smallest
  sensible decision, implement it, and append it to `docs/decisions.md` (date,
  decision, reason). Do not silently diverge.
- **All game rules live in `crates/ffs-core`.** The TypeScript UI renders the
  view model and sends commands. No rule, number or random roll in TS. The
  server and CLI are hosts too.
- **Determinism is sacred** (03-architecture §3): integer maths, seeded
  per-decision ChaCha streams, ordered maps, no clock/IO in core. Replays must
  reproduce state hashes. If you change rules or content intentionally,
  regenerate the replay corpus in the same commit.
- **The LLM is never the referee.** It can only pick from the action catalog,
  score `style` and `plausibility`, rank options and narrate. Validate
  everything; fall back deterministically. Treat player text as untrusted.
- **Keyboard only.** Never add mouse-dependent UI. Every feature must be
  reachable by keys and covered by keyboard-only Playwright tests.
- **Original content only.** No names, characters, catchphrases or stories from
  existing works (in particular nothing from "Bastard Operator From Hell": not
  the name, "BOFH", "PFY", or its stories). No real people or companies.
  No violence; no actionable real-world crime or hacking instructions.
  Profanity is fine.
- **Text only.** ASCII art and text in the shipped game; no images, no 3D, no
  emoji glyphs (emoji are translated to text emoticons).
- Work milestone by milestone (05-milestones). Don't start M(n+1) until M(n)'s
  acceptance list passes in CI.

## Documentation

Project paperwork (business plan, risks, deployment, ops, contacts...) lives in
`Documentation/`, based on the twelve-file layout at
https://github.com/david-brandt-the-icelander/processes/tree/main/project-management/deployable-documentation,
kept deliberately loose for an indie project. `docs/` is the game spec; `Documentation/`
is everything around it. Update the relevant file in the same commit as the change.
A change that adds a dependency, port, secret, alert or external service is not finished until
the file that lists those says so.

## Commands (once scaffolded)

```
make dev        # wasm-pack watch + Vite dev server
make test       # cargo test + vitest + Playwright
make wasm       # build crates/ffs-wasm into web/src/wasm
make content    # ffs content validate
make balance    # ffs bot report + assertions
cargo run -p ffs-cli -- play --seed 42
cargo run -p ffs-cli -- jsonl --seed 42 --gm mock
```

Keep this list accurate as the Makefile evolves.

## Code conventions

- Rust: `cargo fmt`, `cargo clippy -D warnings`, no `unwrap()` in core outside
  tests, errors via `thiserror`, public types `serde`-serialisable with
  `#[serde(tag = "type")]` for enums crossing the WASM/websocket boundary.
- TypeScript: `strict`, no framework, ES modules, small files; types for
  commands/events/view generated from Rust (e.g. `ts-rs` or `specta`) — never
  hand-maintained twice.
- Tests accompany every rule and every UI behaviour listed in acceptance
  criteria. Prefer property tests for engine invariants.
- Commits: small, imperative subject, reference the milestone (`M2: keymap
  persistence`).
