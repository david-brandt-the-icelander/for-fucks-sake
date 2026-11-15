// A 160x45 character grid with style codes, rendered to one <pre>.
export const COLS = 160;
export const ROWS = 45;

/** Style codes: n normal, b bright, d dim, a art, r rule, x alert, i inverted, j dim-inverted, s selected, k alert-inverted, f bar background */
export type Style = "n" | "b" | "d" | "a" | "r" | "x" | "i" | "j" | "s" | "k" | "f";

export class Grid {
  ch: string[][];
  st: Style[][];
  constructor() {
    this.ch = [];
    this.st = [];
    this.clear();
  }
  clear(): void {
    this.ch = Array.from({ length: ROWS }, () => Array<string>(COLS).fill(" "));
    this.st = Array.from({ length: ROWS }, () => Array<Style>(COLS).fill("n"));
  }
  put(x: number, y: number, text: string, style: Style = "n"): void {
    if (y < 0 || y >= ROWS) return;
    const chars = Array.from(text);
    for (let i = 0; i < chars.length; i++) {
      const cx = x + i;
      if (cx < 0 || cx >= COLS) continue;
      (this.ch[y] as string[])[cx] = chars[i] as string;
      (this.st[y] as Style[])[cx] = style;
    }
  }
  /** Style-only fill of a row range. */
  fill(x: number, y: number, w: number, style: Style, ch = " "): void {
    for (let i = 0; i < w; i++) this.put(x + i, y, ch, style);
  }
  /** Text right-aligned so that it ends at x+w. */
  right(x: number, y: number, w: number, text: string, style: Style = "n"): void {
    this.put(x + w - Array.from(text).length, y, text, style);
  }
  box(x: number, y: number, w: number, h: number, title = "", style: Style = "r"): void {
    const top = "┌" + "─".repeat(w - 2) + "┐";
    this.put(x, y, top, style);
    for (let j = 1; j < h - 1; j++) {
      this.put(x, y + j, "│", style);
      this.put(x + w - 1, y + j, "│", style);
    }
    this.put(x, y + h - 1, "└" + "─".repeat(w - 2) + "┘", style);
    if (title) this.put(x + 2, y, ` ${title} `, "n");
  }
  /** Clear the interior and draw a box (overlays). */
  panel(x: number, y: number, w: number, h: number, title = ""): void {
    for (let j = 0; j < h; j++) this.fill(x, y + j, w, "n");
    this.box(x, y, w, h, title);
  }
  /** Draw multi-line art; returns lines used. */
  art(x: number, y: number, art: string, style: Style = "a"): number {
    const lines = art.replace(/\n$/, "").split("\n");
    lines.forEach((l, i) => this.put(x, y + i, l, style));
    return lines.length;
  }
  text(): string {
    return this.ch.map((r) => r.join("").replace(/\s+$/, "")).join("\n");
  }
  html(): string {
    const out: string[] = [];
    for (let y = 0; y < ROWS; y++) {
      let run = "";
      let cur: Style | null = null;
      const row = this.ch[y] as string[];
      const srow = this.st[y] as Style[];
      for (let x = 0; x < COLS; x++) {
        const s = srow[x] as Style;
        if (s !== cur) {
          if (cur !== null) out.push(span(cur, run));
          cur = s;
          run = "";
        }
        run += row[x];
      }
      if (cur !== null) out.push(span(cur, run));
      out.push("\n");
    }
    return out.join("");
  }
}

function esc(s: string): string {
  return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
}
function span(s: Style, text: string): string {
  return `<span class="${s}">${esc(text)}</span>`;
}

/** Greedy word wrap to a width. */
export function wrap(text: string, width: number): string[] {
  const out: string[] = [];
  for (const para of text.split("\n")) {
    let line = "";
    for (const word of para.split(/\s+/).filter(Boolean)) {
      if (!line) line = word;
      else if (Array.from(line + " " + word).length <= width) line += " " + word;
      else {
        out.push(line);
        line = word;
      }
      while (Array.from(line).length > width) {
        out.push(Array.from(line).slice(0, width).join(""));
        line = Array.from(line).slice(width).join("");
      }
    }
    out.push(line);
  }
  return out;
}

export function meter(value: number, max: number, cells: number): string {
  const lit = Math.max(0, Math.min(cells, Math.round((value / max) * cells)));
  return "█".repeat(lit) + "·".repeat(cells - lit);
}

const BARS = "▁▂▃▄▅▆▇█";
export function sparkline(values: number[], max = 100): string {
  return values
    .map((v) => BARS[Math.max(0, Math.min(BARS.length - 1, Math.floor((v / max) * (BARS.length - 1))))])
    .join("");
}

export function money(n: number): string {
  return "$" + n.toLocaleString("en-US");
}
