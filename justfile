default: build test run

build:
	cargo build
	tailwindcss -i input.css -o assets/style.css

test:
	cargo test

run: 
	cargo run
