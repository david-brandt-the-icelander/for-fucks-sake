import { Grid } from "./grid";
import { boot, Game } from "./game";
import type { Command, View } from "./game";
import * as KM from "./keymap";
import { ACTIONS } from "./keymap";
import { drawGame, drawOverlay, drawTitle } from "./screens";
import type { BestRun, UI } from "./screens";

const screenEl = document.getElementById("screen") as HTMLPreElement;
const stageEl = document.getElementById("stage") as HTMLDivElement;
const liveEl = document.getElementById("live") as HTMLDivElement;
const smallEl = document.getElementById("small") as HTMLDivElement;

const store = ((): Storage | null => {
  try {
    return window.localStorage;
  } catch {
    return null;
  }
})();

let km = KM.load(store);
const grid = new Grid();
let game: Game | null = null;
let view: View | null = null;
let logo = "";

const ui: UI = {
  screen: "title", desk: "helpdesk", sel: 0, opt: 0, overlay: null, cursor: 0, rantText: "", configSel: 0,
  capturing: false, configMsg: "", pressed: new Set(), best: loadBest(), hasSave: Game.hasSave(), copied: false, tooSmallNote: "",
};

function loadBest(): BestRun[] {
  try {
    return JSON.parse(store?.getItem("ffs.best.v1") ?? "[]") as BestRun[];
  } catch {
    return [];
  }
}
function saveBest(b: BestRun): void {
  const all = [...loadBest(), b].sort((a, c) => c.days - a.days || c.cash - a.cash).slice(0, 10);
  try {
    store?.setItem("ffs.best.v1", JSON.stringify(all));
  } catch {
    /* ignore */
  }
  ui.best = all;
}

// ---------- rendering ----------
function render(): void {
  if (ui.screen === "title") {
    grid.clear();
    drawTitle(grid, { logo }, ui, km);
    if (ui.overlay === "config") drawOverlay(grid, {} as View, ui, km);
  } else if (view) {
    drawGame(grid, view, ui, km);
  }
  screenEl.innerHTML = grid.html();
}

function fit(): void {
  const w = window.innerWidth;
  const h = window.innerHeight;
  const scale = Math.min(w / 1440, h / 810, 1.6);
  stageEl.style.transform = `scale(${scale})`;
  stageEl.style.left = `${Math.max(0, (w - 1440 * scale) / 2)}px`;
  stageEl.style.top = `${Math.max(0, (h - 810 * scale) / 2)}px`;
  smallEl.hidden = scale >= 0.75;
}

function announce(text: string): void {
  liveEl.textContent = "";
  window.setTimeout(() => (liveEl.textContent = text), 20);
}

// ---------- game flow ----------
function newRun(seed?: number): void {
  Game.clearSave();
  const params = new URLSearchParams(location.search);
  const s = seed ?? (params.get("seed") ? Number(params.get("seed")) : (Date.now() ^ (Math.random() * 0x7fffffff)) >>> 0);
  game = Game.fresh(s);
  startPlaying();
}

function startPlaying(): void {
  ui.screen = "game";
  ui.desk = "helpdesk";
  ui.sel = 0;
  ui.opt = 0;
  ui.overlay = null;
  ui.copied = false;
  refresh();
}

function refresh(): void {
  if (!game) return;
  view = game.view();
  ui.sel = Math.min(ui.sel, Math.max(0, view.tickets.length - 1));
  const t = view.tickets[ui.sel];
  ui.opt = Math.min(ui.opt, Math.max(0, (t?.options.length ?? 1) - 1));
  ui.hasSave = Game.hasSave();
  render();
}

let recorded = false;
function send(cmd: Command): boolean {
  if (!game) return false;
  const r = game.apply(cmd);
  if (!r.ok) {
    announce(r.error);
    return false;
  }
  for (const e of r.events) {
    if (e.type === "Toast") announce(e.text);
    if (e.type === "DayEnded") announce(`Day ${e.day} ends. ${e.quip}`);
  }
  ui.opt = 0;
  refresh();
  if (view?.ended && !recorded) {
    recorded = true;
    saveBest({ days: view.ended.days, cash: view.ended.cash, np: view.ended.np, kind: view.ended.kind });
    announce(`${view.ended.title}. ${view.ended.cause}`);
  }
  if (!view?.ended) recorded = false;
  return true;
}

// ---------- keyboard ----------
function printable(e: KeyboardEvent): string | null {
  if (e.ctrlKey || e.altKey || e.metaKey || e.isComposing) return null;
  return e.key.length === 1 ? e.key : null;
}

function onKey(e: KeyboardEvent): void {
  if (e.isComposing) return;
  const c = KM.comboOf(e);
  if (!c) return;
  // The game owns the keyboard: always stop the browser handling keys we know (F-keys, Ctrl combos in config...).
  const handled = handle(e, c);
  if (handled) e.preventDefault();
}

function handle(e: KeyboardEvent, c: string): boolean {
  // Title screen
  if (ui.screen === "title") return handleTitle(e, c);
  if (!game || !view) return false;
  // End screen
  if (view.ended) {
    if (c === "Enter") {
      newRun();
      return true;
    }
    if (KM.actionFor(km, c) === "copy") {
      void navigator.clipboard?.writeText(view.ended.share).catch(() => undefined);
      ui.copied = true;
      render();
      return true;
    }
    return KM.actionFor(km, c) !== null;
  }
  if (ui.overlay === "config") return configKeys(c);
  // Rant text entry
  if (ui.overlay === "rant") {
    if (c === "Esc") return close();
    if (c === "Enter") {
      ui.overlay = null;
      ui.rantText = "";
      send({ type: "Rant", send: true });
      return true;
    }
    if (c === "Ctrl+D") {
      ui.overlay = null;
      ui.rantText = "";
      send({ type: "Rant", send: false });
      return true;
    }
    if (c === "Backspace") ui.rantText = ui.rantText.slice(0, -1);
    else {
      const p = printable(e);
      if (p && ui.rantText.length < 400) ui.rantText += p;
      else return false;
    }
    render();
    return true;
  }
  if (c === "Esc") return close();
  // Overlay-specific keys first
  if (ui.overlay === "run") {
    if (c === "Y") {
      ui.overlay = null;
      send({ type: "Retire" });
      return true;
    }
    return KM.actionFor(km, c) === null ? true : doAction(KM.actionFor(km, c) as string);
  }
  if (ui.overlay === "scheme" || ui.overlay === "boast") {
    const n = ui.overlay === "scheme" ? view.schemes.length : view.stories.length;
    if (c === "↑") ui.cursor = (ui.cursor + n - 1) % Math.max(1, n);
    else if (c === "↓") ui.cursor = (ui.cursor + 1) % Math.max(1, n);
    else if (/^[1-9]$/.test(c) && Number(c) <= n) {
      ui.cursor = Number(c) - 1;
      return pickOverlay();
    } else if (c === "Enter") return pickOverlay();
    else {
      const a = KM.actionFor(km, c);
      return a ? doAction(a) : false;
    }
    render();
    return true;
  }
  if (ui.overlay) {
    const a = KM.actionFor(km, c);
    return a ? doAction(a) : true;
  }
  // No overlay: context keys
  const t = view.tickets[ui.sel];
  if (ui.desk === "helpdesk" && t) {
    if (/^[1-9]$/.test(c) && Number(c) <= t.options.length) {
      ui.opt = Number(c) - 1;
      render();
      return true;
    }
    if (c === "↑" || c === "↓") {
      ui.opt = (ui.opt + (c === "↑" ? t.options.length - 1 : 1)) % t.options.length;
      render();
      return true;
    }
    if (c === "Enter") {
      send({ type: "Resolve", ticket: t.id, option: ui.opt });
      return true;
    }
  }
  const a = KM.actionFor(km, c);
  return a ? doAction(a) : false;
}


function configKeys(c: string): boolean {
  if (ui.capturing) {
    if (c === "Esc") {
      ui.capturing = false;
      ui.configMsg = "cancelled";
    } else {
      const a = ACTIONS[ui.configSel];
      if (a) {
        const r = KM.bind(km, a.id, c);
        ui.configMsg = r.ok ? (r.unbound ? `${c} bound. ${r.unbound} is now unbound.` : `${c} bound.`) : r.reason;
        if (r.ok) {
          KM.save(store, km);
          ui.capturing = false;
        }
      }
    }
    render();
    return true;
  }
  if (c === "Esc") return close();
  if (c === "↑") ui.configSel = (ui.configSel + ACTIONS.length - 1) % ACTIONS.length;
  else if (c === "↓") ui.configSel = (ui.configSel + 1) % ACTIONS.length;
  else if (c === "Enter") {
    ui.capturing = true;
    ui.configMsg = "";
  } else if (c === "Del") {
    const a = ACTIONS[ui.configSel];
    if (a) km[a.id] = a.def;
    KM.save(store, km);
    ui.configMsg = "reset";
  } else if (c === "Ctrl+Del") {
    km = KM.defaults();
    KM.save(store, km);
    ui.configMsg = "all keys reset";
  } else if (KM.actionFor(km, c) === "config") return close();
  else return KM.actionFor(km, c) !== null;
  render();
  return true;
}

function handleTitle(e: KeyboardEvent, c: string): boolean {
  void e;
  if (ui.overlay === "config") return configKeys(c);
  if (KM.actionFor(km, c) === "config") return open("config");
  ui.pressed.add(c);
  const a = KM.actionFor(km, c);
  if (c === "Enter" && ui.hasSave) {
    game = Game.resume();
    if (game) startPlaying();
    else {
      ui.hasSave = false;
      newRun();
    }
    return true;
  }
  if (c === "Enter" || c === "Space") {
    newRun();
    return true;
  }
  render();
  return a !== null || /^F\d+$/.test(c);
}

function close(): boolean {
  if (ui.overlay) {
    ui.overlay = null;
    ui.capturing = false;
    render();
  }
  return true;
}

function pickOverlay(): boolean {
  if (!view) return true;
  if (ui.overlay === "scheme") {
    const s = view.schemes[ui.cursor];
    if (s && s.available) {
      ui.overlay = null;
      send({ type: "Scheme", id: s.id });
    }
    return true;
  }
  if (ui.overlay === "boast") {
    const s = view.stories[ui.cursor];
    if (s) {
      ui.overlay = null;
      ui.desk = "warroom";
      send({ type: "Boast", story: s.id });
    }
    return true;
  }
  return true;
}

function open(o: UI["overlay"]): boolean {
  ui.overlay = ui.overlay === o ? null : o;
  ui.cursor = 0;
  render();
  return true;
}

function doAction(a: string): boolean {
  if (!view) return false;
  switch (a) {
    case "help":
      if (ui.overlay !== "help") send({ type: "HelpOpened" });
      return open("help");
    case "np_log":
      return open("nplog");
    case "config":
      return open("config");
    case "run":
      return open("run");
    case "desk_helpdesk":
    case "desk_warroom":
      ui.desk = a === "desk_helpdesk" ? "helpdesk" : "warroom";
      ui.overlay = null;
      render();
      return true;
    case "coffee":
      ui.overlay = null;
      send({ type: "Coffee" });
      return true;
    case "rant":
      ui.rantText = "";
      return open("rant");
    case "scheme":
      return open("scheme");
    case "boast":
      return open("boast");
    case "end_day":
      ui.overlay = null;
      send({ type: "EndDay" });
      return true;
    case "next":
    case "prev": {
      const n = view.tickets.length;
      if (n) ui.sel = (ui.sel + (a === "next" ? 1 : n - 1)) % n;
      ui.opt = 0;
      render();
      return true;
    }
    case "dirt": {
      const t = view.tickets[ui.sel];
      if (t?.dirt) send({ type: "UseDirt", ticket: t.id });
      return true;
    }
    default:
      return false;
  }
}

window.addEventListener("keydown", onKey, true);
window.addEventListener("resize", fit);
window.addEventListener("beforeunload", (e) => {
  if (ui.screen === "game" && view && !view.ended) {
    e.preventDefault();
  }
});

// Test hooks (read-only).
(window as unknown as { __ffs: unknown }).__ffs = {
  text: () => grid.text(),
  view: () => view,
  keymap: () => km,
};

async function main(): Promise<void> {
  fit();
  await boot();
  // logo comes from the engine's content via a throwaway game
  logo = Game.fresh(1).view().logo;
  const params = new URLSearchParams(location.search);
  if (params.has("seed") && params.has("autostart")) newRun();
  else render();
  screenEl.focus();
}
void main();
