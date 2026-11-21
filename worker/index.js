// The game is static files (wrangler.jsonc, "assets"). This script sees each request first:
//   - it sends www.forfuckssake.today to the bare domain;
//   - it signs people in through icelander.net's login service (the shared library in vendor/icelander-auth, app
//     "ffs", host-only cookie ffs_session): /auth/sign-in, /auth/callback, /auth/sign-out, /auth/me;
//   - it asks a visitor who is not signed in to sign in before the page is shown (a navigation to an HTML page; the
//     scripts, styles and WASM stay public, they hold nothing of anyone's), and sends them back to where they were;
//   - everything else goes to the assets.
// The game is unranked: signing in is all that is asked, whatever the person's rank at icelander.net.
// Without the ACCOUNTS binding or ACCOUNTS_JWKS nobody can sign in, so nobody gets the page (fail closed).
import { createAuth } from "./vendor/icelander-auth/index.js";

export const auth = createAuth({ app: "ffs", cookie: "ffs_session", home: "/" });

/** A navigation to a page: a browser asking for HTML at the root or at an .html path. */
export function isPage(request, url) {
  if (request.method !== "GET" && request.method !== "HEAD") return false;
  if (!(request.headers.get("accept") ?? "").includes("text/html")) return false;
  return url.pathname === "/" || url.pathname.endsWith(".html") || !/\.[A-Za-z0-9]{1,8}$/.test(url.pathname);
}

export default {
  async fetch(request, env) {
    const url = new URL(request.url);
    if (url.hostname === "www.forfuckssake.today") {
      url.hostname = "forfuckssake.today";
      return Response.redirect(url.toString(), 301);
    }
    const answered = await auth.handle(request, env);
    if (answered) return answered;
    if (isPage(request, url)) {
      const user = await auth.currentUser(request, env);
      if (!user) return new Response(null, { status: 302, headers: { location: auth.signInPath(url.pathname + url.search), "cache-control": "no-store" } });
      const page = await env.ASSETS.fetch(request);
      const out = new Response(page.body, page);
      out.headers.set("cache-control", "private, no-store");
      return out;
    }
    return env.ASSETS.fetch(request);
  },
};
