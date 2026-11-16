// Thin typed wrapper over the WASM engine. No rules here.
import init, { Game as WasmGame } from "./wasm/ffs_wasm.js";
import type { Command } from "./gen/Command";
import type { Event } from "./gen/Event";
import type { View } from "./gen/View";

export type { Command, Event, View };
export type { Chip } from "./gen/Chip";
export type { OptionView } from "./gen/OptionView";
export type { TicketView } from "./gen/TicketView";

export type ApplyResult = { ok: true; events: Event[] } | { ok: false; error: string };

const SAVE_KEY = "ffs.save.v1";

export async function boot(): Promise<void> {
  await init();
}

export class Game {
  private g: WasmGame;
  constructor(g: WasmGame) {
    this.g = g;
  }
  static fresh(seed: number): Game {
    return new Game(new WasmGame(seed >>> 0));
  }
  /** Resume a saved run, or null if none/invalid (e.g. produced by another game version). */
  static resume(): Game | null {
    try {
      const raw = localStorage.getItem(SAVE_KEY);
      if (!raw) return null;
      return new Game(WasmGame.load(raw));
    } catch {
      return null;
    }
  }
  static hasSave(): boolean {
    try {
      return localStorage.getItem(SAVE_KEY) !== null;
    } catch {
      return false;
    }
  }
  static clearSave(): void {
    try {
      localStorage.removeItem(SAVE_KEY);
    } catch {
      /* ignore */
    }
  }
  apply(cmd: Command): ApplyResult {
    const r = JSON.parse(this.g.apply(JSON.stringify(cmd))) as { ok: boolean; events?: Event[]; error?: string };
    if (!r.ok) return { ok: false, error: r.error ?? "error" };
    try {
      localStorage.setItem(SAVE_KEY, this.g.save());
    } catch {
      /* storage unavailable: the run just won't resume */
    }
    return { ok: true, events: r.events ?? [] };
  }
  view(): View {
    return JSON.parse(this.g.view()) as View;
  }
  seed(): number {
    return this.g.seed();
  }
  save(): string {
    return this.g.save();
  }
}
