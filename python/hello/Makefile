default_target:

include ../../env.mk

install:
	poetry install
.PHONY: install

build:
	cargo post build --package hello-python
.PHONY: build

test:
	poetry run pytest -s
.PHONY: test
