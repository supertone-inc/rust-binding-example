default_target:

install:
	$(MAKE) -C python/hello install
.PHONY: install

build:
	cargo build
	$(MAKE) -C python/hello/binding post-build
.PHONY: build

test:
	cargo test
.PHONY: test
