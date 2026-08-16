#!/bin/sh

cargo clean
cargo test
cargo test --test integration_test -- --show-output
#cargo test -- --show-output

exit 0
