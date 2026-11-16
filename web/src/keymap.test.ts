import { describe, expect, it } from "vitest";
import { ACTIONS, actionFor, bind, comboOf, defaults, exportText, importText, load, save } from "./keymap";

const ev = (o: Partial<Parameters<typeof comboOf>[0]>) => ({ code: "KeyA", key: "a", ctrlKey: false, altKey: false, shiftKey: false, metaKey: false, ...o });

describe("keymap", () => {
  it("normalises combos", () => {
    expect(comboOf(ev({ code: "F3", key: "F3" }))).toBe("F3");
    expect(comboOf(ev({ code: "KeyK", key: "k", altKey: true }))).toBe("Alt+K");
    expect(comboOf(ev({ code: "ArrowLeft", key: "ArrowLeft", ctrlKey: true }))).toBe("Ctrl+←");
    expect(comboOf(ev({ code: "KeyR", key: "R", shiftKey: true }))).toBe("R");
    expect(comboOf(ev({ code: "ShiftLeft", key: "Shift" }))).toBeNull();
  });
  it("rebinding moves a combo and reports the unbound action", () => {
    const m = defaults();
    const r = bind(m, "desk_warroom", "F3");
    expect(r).toEqual({ ok: true, unbound: "desk_helpdesk" });
    expect(m["desk_helpdesk"]).toBe("");
    expect(actionFor(m, "F3")).toBe("desk_warroom");
  });
  it("refuses reserved keys", () => {
    const m = defaults();
    for (const c of ["F11", "F12", "Ctrl+W", "Cmd+W", "Esc"]) expect(bind(m, "coffee", c).ok).toBe(false);
    expect(m["coffee"]).toBe("F8");
  });
  it("persists through storage", () => {
    const mem = new Map<string, string>();
    const store = { getItem: (k: string) => mem.get(k) ?? null, setItem: (k: string, v: string) => void mem.set(k, v) };
    const m = defaults();
    bind(m, "desk_helpdesk", "Alt+K");
    save(store, m);
    expect(load(store)["desk_helpdesk"]).toBe("Alt+K");
  });
  it("export/import round-trips", () => {
    const m = defaults();
    bind(m, "coffee", "Alt+C");
    expect(importText(exportText(m))).toEqual(m);
    expect(ACTIONS.length).toBeGreaterThan(10);
  });
});
