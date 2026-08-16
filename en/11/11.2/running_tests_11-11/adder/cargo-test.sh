#!/bin/sh

cargo clean
cargo test
cargo test one_hundred
cargo test add

exit 0
