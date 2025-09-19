# Flix

Libraries and tools for dealing with media metadata

## Various builds

- build: `cargo hack --feature-powerset build`
- clippy: `cargo hack --feature-powerset clippy -- -D warnings`
- test: `cargo hack --feature-powerset test`
- test old: `cargo +1.85 hack --feature-powerset test`
- fmt: `cargo fmt --check`
- docs: `RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features`
- install: `cargo install --path crates/cli`
- publish: `cargo publish --dry-run --workspace`
