#!/bin/sh

cargo clean
cargo run
cargo run -- needle haystack

exit 0
