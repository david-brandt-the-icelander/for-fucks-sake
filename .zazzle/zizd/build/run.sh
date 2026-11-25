# Build the game and lay out what `e2e` tests and `deploy` deploys:
#   web/dist/      the built game (vite), the Worker's static assets (wrangler.jsonc "assets": web/dist)
#   bundle/        the Worker (wrangler deploy --dry-run --outdir), kept apart from the assets
#   wrangler.jsonc the repository's configuration
#   wasm/          web/src/wasm, the engine as WASM, for the e2e step
#   VERSION        what the title bar says (FFS_VERSION): `git describe --tags --always`, as `make deploy` has it
#   BUILD          the commit
set -eu
export HOME=/tmp CARGO_HOME=/tmp/cargo CARGO_TERM_COLOR=never npm_config_cache=/tmp/.npm CI=1 WRANGLER_SEND_METRICS=false
out=${ZAZZLE_OUTPUT:-/zazzle/output}
src=$(ls -d /zazzle/input/repos/*/for-fucks-sake | head -1)
cp -R "$src" /tmp/repo
cd /tmp/repo
git config --global --add safe.directory '*'
step() { echo "── $*"; t0=$(date +%s); "$@"; echo "── ok ($(( $(date +%s) - t0 )) s)"; }
version=$(git describe --tags --always)
echo "building ${ZAZZLE_COMMIT:-?} as $version with $(rustc --version), node $(node --version), wrangler $(wrangler --version)"
# The cargo target directory is /tmp/repo/target, inside /tmp: the Makefile names it.
step make wasm
cd web
step npm ci --no-audit --no-fund
step npm run typecheck
step npm test
FFS_VERSION=$version step npm run build
cd ..
if grep -qE '"build"[[:space:]]*:' wrangler.jsonc; then echo "wrangler.jsonc has a build command; the pipeline does not run one" >&2; exit 1; fi
step wrangler deploy --dry-run -c wrangler.jsonc --outdir "$out/bundle"
rm -f "$out/bundle/README.md"   # wrangler's note about the folder, not part of the Worker
mkdir -p "$out/web"
cp -R web/dist "$out/web/dist"
cp -R web/src/wasm "$out/wasm"
cp wrangler.jsonc "$out/"
echo "$version" >"$out/VERSION"
git rev-parse HEAD >"$out/BUILD"
ls -lR "$out"
echo "for-fucks-sake $version is built"
