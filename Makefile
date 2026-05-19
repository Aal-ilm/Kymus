all:
	cargo clean
	cargo nextest run

build:
	cargo build
docs:
	cargo docs --open
setup:
	cargo install --locked cargo-nextest
	rustup component add clippy
	rustup component add rustfmt
test:
	cargo nextest run
fmt:
	cargo fmt

run:
	cargo run -p kymus-cli
lint:
	cargo clippy
	cargo fmt --check