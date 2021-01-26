default_target:

install:
	$(MAKE) -C python/hello install
.PHONY: install

build:
	$(MAKE) -C python/hello build
.PHONY: build

test:
	$(MAKE) -C rust/hello test
	$(MAKE) -C python/hello test
.PHONY: test
