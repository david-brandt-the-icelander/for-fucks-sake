// Pure drawing: (view, ui, keymap) -> grid. No game rules.
import { COLS, Grid, meter, money, sparkline, wrap } from "./grid";
import type { Style } from "./grid";
import type { Chip, View } from "./game";
import { ACTIONS, type Keymap } from "./keymap";

export type Overlay = null | "help" | "nplog" | "config" | "run" | "rant" | "scheme" | "boast";

export interface BestRun {
  days: number;
  cash: number;
  np: number;
  kind: string;
}

export interface UI {
  screen: "title" | "game";
  desk: "helpdesk" | "warroom";
  sel: number; // selected ticket index
  opt: number; // selected option index
  overlay: Overlay;
  cursor: number; // list cursor inside overlays
  rantText: string;
  configSel: number;
  capturing: boolean;
  configMsg: string;
  pressed: Set<string>; // title key check
  best: BestRun[];
  hasSave: boolean;
  copied: boolean;
  tooSmallNote: string;
}

const toneStyle = (c: Chip): Style => (c.tone === "good" ? "b" : c.tone === "bad" ? "x" : "d");

function clip(s: string, n: number): string {
  const a = Array.from(s);
  return a.length <= n ? s : a.slice(0, n - 1).join("") + "~";
}

function ordinal(n: number): string {
  const s = ["th", "st", "nd", "rd"];
  const v = n % 100;
  return n + (s[(v - 20) % 10] ?? s[v] ?? s[0] ?? "th");
}

function combo(km: Keymap, id: string): string {
  return km[id] || "unbound";
}

export function drawTop(g: Grid, v: View, ui: UI, km: Keymap): void {
  g.fill(0, 0, COLS, "i");
  const h = v.header;
  g.put(1, 0, `FFS! v0.1 > ${h.corp.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/-+$/, "")} > sysop@${ui.desk}`, "i");
  const t1 = `[${combo(km, "desk_helpdesk")} HELPDESK]`;
  const t2 = `[${combo(km, "desk_warroom")} WAR-ROOM]`;
  g.put(52, 0, t1, ui.desk === "helpdesk" ? "n" : "j");
  g.put(52 + t1.length + 1, 0, t2, ui.desk === "warroom" ? "n" : "j");
  const right = `DAY ${String(h.day).padStart(2, "0")} ${h.clock} | ${money(h.cash)} | SUSP ${h.susp}% | NP ${h.np} | #${h.rank}`;
  g.right(0, 0, COLS - 1, right, "i");
  if (h.susp >= 60) {
    const s = `SUSP ${h.susp}%`;
    g.put(COLS - 1 - right.length + right.indexOf("SUSP"), 0, s, "k");
  }
}

export function drawStatus(g: Grid, v: View): void {
  const h = v.header;
  const crit = h.fucks < 30;
  g.put(1, 42, "REMAINING FUCKS", crit ? "x" : "b");
  g.put(17, 42, "[" + meter(h.fucks, 100, 30) + "]", crit ? "x" : "n");
  g.put(49, 42, `${String(h.fucks).padStart(3)}/100${crit ? " !" : ""}`, crit ? "x" : "b");
  g.put(60, 42, sparkline(h.fucks_history.slice(-24)), "a");
  g.put(86, 42, clip(h.mood, 72), crit ? "x" : "d");
  g.put(1, 43, h.toast ? "> " + clip(h.toast, 150) : "", "b");
}

const FKEY_SLOTS = ["help", "np_log", "desk_helpdesk", "desk_warroom", "rant", "scheme", "boast", "coffee", "config", "run"];
const FKEY_LABELS: Record<string, string> = {
  help: "Help-1NP", np_log: "NP-log", desk_helpdesk: "Helpdesk", desk_warroom: "War-room", rant: "Rant",
  scheme: "Scheme", boast: "Boast", coffee: "Coffee", config: "Config", run: "RUN",
};

export function drawFKeys(g: Grid, km: Keymap): void {
  g.fill(0, 44, COLS, "f");
  FKEY_SLOTS.forEach((id, i) => {
    const x = i * 16;
    const c = combo(km, id);
    g.put(x, 44, c, "i");
    g.put(x + c.length + 1, 44, clip(FKEY_LABELS[id] ?? id, 15 - c.length), "f");
  });
}

function drawChips(g: Grid, xr: number, y: number, chips: Chip[], sel: boolean): void {
  let x = xr;
  for (let i = chips.length - 1; i >= 0; i--) {
    const c = chips[i] as Chip;
    x -= c.text.length;
    g.put(x, y, c.text, sel ? "s" : toneStyle(c));
    x -= 2;
  }
}

export function drawHelpdesk(g: Grid, v: View, ui: UI, km: Keymap): void {
  g.box(1, 1, 112, 41, "TICKET");
  const t = v.tickets[ui.sel];
  if (!t) {
    g.put(4, 4, "INBOX ZERO", "b");
    g.put(4, 6, "Nobody needs you right now. It will not last.", "n");
    g.put(4, 8, `Skim a scheme [${combo(km, "scheme")}], cover your tracks [${combo(km, "cover")}], rant [${combo(km, "rant")}], brag [${combo(km, "desk_warroom")}], or go home [${combo(km, "end_day")}].`, "d");
  } else {
    const nth = t.nth > 1 ? ` (${ordinal(t.nth)} time)` : "";
    g.put(4, 2, `INCIDENT #${t.id}  ${t.title}`, "b");
    if (nth) g.put(4 + `INCIDENT #${t.id}  ${t.title}`.length + 1, 2, nth, "x");
    if (t.age > 0) g.right(2, 2, 109, `WAITING ${t.age} DAY${t.age > 1 ? "S" : ""}`, "x");
    g.put(4, 3, `from ${t.who_name}, ${t.who_role}`, "d");
    g.box(3, 5, 26, 11, "", "r");
    g.art(5, 6, t.portrait, "a");
    const q = wrap(`"${t.quote}"`, 78);
    q.slice(0, 7).forEach((l, i) => g.put(32, 5 + i, l, "b"));
    if (t.details) wrap(t.details, 78).slice(0, 2).forEach((l, i) => g.put(32, 13 + i, l, "d"));
    g.put(3, 17, `── RESPOND · [1-${t.options.length}] pick · [Enter] do it ` + "─".repeat(60), "r");
    let y = 19;
    t.options.forEach((o, i) => {
      const sel = i === ui.opt;
      if (sel) g.fill(3, y, 108, "s");
      g.put(4, y, `[${i + 1}] ${o.text}`, sel ? "s" : "n");
      drawChips(g, 110, y, o.hint, sel);
      y += 2;
    });
    if (t.dirt) {
      g.put(4, y, `[${combo(km, "dirt")}] Play dirt on ${t.dirt.name}: ${clip(t.dirt.label, 60)}`, "a");
      g.right(3, y, 107, "+10 fucks  +2 NP", "b");
      y += 2;
    }
    g.put(4, 38, `[${combo(km, "next")}/${combo(km, "prev")}] other ticket  [${combo(km, "rant")}] rant  [${combo(km, "coffee")}] coffee  [${combo(km, "scheme")}] scheme  [${combo(km, "cover")}] cover tracks  [${combo(km, "end_day")}] go home  [${combo(km, "run")}] run`, "d");
  }
  // right column
  g.box(114, 1, 46, 10, `QUEUE · ${v.tickets.length} waiting`);
  v.tickets.slice(0, 8).forEach((tk, i) => {
    const mark = i === ui.sel ? "▸" : " ";
    const line = `${mark}#${tk.id} ${tk.who_name.split(" ")[0]}: ${tk.title}${tk.nth > 1 ? " x" + tk.nth : ""}`;
    g.put(116, 2 + i, clip(line, 43), i === ui.sel ? "i" : tk.age > 0 ? "x" : "n");
  });
  g.box(114, 11, 46, 9, "LEDGER");
  const h = v.header;
  g.put(116, 12, `cash ${money(h.cash)}`, "b");
  g.put(116, 13, `retire ${meter(h.cash, h.retire_goal, 14)} ${money(h.retire_goal)}`, "n");
  g.put(116, 14, `suspicion ${meter(h.susp, 100, 14)} ${h.susp}%`, h.susp >= 60 ? "x" : "n");
  g.put(116, 16, `day ends 17:00 (${h.minutes_left} min left)`, "d");
  g.put(116, 17, `rank #${h.rank}: ${h.title}`, "d");
  g.put(116, 15, `cover tracks [${combo(km, "cover")}]: -${v.today.cover_gain} susp, -${v.today.cover_fucks} fucks`, "d");
  g.put(116, 18, h.can_retire ? "F10 would be a LEGENDARY retirement" : "retire needs goal cash + susp < 60", h.can_retire ? "b" : "d");
  g.box(114, 20, 46, 8, `DIRT · ${v.dirt.length}`);
  if (!v.dirt.length) g.put(116, 21, "Nothing on anyone. Yet.", "d");
  v.dirt.slice(0, 6).forEach((d, i) => g.put(116, 21 + i, clip(`${d.name.split(" ")[0]}: ${d.label}`, 43), "a"));
  g.box(114, 28, 46, 14, `WAR-ROOM`);
  const lines = v.chat.slice(-12);
  lines.forEach((c, i) => g.put(116, 29 + i, clip(c.kind === "system" ? c.text : `<${c.who}> ${c.text}`, 43), c.kind === "you" ? "b" : c.kind === "system" ? "d" : "n"));
}

export function drawWarroom(g: Grid, v: View, ui: UI, km: Keymap): void {
  g.box(1, 1, 112, 41, "#war-room");
  const rows: { text: string; style: Style; nick?: string }[] = [];
  for (const c of v.chat) {
    const prefix = c.kind === "system" ? "" : `<${c.who}> `;
    const lines = wrap(c.text, 104 - prefix.length);
    lines.forEach((l, i) => rows.push({ text: (i === 0 ? prefix : " ".repeat(prefix.length)) + l, style: c.kind === "you" ? "b" : c.kind === "system" ? "d" : "n", nick: i === 0 && prefix ? c.who : undefined }));
  }
  const visible = rows.slice(-37);
  visible.forEach((r, i) => {
    g.put(3, 2 + i, r.text, r.style);
    if (r.nick) g.put(3, 2 + i, `<${r.nick}>`, r.style === "n" ? "a" : "b");
  });
  g.put(3, 40, `[${combo(km, "boast")}] boast about one of your stories   [${combo(km, "desk_helpdesk")}] back to work`, "d");
  g.box(114, 1, 46, 9, "NERD POWER");
  v.board.forEach((r, i) => g.put(116, 2 + i, clip(`#${i + 1} ${r.nick.padEnd(12)} ${String(r.np).padStart(4)} ${meter(r.np, Math.max(40, v.board[0]?.np ?? 40), 10)}`, 43), r.you ? "i" : "n"));
  g.put(116, 8, clip(`you are: ${v.header.title}`, 43), "b");
  g.box(114, 10, 46, 21, "ROSTER");
  v.board.filter((r) => !r.you).slice(0, 4).forEach((r, i) => {
    const x = 116 + (i % 2) * 22;
    const y = 11 + Math.floor(i / 2) * 9;
    g.art(x, y, r.avatar, "a");
    g.put(x, y + 6, clip(r.nick, 20), "n");
    g.put(x, y + 7, clip(r.corp, 20), "d");
  });
  g.box(114, 31, 46, 11, `YOUR STORIES · ${v.stories.length}`);
  if (!v.stories.length) g.put(116, 32, "Nothing to brag about. Go cause trouble.", "d");
  v.stories.slice(-8).forEach((s, i) => g.put(116, 32 + i, clip(`${"*".repeat(s.value)} I ${s.text}`, 43), s.risky ? "x" : "n"));
}

function center(g: Grid, w: number, h: number, title: string): { x: number; y: number } {
  const x = Math.floor((COLS - w) / 2);
  const y = Math.floor((42 - h) / 2) + 1;
  g.panel(x, y, w, h, title);
  return { x, y };
}

export function drawOverlay(g: Grid, v: View, ui: UI, km: Keymap): void {
  switch (ui.overlay) {
    case "help": {
      const { x, y } = center(g, 100, 36, "HELP (you asked for it: -1 NP)");
      const lines = [
        "You are the sysadmin. Idiots send tickets. Each answer costs or restores REMAINING FUCKS.",
        "At zero you melt down, which is the normal ending. Days survived is your score.",
        "",
        "Pick a ticket option with 1-4 and press Enter. The chips show what it does (honest).",
        "Spite is tempting and costs Suspicion. At 100% Suspicion you are fired.",
        "Schemes make money (and Suspicion), and sometimes hit a jackpot. Dirt beats a coworker's ticket.",
        "Cover your tracks to bring Suspicion down: it costs time and a few fucks, less each time a day.",
        "Stories from your best moves can be bragged about in the war-room for Nerd Power.",
        "Retire with enough cash and low Suspicion for the legendary ending. F10 any time to run.",
      ];
      lines.forEach((l, i) => g.put(x + 3, y + 2 + i, l, "n"));
      ACTIONS.forEach((a, i) => {
        const col = i < 8 ? 0 : 1;
        const row = i % 8;
        g.put(x + 3 + col * 48, y + 13 + row, `${combo(km, a.id).padEnd(8)} ${a.label}`, "b");
      });
      g.put(x + 3, y + 23, "Fixed keys: 1-4 pick option, Up/Down move, Enter confirm, Esc close.", "d");
      g.put(x + 3, y + 25, "Rebind keys in Config (F9 by default).", "d");
      g.put(x + 3, y + 34, "[Esc] close", "d");
      break;
    }
    case "nplog": {
      const { x, y } = center(g, 90, 28, `NERD-POWER LOG · ${v.header.np} NP · ${v.header.title} · rank #${v.header.rank}`);
      const log = v.np_log.slice(-23).reverse();
      if (!log.length) g.put(x + 3, y + 2, "No nerd points earned or lost yet.", "d");
      log.forEach((e, i) => g.put(x + 3, y + 2 + i, clip(`day ${String(e.day).padStart(2)}  ${(e.delta > 0 ? "+" : "") + e.delta}  ${e.reason}`, 84), e.delta < 0 ? "x" : "n"));
      g.put(x + 3, y + 26, "[Esc] close   (this screen is free)", "d");
      break;
    }
    case "config": {
      const { x, y } = center(g, 80, 26, "CONFIG: KEYMAP");
      ACTIONS.forEach((a, i) => {
        const sel = i === ui.configSel;
        if (sel) g.fill(x + 2, y + 2 + i, 76, "s");
        const val = ui.capturing && sel ? "press a key..." : combo(km, a.id);
        g.put(x + 3, y + 2 + i, `${a.label.padEnd(40)} ${val}`, sel ? "s" : km[a.id] ? "n" : "x");
      });
      g.put(x + 3, y + 19, ui.configMsg, "b");
      g.put(x + 3, y + 21, "[Up/Down] choose  [Enter] rebind  [Del] reset this key  [Ctrl+Del] reset all", "d");
      g.put(x + 3, y + 22, "F11, F12, Ctrl+W/T/N/Q, Esc, Enter and Tab cannot be bound.  [Esc] close", "d");
      g.put(x + 3, y + 24, "Saved automatically in this browser.", "d");
      break;
    }
    case "run": {
      const { x, y } = center(g, 70, 12, "TAKE THE MONEY AND RUN");
      const h = v.header;
      g.put(x + 3, y + 2, `Cash ${money(h.cash)}   Suspicion ${h.susp}%   Goal ${money(h.retire_goal)}`, "b");
      if (h.can_retire) {
        g.put(x + 3, y + 4, "You have enough, and nobody is looking. This is the LEGENDARY retirement.", "b");
      } else {
        g.put(x + 3, y + 4, "That is not retirement-money, or you are too hot. You will just run.", "x");
      }
      g.put(x + 3, y + 6, "The run ends here. Score counts days survived.", "n");
      g.put(x + 3, y + 9, "[Y] yes, run    [Esc] no, back to the suffering", "d");
      break;
    }
    case "rant": {
      const { x, y } = center(g, 100, 18, "RANT: THE UNSENT EMAIL");
      g.put(x + 3, y + 2, "To: all-staff@grubb-logistics     Subject: (no subject)", "d");
      const lines = wrap(ui.rantText + "_", 92);
      lines.slice(-8).forEach((l, i) => g.put(x + 3, y + 4 + i, l, "b"));
      g.put(x + 3, y + 13, "[Enter] SEND (+10 fucks, +5 susp, might summon HR)   [Ctrl+D] delete draft (+5 fucks)", "n");
      g.put(x + 3, y + 15, "[Esc] cancel, change nothing. Your words are never stored or sent anywhere.", "d");
      break;
    }
    case "scheme": {
      const n = Math.max(1, v.schemes.length);
      const { x, y } = center(g, 100, 4 + n * 3 + 4, "SCHEMES: GET RICH AT THE COMPANY'S EXPENSE");
      v.schemes.forEach((s, i) => {
        const sel = i === ui.cursor;
        const yy = y + 2 + i * 3;
        if (sel) g.fill(x + 2, yy, 96, "s");
        g.put(x + 3, yy, `[${i + 1}] ${s.name}  ${"$".repeat(s.tier)}`, sel ? "s" : "n");
        g.put(x + 7, yy + 1, s.desc, sel ? "s" : "d");
        drawChips(g, x + 97, yy, s.hint, sel);
      });
      if (!v.schemes.length) g.put(x + 3, y + 2, "You have bled the company dry. There is nothing left to skim.", "d");
      else g.put(x + 3, y + 3 + n * 3, `[1-${v.schemes.length}] or [Enter] run it   [Esc] back`, "d");
      g.put(x + 3, y + 4 + n * 3, `Each scheme works once; a bigger, riskier one takes its place. ${v.schemes_left} more in the pipeline.`, "d");
      break;
    }
    case "boast": {
      const n = Math.max(1, v.stories.length);
      const { x, y } = center(g, 100, 6 + n * 2, "BOAST TO THE WAR-ROOM");
      if (!v.stories.length) g.put(x + 3, y + 2, "You have no stories. Do something worth bragging about first.", "d");
      v.stories.forEach((s, i) => {
        const sel = i === ui.cursor;
        if (sel) g.fill(x + 2, y + 2 + i * 2, 96, "s");
        g.put(x + 3, y + 2 + i * 2, clip(`[${i + 1}] ${"*".repeat(s.value)} I ${s.text}`, 80), sel ? "s" : "n");
        if (s.risky) g.right(x + 2, y + 2 + i * 2, 94, "+6 susp", sel ? "s" : "x");
      });
      g.put(x + 3, y + 3 + n * 2, "[Up/Down] choose  [Enter] brag   [Esc] back   (3rd boast of a day is worth less)", "d");
      break;
    }
    default:
  }
}

export function drawTitle(g: Grid, v: { logo: string } | null, ui: UI, km: Keymap): void {
  g.fill(0, 0, COLS, "i");
  g.put(1, 0, "FFS! v0.1", "i");
  const logo = v?.logo ?? "";
  const lines = logo.replace(/\n$/, "").split("\n");
  lines.forEach((l, i) => g.put(60, 4 + i, l, "b"));
  g.put(52, 13, "Do you have what it takes to survive IT?", "n");
  g.put(54, 16, ui.hasSave ? "[Enter] resume your run     [Space] new run" : "[Enter] or [Space] start a run", "b");
  g.put(54, 18, `[${combo(km, "config")}] config keys     keyboard only. F-keys are your friends.`, "d");
  g.put(30, 21, "KEYBOARD CHECK: press these keys; they light up if the browser lets the game see them", "d");
  const keys = FKEY_SLOTS.map((id) => combo(km, id));
  let x = 30;
  for (const k of keys) {
    g.put(x, 23, ` ${k} `, ui.pressed.has(k) ? "i" : "r");
    x += k.length + 3;
  }
  if (ui.best.length) {
    g.put(54, 27, "BEST RUNS (this browser)", "n");
    ui.best.slice(0, 5).forEach((b, i) => g.put(54, 29 + i, `${i + 1}. ${b.days} days  ${money(b.cash)}  NP ${b.np}  ${b.kind}`, "d"));
  }
  g.put(30, 42, "All people and companies are fictional. Profanity is the product. Nothing leaves your browser.", "d");
}

export function drawEnd(g: Grid, v: View, ui: UI, km: Keymap): void {
  const e = v.ended;
  if (!e) return;
  g.fill(0, 0, COLS, "i");
  g.put(1, 0, `FFS! > ${v.header.corp}`, "i");
  g.box(20, 2, 120, 38, e.title);
  let y = 4;
  for (const l of e.lines) {
    g.put(24, y++, l, "a");
  }
  y = Math.max(y + 1, 22);
  const kindText = { meltdown: "GAME OVER", fired: "FIRED", retired: "LEGEND: RETIRED", ran: "TOOK THE MONEY AND RAN" }[e.kind];
  g.put(24, y, kindText, e.kind === "retired" ? "b" : "x");
  g.put(24, y + 2, `Days survived   ${e.days}`, "b");
  g.put(24, y + 3, `Fortune         ${money(e.cash)}`, "n");
  g.put(24, y + 4, `Nerd Power      ${e.np}  (${e.nick_title}, rank #${e.rank})`, "n");
  g.put(24, y + 5, `Cause           ${e.cause}`, "n");
  wrap(e.share, 110).forEach((l, i) => g.put(24, y + 8 + i, l, "d"));
  g.put(24, y + 12, `[${combo(km, "copy")}] ${ui.copied ? "copied!" : "copy share card"}     [Enter] new run`, "b");
}

export function drawGame(g: Grid, v: View, ui: UI, km: Keymap): void {
  g.clear();
  if (v.ended) {
    drawEnd(g, v, ui, km);
    return;
  }
  drawTop(g, v, ui, km);
  if (ui.desk === "helpdesk") drawHelpdesk(g, v, ui, km);
  else drawWarroom(g, v, ui, km);
  drawStatus(g, v);
  drawFKeys(g, km);
  drawOverlay(g, v, ui, km);
}
