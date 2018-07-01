.PHONY: build clean link install release run

BIN_NAME = detector
CARGO=$(shell which cargo)

build:
	@$(CARGO) build

ci: install-fmt lint build test

install-fmt:
	@rustup component add rustfmt-preview

clean:
	rm -rf ./target

fmt:
	@$(CARGO) fmt

lint:
	cargo fmt --all -- --check

link:
	@ln -sf ./target/debug/$(BIN_NAME) .

install:
	@cp ./target/release/$(BIN_NAME) /usr/local/bin/$(BIN_NAME)

release:
	@$(CARGO) build --release

run:
	@RUST_BACKTRACE=1 $(CARGO) run

test:
	@$(CARGO) test
