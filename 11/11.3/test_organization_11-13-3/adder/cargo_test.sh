#!/bin/sh

cargo clean
cargo test
cargo test --test integration_test

exit 0
