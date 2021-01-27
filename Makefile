default_target:

SELF_DIR := $(realpath $(dir $(abspath $(lastword $(MAKEFILE_LIST)))))

export PYO3_PYTHON = $(SELF_DIR)/python/hello/.venv/bin/python

install:
	$(MAKE) -C python/hello install
.PHONY: install

build:
	cargo build
	$(MAKE) -C python/hello post-build
.PHONY: build

test:
	cargo test
.PHONY: test
