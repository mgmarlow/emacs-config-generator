install:
	cargo install cargo-watch

clean:
	cargo clean

run:
	cargo run

build:
	cargo build

test:
	cargo test

watch:
	cargo watch -x run

deploy:
	fly deploy
