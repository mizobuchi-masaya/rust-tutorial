#!/bin/sh

cargo clean
cargo test
cargo test -- --ignored

exit 0
