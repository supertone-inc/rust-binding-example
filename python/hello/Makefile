default_target:

SOURCE_FILE_NAME := libhello.dylib

install:
	poetry install
.PHONY: install

build:
	poetry run cargo build
	$(MAKE) post-build
.PHONY: build

post-build:
	cp ../../target/debug/$(SOURCE_FILE_NAME) hello/hello.so
.PHONY: build

test:
	poetry run pytest -s
.PHONY: test
