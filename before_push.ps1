# Sanity checks before commit to repo. Mimic CI checks
cargo test --all
cargo fmt --all -- --check
cargo clippy -- -D warnings