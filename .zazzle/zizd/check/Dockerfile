# The `check` step of for-fucks-sake's zizd pipeline: the `rust` job of .github/workflows/ci.yml (fmt, clippy, tests,
# content validation, the bot replays, and the generated TypeScript types being up to date).
# Pinned: Rust 1.100.0 (the workflow took stable).
FROM rust:1.100.0-slim-bookworm
RUN apt-get update && apt-get install -y --no-install-recommends git ca-certificates && rm -rf /var/lib/apt/lists/* \
 && rustup component add clippy rustfmt && chmod -R a+rX /usr/local/rustup /usr/local/cargo
# RUSTUP_TOOLCHAIN holds every run to this Rust, even if a branch adds a rust-toolchain file.
ENV RUSTUP_TOOLCHAIN=1.100.0
COPY run.sh /run.sh
CMD ["sh", "/run.sh"]
