default_target:

include env.mk

install:
	cargo install cargo-post --root .
	$(MAKE) -C python/hello install
.PHONY: install

build:
	cargo post build --package hello-python
.PHONY: build

test:
	cargo test
.PHONY: test
