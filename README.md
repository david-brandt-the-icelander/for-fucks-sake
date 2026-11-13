# for-fucks-sake
Do you have what it takes to survive IT?

**FOR FUCK'S SAKE!** is a ten-minute, turn-based, keyboard-only dark comedy played
in a green-on-black terminal. You're the sysadmin of a dysfunctional company:
handle idiotic tickets, get rich at the company's expense, collect kompromat,
and out-brag rival sysadmins in the war-room — before you inevitably lose it.

- F-keys and shortcuts only (F1 shows them — and costs you a nerd point)
- ASCII art at mixed text resolutions, four switchable desktops
- Rust engine compiled to WASM, thin TypeScript UI, optional local LLM game master
- Single-player in the browser; multiplayer rooms with a server-side game master

## Design docs

| Doc | What |
|---|---|
| [docs/01-game-design.md](docs/01-game-design.md) | Rules, meters, Nerd Power, kompromat, rivals, endings |
| [docs/02-ui-and-input.md](docs/02-ui-and-input.md) | Screen grid, look, desktops, overlays, keyboard |
| [docs/03-architecture.md](docs/03-architecture.md) | Engine, WASM, GM/LLM, headless CLI, multiplayer server |
| [docs/04-content.md](docs/04-content.md) | Content packs, schemas, AI generation pipeline |
| [docs/05-milestones.md](docs/05-milestones.md) | Build order, acceptance criteria, balance targets |
| [docs/decisions.md](docs/decisions.md) | Decisions log and open questions |

Implementation agents: start with [AGENTS.md](AGENTS.md).
