start:
    cargo run

dev:
    cargo run --features dev

lint:
    cargo check
    cargo fmt --all
    cargo clippy --all-targets --all-features

build-install: lint
    cargo build --release
    cp target/release/metro5 ~/.cargo/bin/

