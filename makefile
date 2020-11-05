.PHONY: run

run:
	cargo run

build: src/*
	cargo build

doc: src/*
	cargo doc

doccopy: doc
	cp -rf target/doc/ docs/documentation

test: src/*
	cargo test

clean:
	rm -rf target/*