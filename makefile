.PHONY: run

run:
	cargo run

build: src/*
	cargo build

install:
	echo "Installed"

doc: src/*
	cargo doc

doccopy: doc
	cp -rf target/doc/ docs/documentation

test: src/*
	cargo test

clean:
	rm -rf target/*