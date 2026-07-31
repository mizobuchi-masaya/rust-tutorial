#!/bin/sh

cargo clean
cargo build
cargo run -p adder

exit 0
