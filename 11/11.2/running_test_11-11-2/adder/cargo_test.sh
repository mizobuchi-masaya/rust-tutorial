#1/bin/sh

cargo clean
cargo test
cargo test -- --ignored
cargo test -- --include-ignored
exit 0
