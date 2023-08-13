

build:
	cargo build

test:
	cargo test

fmt:
	#cargo fmt
	cargo +nightly fmt

check:
	cargo check

fix:
	cargo fix --allow-dirty --allow-staged

clippy:
	cargo clippy

lint: check fix clippy

checkdep:
	cargo outdated