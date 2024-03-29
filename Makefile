.PHONY: build check ci clean fmt install install-rustfmt link lint release run test

BIN_NAME = commentective
CARGO = $(shell which cargo)

build:
	@$(CARGO) build

check:
	$(CARGO) check --release

clippy:
	$(CARGO) clippy

ci: lint check test

clean:
	rm -rf ./target

fmt:
	@$(CARGO) fmt

install:
	@cp ./target/release/$(BIN_NAME) /usr/local/bin/$(BIN_NAME)

link:
	@ln -sf ./target/debug/$(BIN_NAME) .

lint:
	$(CARGO) fmt --all -- --check

package:
	@$(CARGO) package

publish:
	@$(CARGO) publish

release:
	@$(CARGO) build --release

run:
	@RUST_BACKTRACE=1 $(CARGO) run

test:
	@$(CARGO) test --features dumb_terminal -- --nocapture
