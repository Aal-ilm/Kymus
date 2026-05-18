all:
	cargo clean
	cargo nextest run

build:
	cargo build
docs:
	cargo docs --open