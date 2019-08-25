cargo check --all --benches --bins --examples --tests
cargo fmt --all -- --check
cargo clippy -- -D clippy::all
cargo test --all  --verbose
cargo build --verbose
