.PHONY: build ci clean fmt install install-fmt link lint release run test

BIN_NAME = commentective
CARGO=$(shell which cargo)

build:
	@$(CARGO) build

ci: install-rustfmt lint build test

clean:
	rm -rf ./target

fmt:
	@$(CARGO) fmt

install:
	@cp ./target/release/$(BIN_NAME) /usr/local/bin/$(BIN_NAME)

install-rustfmt:
	@rustup component add rustfmt-preview

link:
	@ln -sf ./target/debug/$(BIN_NAME) .

lint:
	cargo fmt --all -- --check

release:
	@$(CARGO) build --release

run:
	@RUST_BACKTRACE=1 $(CARGO) run

test:
	@$(CARGO) test -- --nocapture
