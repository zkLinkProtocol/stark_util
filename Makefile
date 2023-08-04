

build:
	cargo build

test:
	cargo test

fmt:
	#cargo fmt
	cargo +nightly fmt


fix:
	cargo fix --allow-dirty


clippy:
	cargo clippy


lint: fix clippy


checkdep:
	cargo outdated