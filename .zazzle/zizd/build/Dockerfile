# The `build` step of for-fucks-sake's zizd pipeline: the WASM engine, the web tests and the game's build (the `web` job
# of .github/workflows/ci.yml, without Playwright, which is the `e2e` step), then the Worker bundled for the deploy.
# Pinned: Rust 1.100.0 with wasm32-unknown-unknown, wasm-bindgen-cli 0.2.130 (the crate's), Node 22.21.1, and wrangler
# 4.156.0 (release.yml's).
FROM rust:1.100.0-slim-bookworm
ARG NODE_VERSION=22.21.1
RUN apt-get update && apt-get install -y --no-install-recommends git make curl xz-utils ca-certificates \
 && rm -rf /var/lib/apt/lists/* \
 && rustup target add wasm32-unknown-unknown \
 && cargo install wasm-bindgen-cli --version 0.2.130 --locked && rm -rf /usr/local/cargo/registry \
 && case "$(dpkg --print-architecture)" in amd64) a=x64 ;; arm64) a=arm64 ;; *) echo "no node for this CPU" >&2; exit 1 ;; esac \
 && curl -fsSL "https://nodejs.org/dist/v${NODE_VERSION}/node-v${NODE_VERSION}-linux-${a}.tar.xz" | tar -xJ -C /usr/local --strip-components=1 \
 && npm install -g --no-audit --no-fund wrangler@4.156.0 && npm cache clean --force \
 && chmod -R a+rX /usr/local/rustup /usr/local/cargo
ENV RUSTUP_TOOLCHAIN=1.100.0
COPY run.sh /run.sh
CMD ["sh", "/run.sh"]
