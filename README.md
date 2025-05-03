# Flix

Libraries and tools for dealing with media metadata

## Various builds

- build: `cargo hack --feature-powerset build`
- clippy: `cargo hack --feature-powerset clippy -- -D warnings`
- fmt: `cargo fmt --check`
- docs: `RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features`
- install: `cargo install --path crates/cli`
- publish: `cargo publish --dry-run -p flix-tmdb`
- publish: `cargo publish --dry-run -p flix`
- publish: `cargo publish --dry-run -p flix-cli`
