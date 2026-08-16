#!/bin/sh

cargo clean
cargo test
cargo test -- --show-output

exit 0
