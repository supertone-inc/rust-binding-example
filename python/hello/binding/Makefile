default_target:

SOURCE_FILE_NAME := libhello.dylib

build:
	poetry run cargo build
	$(MAKE) post-build
.PHONY: build

post-build:
	cp ../../../target/debug/$(SOURCE_FILE_NAME) ../hello/hello.so
.PHONY: build
