# Flix

Libraries and tools for dealing with media metadata

## Development

Run the full validation suite before committing:

```sh
just all
just install
```

##### Publish

- `cargo semver-checks --all-features`
- `cargo publish --dry-run -p <crate>`
- `cargo publish --dry-run --workspace`

## Building flix.db

`./catalog/flix.sh`
