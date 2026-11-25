# Playwright on a copy of the repository, with the build step's WASM engine in web/src/wasm (so no Rust here). The tests
# start the Vite dev server themselves (web/playwright.config.ts). Leaves nothing in /zazzle/output.
set -eu
export HOME=/tmp npm_config_cache=/tmp/.npm CI=1
art=${ZAZZLE_ARTIFACTS:-/zazzle/input/artifacts}/build
[ -d "$art/wasm" ] || { echo "the build step's artifacts have no wasm/ ($art)" >&2; exit 1; }
src=$(ls -d /zazzle/input/repos/*/for-fucks-sake | head -1)
cp -R "$src" /tmp/repo
rm -rf /tmp/repo/web/src/wasm && cp -R "$art/wasm" /tmp/repo/web/src/wasm
cd /tmp/repo/web
echo "e2e of ${ZAZZLE_COMMIT:-?} with node $(node --version)"
npm ci --no-audit --no-fund
npx playwright --version
npx playwright test --trace off
echo "the e2e tests passed"
