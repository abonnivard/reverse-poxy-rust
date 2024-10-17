all: build

build:
	cargo build

run: build
	cargo


clean:
	cargo clean

.PHONY: all build run test clean
