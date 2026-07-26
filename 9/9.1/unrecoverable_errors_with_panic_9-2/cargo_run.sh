#!/bin/sh

cargo clean
RUST_BACKTRACE=1 cargo run
exit 0
