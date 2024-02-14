dev:
	cargo build --features DEBUG

release:
	cargo build --release --features RELEASE

run:
	cargo build --features DEBUG
	./target/debug/moover_rust

run_release:
	./target/release/moover_rust