# Deploy for-fucks-sake from the build step's artifacts, as release.yml does on a v* tag, in the release environment
# only (the release branch takes the place of the tags): refuse outside it, without CLOUDFLARE_API_TOKEN or without the
# artifacts; then wrangler deploy of the bundled Worker as it is (--no-bundle), with web/dist as its assets. A halt
# request (ZAZZLE_HALT_FILE) is honoured before the deploy, with exit 75. No migrations: the game has no database.
#
# Rolling back is a rerun of this step for an older commit, with that commit's artifacts from z3 (README.md here).
#
# DEPLOY_DRY_RUN=1, for trying the step by hand only (no pipeline names it): no credentials needed, and
# `wrangler deploy --dry-run` instead of the deploy.
set -eu
export HOME=/tmp npm_config_cache=/tmp/.npm CI=1 WRANGLER_SEND_METRICS=false
art=${ZAZZLE_ARTIFACTS:-/zazzle/input/artifacts}/build
dry=${DEPLOY_DRY_RUN:-}
halted() { if [ -n "${ZAZZLE_HALT_FILE:-}" ] && [ -f "$ZAZZLE_HALT_FILE" ]; then echo "halting before $1: $(cat "$ZAZZLE_HALT_FILE")"; exit 75; fi; }
[ "${ZAZZLE_ENV:-}" = release ] || { echo "refusing: forfuckssake.today is deployed from the release environment only (ZAZZLE_ENV=${ZAZZLE_ENV:-none})" >&2; exit 1; }
if [ -z "$dry" ] && [ -z "${CLOUDFLARE_API_TOKEN:-}" ]; then echo "refusing: CLOUDFLARE_API_TOKEN is needed" >&2; exit 1; fi
for f in bundle/index.js web/dist/index.html wrangler.jsonc; do
  [ -e "$art/$f" ] || { echo "refusing: the build step's artifacts have no $f ($art)" >&2; exit 1; }
done
rm -rf /tmp/deploy && cp -R "$art" /tmp/deploy
cd /tmp/deploy
if grep -qE '"build"[[:space:]]*:' wrangler.jsonc; then echo "refusing: wrangler.jsonc has a build command" >&2; exit 1; fi
echo "deploying for-fucks-sake $(cat VERSION 2>/dev/null || echo '?') ($(cat BUILD 2>/dev/null || echo '?')) with wrangler $(wrangler --version)${dry:+ (dry run)}"
halted "the deploy"
if [ -n "$dry" ]; then
  wrangler deploy bundle/index.js --no-bundle -c wrangler.jsonc --dry-run
else
  wrangler deploy bundle/index.js --no-bundle -c wrangler.jsonc
fi
echo "for-fucks-sake deployed${dry:+ (dry run)}"
