// The Worker in front of the game (worker/index.js): www goes to the bare domain, /auth/* is the login service's
// sign-in, a page needs a signed-in visitor and everything else is public.
import { describe, expect, it } from "vitest";
// @ts-expect-error -- plain JavaScript, not part of the web build
import worker from "../../worker/index.js";

const JWKS = JSON.stringify({ keys: [{ kty: "OKP", crv: "Ed25519", kid: "k2026a", x: "WIEomi0LTalJvQndhD7abl8ty3YcliQ0vEz4XjLXayc" }] });
const assets = { fetch: async (r: Request) => new Response(new URL(r.url).pathname.endsWith(".js") ? "js" : "<html>game</html>", { headers: { "content-type": "text/html" } }) };
const person = { id: "a-1", name: "Ann", emails: ["a@example.com"], roles: [] };
const env = (signedIn: boolean) => ({ ASSETS: assets, ACCOUNTS_JWKS: JWKS, ACCOUNTS: { session: async (v: string, app: string) => (signedIn && v.length > 20 && app === "ffs" ? person : null) } });
const get = (path: string, headers: Record<string, string> = {}, host = "https://forfuckssake.today") => new Request(host + path, { headers: { accept: "text/html", ...headers } });

describe("the worker in front of the game", () => {
  it("sends www to the bare domain", async () => {
    const r = await worker.fetch(get("/x?y=1", {}, "https://www.forfuckssake.today"), env(true));
    expect(r.status).toBe(301);
    expect(r.headers.get("location")).toBe("https://forfuckssake.today/x?y=1");
  });

  it("sends a visitor who is not signed in to icelander.net's sign-in, and brings them back to the same place", async () => {
    const r = await worker.fetch(get("/?seed=7"), env(false));
    expect(r.status).toBe(302);
    expect(r.headers.get("location")).toBe("/auth/sign-in?return=%2F%3Fseed%3D7");
    const s = await worker.fetch(get("/auth/sign-in?return=%2F%3Fseed%3D7"), env(false));
    const to = new URL(s.headers.get("location")!);
    expect(to.origin + to.pathname).toBe("https://icelander.net/accounts/sign-in");
    expect(to.searchParams.get("app")).toBe("ffs");
    expect(to.searchParams.get("callback")).toBe("https://forfuckssake.today/auth/callback");
    expect(to.searchParams.get("return")).toBe("/?seed=7");
    expect(s.headers.getSetCookie().join(";")).toMatch(/ffs_session_state=.*HttpOnly.*Secure/);
  });

  it("never sends anyone to another site after signing in", async () => {
    const s = await worker.fetch(get("/auth/sign-in?return=https://evil.example/"), env(false));
    expect(new URL(s.headers.get("location")!).searchParams.get("return")).toBe("/");
  });

  it("shows the game to a signed-in visitor, privately", async () => {
    const r = await worker.fetch(get("/", { cookie: `ffs_session=${"x".repeat(40)}` }), env(true));
    expect(r.status).toBe(200);
    expect(await r.text()).toContain("game");
    expect(r.headers.get("cache-control")).toBe("private, no-store");
  });

  it("keeps scripts, styles and other assets public: they hold nothing of anyone's", async () => {
    const r = await worker.fetch(get("/assets/app.js", { accept: "*/*" }), env(false));
    expect(r.status).toBe(200);
    expect(await r.text()).toBe("js");
  });

  it("fails closed: with no way to ask the login service nobody gets the page", async () => {
    const r = await worker.fetch(get("/", { cookie: `ffs_session=${"x".repeat(40)}` }), { ASSETS: assets });
    expect(r.status).toBe(302);
  });

  it("only the owner's own login service is trusted: a forged cookie is nobody", async () => {
    const r = await worker.fetch(get("/", { cookie: "ffs_session=short" }), env(true));
    expect(r.status).toBe(302);
  });
});
