// Keymap: one combo per action. Persisted in localStorage. UI-only (no game rules).

export interface ActionDef {
  id: string;
  label: string;
  def: string;
  hint?: string;
}

export const ACTIONS: ActionDef[] = [
  { id: "help", label: "Help (costs 1 NP)", def: "F1" },
  { id: "np_log", label: "Nerd-point log", def: "F2" },
  { id: "desk_helpdesk", label: "Desktop: Helpdesk", def: "F3" },
  { id: "desk_warroom", label: "Desktop: War-room", def: "F4" },
  { id: "coffee", label: "Coffee", def: "F8" },
  { id: "config", label: "Config / keymap", def: "F9" },
  { id: "run", label: "Take the money and run", def: "F10" },
  { id: "next", label: "Next ticket", def: "N" },
  { id: "prev", label: "Previous ticket", def: "P" },
  { id: "rant", label: "Rant (unsent email)", def: "R" },
  { id: "scheme", label: "Schemes", def: "S" },
  { id: "cover", label: "Cover your tracks (-susp)", def: "T" },
  { id: "dirt", label: "Play dirt", def: "D" },
  { id: "boast", label: "Boast (war-room)", def: "B" },
  { id: "end_day", label: "Go home (end day)", def: "E" },
  { id: "copy", label: "Copy share card (end screen)", def: "C" },
];

/** Keys that can never be bound. */
export const REFUSED = new Set(["F11", "F12", "Ctrl+W", "Ctrl+T", "Ctrl+N", "Ctrl+Q", "Cmd+W", "Cmd+T", "Cmd+N", "Cmd+Q", "Alt+F4", "Ctrl+Tab", "Esc", "Enter", "Tab"]);

export type Keymap = Record<string, string>;

export function defaults(): Keymap {
  const k: Keymap = {};
  for (const a of ACTIONS) k[a.id] = a.def;
  return k;
}

/** Normalise a KeyboardEvent-like object to a combo string, or null for bare modifiers. */
export function comboOf(e: { code: string; key: string; ctrlKey: boolean; altKey: boolean; shiftKey: boolean; metaKey: boolean }): string | null {
  const code = e.code;
  if (/^(Shift|Control|Alt|Meta)(Left|Right)$/.test(code) || e.key === "Dead") return null;
  let key: string;
  if (/^Key[A-Z]$/.test(code)) key = code.slice(3);
  else if (/^Digit[0-9]$/.test(code)) key = code.slice(5);
  else if (/^Numpad[0-9]$/.test(code)) key = code.slice(6);
  else {
    const map: Record<string, string> = {
      ArrowLeft: "←", ArrowRight: "→", ArrowUp: "↑", ArrowDown: "↓", Space: "Space", Enter: "Enter", NumpadEnter: "Enter",
      Tab: "Tab", Escape: "Esc", Delete: "Del", Backspace: "Backspace", PageUp: "PgUp", PageDown: "PgDn", Home: "Home", End: "End",
    };
    if (map[code]) key = map[code] as string;
    else if (/^F([1-9]|1[0-2])$/.test(code)) key = code;
    else key = e.key.length === 1 ? e.key.toUpperCase() : e.key;
  }
  const mods: string[] = [];
  if (e.ctrlKey) mods.push("Ctrl");
  if (e.altKey) mods.push("Alt");
  const printable = key.length === 1;
  // Shift is only part of the combo for non-printable keys or alongside another modifier.
  if (e.shiftKey && (!printable || e.ctrlKey || e.altKey || e.metaKey)) mods.push("Shift");
  if (e.metaKey) mods.push("Cmd");
  return [...mods, key].join("+");
}

export type BindResult = { ok: true; unbound?: string } | { ok: false; reason: string };

export function bind(map: Keymap, action: string, combo: string): BindResult {
  if (REFUSED.has(combo)) return { ok: false, reason: `${combo} is reserved and cannot be bound` };
  if (/^[0-9]$/.test(combo)) return { ok: false, reason: "digits pick options and cannot be rebound" };
  if (/^[↑↓←→]$|^(PgUp|PgDn|Home|End)$/.test(combo)) return { ok: false, reason: `${combo} is used for list navigation` };
  let unbound: string | undefined;
  for (const [id, c] of Object.entries(map)) {
    if (c === combo && id !== action) {
      map[id] = "";
      unbound = id;
    }
  }
  map[action] = combo;
  return unbound ? { ok: true, unbound } : { ok: true };
}

export function actionFor(map: Keymap, combo: string): string | null {
  for (const [id, c] of Object.entries(map)) if (c === combo) return id;
  return null;
}

const KEY = "ffs.keymap.v1";

export function load(store: Pick<Storage, "getItem"> | null): Keymap {
  const m = defaults();
  try {
    const raw = store?.getItem(KEY);
    if (raw) {
      const o = JSON.parse(raw) as Record<string, unknown>;
      for (const a of ACTIONS) if (typeof o[a.id] === "string") m[a.id] = o[a.id] as string;
    }
  } catch {
    /* storage unavailable or corrupt: defaults */
  }
  return m;
}

export function save(store: Pick<Storage, "setItem"> | null, m: Keymap): void {
  try {
    store?.setItem(KEY, JSON.stringify(m));
  } catch {
    /* ignore */
  }
}

export function exportText(m: Keymap): string {
  return "# FFS! keymap v1\n" + ACTIONS.map((a) => `${a.id.padEnd(14)}= ${m[a.id] ?? ""}`).join("\n") + "\n";
}

export function importText(text: string): Keymap {
  const m = defaults();
  for (const line of text.split("\n")) {
    const mm = /^\s*([a-z_]+)\s*=\s*(.*?)\s*$/.exec(line);
    if (mm && ACTIONS.some((a) => a.id === mm[1])) m[mm[1] as string] = mm[2] as string;
  }
  return m;
}
