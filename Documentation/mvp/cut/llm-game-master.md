# LLM game master (cut from MVP)

Authoritative spec: `docs/01-game-design.md` §10, `docs/03-architecture.md` §6, `docs/04-content.md` §6, M0.1 and M3.

**What it was:** an optional model (local via WebLLM, or server-side) that interprets free text into a catalog action, scores
style and plausibility, ranks options and narrates. The engine validates everything; the deterministic fallback always works.

**Why cut:** model download size, WebGPU support, latency, prompt injection, content safety and cost. A good fixed-option game
doesn't need it. The "engine is the referee" principle means cutting it loses no rules.

**What stays true:** the design is GM-ready. Options are catalog actions with ids; effects are data; a GM would only pick or
score. Don't hard-code anything that assumes there will never be one.

**Revive checklist:** run the M0.1 latency spike; decide model or "no-LLM default"; write the injection suite first;
reinstate risks R4, R5 and R11 from `risks-parked.md`.
