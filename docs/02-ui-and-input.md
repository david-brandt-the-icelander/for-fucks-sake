# 02 — Screen, look, and keyboard

> Status: authoritative UI/input spec, v0.5. Mockups in this file are
> illustrative; the region tables and rules are normative.

## 1. Principles

- **Green phosphor terminal.** Black background, green text, ASCII art. One
  alert colour (red) for danger. Nick colours in chat are the only other hues.
- **Keyboard only.** No mouse interactions. A click anywhere just focuses the
  game. There is no clickable button anywhere in the game UI.
- **F-keys first.** Function keys are the primary controls; desktops are on
  F-keys too. Arrow-key combos are optional secondary bindings.
- **Mixed text resolutions are allowed and encouraged**: tiny text for ASCII
  art, normal text for content, double/triple size for headlines and endings —
  all aligned to one character grid.
- **Fits a 15" laptop** with the browser chrome visible: design size
  1440×810 CSS px.
- **Thin UI.** The TypeScript layer renders the engine's view model and turns
  key presses into commands. It holds no game rules (03-architecture.md).

## 2. The character grid

| Property | Value |
|---|---|
| Base cell | 9 × 18 CSS px at scale 1 |
| Grid | 160 columns × 45 rows |
| Design size | 1440 × 810 CSS px |
| Body font | IBM Plex Mono (OFL), 15 px, line-height 18 px (advance 0.6 em = 9 px) |
| Display font | VT323 (OFL) for 2× and 3× layers, letter-spacing forced so each glyph is exactly one scaled cell wide (measure advance at load; `letter-spacing = cell_w − advance`) |
| Fallback | DejaVu Sans Mono (self-hosted) for any glyph Plex Mono lacks |
| Fonts | Self-hosted woff2 under `web/public/fonts/`. No third-party font CDN. |

Glyph coverage must include ASCII, Latin-1, box drawing (U+2500–257F), block
elements (U+2580–259F: `█ ▀ ▄ ░ ▒ ▓ ▁▂▃▄▅▆▇`), geometric `▮ ▯ ● ○ ▸ ◂`,
arrows `← → ↑ ↓`, `¯`, `ツ`. Add a CI script that checks every character used in
content packs is covered by the font stack.

### 2.1 Text layers (resolutions)

| Layer | Scale | Cell (px) | Used for |
|---|---|---|---|
| `art` | 0.5× | 4.5 × 9 | ASCII portraits, ASCII art in chat, sparklines, small diagrams |
| `body` | 1× | 9 × 18 | Everything by default |
| `display` | 2× | 18 × 36 | Headlines (`INCIDENT #0042`), the fucks meter, overlay titles |
| `huge` | 3× | 27 × 54 | GAME OVER, LEGEND, meltdown punchlines |

A region declared in base cells (`x, y, w, h`) can hold any layer; an `art`
region of w×h base cells has 2w × 2h art cells. Layers never overlap within a
region except overlays.

### 2.2 Scaling to the viewport

- `scale = min(viewport_w / 1440, viewport_h / 810)`.
- Apply the scale by setting CSS custom properties (`--cell-w`, `--cell-h`,
  font sizes), **not** with a CSS transform (transforms blur text).
- Centre the screen; letterbox with `--bg`.
- Scale < 0.75: show a "window too small — make it bigger or go fullscreen
  (F11 / ⌃⌘F)" notice instead of the game. Scale is uncapped upward but
  rounded to 2 decimals.
- A 15" MacBook (1440×900 points) in Chrome/Safari with tabs and address bar
  leaves ≈ 1440×790 → scale ≈ 0.975. That is the reference machine.

## 3. Palette and effects

| Token | Value | Use |
|---|---|---|
| `--bg` | `#030a04` | Background |
| `--fg` | `#33ff66` | Primary text, active elements, inverted bars |
| `--fg-bright` | `#b8ffc9` | Quoted requester text, typed input, highlights |
| `--fg-art` | `#7dffa0` | ASCII art |
| `--fg-dim` | `#1fae4a` | Labels, hints, separators text |
| `--rule` | `#1d7a35` | Box borders only (never text) |
| `--alert` | `#ff5555` | Danger: critical meter, escalations, dirt on you, NP losses |
| `--bar-bg` | `#071a0b` | F-key bar background |
| Inverted | `--bg` on `--fg` | Top bar, selected rows, F-key labels |
| Nick colours | `#7dc8ff #ff8cf0 #ffd36b #c3a6ff #ff9f5a #5af0e6 #e6f05a #ffffff` | Chat nicks only (assigned per rival, stable per run) |

High-contrast mode (F9 › Display): fg `#ffffff`, dim `#c8c8c8`, art `#ffffff`,
rule `#8a8a8a`, alert `#ff6b6b`, bg `#000000`.

CRT effects (each toggleable in F9 › Display, all on by default):
scanlines (repeating 3 px gradient, 20 % black), vignette (inset shadow),
phosphor glow (`text-shadow: 0 0 4px rgb(51 255 102 / 0.35)`), cursor blink
(530 ms), meltdown glitch. `prefers-reduced-motion` disables blink, flicker and
glitch (static substitutes).

## 4. Screen layout (base cells)

```
row 0        TOP BAR (inverted)
rows 1–41    DESKTOP AREA (41 rows) — content of the active desktop
rows 42–43   STATUS: REMAINING FUCKS (display layer) + mood + toast
row 44       F-KEY BAR
```

| Region | x | y | w | h | Content |
|---|---|---|---|---|---|
| `topbar` | 0 | 0 | 160 | 1 | `FFS! v0.x ▸ <corp-slug> ▸ sysop@<desk>`, desktop tabs, clock, money, suspicion, NP, rank |
| `desk` | 0 | 1 | 160 | 41 | Active desktop (§5) |
| `status` | 0 | 42 | 160 | 2 | Fucks label + meter (2×), `NN/100`, trend sparkline (art), mood, toast |
| `fkeys` | 0 | 44 | 160 | 1 | Ten slots × 16 cols: `F1 Help-1NP`, … from the live keymap |

### 4.1 Top bar

`FFS! v0.5 ▸ grubb-logistics ▸ sysop@helpdesk   [F3 HELPDESK] [F4 WAR-ROOM 3] [F5 KOMPROMAT] [F6 CORP]   DAY 012 · TUE 10:43 · $247,850 · SUSP 18% · NP 1,240 · #7`

- Active tab drawn as normal (`--fg` on `--bg`) inside the inverted bar; other
  tabs dim-inverted. Unread counts after tab names (war-room messages,
  new kompromat, new incidents).
- Tab labels show the **current** keymap binding for that desktop.
- `SUSP` turns alert colour at ≥ 60. `NP` flashes briefly (or `*` under reduced
  motion) on change.

### 4.2 Status bar

`REMAINING FUCKS [██████████····················] 23/100   trend ▇▇▆▆▅▅▄▄▃▃▂▁   CRITICAL · one more stupid question…        Done: Form IT-17B issued.`

- Label and meter in the `display` layer (2×). Meter: 30 cells, `█` lit,
  `·` unlit. Colour `--fg`, `--alert` below 30.
- Last-straw danger (01-game-design §7.2): the leading lit cell flickers;
  reduced motion → a `!` after the number.
- Trend sparkline in the `art` layer: last 12 end-of-turn values.
- Toast: last result line (right-aligned, `--fg-bright`), cleared by the next
  action. Also mirrored to an ARIA live region.

### 4.3 F-key bar

Ten slots from the live keymap, Midnight-Commander style: key label inverted,
action label normal. Default: `F1 Help-1NP · F2 NP-log · F3 Helpdesk · F4 War-room
· F5 Dirt · F6 Corp · F7 Intern · F8 Coffee · F9 Config · F10 RUN`.
If an action's binding is not an F-key, the slot shows the actual combo
(e.g. `^1 Help-1NP`).

## 5. The four desktops

Desktops are full-area screens switched instantly (optional 120 ms horizontal
wipe, off under reduced motion). Switching is free (no clock cost). Each
desktop remembers its selection, scroll and half-typed input.

### 5.1 Desktop 1 — HELPDESK (F3)

| Region | x | y | w | h |
|---|---|---|---|---|
| ticket pane (bordered) | 1 | 2 | 112 | 39 |
| queue (bordered) | 114 | 2 | 45 | 10 |
| intern | 114 | 13 | 45 | 8 |
| war-room preview | 114 | 22 | 45 | 7 |
| ledger mini | 114 | 30 | 45 | 11 |

Ticket pane contents, top to bottom:

1. `display` headline `INCIDENT #0042`, priority badge (inverted alert for
   MGMT ESCALATION), requester line (`from Richard, Head of Sales · CEO meeting in
   20 min`), deadline countdown in in-game time.
2. Portrait box (art layer, 22×16 base cells = 44×32 art cells) beside the
   requester's quote (`--fg-bright`, may use display layer for short
   shouty quotes), plus a details line (attachments, ticket history, repeat count
   in alert colour: `(8th time)`).
3. `── RESPOND · [1-4] pick · [Enter] execute · [/] write your own · [R] rant ──`
4. Options list: `[n] text` padded, effect hint right-aligned at column 60 of
   the pane, selected row inverted. GM note (if LLM) in dim text under the
   selected option.
5. Spacer, then the prompt at the bottom: `sysop@helpdesk:~$ ▌` with block
   cursor. Up to 3 wrapped lines. `↑` on an empty prompt recalls history.

Queue: next incidents (`#0043 CEO "how do I attach a file" (8th time)`), `N`/`P`
moves between them (opening one costs its read irritation the first time).
Intern: small art portrait of Pip, competence/loyalty bars, `knows your tricks:
3` (alert when ≥ 4). War-room preview: unread count and the pinned challenge.
Ledger mini: cash, net worth, retirement progress `▮▮▯▯▯▯▯▯▯▯ 2.5 % of $10M`,
active schemes count.

Mockup (illustrative, 160 columns):

```
 FFS! v0.5 ▸ grubb-logistics ▸ sysop@helpdesk  [F3 HELPDESK] F4 WAR-ROOM 3  F5 KOMPROMAT  F6 CORP     DAY 012 · TUE 10:43 · $247,850 · SUSP 18% · NP 1,240 · #7
 ┌──────────────────────────────────────────────────────────────────────────────────────────────────────────────┐ ┌─ QUEUE · 5 waiting ────────────────────────┐
 │ ▄█▄ INCIDENT #0042 (display 2×)       [ MGMT ESCALATION ]  from Richard, Head of Sales · due 11:00            │ │ #0043 CEO  "how do I attach a file" (8th)  │
 │                                                                                                              │ │ #0044 HR   "Feelings@Work" module          │
 │ ┌──────────────────────┐  > "I can't send my presentation. It says the attachment is too large.             │ │ #0045 3rd floor printer "makes a noise"    │
 │ │ (art 0.5×: Richard)  │     Fix this immediately."                                                          │ │ #0046 Richard (again)                      │
 │ │   .-""""""""-.       │  attachment: Q4_DOMINATION_final_FINAL_v7.pptx · 2.1 GB · 4,000 slides               │ │ #0047 ████████ [redacted by auditor]       │
 │ │  /  (o)  (o) \  ;    │                                                                                      │ └────────────────────────────────────────────┘
 │ │  |    ____    | ;    │  ── RESPOND · [1-4] pick · [Enter] execute · [/] own words · [R] rant ─────────────  │ ┌─ INTERN PIP (F7) ──────────────────────────┐
 │ │   '-.______.-'       │  [1] Raise the attachment limit                     -2 fucks  +stability             │ │ (art) competence ▮▮▮▯▯  loyalty ▮▮▮▮▯      │
 │ │  [ 2.1 GB .pptx ]    │ ▐[2] Require Form IT-17B, in triplicate             +4 fucks  +suspicion ▌           │ │       knows your tricks: 3                 │
 │ └──────────────────────┘  [3] Enrol him in security training, effective now  +6 fucks  new enemy              │ └────────────────────────────────────────────┘
 │                           [4] Sell him your "enterprise transfer" licence   +$4,800  +risk                   │ ┌─ WAR-ROOM · 3 unread (F4) ─────────────────┐
 │                               GM: "Bureaucracy as a weapon. Chef's kiss."                                    │ │ <kernel_kate> got their head of mktg FIRED │
 │                                                                                                              │ │ -!- CHALLENGE +180 NP · one-up it          │
 │  …                                                                                                           │ └────────────────────────────────────────────┘
 │                                                                                                              │ ┌─ LEDGER ───────────────────────────────────┐
 │ sysop@helpdesk:~$ compress it to 240p and blame the CEO's firewall▌                                          │ │ cash $247,850   net worth $251,200         │
 └──────────────────────────────────────────────────────────────────────────────────────────────────────────────┘ │ retire ▮▯▯▯▯▯▯▯▯▯ 2.5 % of $10M · schemes 2 │
 REMAINING FUCKS [██████████····················] 23/100   ▇▆▆▅▅▄▄▃▃▂▁   CRITICAL · one more stupid question…                      Done: Form IT-17B issued.
 F1 Help-1NP  F2 NP-log  F3 Helpdesk  F4 War-room  F5 Dirt  F6 Corp  F7 Intern  F8 Coffee  F9 Config  F10 RUN
```

### 5.2 Desktop 2 — WAR-ROOM (F4)

| Region | x | y | w | h |
|---|---|---|---|---|
| chat log | 1 | 2 | 112 | 36 |
| chat input | 1 | 39 | 112 | 2 |
| pinned challenge (double border, alert) | 114 | 2 | 45 | 9 |
| leaderboard | 114 | 12 | 45 | 11 |
| flair meter | 114 | 24 | 45 | 5 |
| rivals list | 114 | 30 | 45 | 11 |

- Chat lines: `HH:MM <nick> text`. Nick coloured; system lines `-!-` in alert;
  ASCII art posted by rivals renders in the `art` layer indented under the line
  (max 60×24 art cells per post). Scrollback 500 lines; `PgUp/PgDn`, `↑/↓`
  select a message (to `I`nvestigate a boast).
- Input: `[#war-stories] ▌`. `/boast` opens the boast composer, `/challenge`
  likewise, `/me` emote, `/art <name>` posts one of your unlocked ASCII art
  pieces (from content). `Tab` completes `@nicks`. Emoji translated on input
  (§8). Enter sends.
- Boast composer (overlay, 100×22): pick a verified outcome from your list (↑↓),
  pick detail level (`1` vague / `2` specific / `3` verifiable, showing NP and
  exposure for each), write the telling or pick a template line, Enter posts.
- Leaderboard: rank, nick, NP, title; your row inverted.
- Flair meter: `flair in your last 5 msgs: ●○○○○ · next flair: +1 tasteful`
  (or `−1 tryhard` in alert).
- Rivals list: nick, corp, title, status (`active`, `under investigation`,
  `FIRED`, `JAILED`), and `∆ has dirt on you` marker when known.

### 5.3 Desktop 3 — KOMPROMAT (F5)

| Region | x | y | w | h |
|---|---|---|---|---|
| inventory list | 1 | 2 | 112 | 18 |
| leverage network (art/body) | 1 | 21 | 112 | 20 |
| detail pane | 114 | 2 | 45 | 27 |
| dirt on YOU | 114 | 30 | 45 | 11 |

Inventory list is an `ls -l` table:

```
$ ls -l ~/.kompromat                                          cred   status       known by
-rw------- cfo_margaret   unapproved_purchase.eml             ███░   private      you
-rw------- richard_sales  400GB_personal_video.dir            ████   private      you
-rw-r----- kernel_kate    yacht_story_inconsistency.txt       █░░░   unverified   you
-rw-r--r-- YOU            consulting_income.xls               ██░░   shared       sudo_sven
```

Permission bits encode status (`------` private, `r-----` shared with one,
`r--r--` public). `↑↓` select; actions on the detail pane:
`[V]erify [L]ean on [P]ublish [S]hare [I]nvestigate [X] discard`. Each action
shows its clock cost and risk before confirming with Enter.

Leverage network: generated ASCII graph (engine supplies nodes/edges; UI lays
them out on a fixed radial template: you centre, employees above, rivals
right/left, investigators below). Edge labels are allegation keys.

### 5.4 Desktop 4 — CORP (F6)

Two tabs toggled with `M`: **People** and **Ledger**.

- People: `$ tree /corp/grubb-logistics --people` — org tree (box-drawing
  `├── └──`) with name, role, dirt marker `[K]`, rigidity bar, disposition
  (`enemy` in alert). Select (`↑↓`), `Enter` opens a dossier (art portrait,
  attributes, history with you, what you hold on them). `T` trains Pip, `G`
  gives Pip a bribe.
- Ledger: cash, investments, liabilities, net worth, retirement progress, then
  schemes table (`name · income/day · suspicion/day · criminal · status`) with
  `[S]tart [U]p [D]own [B]ury`, and comfort purchases (`[C]omfort shop`).
- Right column: `/corp/metadata` — IT budget, company health bar, stability,
  auditor present, policies list (rule flags the player can weaponise).

## 6. Overlays

Overlays are centred boxes with a double border (`╔═╗║╚╝`), title in the
display layer, background dimmed (75 % `--bg`). `Esc` closes any overlay.
Only one overlay at a time; opening another replaces it.

| Overlay | Key | Size (cells) | Cost |
|---|---|---|---|
| Help / shortcuts | F1 | 100 × 34 | **−1 NP per open** |
| Nerd-point log | F2 | 90 × 30 | free |
| Config (Keymap · Display · GM · Audio) | F9 | 110 × 38 | free |
| Take the money and run | F10 | 72 × 16 | free (confirm ends run) |
| Rant composer | R (helpdesk) | 100 × 20 | 10 min |
| Boast composer | /boast, B | 100 × 22 | 10 min |
| End-of-day summary | automatic | 80 × 24 | — |
| Defence incident (rival/insider attack) | automatic, cannot be dismissed without choosing | 90 × 24 | — |

### 6.1 F1 — Help

```
╔══════════════════════════════════════════════════════════════════════════════╗
║  _____ _____ ____  _       −1 NERD POINT. RTFM next time.                    ║
║ |  ___|  ___/ ___|| |      You've opened help 3× this run.                   ║
║ | |_  | |_  \___ \| |      The war-room has been notified.                   ║
║ |  _| |  _|  ___) |_|                                                        ║
║ |_|   |_|   |____/(_)                                                        ║
║                                                                              ║
║  F1   help (costs 1 NP)               F6   desktop 4: corp & ledger          ║
║  F2   nerd-point log                  F7   dump it on the intern             ║
║  F3   desktop 1: helpdesk             F8   coffee break                      ║
║  F4   desktop 2: war-room             F9   config & keymap                   ║
║  F5   desktop 3: kompromat            F10  take the money and run            ║
║  ^←/^→ previous/next desktop         Esc  close / leave input               ║
║                                                                              ║
║  helpdesk  1-4 pick · Enter do it · / own words · R rant · N/P next/prev     ║
║  war-room  / chat · B boast · A accept · I investigate · Tab complete @nick  ║
║  dirt      V verify · L lean on · P publish · S share · I investigate · X    ║
║  corp      M people/ledger · Enter dossier · T train Pip · S/U/D/B schemes   ║
╚══════════════════════════════════════════════════════════════════════════════╝
```

The binding column is generated from the live keymap (remaps appear).
The first time per in-game hour that F1 is opened, the war-room gets a system
line: `* <you> asked for help again.`

### 6.2 F2 — Nerd-point log

Newest first: `  +12  Day 11 10:20 · boasted: CFO now reports to the printer`,
losses in alert colour, footer with total, current title, next title threshold
and the high score to beat. Header note: `free to open. unlike F1.`
Scrollable (`↑↓ PgUp PgDn`). Every NP change in the engine produces exactly
one log entry with a reason string.

### 6.3 F9 — Config

Tabs switched with `←/→` (inside the overlay arrow keys are local) or `K D G A`:

- **Keymap** — list of all actions (§7), two binding slots each (primary,
  secondary). `↑↓` select, `Enter` rebind primary, `Shift+Enter` rebind
  secondary, `Del` unbind slot, `P` load a profile (`classic`, `mac-no-fn`),
  `X` export / `I` import as text, `R` reset to defaults (confirm). While
  capturing: "press a key… (Esc cancels)". Platform warnings inline next to
  affected bindings (see §7.3).
- **Display** — scanlines, glow, vignette, cursor blink, glitch, high contrast,
  text scale nudge (−10 %…+10 %).
- **GM** — LLM: off / local (model name, size, download state, `D` download,
  `X` delete cached model), test prompt latency, "show GM notes" toggle.
- **Audio** — off by default; optional PC-speaker-style beeps (WebAudio square
  wave) for errors, NP change, meltdown.

### 6.4 F10 — Take the money and run

Shows current net worth, liquid/disputed/criminal split, the ending tier you
would get, the next tier threshold, and whether retirement is "clean"
(investigation/suspicion). `F10` again or `Y` confirms; `Esc`/`N` cancels.

## 7. Keyboard

### 7.1 Default keymap (`classic` profile)

| Action id | Primary | Secondary | Notes |
|---|---|---|---|
| `help` | F1 | — | −1 NP per open |
| `np_log` | F2 | — | free |
| `desk_helpdesk` | F3 | — | |
| `desk_warroom` | F4 | — | |
| `desk_kompromat` | F5 | — | prevent browser reload while game focused |
| `desk_corp` | F6 | — | |
| `delegate` | F7 | — | helpdesk: selected incident to Pip |
| `coffee` | F8 | — | |
| `config` | F9 | — | must always be reachable (§7.4) |
| `run` | F10 | — | opens confirm |
| `desk_prev` | Ctrl+← | — | macOS uses this for Spaces unless disabled |
| `desk_next` | Ctrl+→ | — | " |
| `focus_prompt` | / | — | |
| `close` | Esc | — | not rebindable |

Context keys (rebindable, same table in the keymap editor, grouped by desktop):
helpdesk `1`–`4` pick, `Enter` execute, `R` rant, `N`/`P` next/previous ticket;
war-room `B` boast, `A` accept challenge, `I` investigate; kompromat
`V L P S I X`; corp `M T G` and scheme keys `S U D B`; lists `↑ ↓ PgUp PgDn
Home End`; `Tab`/`Shift+Tab` cycle focus between panes on a desktop.

`mac-no-fn` profile: F1…F10 actions on `Ctrl+1`…`Ctrl+9`, `Ctrl+0`
(on macOS browsers Ctrl+digit is free; Cmd+digit switches tabs). Do not offer it
as default on Windows/Linux (Ctrl+digit switches tabs there).

### 7.2 Combo normalisation

Combos are strings like `F3`, `Ctrl+←`, `Alt+Shift+K`, `Cmd+Enter`.

- Modifier order: `Ctrl+Alt+Shift+Cmd+<key>` (`Cmd` = Meta).
- Key name: for `e.code` `KeyA`…`KeyZ` use the letter; `Digit0`…`Digit9` use
  the digit (layout-independent, and Alt on macOS produces symbols in `e.key`);
  arrows `← → ↑ ↓`; `Space`, `Enter`, `Tab`, `Esc`, `Del`, `Backspace`,
  `PgUp`, `PgDn`, `Home`, `End`, `F1`…`F12`; otherwise `e.key`.
- `Shift` is only part of a combo for non-printable keys or when combined with
  another modifier; plain `Shift+r` while typing is just `R` text.
- When a text input has focus: printable keys (without Ctrl/Alt/Cmd) type text;
  F-keys, `Esc`, `Enter`, `Tab`, `↑` (history on empty prompt) and modifier
  combos are still handled by the keymap.
- `preventDefault()` on every handled key. Unhandled keys pass through.

### 7.3 Platform pitfalls (must be handled)

- **macOS function keys** default to media/brightness keys; F-keys need `fn`
  unless *System Settings › Keyboard › Keyboard Shortcuts › Function Keys ›
  "Use F1, F2, etc. keys as standard function keys"* is on. The first-run
  keyboard check (§9) detects this.
- **macOS Ctrl+←/→** switch Spaces (Mission Control) and never reach the
  browser unless disabled. The keymap editor marks them `macOS steals this`.
- **Browser-reserved keys:** F11 (fullscreen) and F12 (devtools) are not
  bindable. Cmd/Ctrl+W, T, N, Q, Tab and Alt+F4 are not bindable. F1 (help in
  some browsers) and F5 (reload) must be intercepted while the game has focus;
  verify on Chrome, Firefox and Safari via the e2e suite (05-milestones). If a
  key cannot be intercepted on a browser/OS, the keymap editor marks it
  `not available here` and the first-run check suggests `mac-no-fn` or a
  remap.
- Show a `beforeunload` confirmation while a run is active (accidental ⌘W).
- IME/composition events: ignore key handling while `isComposing`.

### 7.4 Keymap rules

- Two slots per action. A combo can belong to only one action per context;
  rebinding a combo already in use unbinds it from the old action and shows
  `<old action> is now unbound`.
- `close` (Esc) is fixed. `config` must remain bound; if the player unbinds it,
  holding `Esc` for 2 s opens config anyway (and the editor warns).
- Persistence: `localStorage["ffs.keymap.v1"]`; multiplayer accounts also sync
  it to the server profile. Schema-versioned; invalid entries fall back to
  defaults for those actions only.
- Export/import text format (`~/.ffs/keymap`):

```
# FFS! keymap v1
help          = F1
np_log        = F2
desk_helpdesk = F3
desk_prev     = Ctrl+←, Alt+←
```

### 7.5 Engine involvement

The UI owns the keymap. But **NP-affecting UI actions are engine commands**:
opening F1 sends `HelpOpened`; F2/F9/F10-confirm-overlay opening are not sent
(only the final `Retire` is). This keeps NP authoritative in the engine and
lets multiplayer verify the F1 penalty.

## 8. Text input, emoji and flair

- All player text passes through `ffs_core::text::normalize` (exported via
  WASM) **on every input change**, so the player sees the translation
  immediately: emoji → text emoticons (table in 04-content.md), strip U+FE0F,
  unknown `\p{Extended_Pictographic}` → `(?)`, NFC normalise, collapse control
  chars. Works for typed emoji, the macOS emoji picker (⌃⌘Space) and paste.
- The engine re-normalises on receipt (server authority in multiplayer).
- Flair detection (`text::flair(&str) -> Flair { emoticon: bool, leet: bool }`)
  is in the engine; the flair meter on desktop 2 shows what the next flair
  message would earn.
- Length limits: chat 200, boast 280, free action 240, rant 1,000 characters.
  The prompt shows `n/limit` in dim text once 80 % is used.
- `Tab` completion: employee ids, rival nicks (`@`), kompromat file names,
  scheme names — from the engine's view model (only names the player knows).

## 9. First run and onboarding

1. **Title screen**: huge-layer `FFS!` logo, tagline *Do you have what it takes
   to survive IT?*, menu (`Enter` new run, `D` difficulty, `M` multiplayer,
   `L` leaderboards, `F9` config) — keyboard only.
2. **Keyboard check** (first run only, free, skippable with `S`): "Press F3."
   → ✓. "Press F1." → ✓ (free here, says so). If 10 s pass or the player
   presses `M` ("my Mac did something else"), show the macOS fn instructions and
   offer `mac-no-fn` profile. Also checks Ctrl+→ and reports if the OS ate it.
3. **Tutorial day 0** (optional, 3 incidents) teaching options, `/` free text,
   F4 war-room, F5 kompromat, the meters. No NP cost for F1 during it.
4. **GM offer**: if WebGPU is available, offer local GM download with size and
   expected speed; otherwise explain no-LLM mode.

## 10. Accessibility

- The game is fully keyboard operable by design. Focus never leaves the game
  container except into its own inputs.
- A visually hidden ARIA live region announces toasts, NP changes, incident
  arrivals and turn results in plain English.
- Each desktop has a semantic mirror (headings, lists, tables) for screen
  readers; ASCII art carries `alt` text from content.
- Reduced motion respected (§3). High-contrast mode. Text scale nudge.
- Text contrast: all text tokens ≥ 4.5:1 on `--bg`; `--rule` is for borders only.

## 11. Rendering implementation notes (for the TS layer)

- DOM-based: one absolutely positioned `<pre>` (or `<div>` of spans) per region
  per layer, positioned in cell units via CSS variables. Colour runs as spans.
  No canvas (keeps text crisp, selectable for the share card, accessible).
- The UI derives everything from `engine.view()` (03-architecture §5) plus local
  UI state (active desktop, selections, scroll, input buffers, overlay, keymap,
  display settings).
- Re-render on each command result; the screen is small (≤ 7,200 base cells),
  so a straightforward keyed diff per region is enough. Target ≤ 4 ms per frame
  on the reference machine.
- Layout helpers: `boxed(region, title)`, `padRight`, `wrap(text, width)` (word
  wrap, hard-break long tokens), `table(columns)`, `meter(value, cells, glyphs)`,
  `sparkline(values)`. Unit-test them.
