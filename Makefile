example-add: FORCE
	cargo run --example add

example-if: FORCE
	cargo run --example if

example-custom-word: FORCE
	cargo run --example custom_word

example-print-documentation: FORCE
	cargo run --example print_documentation

example-custom-builtin: FORCE
	cargo run --example custom_builtin

example-repl: FORCE
	cargo run --example repl

example-loop: FORCE
	cargo run --example loop

test: FORCE
	cargo test

test-watch: FORCE
	cargo watch -x test

installs: FORCE
	cargo binstall cargo-watch

docs: FORCE
	cargo doc --no-deps --open

FORCE:

