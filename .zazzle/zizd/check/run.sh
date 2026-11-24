# The Rust checks, on a copy of the repository (the input is read-only), with cargo's registry and target directory in
# /tmp. Leaves nothing in /zazzle/output. cargo test writes the ts-rs types into web/src/gen (.cargo/config.toml), so
# they are compared with the committed ones last, as the workflow does.
set -eu
export HOME=/tmp CARGO_HOME=/tmp/cargo CARGO_TARGET_DIR=/tmp/target CARGO_TERM_COLOR=never CI=1
src=$(ls -d /zazzle/input/repos/*/for-fucks-sake | head -1)
cp -R "$src" /tmp/repo
cd /tmp/repo
git config --global --add safe.directory '*'
step() { echo "── $*"; t0=$(date +%s); "$@"; echo "── ok ($(( $(date +%s) - t0 )) s)"; }
echo "checking ${ZAZZLE_COMMIT:-?} with $(rustc --version)"
step cargo fmt --check
step cargo clippy --all-targets -- -D warnings
step cargo test
step cargo run -q -p ffs-cli -- content validate
step cargo run -q --release -p ffs-cli -- bot --runs 100 --assert
step git diff --exit-code web/src/gen
echo "the Rust checks passed"
