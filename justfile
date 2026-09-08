set minimum-version := '1.55.0'
set default-list

alias update := update-min

all:
	just all-min
	just all-max
all-min: update-min build lint test example doc format
all-max: update-max build lint test example doc format

update-min:
	cargo +nightly update -Z direct-minimal-versions

update-max:
	cargo update

build:
	cargo hack --feature-powerset build

lint:
	cargo hack --feature-powerset clippy -- -D warnings
	cargo hack --feature-powerset clippy --tests -- -D warnings

test:
	cargo hack --feature-powerset test
	cargo +1.94 hack --feature-powerset test
	cargo +nightly hack --feature-powerset miri test

example:
	# cargo run --example=<example> --features=<req_features>

doc *args:
	RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc \
		--workspace --all-features --document-private-items \
		--lib --bins --no-deps --locked \
		{{ args }}

format:
	cargo fmt --check

install:
	cargo install --path crates/cli
	cargo install --path crates/cli-mux
