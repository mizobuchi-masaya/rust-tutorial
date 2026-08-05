#!/bin/sh

cargo clean
cargo build
cargo run
cargo check

exit 0
