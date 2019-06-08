.PHONY: build test test@script
build:
	cargo build
test:
	cargo test
test@script : build
	./target/debug/baik examples/baik.ina