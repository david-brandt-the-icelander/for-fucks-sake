import { expect, test, type Page } from "@playwright/test";

// Keyboard-only: the only "interaction" besides keys is page.goto.
declare global {
  interface Window {
    __ffs: { text(): string; view(): { header: { np: number; fucks: number; day: number; susp: number }; today: { cover_gain: number }; schemes: { id: string; name: string; tier: number }[]; schemes_left: number; tickets: unknown[]; ended: unknown; hash: string } | null; keymap(): Record<string, string> };
  }
}

const text = (p: Page) => p.evaluate(() => window.__ffs.text());
const view = (p: Page) => p.evaluate(() => window.__ffs.view());

async function start(p: Page, seed = 7) {
  await p.goto(`/?seed=${seed}`);
  await p.evaluate(() => localStorage.clear());
  await p.reload();
  await p.waitForFunction(() => window.__ffs.text().includes("start a run"));
  await p.keyboard.press("Space");
  await p.waitForFunction(() => window.__ffs.view() !== null);
}

test("title to run, desktops switch with F3/F4", async ({ page }) => {
  await start(page);
  expect(await text(page)).toContain("INCIDENT #1");
  await page.keyboard.press("F4");
  expect(await text(page)).toContain("#war-room");
  await page.keyboard.press("F3");
  expect(await text(page)).toContain("RESPOND");
});

test("fits a 1440x790 window; tiny window shows the size message", async ({ page }) => {
  await start(page);
  const sizes = await page.evaluate(() => ({ sw: document.documentElement.scrollWidth, sh: document.documentElement.scrollHeight, w: innerWidth, h: innerHeight }));
  expect(sizes.sw).toBeLessThanOrEqual(sizes.w);
  expect(sizes.sh).toBeLessThanOrEqual(sizes.h);
  await page.setViewportSize({ width: 1000, height: 560 });
  await expect(page.locator("#small")).toBeVisible();
});

test("F1 costs a nerd point each time it opens; F2 is free and lists it", async ({ page }) => {
  await start(page);
  await page.keyboard.press("F1");
  expect(await text(page)).toContain("HELP");
  await page.keyboard.press("Escape");
  await page.keyboard.press("F1");
  await page.keyboard.press("Escape");
  await page.keyboard.press("F2");
  const t = await text(page);
  expect(t).toContain("NERD-POWER LOG");
  expect((t.match(/-1 {2}opened F1 help/g) ?? []).length).toBe(2);
  await page.keyboard.press("Escape");
  await page.keyboard.press("F2");
  await page.keyboard.press("Escape");
  await page.keyboard.press("F2");
  expect(((await text(page)).match(/opened F1 help/g) ?? []).length).toBe(2); // opening F2 is free
});

test("2 then Enter resolves an incident and shows its result", async ({ page }) => {
  await start(page);
  const first = await page.evaluate(() => (window.__ffs.view() as unknown as { tickets: { id: number }[] }).tickets[0]?.id);
  await page.keyboard.press("2");
  await page.keyboard.press("Enter");
  const v = await page.evaluate(() => window.__ffs.view() as unknown as { tickets: { id: number }[]; header: { toast: string } });
  expect(v.tickets.some((t) => t.id === first)).toBe(false);
  expect(v.header.toast.length).toBeGreaterThan(3);
});

test("rant: type and send, or delete", async ({ page }) => {
  await start(page);
  await page.keyboard.press("r");
  expect(await text(page)).toContain("THE UNSENT EMAIL");
  await page.keyboard.type("dear all, the printer is not a toy");
  expect(await text(page)).toContain("the printer is not a toy");
  await page.keyboard.press("Enter");
  expect(await text(page)).toContain("SEND");
  await page.keyboard.press("r");
  await page.keyboard.press("Control+d");
  expect(await text(page)).toContain("delete the draft");
});

test("keymap: rebind desk_warroom to Alt+K, persists over reload and resume", async ({ page }) => {
  await start(page);
  await page.keyboard.press("F9");
  expect(await text(page)).toContain("CONFIG: KEYMAP");
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown"); // desk_warroom
  await page.keyboard.press("Enter");
  await page.keyboard.press("Alt+k");
  expect(await text(page)).toContain("Alt+K bound");
  await page.keyboard.press("Escape");
  expect(await text(page)).toContain("Alt+K War-room");
  await page.keyboard.press("F8"); // a coffee, so there is a run to resume
  await page.reload();
  await page.waitForFunction(() => window.__ffs.text().includes("resume your run"));
  await page.keyboard.press("Enter"); // resume
  expect(await text(page)).toContain("Alt+K War-room");
  await page.keyboard.press("Alt+k");
  expect(await text(page)).toContain("#war-room");
});

test("keymap: conflicts unbind the old action; F11/F12 refused", async ({ page }) => {
  await start(page);
  await page.keyboard.press("F9");
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("Enter");
  await page.keyboard.press("F11");
  expect(await text(page)).toContain("reserved");
  await page.keyboard.press("F3"); // taken by desk_helpdesk
  expect(await text(page)).toContain("desk_helpdesk is now unbound");
  const km = await page.evaluate(() => window.__ffs.keymap());
  expect(km["desk_helpdesk"]).toBe("");
  expect(km["desk_warroom"]).toBe("F3");
});

test("autosave: reload mid-run resumes with an identical state hash", async ({ page }) => {
  await start(page);
  await page.keyboard.press("1");
  await page.keyboard.press("Enter");
  await page.keyboard.press("c");
  const h = (await view(page))!.hash;
  await page.reload();
  await page.waitForFunction(() => window.__ffs.text().includes("resume your run"));
  await page.keyboard.press("Enter");
  expect((await view(page))!.hash).toBe(h);
});

test("scheme then boast in the war-room earns nerd power", async ({ page }) => {
  await start(page);
  await page.keyboard.press("s");
  expect(await text(page)).toContain("SCHEMES");
  await page.keyboard.press("1");
  await page.keyboard.press("F4");
  await page.keyboard.press("b");
  expect(await text(page)).toContain("BOAST TO THE WAR-ROOM");
  await page.keyboard.press("Enter");
  const t = await text(page);
  expect(t).toContain("<you> I ");
});

test("T covers your tracks: a scheme's suspicion comes back down", async ({ page }) => {
  await start(page);
  expect(await text(page)).toContain("cover tracks [T]");
  await page.keyboard.press("s");
  await page.keyboard.press("1");
  const hot = (await view(page))?.header.susp ?? 0;
  expect(hot).toBeGreaterThan(0);
  const gain = (await view(page))?.today.cover_gain ?? 0;
  await page.keyboard.press("t");
  const v = await view(page);
  expect(v?.header.susp).toBe(Math.max(0, hot - gain));
  expect(v?.today.cover_gain).toBeLessThan(gain);
  expect(await text(page)).toContain("Suspicion -");
});

test("each scheme works once; the next rung up the ladder takes its slot", async ({ page }) => {
  await start(page);
  const before = (await view(page))?.schemes ?? [];
  const left = (await view(page))?.schemes_left ?? 0;
  expect(before.length).toBe(3);
  await page.keyboard.press("s");
  let t = await text(page);
  expect(t).toContain("[1-3] or [Enter] run it");
  expect(t).toContain(`${left} more in the pipeline`);
  for (const sc of before) expect(t).toContain(sc.name);
  await page.keyboard.press("2");
  const after = (await view(page))?.schemes ?? [];
  expect(after.map((s) => s.id)).not.toContain(before[1]?.id);
  expect(after[0]?.id).toBe(before[0]?.id);
  expect(after[2]?.id).toBe(before[2]?.id);
  expect(after[1]?.tier ?? 0).toBeGreaterThanOrEqual(before[1]?.tier ?? 0);
  await page.keyboard.press("s");
  t = await text(page);
  expect(t).not.toContain(before[1]?.name ?? "");
  expect(t).toContain(after[1]?.name ?? "?");
  expect(t).toContain(`${left - 1} more in the pipeline`);
});

test("F10 then Y ends the run; Enter starts a new one", async ({ page }) => {
  await start(page);
  await page.keyboard.press("F10");
  expect(await text(page)).toContain("TAKE THE MONEY AND RUN");
  await page.keyboard.press("y");
  const t = await text(page);
  expect(t).toContain("TOOK THE MONEY AND RAN");
  expect(t).toContain("Days survived");
  await page.keyboard.press("Enter");
  expect(await text(page)).toContain("INCIDENT #1");
});

test("a whole run to the meltdown, keyboard only", async ({ page }) => {
  await start(page, 3);
  for (let i = 0; i < 400; i++) {
    const v = await view(page);
    if (v?.ended) break;
    await page.keyboard.press("1");
    await page.keyboard.press("Enter");
    if (i % 5 === 4) await page.keyboard.press("e");
  }
  const t = await text(page);
  expect(t).toContain("Days survived");
  expect(t).toMatch(/GAME OVER|FIRED/);
  await page.keyboard.press("c"); // copy share card does not throw
});

test("no mouse needed: the only focusable thing is the game", async ({ page }) => {
  await start(page);
  const tag = await page.evaluate(() => document.activeElement?.tagName);
  expect(["PRE", "BODY"]).toContain(tag);
});
