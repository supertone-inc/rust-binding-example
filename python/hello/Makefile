default_target:

install:
	poetry install
.PHONY: install

build:
	$(MAKE) -C binding build
.PHONY: build

test:
	poetry run pytest -s
.PHONY: test
