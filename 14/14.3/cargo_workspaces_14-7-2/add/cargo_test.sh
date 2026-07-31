#!/bin/sh

cargo clean
cargo build
cargo test
cargo test -p add_one
cargo run
cargo run -p adder

exit 0
