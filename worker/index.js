// The game is static files (wrangler.jsonc, "assets"). This script sees each request first: it sends
// www.forfuckssake.today to the bare domain, and hands everything else to the assets.
export default {
  async fetch(request, env) {
    const url = new URL(request.url);
    if (url.hostname === "www.forfuckssake.today") {
      url.hostname = "forfuckssake.today";
      return Response.redirect(url.toString(), 301);
    }
    return env.ASSETS.fetch(request);
  },
};
