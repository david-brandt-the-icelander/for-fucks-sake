.PHONY: dev test wasm content balance replays web-test e2e build types

wasm:
	cargo build -p ffs-wasm --target wasm32-unknown-unknown --release
	wasm-bindgen --target web --out-dir web/src/wasm target/wasm32-unknown-unknown/release/ffs_wasm.wasm

types:
	cargo test -p ffs-core --test engine -- --list >/dev/null  # ts-rs exports on `cargo test`

dev: wasm
	cd web && npm run dev

content:
	cargo run -q -p ffs-cli -- content validate

balance:
	cargo run -q --release -p ffs-cli -- bot --runs 400 --assert

# Regenerate the replay corpus (do this in the same commit as any intentional rules/content change).
replays:
	rm -f tests/replays/*.json
	for s in 1 2 3 4 5 6 7 8; do cargo run -q --release -p ffs-cli -- record --strategy clever --seed $$s --out tests/replays/clever_$$s.json; done
	for s in 11 12 13 14; do cargo run -q --release -p ffs-cli -- record --strategy random --seed $$s --out tests/replays/random_$$s.json; done
	for s in 21 22; do cargo run -q --release -p ffs-cli -- record --strategy lazy --seed $$s --out tests/replays/lazy_$$s.json; done

web-test:
	cd web && npm test

e2e: wasm
	cd web && npx playwright test

build: wasm
	cd web && npm run build

test: wasm
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings
	cargo test
	cd web && npm run typecheck && npm test
