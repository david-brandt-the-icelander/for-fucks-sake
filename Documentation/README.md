# Documentation

The project documents, indie-sized. Start with [mvp/](mvp/README.md) for what we build first; the
files below describe the full game. `operations.md` and `observability.md` moved to `mvp/cut/` (no server in the MVP). Based on the templates in
[processes/project-management/deployable-documentation](https://github.com/david-brandt-the-icelander/processes/tree/main/project-management/deployable-documentation),
but deliberately loose: a few honest lines beat a perfect table nobody updates.

**Not to be confused with `docs/`.** `docs/` is the *game spec* (what we're building).
`Documentation/` is the *project paperwork* (why, who, how it ships, what could bite us).

**State of the project:** pre-code. Spec v0.5 exists, no crates or web app yet. Each file
says what is *decided* versus *planned*. Where something is simply not known yet, it says
`TBD` rather than inventing it.

| File | Answers |
| --- | --- |
| [business-plan.md](business-plan.md) | Why this exists, what it costs, when to stop |
| [stakeholders.md](stakeholders.md) | Who cares, who decides |
| [risk-assessment.md](risk-assessment.md) | What could go wrong |
| [requirements.md](requirements.md) | What it does (points at `docs/`) |
| [non-functional-requirements.md](non-functional-requirements.md) | How well it has to do it |
| [architecture.md](architecture.md) | How it's built (points at `docs/03-architecture.md`) |
| [deployment.md](deployment.md) | Where it runs, how it ships, rollback |
| [cicd.md](cicd.md) | What CI checks |
| [development.md](development.md) | Getting it running, how changes get in |
| [mvp/](mvp/README.md) | **The cut-down first game, and what was cut** |
| [contacts.md](contacts.md) | Who to poke |

## Ground rules (loose on purpose)

- Update a file in the same commit as the change that makes it stale. Don't schedule "docs days".
- Pointer files are fine. If the truth is in `docs/` or CI config, link to it and say in a sentence why it matters.
- `TBD` is a valid answer. A wrong confident answer isn't.
- Review the risk list when a milestone ends (see `docs/05-milestones.md`), not on a calendar.
- Ship-blocking things get a row in `risk-assessment.md`; everything else can live in `docs/decisions.md`.
