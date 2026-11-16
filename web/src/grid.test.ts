import { describe, expect, it } from "vitest";
import { Grid, meter, money, sparkline, wrap } from "./grid";

describe("grid helpers", () => {
  it("wraps words to width", () => {
    expect(wrap("the quick brown fox", 9)).toEqual(["the quick", "brown fox"]);
  });
  it("splits overlong words", () => {
    expect(wrap("abcdefghij", 4)).toEqual(["abcd", "efgh", "ij"]);
  });
  it("draws a meter", () => {
    expect(meter(50, 100, 10)).toBe("█████·····");
    expect(meter(0, 100, 4)).toBe("····");
  });
  it("formats money", () => {
    expect(money(1234)).toBe("$1,234");
  });
  it("sparkline length matches", () => {
    expect(Array.from(sparkline([0, 50, 100])).length).toBe(3);
  });
  it("box and put clip safely", () => {
    const g = new Grid();
    g.put(158, 0, "abcd");
    g.box(0, 0, 4, 3);
    expect(g.text().split("\n")[0]).toContain("┌──┐");
  });
});
