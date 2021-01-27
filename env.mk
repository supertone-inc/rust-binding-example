ROOT_DIR := $(realpath $(dir $(abspath $(lastword $(MAKEFILE_LIST)))))

PATH := $(ROOT_DIR)/bin:$(PATH)
PATH := $(ROOT_DIR)/python/hello/.venv/bin:$(PATH)
