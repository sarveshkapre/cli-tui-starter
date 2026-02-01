SHELL := /bin/bash

.PHONY: setup dev test lint typecheck build check release fmt audit

setup:
	rustup component add rustfmt clippy
	cargo install cargo-audit --locked

dev:
	cargo run -- demo

test:
	cargo test

lint:
	cargo clippy --all-targets --all-features -- -D warnings

typecheck:
	cargo check --all-targets

fmt:
	cargo fmt --all -- --check

audit:
	cargo audit

build:
	cargo build --release

check: fmt lint typecheck test build audit

release: build
